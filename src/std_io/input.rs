use mluau::prelude::*;
use crate::prelude::*;

use crate::globals::warn;
use crate::std_err::WrappedError;
use crate::std_terminal::events::Interrupt;
use rustyline::error::ReadlineError;

use atty::Stream::{Stdout, Stderr};

use std::io::{self, BufRead, Read, Write};
use std::sync::OnceLock;
use std::time::{Duration, Instant};

use crate::std_fs::file_size::FileSize;
use crate::std_time::duration::TimeDuration;

#[derive(Debug)]
pub struct ExpectSaneOutputStream {
    inner: OnceLock<(bool, bool)>,
}
impl ExpectSaneOutputStream {
    pub const fn uninitialized() -> Self {
        Self {
            inner: OnceLock::new()
        }
    }
    pub fn initialize_and_check(&self) {
        let stdout_sane = atty::is(Stdout);
        let stderr_sane = atty::is(Stderr);

        self.inner.set((stdout_sane, stderr_sane)).expect("ExpectSaneOutputStream already initialized smhmh");
    }
    pub fn stdout(&self) -> bool {
        self.inner.get().expect("ExpectSaneOutputStream not initialized").0
    }
    pub fn stderr(&self) -> bool {
        self.inner.get().expect("ExpectSaneOutputStream not initialized").1
    }
}

pub static EXPECT_OUTPUT_STREAMS: ExpectSaneOutputStream = ExpectSaneOutputStream::uninitialized();

fn input_get(luau: &Lua, raw_prompt: Option<String>) -> LuaValueResult {
    if let Some(prompt) = raw_prompt {
        put!("{}", prompt)?;
        match io::stdout().flush() {
            Ok(_) => {},
            Err(err) => {
                return wrap_err!("input.get: unable to flush stdout due to err: {}", err);
            }
        }
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim_end().to_string();

    Ok(LuaValue::String(luau.create_string(&input)?))
}

pub(super) fn get_line_bytes_from_stdin(prompt: Option<String>) -> LuaResult<Vec<u8>> {
    if let Some(prompt) = prompt {
        put!("{}", prompt)?;
        io::stdout().flush()?;
    }

    let mut buf = Vec::new();
    io::stdin().lock().read_until(b'\n', &mut buf)?;

    if buf.last() == Some(&b'\n') {
        buf.pop();
        if buf.last() == Some(&b'\r') {
            buf.pop();
        }
    }

    Ok(buf)
}

pub fn input_rawline(luau: &Lua, raw_prompt: Option<String>) -> LuaValueResult {
    let bytes = get_line_bytes_from_stdin(raw_prompt)?;
    ok_string(bytes, luau)
}

pub fn input_readline(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
    let function_name = "input.readline(prompt: string)";
    let prompt = match multivalue.pop_front() {
        Some(LuaValue::String(s)) => s.to_string_lossy(),
        Some(LuaNil) | None => {
            return wrap_err!("{} expected message to be a string, got nothing or nil", function_name);
        },
        Some(other) => {
            return wrap_err!("{} expected message to be a string, got: {:?}", function_name, other);
        }
    };

    if atty::isnt(atty::Stream::Stdin) || atty::isnt(atty::Stream::Stdout) {
        match get_line_bytes_from_stdin(if prompt.is_empty() { None } else { Some(prompt) }) {
            Ok(bytes) => {
                return ok_string(bytes, luau);
            },
            Err(err) => {
                return wrap_err!("{}: unable to fallback to non-tty due to err: {}", function_name, err);
            }
        }
    }

    let mut rl = match rustyline::DefaultEditor::new() {
        Ok(editor) => editor,
        Err(err) => {
            return wrap_err!("{}: unable to make rustyline DefaultEditor due to ReadlineError: {}", function_name, err);
        }
    };

    let line = match rl.readline(&prompt) {
        Ok(line) => {
            if let Err(err) = rl.add_history_entry(line.as_str()) {
                warn(luau, ok_string(format!("error adding prompt history: {}", err), luau)?)?;
            }
            line
        },
        Err(ReadlineError::Interrupted) => {
            return Interrupt::ctrlc().get_userdata(luau);
        },
        Err(ReadlineError::Eof) => {
            return Interrupt::ctrld().get_userdata(luau);
        },
        Err(ReadlineError::Io(err)) => {
            return WrappedError::from_message(format!("terminal io error: {}", err)).get_userdata(luau);
        }
        Err(err) => {
            return wrap_err!("{}: encountered unexpected ReadlineError: {}", function_name, err);
        }
    };

    ok_string(line, luau)
}

pub fn input_editline(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
    let function_name = "input.editline(prompt: string, left: string, right: string?)";

    let prompt = match multivalue.pop_front() {
        Some(LuaValue::String(s)) => s.to_string_lossy(),
        Some(LuaNil) | None => {
            return wrap_err!("{} expected message to be a string, got nothing or nil", function_name);
        },
        Some(other) => {
            return wrap_err!("{} expected message to be a string, got: {:?}", function_name, other);
        }
    };

    let left = match multivalue.pop_front() {
        Some(LuaValue::String(left)) => left.to_string_lossy(),
        Some(LuaNil) | None => {
            return wrap_err!("{} expected string to the left of cursor to be a string, got nothing or nil", function_name);
        },
        Some(other) => {
            return wrap_err!("{} expected string to the left of cursor to be a string, got: {:?}", function_name, other);
        }
    };

    let right = match multivalue.pop_front() {
        Some(LuaValue::String(right)) => Some(right.to_string_lossy()),
        Some(LuaNil) | None => None,
        Some(other) => {
            return wrap_err!("{} expected string to the right of cursor to be a string or nil/unspecified, got: {:?}", function_name, other);
        }
    };

    if atty::isnt(atty::Stream::Stdin) || atty::isnt(atty::Stream::Stdout) {
        // not an amazing solution because program will have to copy/read from stdout, gsub, and send left + response + right back
        let combined = left.clone() + "<CURSOR>" + &right.unwrap_or_default();
        puts!("{}", &combined)?;
        match get_line_bytes_from_stdin(Some(combined)) {
            Ok(bytes) => {
                return ok_string(bytes, luau);
            },
            Err(err) => {
                return wrap_err!("{}: unable to fallback to non-tty due to err: {}", function_name, err);
            }
        }
    }

    let mut rl = match rustyline::DefaultEditor::new() {
        Ok(editor) => editor,
        Err(err) => {
            return wrap_err!("{}: unable to make rustyline DefaultEditor due to ReadlineError: {}", function_name, err);
        }
    };

    let line = match rl.readline_with_initial(&prompt, (&left, &right.unwrap_or_default())) {
        Ok(line) => {
            if let Err(err) = rl.add_history_entry(line.as_str()) {
                warn(luau, ok_string(format!("error adding prompt history: {}", err), luau)?)?;
            }
            line
        },
        Err(ReadlineError::Interrupted) => {
            return Interrupt::ctrlc().get_userdata(luau);
        },
        Err(ReadlineError::Eof) => {
            return Interrupt::ctrld().get_userdata(luau);
        },
        Err(ReadlineError::Io(err)) => {
            return WrappedError::from_message(format!("terminal io error: {}", err)).get_userdata(luau);
        }
        Err(err) => {
            return wrap_err!("{}: encountered unexpected ReadlineError: {}", function_name, err);
        }
    };

    ok_string(line, luau)
}

/// Parsed options for `input.read`; see `InputReadOptions::from_value` for how they're pulled out
/// of the Luau `{ bytes: (number | FileSize)?, timeout: Duration? }` table.
struct InputReadOptions {
    /// Maximum number of bytes to read before returning; `None` means read until EOF.
    max_bytes: Option<u64>,
    /// How long to wait for input before returning; `None` means block indefinitely.
    timeout: Option<Duration>,
}

impl InputReadOptions {
    /// Reads the options table out of the Luau value passed to `input.read`. A missing/nil value
    /// means "no options" (read everything, block indefinitely).
    fn from_value(value: LuaValue, function_name: &'static str) -> LuaResult<Self> {
        match value {
            LuaNil => Ok(Self { max_bytes: None, timeout: None }),
            LuaValue::Table(options) => Ok(Self {
                max_bytes: Self::parse_bytes(options.raw_get("bytes")?, function_name)?,
                timeout: Self::parse_timeout(options.raw_get("timeout")?, function_name)?,
            }),
            other => {
                wrap_err!("{}: expected options to be a table or nil, got: {:?}", function_name, other)
            }
        }
    }

    /// Parses the `bytes` option (a plain number or a FileSize) into a max byte count.
    fn parse_bytes(value: LuaValue, function_name: &'static str) -> LuaResult<Option<u64>> {
        match value {
            LuaNil => Ok(None),
            LuaValue::Integer(i) => Ok(Some(int_to_u64(i, function_name, "options.bytes")?)),
            LuaValue::Number(f) => Ok(Some(float_to_u64(f, function_name, "options.bytes")?)),
            LuaValue::UserData(ud) if let Ok(file_size) = ud.borrow::<FileSize>() => {
                Ok(Some(file_size.as_bytes()))
            },
            LuaValue::UserData(ud) => {
                let type_name = ud.type_name()?.unwrap_or("userdata (missing __type metafield)".to_string());
                wrap_err!("{}: expected options.bytes to be a number or FileSize (from @std/fs/filesize), got a different kind of userdata: {}", function_name, type_name)
            },
            other => {
                wrap_err!("{}: expected options.bytes to be a number, FileSize, or nil, got: {:?}", function_name, other)
            }
        }
    }

    /// Parses the `timeout` option (a Duration userdata) into a std Duration.
    fn parse_timeout(value: LuaValue, function_name: &'static str) -> LuaResult<Option<Duration>> {
        match value {
            LuaNil => Ok(None),
            LuaValue::UserData(ud) if let Ok(duration) = ud.borrow::<TimeDuration>() => {
                let timeout = (*duration).inner; // SignedDuration is Copy, no drop worries
                if !timeout.is_positive() {
                    return wrap_err!("{}: options.timeout must be a positive Duration, got: {:#?}", function_name, timeout);
                }
                Ok(Some(timeout.unsigned_abs()))
            },
            LuaValue::UserData(ud) => {
                let type_name = ud.type_name()?.unwrap_or("userdata (missing __type metafield)".to_string());
                wrap_err!("{}: expected options.timeout to be a Duration (from @std/time), got a different kind of userdata: {}", function_name, type_name)
            },
            LuaValue::Number(_) | LuaValue::Integer(_) => {
                wrap_err!("{}: options.timeout should be a Duration (from @std/time), not a plain number", function_name)
            },
            other => {
                wrap_err!("{}: expected options.timeout to be a Duration or nil, got: {:?}", function_name, other)
            }
        }
    }
}

/// Blocking read with no timeout: consume stdin until EOF, or until `max_bytes` have been read.
/// Returns the bytes read plus whether reading stopped before EOF (i.e. because the byte limit was
/// reached and there may be more to read).
fn read_stdin_blocking(max_bytes: Option<u64>) -> io::Result<(Vec<u8>, bool)> {
    let stdin = io::stdin();
    let mut lock = stdin.lock();
    match max_bytes {
        None => {
            let mut buf: Vec<u8> = Vec::new();
            lock.read_to_end(&mut buf)?;
            Ok((buf, false))
        },
        Some(max) => {
            let mut buf: Vec<u8> = Vec::new();
            let mut chunk = [0u8; 8192];
            while (buf.len() as u64) < max {
                let remaining = (max - buf.len() as u64).min(chunk.len() as u64) as usize;
                let n = lock.read(&mut chunk[..remaining])?;
                if n == 0 {
                    return Ok((buf, false)); // EOF before hitting the limit
                }
                buf.extend_from_slice(&chunk[..n]);
            }
            Ok((buf, true)) // hit the byte limit with the stream possibly still open
        }
    }
}

/// Unix timeout path: use `poll(2)` to wait for input up to the deadline while reading straight
/// from stdin's file descriptor, so we never leave a blocked reader thread behind.
#[cfg(unix)]
fn read_stdin_until_deadline(max_bytes: Option<u64>, deadline: Instant) -> io::Result<(Vec<u8>, bool)> {
    use std::os::unix::io::AsRawFd;

    let fd = io::stdin().as_raw_fd();
    let mut buf: Vec<u8> = Vec::new();
    let mut chunk = [0u8; 8192];

    loop {
        let to_read = match max_bytes {
            Some(max) => {
                let already = buf.len() as u64;
                if already >= max {
                    return Ok((buf, true));
                }
                (max - already).min(chunk.len() as u64) as usize
            },
            None => chunk.len(),
        };

        let now = Instant::now();
        if now >= deadline {
            return Ok((buf, true)); // timed out with the stream still open
        }
        let remaining_ms = (deadline - now).as_millis().min(i32::MAX as u128) as i32;
        let mut pfd = libc::pollfd { fd, events: libc::POLLIN, revents: 0 };
        // SAFETY: `pfd` is a single valid, mutable pollfd we own for the duration of the call, and
        // we pass a matching count of 1; `fd` is stdin's descriptor, valid for as long as this
        // process owns stdin. poll only reads/writes through the pointer during the call.
        let ret = unsafe { libc::poll(&mut pfd, 1, remaining_ms) };
        if ret < 0 {
            let err = io::Error::last_os_error();
            if err.kind() == io::ErrorKind::Interrupted {
                continue; // interrupted by a signal, retry
            }
            return Err(err);
        }
        if ret == 0 {
            return Ok((buf, true)); // poll timed out
        }

        // SAFETY: `chunk` is a stack buffer of `chunk.len()` bytes and `to_read <= chunk.len()`
        // (clamped above), so the pointer is valid and writable for `to_read` bytes for the whole
        // call; `fd` is stdin's descriptor, valid for as long as this process owns stdin.
        let n = unsafe { libc::read(fd, chunk.as_mut_ptr() as *mut libc::c_void, to_read) };
        if n < 0 {
            let err = io::Error::last_os_error();
            if err.kind() == io::ErrorKind::Interrupted {
                continue;
            }
            return Err(err);
        }
        if n == 0 {
            return Ok((buf, false)); // EOF
        }
        buf.extend_from_slice(&chunk[..n as usize]);
    }
}

/// Non-unix timeout path: read stdin on a background thread and collect chunks with a deadline.
/// Mirrors the OSC-query reader in `std_terminal`; if the timeout elapses we return what we have
/// and leave the reader thread to finish on its own.
#[cfg(not(unix))]
fn read_stdin_until_deadline(max_bytes: Option<u64>, deadline: Instant) -> io::Result<(Vec<u8>, bool)> {
    use std::sync::mpsc::{self, RecvTimeoutError};

    enum ReadMsg {
        Chunk(Vec<u8>),
        Eof,
        LimitReached,
        Err(io::Error),
    }

    let (tx, rx) = mpsc::channel::<ReadMsg>();
    std::thread::spawn(move || {
        let stdin = io::stdin();
        let mut lock = stdin.lock();
        let mut chunk = [0u8; 8192];
        let mut total: u64 = 0;
        loop {
            let to_read = match max_bytes {
                Some(max) => {
                    if total >= max {
                        let _ = tx.send(ReadMsg::LimitReached);
                        return;
                    }
                    (max - total).min(chunk.len() as u64) as usize
                },
                None => chunk.len(),
            };
            match lock.read(&mut chunk[..to_read]) {
                Ok(0) => {
                    let _ = tx.send(ReadMsg::Eof);
                    return;
                },
                Ok(n) => {
                    total += n as u64;
                    if tx.send(ReadMsg::Chunk(chunk[..n].to_vec())).is_err() {
                        return; // receiver hung up (we timed out); stop reading
                    }
                },
                Err(err) => {
                    let _ = tx.send(ReadMsg::Err(err));
                    return;
                }
            }
        }
    });

    let mut buf: Vec<u8> = Vec::new();
    loop {
        let now = Instant::now();
        if now >= deadline {
            return Ok((buf, true));
        }
        match rx.recv_timeout(deadline - now) {
            Ok(ReadMsg::Chunk(c)) => buf.extend_from_slice(&c),
            Ok(ReadMsg::Eof) => return Ok((buf, false)),
            Ok(ReadMsg::LimitReached) => return Ok((buf, true)),
            Ok(ReadMsg::Err(err)) => return Err(err),
            Err(RecvTimeoutError::Timeout) => return Ok((buf, true)),
            Err(RecvTimeoutError::Disconnected) => return Ok((buf, false)),
        }
    }
}

/// Reads from stdin, stopping at EOF, at `max_bytes` (if set), or once `timeout` (if set) elapses.
/// Returns the bytes read plus whether reading stopped before EOF (there may be more to read).
fn read_from_stdin(max_bytes: Option<u64>, timeout: Option<Duration>) -> io::Result<(Vec<u8>, bool)> {
    match timeout {
        None => read_stdin_blocking(max_bytes),
        Some(timeout) => read_stdin_until_deadline(max_bytes, Instant::now() + timeout),
    }
}

fn input_read(luau: &Lua, options: LuaValue) -> LuaResult<(LuaValue, bool)> {
    let function_name = "input.read(options: { bytes: (number | FileSize)?, timeout: Duration? }?)";

    let options = InputReadOptions::from_value(options, function_name)?;

    let (buffy, more_to_read) = match read_from_stdin(options.max_bytes, options.timeout) {
        Ok(result) => result,
        Err(err) => {
            return wrap_err!("{}: unable to read from stdin: {}", function_name, err);
        }
    };

    let value = if buffy.is_empty() {
        LuaNil
    } else {
        LuaValue::String(luau.create_string(&buffy)?)
    };
    
    Ok((value, more_to_read))
}

fn input_interrupt(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "input.interrupt(code: \"CtrlC\" | \"CtrlD\")";
    match value {
        LuaValue::String(c) => {
            let bytes = c.as_bytes();
            if bytes == &b"CtrlC"[..] {
                Interrupt::ctrlc().get_userdata(luau)
            } else if bytes == &b"CtrlD"[..] {
                Interrupt::ctrld().get_userdata(luau)
            } else {
                wrap_err!("{} expected code to be \"CtrlC\" or \"CtrlD\", got some other string: {}", function_name, c.display())
            }
        },
        other => {
            wrap_err!("{} expected code to be \"CtrlC\" or \"CtrlD\", got: {:?}", function_name, other)
        }
    }
}

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_function_and_signature("get", input_get, c"io.input.get is deprecated, use io.prompt.text OR io.input.readline OR io.input.rawline instead")?
        .with_function_and_signature("readline", input_readline, signatures::STD_IO_INPUT_READLINE)?
        .with_function_and_signature("editline", input_editline, signatures::STD_IO_INPUT_EDITLINE)?
        .with_function_and_signature("rawline", input_rawline, signatures::STD_IO_INPUT_RAWLINE)?
        .with_function_and_signature("read", input_read, signatures::STD_IO_INPUT_READ)?
        .with_function_and_signature("interrupt", input_interrupt, signatures::STD_IO_INPUT_INTERRUPT)?
        .build_readonly()
}