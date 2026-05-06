
use mluau::prelude::*;
use crate::prelude::*;
use crate::std_json;

use ureq::http::Method;

mod sender;
use sender::Sender;

use super::TimeoutInfo;
use super::HttpResponse;
use super::HttpResponseResult;

use crate::std_fs::file_size::FileSize;

use ureq::Error as UreqError;

/// A request's body *could* be a valid string (either json or regular text) 
/// or it could be invalid utf8 (octet-stream) which means we have to treat
/// it differently on the Rust side.
/// 
/// This stupid little enum abstracts it so we know when to put json headers
/// on json content (user passed a table to RequestOptions.body) and know to
/// handle binary differently from text/plain.
pub enum RequestBody {
    Json(String),
    Bytes(Vec<u8>),
    Text(String),
}

pub struct HttpRequest {
    method: Method,
    url: String,
    headers: Option<Vec<(String, String)>>,
    params: Option<Vec<(String, String)>>,
    body: Option<RequestBody>,
    timeout: Option<TimeoutInfo>,
    max_body_size: Option<u64>,
    max_redirects: Option<u32>,
}

impl HttpRequest {
    pub fn from_config(method: Method, config: LuaTable, luau: &Lua, function_name: &'static str) -> LuaResult<Self> {
        let url = match config.raw_get("url")? {
            LuaValue::String(s) => {
                let url = Self::check_valid_utf8(s, "RequestOptions.url", function_name)?;
                if let Err(err) = url.parse::<ureq::http::Uri>() {
                    return wrap_err!("{}: invalid url {:?}: {}", function_name, url, err);
                }
                url
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

        let max_body_size = match config.raw_get("max_body_size")? {
            LuaValue::UserData(ud) if let Ok(file_size) = ud.borrow::<FileSize>() => {
                Some(file_size.as_bytes())
            },
            LuaValue::UserData(other) => {
                return wrap_err!("{}: expected config.max_body_size to be a FileSize userdata from std/fs/filesize, got an unexpected userdata instance: {:?}", function_name, other);
            },
            LuaValue::Number(_) | LuaValue::Integer(_) => {
                return wrap_err!("{}: expected config.max_body_size to be a FileSize (userdata) from std/fs/filesize, not a number of bytes");
            },
            LuaNil => None,
            other => {
                return wrap_err!("{}: expected config.max_body_size to be a FileSize from std/fs/filesize or nil, got: {:?}", function_name, other);
            }
        };

        let max_redirects = match config.raw_get("max_redirects")? {
            LuaValue::Number(f) => Some(float_to_u32(f, function_name, "max_redirects")?),
            LuaValue::Integer(i) => Some(int_to_u32(i, function_name, "max_redirects")?),
            LuaNil => None,
            other => {
                return wrap_err!("{}: expected config.max_redirects to be a number or nil, got: {:?}", function_name, other);
            }
        };

        let timeout = TimeoutInfo::from_config(config, function_name)?;

        Ok(Self {
            method,
            url,
            headers,
            params,
            body,
            timeout,
            max_body_size,
            max_redirects
        })
    }

    /// Send the request without throwing errors in the common expected cases of io/network errors.
    pub fn try_send(self, function_name: &'static str) -> LuaResult<HttpResponseResult>{
        let Self { 
            method,
            url,
            headers,
            params,
            body,
            timeout,
            max_body_size,
            max_redirects
        } = self;

        let result = Sender::from_http_method(method, url, function_name)?
            .configure(timeout.clone(), max_redirects)
            .send(headers, params, body);

        match result {
            Ok(response) => {
                Ok(HttpResponseResult::Ok(HttpResponse::new(response, max_body_size)))
            },
            Err(UreqError::Timeout(which)) => {
                Ok(HttpResponseResult::Timeout(which, timeout))
            },
            Err(err) => {
                Ok(HttpResponseResult::Err(err))
            }
        }
    }

    /// Send the request, throwing a Luau error if the request was unsuccessful. This is a convenience
    /// function for try_send that handles most of the cases; if you want to match against the actual 
    /// ureq errors directly, use try_send
    pub fn send(self, function_name: &'static str) -> LuaResult<HttpResponse> {
        // all the luau errors (api usage errors) should be propagated upwards
        // whereas all the ureq errors we can now match against

        match self.try_send(function_name)? {
            HttpResponseResult::Ok(mut response) => {
                if let Err(err) = response.read_body_into_self() {
                    return wrap_err!("{}: cannot create response due to error reading response body: {}", function_name, err);
                }
                Ok(response)
            },
            HttpResponseResult::Timeout(which, info) => {
                let error_message = info
                    .map(|t| t.describe_elapsed(which))
                    .unwrap_or_else(|| format!("{:?} timeout elapsed", which));
                wrap_err!("{}: {}", function_name, error_message)
            },
            HttpResponseResult::Err(UreqError::Timeout(_)) => unreachable!("should be handled above"),
            HttpResponseResult::Err(UreqError::Io(err)) => {
                // unfortunately i don't think there's a decent way to tell if this will work across all platforms
                if err.to_string().contains("Name or service not known") {
                    wrap_err!("{}: the url/uri you passed may not exist ({})", function_name, err)
                } else {
                    wrap_err!("{}: encountered an io error: {}", function_name, err)
                }
            },
            HttpResponseResult::Err(err) => {
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