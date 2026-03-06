use crate::prelude::*;
use mluau::prelude::*;

mod timeout_info;
use timeout_info::TimeoutInfo;

mod sender;
use sender::Sender;

use ureq::Error as UreqError;
use ureq::http::Method;

use crate::std_json;

type ResponseResult = LuaResult<ureq::http::Response<ureq::Body>>;
type RequestBuilderWithBody = ureq::RequestBuilder<ureq::typestate::WithBody>;

type ResponseWithBody = ureq::http::Response<ureq::Body>;
type UreqResponseResult = Result<ResponseWithBody, UreqError>;

/// A request's body *could* be a valid string (either json or regular text) 
/// or it could be invalid utf8 (octet-stream) which means we have to treat
/// it differently on the Rust side.
/// 
/// This stupid little enum abstracts it so we know when to put json headers
/// on json content (user passed a table to RequestOptions.body) and know to
/// handle binary differently from text/plain.
enum RequestBody {
    Json(String),
    Bytes(Vec<u8>),
    Text(String),
}

struct HttpRequest;
impl HttpRequest {
    fn try_send(method: Method, config: LuaTable, luau: &Lua, function_name: &'static str) -> LuaResult<UreqResponseResult> {
        let url = match config.raw_get("url")? {
            LuaValue::String(s) => {
                Self::check_valid_utf8(s, "RequestOptions.url", function_name)?
            },
            LuaNil => {
                return wrap_err!("{}: RequestOptions missing field url: expected string, got nil", function_name);
            },
            other => {
                return wrap_err!("{}: expected RequestOptions.url to be a string, got: {:?}", function_name, other);
            }
        };

        let headers = match config.raw_get("headers")? {
            LuaValue::Table(t) => Some(Self::map_to_vec(t, "headers", function_name)?),
            LuaNil => None,
            other => {
                return wrap_err!("{}: expected RequestOptions.headers to be {{ [string]: string }} or nil, got: {:?}", function_name, other);
            }
        };

        let params = match config.raw_get("params")? {
            LuaValue::Table(t) => Some(Self::map_to_vec(t, "params", function_name)?),
            LuaNil => None,
            other => {
                return wrap_err!("{}: expected RequestOptions.params to be {{ [string]: string }} or nil, got: {:?}", function_name, other);
            }
        };

        let body = match config.raw_get("body")? {
            LuaValue::String(s) => {
                match s.to_str() {
                    Ok(s) => Some(RequestBody::Text(s.to_string())),
                    Err(err) => {
                        return wrap_err!("{}: request body is not valid utf-8, pass a buffer to send binary data (err: {})", function_name, err);
                    }
                }
            },
            LuaValue::Table(body) => {
                match std_json::json_raw_encode(luau, LuaValue::Table(body)) {
                    Ok(body) => Some(RequestBody::Json(body)),
                    Err(err) => {
                        return wrap_err!("{}: unable to encode body table to json due to err: {}", function_name, err);
                    }
                }
            },
            LuaValue::Buffer(bytes) => {
                Some(RequestBody::Bytes(bytes.to_vec()))
            },
            LuaNil => None,
            other => {
                return wrap_err!("{}: expected request body to be a string, table (to be converted to json), or buffer (to be converted to octet-stream), got: {:?}", function_name, other);
            }
        };

        let timeout = TimeoutInfo::from_config(config, function_name)?;
        
        let result = Sender::from_http_method(method, url, function_name)?
            .configure(timeout)
            .send(headers, params, body);

        Ok(result)
    }

    fn send(method: Method, config: LuaTable, luau: &Lua, function_name: &'static str) -> ResponseResult {
        // all the luau errors (api usage errors) should be propagated upwards
        // whereas all the ureq errors we can now match against
        let result = Self::try_send(method, config, luau, function_name)?;
        
        match result {
            Ok(response) => {
                Ok(response)
            },
            Err(err) => {
                wrap_err!("{}: error sending request: {}", function_name, err)
            }
        }
    }

    fn check_valid_utf8(s: LuaString, what: &'static str, function_name: &'static str) -> LuaResult<String> {
        match s.to_str() {
            Ok(s) => Ok(s.to_string()),
            Err(err) => {
                wrap_err!("{}: {} must be valid utf-8: {}", function_name, what, err)
            }
        }
    }

    fn map_to_vec(map: LuaTable, key: &'static str, function_name: &'static str) -> LuaResult<Vec<(String, String)>> {
        let mut res = Vec::new();
        for pair in map.pairs::<LuaValue, LuaValue>() {
            match pair? {
                (LuaValue::String(current_key), LuaValue::String(value)) => {
                    let checked_key = Self::check_valid_utf8(current_key, "params/headers key", function_name)?;
                    let checked_value = Self::check_valid_utf8(value, "params/headers value", function_name)?;

                    res.push((checked_key, checked_value));
                },
                (other_key, other_val) => {
                    return wrap_err!(
                        "{}: {} map contains invalid key/value types, expected [string] = string, got: [{:?}] = {:?}",
                        function_name, key, other_key, other_val
                    );
                }
            }
        };
        Ok(res)
    }
}

struct HttpResponse;
impl HttpResponse {
    fn expect_json(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
        let function_name = "HttpResponse:expect_json()";

        let self_table = match multivalue.pop_front() {
            Some(LuaValue::Table(t)) => t,
            Some(LuaNil) | None => {
                return wrap_err!("{} incorrectly called without self");
            },
            Some(other) => {
                return wrap_err!("{}: expected self to be a HttpResponse table, got: {:?}", function_name, other);
            }
        };

        let mime_type = self_table.raw_get::<Option<String>>("mime_type")?;
        if let Some(mime) = mime_type && &mime != "application/json" {
            return wrap_err!("{}: expected response to be json, but response's mime type is actually {}", function_name, mime);
        }

        // let body = self_table.raw_get::<("body")?
        let body = match self_table.raw_get("body")? {
            LuaValue::String(body) => {
                match body.to_str() {
                    Ok(s) => s.to_string(),
                    Err(err) => {
                        return wrap_err!("{}: response body must be valid utf-8 to decode to json: {}", function_name, err);
                    }
                }
            },
            _ => {
                return wrap_err!("{}: response.body is not a string; you passed in the wrong table", function_name);
            }
        };

        std_json::json_decode(luau, body)
    }

    fn create_table(response: &mut ResponseWithBody, luau: &Lua, function_name: &'static str) -> LuaResult<LuaTable> {
        let is_ok = response.status().is_success();
        
        let headers = luau.create_table_from(response.headers().iter().map(|(name, val)| {
            (name.to_string(), LuaValue::String(luau.create_string(val.as_bytes()).expect("can't make string?")))
        }))?;

        let code = response.status().as_u16();
        let reason = response.status().canonical_reason().unwrap_or_default();

        let status = TableBuilder::create(luau)?
            .with_value("code", code)?
            .with_value("reason", reason)?
            .build_readonly()?;

        let body = response.body_mut();
        let body = match body.read_to_vec() {
            Ok(body) => body,
            Err(err) => {
                return wrap_err!("{}: cannot create response due to error reading response body: {}", function_name, err);
            }
        };
        
        let charset = response.body().charset();
        let mime_type = response.body().mime_type();

        TableBuilder::create(luau)?
            .with_value("ok", is_ok)?
            .with_value("headers", headers)?
            .with_value("status", status)?
            .with_value("body", luau.create_string(body)?)?
            .with_value("charset", charset)?
            .with_value("mime_type", mime_type)?
            .with_function("expect_json", HttpResponse::expect_json)?
            .build_readonly()
    }
}

pub fn http_request(_luau: &Lua, _value: LuaValue) -> LuaValueResult {
    todo!()
}

pub fn http_get(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "http.get(options: RequestOptions | string)";

    let config = match value {
        LuaValue::Table(t) => t,
        LuaValue::String(s) => {
            // make a dummy table for backwards compat
            TableBuilder::create(luau)?
                .with_value("url", LuaValue::String(s))?
                .build()?
        },
        other => {
            return wrap_err!("{}: expected options to be a RequestOptions table or string (url), got: {:?}", function_name, other);
        }
    };

    let mut response = HttpRequest::send(Method::GET, config, luau, function_name)?;
    ok_table(HttpResponse::create_table(&mut response, luau, function_name))
}

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_function("get", http_get)?
        .build_readonly()
}