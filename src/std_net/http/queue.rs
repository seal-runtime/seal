use crate::prelude::*;
use mluau::prelude::*;

use std::borrow::Cow;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use crossbeam_channel::{Receiver, Sender};
use ureq::http::Method;

use super::{HttpError, HttpRequest, HttpResponse, HttpResponseResult};
use super::http_request::new_agent;

const DEFAULT_WORKERS: usize = 256;

/// A request waiting for a worker. Workers skip it when `cancelled` is set.
struct Job {
    id: u64,
    cancelled: Arc<AtomicBool>,
    request: HttpRequest,
}

/// A request that hasn't been iterated yet. `HttpQueue:cancel` sets `cancelled`, which the request's
/// `Job` shares.
struct Pending {
    key: LuaValue,
    cancelled: Arc<AtomicBool>,
}

/// A finished request with its response body already read.
struct Done {
    id: u64,
    outcome: Outcome,
}

enum Outcome {
    Response(HttpResponse),
    Error(HttpError),
}

impl Outcome {
    fn into_table(self, luau: &Lua, function_name: &'static str) -> LuaResult<LuaTable> {
        match self {
            Outcome::Response(response) => response.into_table(luau, function_name),
            Outcome::Error(err) => err.into_table(luau),
        }
    }
}

/// Sends `request` through `agent` and reads its body like `http.request` does.
fn complete(agent: &ureq::Agent, request: HttpRequest) -> Outcome {
    match request.send_with(agent) {
        HttpResponseResult::Ok(mut response) => match response.read_body_into_self() {
            Ok(()) => Outcome::Response(response),
            Err(err) => Outcome::Error(HttpError::from_error(err, None)),
        },
        HttpResponseResult::Timeout(which, info) => Outcome::Error(HttpError::from_timeout(which, info)),
        HttpResponseResult::Err(err) => Outcome::Error(HttpError::from_error(err, None)),
    }
}

/// Aborts the process when a worker thread panics. Unwinding would only end the worker, and iterating the
/// queue would then wait forever for the result it never sends. Our panic hook has already
/// reported the panic by the time this runs.
struct AbortOnPanic;

impl Drop for AbortOnPanic {
    fn drop(&mut self) {
        if std::thread::panicking() {
            std::process::abort();
        }
    }
}

/// Runs on a worker thread until the queue is dropped (which disconnects `jobs`).
fn work(agent: ureq::Agent, jobs: Receiver<Job>, results: Sender<Done>) {
    let _abort_on_panic = AbortOnPanic;
    while let Ok(Job { id, cancelled, request }) = jobs.recv() {
        // Relaxed is enough: if we miss a cancel, the request gets sent and `next` drops its result
        if cancelled.load(Ordering::Relaxed) {
            continue;
        }

        let outcome = complete(&agent, request);
        if results.send(Done { id, outcome }).is_err() {
            break;
        }
    }
}

struct QueueState {
    agent: ureq::Agent,
    max_workers: usize,
    workers: Cell<usize>,
    next_id: Cell<u64>,
    /// Every request that hasn't been iterated or cancelled, by request id.
    pending: RefCell<HashMap<u64, Pending>>,
    job_sender: Sender<Job>,
    job_receiver: Receiver<Job>,
    result_sender: Sender<Done>,
    result_receiver: Receiver<Done>,
}

impl QueueState {
    /// Starts another worker when there are more unfinished requests than workers.
    fn spawn_worker_if_needed(&self) -> LuaResult<()> {
        let workers = self.workers.get();
        if workers >= self.max_workers || self.pending.borrow().len() <= workers {
            return Ok(());
        }

        let agent = self.agent.clone();
        let jobs = self.job_receiver.clone();
        let results = self.result_sender.clone();
        let spawned = std::thread::Builder::new()
            .name(format!("seal-http-queue-{}", workers + 1))
            .spawn(move || work(agent, jobs, results));

        match spawned {
            Ok(_) => {
                self.workers.set(workers + 1);
                Ok(())
            },
            // the request still gets sent once an existing worker frees up
            Err(_) if workers > 0 => Ok(()),
            Err(err) => wrap_err!("HttpQueue: unable to start a worker thread: {}", err),
        }
    }

    fn add(&self, request: HttpRequest, key: Option<LuaValue>) -> LuaResult<LuaValue> {
        let id = self.next_id.get() + 1;
        self.next_id.set(id);

        let key = key.unwrap_or(LuaValue::Integer(id as LuaInteger));
        let cancelled = Arc::new(AtomicBool::new(false));
        self.pending.borrow_mut().insert(id, Pending {
            key: key.clone(),
            cancelled: Arc::clone(&cancelled),
        });
        self.spawn_worker_if_needed()?;

        let job = Job {
            id,
            cancelled,
            request,
        };
        if self.job_sender.send(job).is_err() {
            self.pending.borrow_mut().remove(&id);
            return wrap_err!("HttpQueue: unable to add request because the queue's workers are gone");
        }

        Ok(key)
    }

    /// Blocks until a request finishes and gives its key and result.
    /// Gives `nil` when no requests are left.
    fn next(&self, luau: &Lua) -> LuaResult<LuaMultiValue> {
        let function_name = "HttpQueue:__iter()";
        loop {
            if self.pending.borrow().is_empty() {
                return Ok(LuaMultiValue::from_vec(vec![LuaNil]));
            }

            // recv can't fail while the queue holds a result sender
            let Ok(Done { id, outcome }) = self.result_receiver.recv() else {
                return wrap_err!("{}: lost connection to the queue's workers", function_name);
            };

            // results for requests cancelled while in flight are skipped
            let Some(Pending { key, .. }) = self.pending.borrow_mut().remove(&id) else {
                continue;
            };

            return Ok(LuaMultiValue::from_vec(vec![
                key,
                LuaValue::Table(outcome.into_table(luau, function_name)?),
            ]));
        }
    }
}

pub struct HttpQueue {
    state: Rc<QueueState>,
}

impl HttpQueue {
    fn new(max_workers: usize) -> Self {
        let (job_sender, job_receiver) = crossbeam_channel::unbounded();
        let (result_sender, result_receiver) = crossbeam_channel::unbounded();
        Self {
            state: Rc::new(QueueState {
                agent: new_agent(Some(max_workers)),
                max_workers,
                workers: Cell::new(0),
                next_id: Cell::new(0),
                pending: RefCell::new(HashMap::new()),
                job_sender,
                job_receiver,
                result_sender,
                result_receiver,
            }),
        }
    }

    fn get(luau: &Lua, this: &HttpQueue, mut multivalue: LuaMultiValue) -> LuaValueResult {
        let function_name = "HttpQueue:get(options: HttpRequestWithoutBody | string, key: unknown?)";
        let config = match multivalue.pop_front() {
            Some(LuaValue::Table(t)) => t,
            Some(LuaValue::String(url)) => {
                TableBuilder::create(luau)?
                    .with_value("url", LuaValue::String(url))?
                    .build()?
            },
            Some(LuaNil) | None => {
                return wrap_err!("{} called without required argument 'options'", function_name);
            },
            Some(other) => {
                return wrap_err!("{}: expected options to be a HttpRequestWithoutBody table or string (url), got: {:?}", function_name, other);
            }
        };

        let request = HttpRequest::from_config(Method::GET, config, luau, function_name)?;
        this.state.add(request, key_from(multivalue.pop_front()))
    }

    fn post(luau: &Lua, this: &HttpQueue, mut multivalue: LuaMultiValue) -> LuaValueResult {
        let function_name = "HttpQueue:post(options: HttpRequestWithBody, key: unknown?)";
        let config = match multivalue.pop_front() {
            Some(LuaValue::Table(t)) => t,
            Some(LuaNil) | None => {
                return wrap_err!("{} called without required argument 'options'", function_name);
            },
            Some(other) => {
                return wrap_err!("{}: expected options to be an HttpRequestWithBody table, got: {:?}", function_name, other);
            }
        };

        let request = HttpRequest::from_config(Method::POST, config, luau, function_name)?;
        this.state.add(request, key_from(multivalue.pop_front()))
    }

    fn request(luau: &Lua, this: &HttpQueue, mut multivalue: LuaMultiValue) -> LuaValueResult {
        let function_name = "HttpQueue:request(method: HttpMethod, options: HttpRequestWithoutBody | HttpRequestWithBody, key: unknown?)";
        let method = match multivalue.pop_front() {
            Some(LuaValue::String(method)) => match Method::from_bytes(&method.as_bytes()) {
                Ok(method) => method,
                Err(err) => {
                    return wrap_err!("{}: invalid http method: {}", function_name, err);
                }
            },
            Some(LuaNil) | None => {
                return wrap_err!("{} called without required argument 'method'", function_name);
            },
            Some(other) => {
                return wrap_err!("{}: expected method to be an http method (string), got: {:?}", function_name, other);
            }
        };

        let config = match multivalue.pop_front() {
            Some(LuaValue::Table(t)) => t,
            Some(LuaNil) | None => {
                return wrap_err!("{} called without required argument 'options'", function_name);
            },
            Some(other) => {
                return wrap_err!("{}: expected options to be an HttpRequestWithoutBody or HttpRequestWithBody table, got: {:?}", function_name, other);
            }
        };

        let request = HttpRequest::from_config(method, config, luau, function_name)?;
        this.state.add(request, key_from(multivalue.pop_front()))
    }

    // when 0 arguments are passed, cancels all pending request
    // when a key is passed, cancels any requests matching the passed key
    fn cancel(_luau: &Lua, this: &HttpQueue, value: LuaValue) -> LuaResult<()> {
        let cancel_all = value.is_nil() || value.is_none();
        this.state.pending.borrow_mut().retain(|_, pending| {
            if cancel_all || pending.key == value {
                pending.cancelled.store(true, Ordering::Relaxed);
                false
            } else {
                true
            }
        });
        Ok(())
    }

    fn len(_luau: &Lua, this: &HttpQueue, _: LuaValue) -> LuaResult<LuaInteger> {
        Ok(this.state.pending.borrow().len() as LuaInteger)
    }

    fn iter(luau: &Lua, this: &HttpQueue, _: LuaValue) -> LuaValueResult {
        let state = Rc::clone(&this.state);
        let iterator = luau.create_function(move |luau, _: LuaMultiValue| state.next(luau))?;
        iterator.into_lua(luau)
    }
}

fn key_from(value: Option<LuaValue>) -> Option<LuaValue> {
    match value {
        Some(LuaNil) | None => None,
        Some(key) => Some(key),
    }
}

impl Borrowable for HttpQueue {
    fn display_name() -> &'static str {
        "HttpQueue"
    }
}

impl WrappedUserData for HttpQueue {
    fn type_name<'a>() -> Cow<'a, str> {
        Cow::Borrowed("HttpQueue")
    }
    fn add_fields<F: WrappedUserDataFields<Self>>(fields: &mut F) {
        fields.add_meta_field("__type", "HttpQueue");
    }
    fn add_methods<M: WrappedUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("get", HttpQueue::get);
        methods.add_method("post", HttpQueue::post);
        methods.add_method("request", HttpQueue::request);
        methods.add_method("cancel", HttpQueue::cancel);
        methods.add_meta_method("__len", HttpQueue::len);
        methods.add_meta_method("__iter", HttpQueue::iter);
    }
}

pub fn http_queue(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "http.queue(workers: number?)";
    let workers = match value {
        LuaNil => DEFAULT_WORKERS,
        LuaValue::Integer(i) if i >= 1 => i as usize,
        LuaValue::Number(f) if f.fract() == 0.0 && f >= 1.0 => f as usize,
        LuaValue::Integer(_) | LuaValue::Number(_) => {
            return wrap_err!("{}: expected workers to be a whole number of at least 1", function_name);
        },
        other => {
            return wrap_err!("{}: expected workers to be a number or nil, got: {:?}", function_name, other);
        }
    };

    HttpQueue::new(workers).into_userdata(luau)
}
