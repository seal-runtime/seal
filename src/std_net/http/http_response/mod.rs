use mluau::prelude::*;
use crate::prelude::*;

use crate::std_json;

use super::ResponseWithBody;
use ureq::Error as UreqError;

enum IsJson {
    Yes,
    Maybe,
    No(String),
}

/// Wrapper around ureq's ResponseWithBody. 
/// You need to call `HttpResponse::read_body_into_self` first
/// otherwise the call to `HttpResponse::into_table` will panic.
/// 
/// We need this to keep track of the max configured body size 
/// which gets set in HttpRequest's options.
/// 
/// We also store a late initialized body_buffer in here
/// because we need able to error out early in `http.request` 
/// without throwing a generic wrap_err! that can't be turned into an HttpError
pub struct HttpResponse {
    inner: ResponseWithBody,
    body_buffer: Option<Vec<u8>>,
    max_body_size: Option<u64>
}

// methods wrt. HttpResponse objects but not exposed to Luau
impl HttpResponse {
    pub fn new(response: ResponseWithBody, max_body_size: Option<u64>) -> Self {
        Self {
            inner: response,
            body_buffer: None,
            max_body_size,
        }
    }

    pub fn read_body_into_self(&mut self) -> Result<(), UreqError> {
        let read_result = if let Some(limit) = self.max_body_size {
            self.inner.body_mut().with_config().limit(limit).read_to_vec()
        } else {
            self.inner.body_mut().read_to_vec()
        };
        let vec = match read_result {
            Ok(vecc) => vecc,
            Err(err) => {
                return Err(err);
            }
        };
        self.body_buffer = Some(vec);
        Ok(())
    }

    pub fn into_table(self, luau: &Lua, function_name: &'static str) -> LuaResult<LuaTable> {
        let response = self.inner;

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

        let body = match self.body_buffer {
            Some(body) => body,
            None => {
                panic!("{}: you forgot to call read_body_into_self before calling this function", function_name);
            }
        };

        let charset = response.body().charset();
        let mime_type = response.body().mime_type();

        let content_type = TableBuilder::create(luau)?
            .with_value("mime_type", mime_type)?
            .with_value("charset", charset)?
            .build_readonly()?;

        TableBuilder::create(luau)?
            .with_value("ok", is_ok)?
            .with_value("headers", headers)?
            .with_value("status", status)?
            .with_value("body", luau.create_string(body)?)?
            .with_value("content_type", content_type)?
            .with_function("expect_json", HttpResponse::expect_json)?
            .with_function("try_json", HttpResponse::try_json)?
            .with_metatable(TableBuilder::create(luau)?
                .with_function("__display", HttpResponse::display)?
                .build_readonly()?
            )?
            .build() // not readonly: allow  __display impl to be removed with setmetatable(response, nil)
    }

    fn is_json(self_table: &LuaTable, function_name: &'static str) -> LuaResult<IsJson> {
        let content_type_table = match self_table.raw_get("content_type")? {
            LuaValue::Table(t) => t,
            LuaNil => {
                return Ok(IsJson::Maybe);
            },
            other => {
                return wrap_err!("{}: HttpResponse.content_type is not a table nor nil (got {:?})", function_name, other);
            }
        };

        match content_type_table.raw_get("mime_type")? {
            LuaValue::String(mime_type) => {
                match mime_type.to_str() {
                    Ok(mime_type) if mime_type.as_str() == "application/json" => {
                        Ok(IsJson::Yes)
                    },
                    Ok(other_mime_type) => {
                        Ok(IsJson::No(other_mime_type.to_string()))
                    },
                    Err(_) => {
                        wrap_err!("{}: HttpResponse.mime_type contains invalid utf-8 (why)", function_name)
                    }
                }
            },
            LuaNil => Ok(IsJson::Maybe),
            other => {
                wrap_err!("{}: HttpResponse.mime_type should be a string or nil, got {:?}", function_name, other)
            }
        }
    }

    fn get_body(self_table: &LuaTable, function_name: &'static str) -> LuaResult<String> {
        match self_table.raw_get("body")? {
            LuaValue::String(body) => {
                match body.to_str() {
                    Ok(s) => Ok(s.to_string()),
                    Err(err) => {
                        wrap_err!("{}: response body must be valid utf-8 to decode to json: {}", function_name, err)
                    }
                }
            },
            _ => {
                wrap_err!("{}: response.body is not a string; you passed in the wrong table", function_name)
            }
        }
    }
}

// HttpResponse table methods exposed to Luau
impl HttpResponse {
    fn try_json(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
        let function_name = "HttpResponse:try_json()";

        let self_table = match multivalue.pop_front() {
            Some(LuaValue::Table(t)) => t,
            Some(LuaNil) | None => {
                return wrap_err!("{} incorrectly called without self", function_name);
            },
            Some(other) => {
                return wrap_err!("{}: expected self to be a HttpResponse table, got: {:?}", function_name, other);
            }
        };

        match Self::is_json(&self_table, function_name)? {
            IsJson::Yes => {
                let body = Self::get_body(&self_table, function_name)?;
                match std_json::json_decode(luau, body) {
                    Ok(value) => Ok(value),
                    Err(err) => {
                        wrap_err!("{}: HttpResponse.content_type.mime_type was application/json but we encountered an error decoding the json body: {}", function_name, err)
                    }
                }
            },
            IsJson::Maybe => {
                let body = Self::get_body(&self_table, function_name)?;
                match std_json::json_decode(luau, body) {
                    Ok(value) => Ok(value),
                    Err(_) => Ok(LuaNil)
                }
            },
            IsJson::No(_) => Ok(LuaNil),
        }
    }

    fn expect_json(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
        let function_name = "HttpResponse:expect_json()";

        let self_table = match multivalue.pop_front() {
            Some(LuaValue::Table(t)) => t,
            Some(LuaNil) | None => {
                return wrap_err!("{} incorrectly called without self", function_name);
            },
            Some(other) => {
                return wrap_err!("{}: expected self to be a HttpResponse table, got: {:?}", function_name, other);
            }
        };

        if let IsJson::No(actually_mime_type) = Self::is_json(&self_table, function_name)? {
            return wrap_err!("{}: expected HttpResponse.mime_type to be application/json, but response's mime_type is actually {}", function_name, actually_mime_type);
        }

        let body = Self::get_body(&self_table, function_name)?;

        std_json::json_decode(luau, body)
    }

    fn display(luau: &Lua, multivalue: LuaMultiValue) -> LuaValueResult {
        const DISPLAY_SRC: &str = include_str!("./display.luau");
        if let Some(display_fn) = luau.named_registry_value::<Option<LuaFunction>>("HttpResponse:__display")? {
            display_fn.call(multivalue)
        } else {
            let display_fn = luau.load(Chunk::Src(DISPLAY_SRC.to_string())).eval::<LuaFunction>()?;
            luau.set_named_registry_value("HttpResponse:__display", &display_fn)?;
            display_fn.call(multivalue)
        }
    }
}