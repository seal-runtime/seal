use crate::prelude::*;
use mluau::prelude::*;
use crate::err;

pub struct WrappedError {
    message: String,
    traceback: Option<String>,
}

impl WrappedError {
    pub fn message(&self) -> &str { &self.message }
    pub fn from_message(message: String) -> Self {
        Self {
            message,
            traceback: None
        }
    }
    pub fn with_traceback(message: String, luau: &Lua) -> LuaResult<Self> {
        let traceback = Some(err::parse_traceback(luau.traceback(None, 0)?.to_string_lossy()));
        Ok(Self {
            message,
            traceback,
        })
    }
    pub fn format(&self) -> String {
        let traceback = self.traceback.clone().unwrap_or_default();
        if traceback.is_empty() {
            // format!("{}[ERR]{} {}", colors::BOLD_RED, colors::RESET, self.message)
            self.message.clone()
        } else {
            format!("{}{}{}\n{}\n", colors::RED, self.message, colors::RESET, traceback)
            // format!("{}[ERR]{} {}\n{}", colors::BOLD_RED, colors::RESET, self.message, traceback)
        }
    }
    pub fn get_userdata(self, luau: &Lua) -> LuaValueResult {
        ok_userdata(self, luau)
    }
}

impl LuaUserData for WrappedError {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_meta_field("__type", "error"); // allow users to typeof check
    }
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, | luau: &Lua, this: &WrappedError, _: LuaValue| -> LuaValueResult {
            this.message.clone().into_lua(luau)
        });
    }
}

impl Borrowable for WrappedError {
    fn type_name() -> &'static str {
        "error"
    }
}

pub fn ecall(luau: &Lua, f: LuaFunction) -> LuaValueResult {
    // Propagate the inner function's debug name to the wrapper so it shows in
    // seal's print/pp output and Luau stack traces. Leaking is intentional —
    // registered functions live for the lifetime of the runtime anyway.
    let debugname: Option<&'static std::ffi::CStr> = match f.info().name {
        Some(name) => match std::ffi::CString::new(name) {
            Ok(cstr) => Some(Box::leak(cstr.into_boxed_c_str())),
            Err(_) => None,
        },
        None => None,
    };
    let result = luau.create_function_with_debug(move |_: &Lua, multivalue: LuaMultiValue| {
        let result = f.call::<LuaMultiValue>(multivalue)?;
        if !result.is_empty()
            && let Some(front) = result.front()
        {
            match front {
                LuaValue::UserData(ud) => {
                    if let Ok(err) = ud.borrow::<WrappedError>() {
                        return wrap_err!("{}", err.message);
                    }
                },
                LuaValue::Error(err) => {
                    return wrap_err!("{}", err.to_string());
                },
                _ => {},
            }
        }
        Ok(result)
    }, debugname)?;
    Ok(LuaValue::Function(result))
}

fn err_message(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "err.message(m: string)";
    let message = match value {
        LuaValue::String(message) => message.to_string_lossy(),
        other => {
            return wrap_err!("{}: error message must be a string, got: {:?}", function_name, other);
        }
    };
    WrappedError::from_message(message).get_userdata(luau)
}

pub fn err_wrap(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "err.wrap(message: string)";
    let message = match value {
        LuaValue::String(message) => {
            format!("{}{}{}", colors::RED, message.to_string_lossy(), colors::RESET)
        },
        other => {
            return wrap_err!("{}: error message must be a string, got: {:?}", function_name, other);
        }
    };
    WrappedError::with_traceback(message, luau)?.get_userdata(luau)
}

fn format_error(value: LuaValue) -> LuaResult<String> {
    let stringified = match value {
        LuaValue::UserData(ud) => {
            if let Ok(wrapped) = ud.borrow::<WrappedError>() {
                wrapped.format()
            } else {
                let err = err::parse_traceback(ud.to_string()?);
                format!("{}[ERR]{} {}", colors::BOLD_RED, colors::RESET, err)
            }
        },
        LuaValue::Error(err) => {
            err::parse_traceback(err.to_string())
            // format!("{}[ERR]{} {}", colors::BOLD_RED, colors::RESET, err)
        }
        other => {
            return wrap_err!("passed error isn't the expected userdata, got: {:?}", other);
        }
    };
    Ok(stringified)
}

fn err_format(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let formatted = format_error(value)?;
    ok_string(formatted, luau)
}

fn err_traceback(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
    let function_name = "err.traceback(message: string?, level: number?)";
    let message = match multivalue.pop_front() {
        Some(LuaValue::String(msg)) => Some(msg.to_string_lossy()),
        Some(LuaNil) | None => None,
        Some(other) => {
            return wrap_err!("{}: expected message to be a string or nil/unspecified, got: {:?}", function_name, other);
        }
    };

    let level = match multivalue.pop_front() {
        Some(LuaValue::Integer(i)) => int_to_usize(i, function_name, "level")?,
        Some(LuaValue::Number(f)) => float_to_usize(f, function_name, "level")?,
        Some(LuaNil) | None => 0,
        Some(other) => {
            return wrap_err!("{}: expected level to be a positive integer (defaults to 0), got: {:?}", function_name, other);
        }
    };

    let traceback = luau.traceback(message.as_deref(), level)?.to_string_lossy();
    ok_string(traceback, luau)
}

fn err_throw(_luau: &Lua, value: LuaValue) -> LuaValueResult {
    let formatted = format_error(value)?;
    Err(LuaError::external(formatted))
}

pub fn err_extract(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "err.extract(err: error)";

    let (message, traceback) = match value {
        LuaValue::UserData(ud) if let Ok(err) = ud.borrow::<WrappedError>() => {
            let message = err.message.clone();
            let traceback = err.traceback.clone();

            (message, traceback)
        },
        LuaValue::UserData(ud) => {
            // this sucks but mluau and seal have other userdatas with typeof(ud) == "error" and not WrappedError
            let Ok(metatable) = ud.metatable() else {
                return wrap_err!("{}: passed error is actually a dynamic userdata not an error", function_name);
            };

            let Some(typ) = metatable.get::<Option<LuaString>>("__type")? else {
                return wrap_err!("{}: passed err is not an error because it doesn't have __type", function_name);
            };

            if typ.as_bytes().eq_ignore_ascii_case(b"error") {
                // this should be the stringified representation of the message
                let stringified = ud.to_string()?;

                (stringified, None)
            } else {
                return wrap_err!("{}: err is not an error, got: {:?}", function_name, ud);
            }
        },
        other => {
            return wrap_err!("{}: expected 'err' to be an error (userdata), got: {:?}", function_name, other);
        }
    };

    ok_table(TableBuilder::create(luau)?
        .with_value("message", message)?
        .with_value("traceback", traceback)?
        .build_readonly()
    )
}

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_function_and_signature("message", err_message, signatures::STD_ERR_MESSAGE)?
        .with_function_and_signature("wrap", err_wrap, signatures::STD_ERR_WRAP)?
        .with_function_and_signature("format", err_format, signatures::STD_ERR_FORMAT)?
        .with_function_and_signature("traceback", err_traceback, signatures::STD_ERR_TRACEBACK)?
        .with_function_and_signature("throw", err_throw, signatures::STD_ERR_THROW)?
        .with_function_and_signature("extract", err_extract, signatures::STD_ERR_EXTRACT)?
        .build_readonly()
}