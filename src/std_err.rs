use std::{borrow::Cow, cell::RefCell, sync::Arc};

use crate::{prelude::*, std_io::format::pretty};
use mluau::{CallbackResult, FromLuaErr, prelude::*};
use crate::err;

#[derive(Clone, Debug)]
pub enum ErrorMessage {
    String(Arc<str>),
    Value(LuaValue)
}

impl IntoLua for ErrorMessage {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        match self {
            ErrorMessage::String(s) => lua.create_string(s.as_ref()).map(LuaValue::String),
            ErrorMessage::Value(v) => Ok(v)
        }
    }
}

#[derive(Clone)]
pub struct WrappedError {
    message: ErrorMessage,
    traceback: Option<Arc<str>>,

    // cached values
    formatted_tb: RefCell<Option<Arc<str>>>,
    formatted_message: RefCell<Option<Arc<str>>>,
    formatted_message_dirty: RefCell<Option<Arc<str>>>,
}

impl WrappedError {
    pub fn new(message: ErrorMessage, traceback: Option<Arc<str>>) -> Self {
        Self {
            message,
            traceback,
            formatted_tb: RefCell::new(None),
            formatted_message: RefCell::new(None),
            formatted_message_dirty: RefCell::new(None)
        }
    }

    pub fn from_message(message: String) -> Self {
        Self::new(ErrorMessage::String(message.into()), None)
    }

    pub fn with_traceback(message: String, luau: &Lua) -> LuaResult<Self> {
        let traceback = luau.traceback(None, 0)?.to_string_lossy();
        Ok(Self::new(ErrorMessage::String(message.into()), Some(traceback.into())))
    }

    pub fn format_from_ud(&self, ud: &LuaAnyUserData) -> Arc<str> {
        self.format(&ud.weak_lua().upgrade())
    }

    pub fn format(&self, luau: &Lua) -> Arc<str> {
        if let Some(ref val) = *self.formatted_tb.borrow() {
            return val.clone()
        }

        let traceback = self.traceback.clone().unwrap_or_default();
        let tb = if traceback.is_empty() {
            // format!("{}[ERR]{} {}", colors::BOLD_RED, colors::RESET, self.message)
            self.format_message(luau)
        } else {
            format!("{}{}{}\n{}\n", colors::RED, self.format_message(luau), colors::RESET, err::parse_traceback(traceback.as_ref())).into()
            // format!("{}[ERR]{} {}\n{}", colors::BOLD_RED, colors::RESET, self.message, traceback)
        };

        *self.formatted_tb.borrow_mut() = Some(tb.clone());
        tb
    }

    pub fn format_message(&self, luau: &Lua) -> Arc<str> {
        if let Some(ref val) = *self.formatted_message.borrow() {
            return val.clone()
        }

        let msg = match &self.message {
            ErrorMessage::String(s) => s.clone(),
            ErrorMessage::Value(v) => {
                match v {
                    LuaValue::UserData(u) if let Some(we) = u.borrow::<SealLock<Self>>() => {
                        return we.borrow().format_message(luau); // Recurse to inner
                    },
                    _ => {
                        match pretty(luau, LuaMultiValue::from_iter([v.clone()])) {
                            Ok(v) => v.into(),
                            Err(_) => return "<unknown error>".into() // should never be happen
                        }
                    }
                }
            }
        };

        *self.formatted_message.borrow_mut() = Some(msg.clone());
        msg
    }

    pub fn format_message_dirty(&self) -> Arc<str> {
        if let Some(ref val) = *self.formatted_message_dirty.borrow() {
            return val.clone()
        }

        let msg = match &self.message {
            ErrorMessage::String(s) => s.clone(),
            ErrorMessage::Value(v) => {
                match v {
                    LuaValue::UserData(u) if let Some(we) = u.borrow::<SealLock<Self>>() => {
                        return we.borrow().format_message_dirty(); // Recurse to inner
                    },
                    _ => {
                        v.to_string().unwrap_or_else(|_x| "<unknown>".to_string()).into()
                    }
                }
            }
        };

        *self.formatted_message_dirty.borrow_mut() = Some(msg.clone());
        msg
    }

    pub fn get_userdata(self, luau: &Lua) -> LuaValueResult {
        ok_userdata_mut(self, luau)
    }
}

impl SealUserData for WrappedError {
    fn type_name<'a>() -> Cow<'a, str> {
        Cow::Borrowed("error")
    }
    fn add_fields<F: SealUserDataFields<Self>>(fields: &mut F) {
        fields.add_meta_field("__type", "error"); // allow users to typeof check
    }
    fn add_methods<M: SealUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |luau: &Lua, this, _: LuaValue| {
            //println!("__tostring {:?}", );
            Ok(luau.create_string(this.format_message(luau).as_ref()).expect("WE FAILED")) // NOTE/TODO: For now, we use normal non-external strings here
        });
    }
}

impl BorrowableMut for WrappedError {}

// WrappedError's are first class error values, so impl FromLuaErr and IntoCallbackResult for them
impl FromLuaErr for WrappedError {
    const NEEDS_TRACEBACK: bool = true;
    fn from_lua_err(_lua: &Lua, err: LuaValue, _errcode: std::os::raw::c_int, tb: String) -> Self {        
        Self::new(ErrorMessage::Value(err), Some(tb.into()))
    }

    fn from_rust_err(error: mluau::Error) -> Self {
        Self::new(ErrorMessage::String(error.to_string().into()), None)
    }
}

impl IntoCallbackResult for WrappedError {
    fn into_callback_result(self, lua: &Lua) -> CallbackResult {
        LuaCustomError(self.into_mut()).into_callback_result(lua)
    }
}

impl std::fmt::Display for WrappedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.format_message_dirty())?;
        f.write_str("\n")?;
        if let Some(ref tb) = self.traceback {
            f.write_str(tb)?;
        }
        Ok(())
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
        // intentionally uses call_with_err directly to better control error
        let result = match f.call_with_err::<LuaMultiValue, WrappedError>(multivalue) {
            Ok(result) => result,
            Err(e) => return LuaEither::Right(LuaCustomError(e.into_mut()))
        };
        if !result.is_empty()
            && let Some(front) = result.front()
        {
            match front {
                LuaValue::UserData(ud) => {
                    if let Some(err) = ud.borrow::<SealLock<WrappedError>>() {
                        return LuaEither::Right(LuaCustomError(err.borrow().clone().into_mut()));
                    }
                },
                _ => {},
            }
        }
        LuaEither::Left(LuaOk(result))
    }, debugname)?;
    Ok(LuaValue::Function(result))
}

fn err_message(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "err.message(m: string)";
    if !value.is_string() {
        return wrap_err!("{}: error message must be a string, got: {:?}", function_name, value.type_name());
    }
    WrappedError::new(ErrorMessage::Value(value), None).get_userdata(luau)
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
    WrappedError::new(ErrorMessage::String(message.into()), None).get_userdata(luau)
}

fn format_error(value: LuaValue) -> LuaResult<Arc<str>> {
    let stringified = match value {
        LuaValue::UserData(ud) => {
            if let Some(wrapped) = ud.borrow::<SealLock<WrappedError>>() {
                wrapped.borrow().format(&ud.weak_lua().upgrade())
            } else {
                return wrap_err!("passed error isn't the expected userdata, got: {:?}", ud);
            }
        },
        other => {
            return wrap_err!("passed error isn't a userdata, got: {:?}", other).into();
        }
    };
    Ok(stringified)
}

fn err_format(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let formatted = format_error(value)?;
    ok_string(formatted.as_ref(), luau)
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
    Err(LuaError::external(formatted.to_string()))
}

pub fn err_extract(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "err.extract(err: error)";

    let (message, traceback) = match value {
        LuaValue::UserData(ref ud) if let Some(err) = ud.borrow::<SealLock<WrappedError>>() => {
            let message = err.borrow().message.clone();
            let traceback = err.borrow().traceback.clone();

            (message, traceback)
        },
        LuaValue::UserData(ud) => {
            // this sucks but mluau and seal have other userdatas with typeof(ud) == "error" and not WrappedError
            let Some(metatable) = ud.metatable() else {
                return wrap_err!("{}: passed error has no metatable, not an error", function_name);
            };

            let Some(typ) = metatable.get::<Option<LuaString>>("__type")? else {
                return wrap_err!("{}: passed err is not an error because it doesn't have __type", function_name);
            };

            if typ.as_bytes().eq_ignore_ascii_case(b"error") {
                // this should be the stringified representation of the message
                let stringified = ud.to_string()?;

                (ErrorMessage::String(stringified.into()), None)
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
        .with_value("traceback", traceback.as_deref())?
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
