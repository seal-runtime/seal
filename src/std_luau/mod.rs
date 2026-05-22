use std::path::PathBuf;

use mluau::prelude::*;
use crate::{Chunk, prelude::*};

use mluau::Compiler;

struct EvalError {
    message: String,
}
impl EvalError {
    fn new(err: LuaError) -> Self {
        Self {
            message: err.to_string()
        }
    }
}
impl LuaUserData for EvalError {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_meta_field("__type", "error"); // allow users to typeof check
    }
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, | luau: &Lua, this: &EvalError, _: LuaValue| -> LuaValueResult {
            this.message.clone().into_lua(luau)
        });
    }
}

enum EvalStdlib {
    Seal,
    Safe,
    None,
}

struct EvalOptions {
    name: Option<String>,
    stdlib: EvalStdlib,
    globals: Option<LuaTable>,
}
impl EvalOptions {
    fn default() -> Self {
        EvalOptions {
            name: None,
            stdlib: EvalStdlib::Safe,
            globals: None,
        }
    }

    fn from_table(value: LuaValue, function_name: &'static str) -> LuaResult<Self> {
        let t = match value {
            LuaValue::Table(t) => Some(t),
            LuaNil => None,
            other => {
                return wrap_err!("{} expected EvalOptions to be a table (with fields stdlib and/or is_bytecode) or nil, got: {:?}", function_name, other);
            }
        };
        if t.is_none() {
            return Ok(Self::default());
        }

        // SAFETY: we just checked .is_none()
        let t = unsafe { t.unwrap_unchecked() };

        let stdlib = match t.raw_get("stdlib")? {
            LuaValue::String(s) => {
                let s_bytes = s.as_bytes();
                if s_bytes.eq_ignore_ascii_case(b"Seal") {
                    EvalStdlib::Seal
                } else if s_bytes.eq_ignore_ascii_case(b"Safe") {
                    EvalStdlib::Safe
                } else if s_bytes.eq_ignore_ascii_case(b"None") {
                    EvalStdlib::None
                } else {
                    return wrap_err!("{} expected EvalOptions.stdlib to be \"Seal\" or \"Safe\" or \"None\" or nil, got an invalid string: {}", function_name, s.display())
                }
            },
            LuaNil => EvalStdlib::Safe,
            other => {
                return wrap_err!("{} expected EvalOptions.stdlib to be \"Seal\" or \"Safe\" or \"None\" or nil, got: {:?}", function_name, other);
            }
        };

        let name = match t.raw_get("name")? {
            Some(LuaValue::String(name)) => {
                Some(name.to_string_lossy())
            },
            Some(LuaNil) | None => {
                None
            },
            Some(other) => {
                return wrap_err!("{} expected name to be a string or nil, got: {:?}", function_name, other);
            }
        };

        let globals = match t.raw_get("globals")? {
            Some(LuaValue::Table(t)) => {
                // sanity check to ensure all k,v pairs have string keys
                for pair in t.pairs::<LuaValue, LuaValue>() {
                    let (key, _) = pair?;
                    if !matches!(key, LuaValue::String(_)) {
                        return wrap_err!("{}: globals environment table should only have string keys, got an {:#?} as a key", function_name, key);
                    }
                }
                Some(t)
            },
            Some(LuaNil) | None => None,
            Some(other) => {
                return wrap_err!("{} expected globals to be a table with string keys or nil, got: {:?}", function_name, other);
            }
        };

        Ok(EvalOptions {
            name,
            stdlib,
            globals
        })
    }
}

fn get_safe_globals(luau: &Lua) -> LuaResult<LuaTable> {
    let safe_globals = [
        // luau standard libraries
        "math", "table", "string", "coroutine", "bit32", "utf8", "os", "debug", "buffer", "vector",
        // some useful functions
        "assert", "error", "getmetatable", "setmetatable", "next", "ipairs", "pairs", "rawequal", "rawget", "rawset", "setmetatable",
        "tonumber", "tostring", "type", "typeof", "pcall", "xpcall", "unpack", "print"
        // note that require is purposely not included
    ];
    let t = luau.create_table()?;
    for glob in safe_globals {
        let value = luau.globals().get::<LuaValue>(glob)?;
        t.raw_set(glob, value)?;
    };
    t.raw_set("_VERSION", "Luau")?;
    let dummy_require_fn = luau.create_function(|_l: &Lua, _v: LuaValue| -> LuaValueResult {
        wrap_err!("require is not allowed in \"Safe\" mode! use \"Seal\" stdlib to allow requires.")
    })?;
    t.raw_set("require", dummy_require_fn)?;
    Ok(t)
}

/// this function is unsafe because invalid bytecode can cause the interpreter to crash and seal to crash
/// with illegal instruction & core dump. caller is responsible for making sure valid bytecode is passed
unsafe fn eval(luau: &Lua, src: Vec<u8>, eval_options: EvalOptions) -> LuaValueResult {
    let name = eval_options.name.unwrap_or("luau.load".to_string());
    let code = match String::from_utf8(src) {
        Ok(src) => Chunk::Src(src),
        Err(err) => Chunk::Bytecode(err.into_bytes()),
    };

    fn merge_globals(standard_globals: LuaTable, extra_globals: Option<LuaTable>) -> LuaResult<LuaTable> {
        if let Some(globals) = extra_globals {
            for pair in globals.pairs::<LuaValue, LuaValue>() {
                let (key, value) = pair?;
                standard_globals.set(key, value)?;
            }
        }
        Ok(standard_globals)
    }

    let globals = match eval_options.stdlib {
        EvalStdlib::Safe => {
            merge_globals(get_safe_globals(luau)?, eval_options.globals)?
        },
        EvalStdlib::None => {
            merge_globals(luau.create_table()?, eval_options.globals)?
        },
        EvalStdlib::Seal => {
            merge_globals(luau.globals(), eval_options.globals)?
        }
    };

    let chunk = luau.load(code)
        .set_name(name)
        .set_environment(globals);

    let res = match chunk.eval::<LuaValue>() {
        Ok(value) => value,
        Err(err) => {
            LuaValue::UserData(luau.create_userdata(EvalError::new(err))?)
        }
    };

    Ok(res)
}

fn luau_eval(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
    let function_name = "luau.eval(src: string, options: EvalOptions?)";
    let src = match multivalue.pop_front() {
        Some(LuaValue::String(src)) => {
            if let Ok(src) = src.to_str() {
                src.as_bytes().to_owned()
            } else {
                return wrap_err!(
                    "{}: Your src appears to be bytecode!\n\nEvaluating bytecode is UNSAFE because it can cause the entire program to crash!! (with an illegal instruction coredump!!!)!\n\nIf you trust your bytecode is valid and safe, use luau.eval_unsafe to evaluate it \n(please don't blame me if seal crashes).",
                    function_name
                )
            }
        },
        Some(LuaNil) | None => {
            return wrap_err!("{} expected src to be a string of Luau source code, but was incorrectly called with zero arguments", function_name);
        },
        other => {
            return wrap_err!("{} expected src to be a string of Luau source code, got: {:?}", function_name, other);
        }
    };
    let eval_options = match multivalue.pop_front() {
        Some(v) => EvalOptions::from_table(v, function_name)?,
        None => EvalOptions::default(),
    };

    // SAFETY: we've verified src is not luau bytecode (which must be invalid utf-8)
    let res = unsafe { eval(luau, src, eval_options) }?;
    Ok(res)
}

// this function is actually unsafe but we can't pass unsafe functions to luau
fn luau_eval_unsafe(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
    let function_name = "luau.eval(src: string, options: EvalOptions?)";
    let src = match multivalue.pop_front() {
        Some(LuaValue::String(src)) => src.as_bytes().to_owned(),
        Some(LuaValue::Buffer(buffy))=> buffy.to_vec(),
        Some(LuaNil) | None => {
            return wrap_err!("{} expected src to be a string (of either source code or bytecode) or buffer, but was incorrectly called with zero arguments", function_name);
        },
        other => {
            return wrap_err!("{} expected src to be a string (of either source code or bytecode) or buffer, got: {:?}", function_name, other);
        }
    };
    let eval_options = match multivalue.pop_front() {
        Some(v) => EvalOptions::from_table(v, function_name)?,
        None => EvalOptions::default(),
    };

    // SAFETY: caller in Luau is responsible for passing valid bytecode
    // if/when invalid bytecode is passed to this function, seal will crash with an "illegal instruction" and coredump.
    let res = unsafe { eval(luau, src, eval_options) }?;
    Ok(res)
}

fn luau_bytecode(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "luau.bytecode(src: string)";
    let src = match value {
        LuaValue::String(src) => src.to_string_lossy(),
        other => {
            return wrap_err!("{} expected src to be a string, got: {:?}", function_name, other);
        }
    };
    let comp = Compiler::new();
    let res = match comp.compile(src) {
        Ok(bytecode) => bytecode,
        Err(err) => {
            return Ok(LuaValue::UserData(
                luau.create_userdata(EvalError::new(err))?
            ))
        }
    };
    ok_buffy(res, luau)
}

fn luau_require_resolver(luau: &Lua, _: LuaValue) -> LuaValueResult {
    ok_table(crate::require::get_resolver(luau))
}

fn luau_bundle(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "luau.bundle(path: string)";
    let path = match value {
        LuaValue::String(s) => s.to_string_lossy(),
        other => {
            return wrap_err!("{} expected path to be a string, got: {:?}", function_name, other);
        }
    };
    match crate::compile::bundle(&PathBuf::from(path)) {
        Ok(bundled) => bundled.into_lua(luau),
        Err(err) => Ok(LuaValue::UserData(luau.create_userdata(EvalError::new(err))?)),
    }
}

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_function("eval", luau_eval)?
        .with_function("eval_unsafe", luau_eval_unsafe)?
        .with_function("bytecode", luau_bytecode)?
        .with_function("require_resolver", luau_require_resolver)?
        .with_function("bundle", luau_bundle)?
        .build_readonly()
}