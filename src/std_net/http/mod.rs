use crate::prelude::*;
use mluau::prelude::*;

mod timeout_info;
use timeout_info::TimeoutInfo;

mod http_request;
use http_request::HttpRequest;

mod http_response;
use http_response::HttpResponse;

mod http_error;
use http_error::HttpError;

use ureq::Error as UreqError;
use ureq::http::Method;

type ResponseWithBody = ureq::http::Response<ureq::Body>;

pub fn http_get(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "http.get(options: HttpRequestWithoutBody | string)";

    let config = match value {
        LuaValue::Table(t) => t,
        LuaValue::String(s) => {
            // make a dummy table for backwards compat
            TableBuilder::create(luau)?
                .with_value("url", LuaValue::String(s))?
                .build()?
        },
        other => {
            return wrap_err!("{}: expected options to be a HttpRequestWithoutBody table or string (url), got: {:?}", function_name, other);
        }
    };

    let request = HttpRequest::from_config(Method::GET, config, luau, function_name)?;
    let response = request.send(function_name)?;

    ok_table(response.into_table(luau, function_name))
}

fn http_post(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "http.post(options: HttpRequestWithBody)";

    let config = match value {
        LuaValue::Table(t) => t,
        other => {
            return wrap_err!("{}: expected options to be an HttpRequestWithBody table, got: {:?}", function_name, other);
        }
    };

    let request = HttpRequest::from_config(Method::POST, config, luau, function_name)?;
    let response = request.send(function_name)?;

    ok_table(response.into_table(luau, function_name))
}

pub enum HttpResponseResult {
    Ok(HttpResponse),
    Timeout(ureq::Timeout, Option<TimeoutInfo>),
    Err(UreqError),
}

pub fn http_request(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
    let function_name = "http.request(method: HttpMethod, options: HttpRequestWithoutBody | HttpRequestWithBody)";

    let http_method = match multivalue.pop_front() {
        Some(LuaValue::String(method)) => {
            match Method::from_bytes(&method.as_bytes()) {
                Ok(method) => method,
                Err(err) => {
                    return wrap_err!("{}: invalid http method: {}", function_name, err);
                }
            }
        },
        Some(LuaNil) | None => {
            return wrap_err!("{} was incorrectly called without an http method", function_name);
        },
        Some(other) => {
            return wrap_err!("{} expected its first argument to be an http method (string), got: {:?}", function_name, other);
        }
    };

    let config = match multivalue.pop_front() {
        Some(LuaValue::Table(t)) => t,
        Some(LuaNil) | None => {
            return wrap_err!("{} called without required argument 'options' (expected table, got nothing or nil)", function_name);
        },
        Some(other) => {
            return wrap_err!("{}: expected options to be an HttpRequestWithBody table, got: {:?}", function_name, other);
        }
    };

    let request = HttpRequest::from_config(http_method, config, luau, function_name)?;
    // we want to throw Luau errors but match on Ureq/Timeout errors and return them instead of throwing
    let result = request.try_send(function_name)?; 

    match result {
        HttpResponseResult::Ok(mut response) => {
            if let Err(err) = response.read_body_into_self() {
                return ok_table(HttpError::from_error(err, None).into_table(luau))
            }

            ok_table(response.into_table(luau, function_name))
        },
        HttpResponseResult::Timeout(which, info) => {
            ok_table(HttpError::from_timeout(which, info).into_table(luau))
        },
        HttpResponseResult::Err(err) => {
            ok_table(HttpError::from_error(err, None).into_table(luau))
        }
    }
}

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_function("get", http_get)?
        .with_function("post", http_post)?
        .with_function("request", http_request)?
        .build_readonly()
}