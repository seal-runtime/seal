use std::path::PathBuf;

use mluau::prelude::*;
use crate::prelude::*;
use crate::compile;
use crate::std_fs::validate_path;

fn standalone_check(_luau: &Lua, value: LuaValue) -> LuaResult<bool> {
    let function_name = "standalone.check(path: string)";
    let path = match value {
        LuaValue::String(path) => {
            PathBuf::from(validate_path(&path, function_name)?)
        },
        other => {
            return wrap_err!("{} expected path to be a string, got: {:?}", function_name, other);
        }
    };
    Ok(compile::is_standalone(Some(path)))
}

fn standalone_extract(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "standalone.extract(path: string)";
    let path = match value {
        LuaValue::String(path) => {
            PathBuf::from(validate_path(&path, function_name)?)
        },
        other => {
            return wrap_err!("{} expected path to be a string, got: {:?}", function_name, other);
        }
    };
    if let Some(bytecode) = compile::extract_bytecode(Some(path)) {
        ok_buffy(&bytecode, luau)
    } else {
        wrap_err!("{}: bytecode could not be extracted :/ check your path?", function_name)
    }
}

fn standalone_eval(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
    let function_name = "standalone.eval(path: string, chunk_name: string)";
    let path = match multivalue.pop_front() {
        Some(LuaValue::String(path)) => {
            PathBuf::from(validate_path(&path, function_name)?)
        },
        Some(LuaNil) | None => {
            return wrap_err!("{} incorrectly called with zero arguments", function_name);
        }
        Some(other) => {
            return wrap_err!("{} expected path to be a string, got: {:?}", function_name, other);
        }
    };
    let chunk_name = match multivalue.pop_front() {
        Some(LuaValue::String(chunk_name)) => {
            chunk_name.to_string_lossy()
        },
        Some(LuaNil) | None => {
            return wrap_err!("{} called without required argument 'chunk_name'", function_name);
        }
        Some(other) => {
            return wrap_err!("{} expected chunk_name to be a string, got: {:?}", function_name, other);
        }
    };

    let Some(bytecode) = compile::extract_bytecode(Some(path)) else {
        return wrap_err!("{}: unable to extract bytecode", function_name);
    };

    match luau.load(&bytecode).set_name(&chunk_name).eval::<LuaValue>() {
        Ok(value) => Ok(value),
        Err(err) => {
            wrap_err!("{}: error evaluating bytecode: {}", function_name, err)
        }
    }
}

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_function("check", standalone_check)?
        .with_function("extract", standalone_extract)?
        .with_function("eval", standalone_eval)?
        .build_readonly()
}