use std::env::VarError;

use mluau::prelude::*;
use crate::prelude::*;

fn vars_get(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "vars.get(key: string)";
    let key = match value {
        LuaValue::String(s) => s.to_string_lossy(),
        other => {
            return wrap_err!("{}: expected key to be a string, got: {:?}", function_name, other);
        }
    };

    match std::env::var(&key) {
        Ok(s) => ok_string(s, luau),
        Err(VarError::NotPresent) => Ok(LuaNil),
        Err(VarError::NotUnicode(not_unicode)) => not_unicode.into_lua(luau),
    }
}

/// # Safety
/// This function should not be called in multithreaded contexts (@std/thread) on unix-like
fn vars_set(_luau: &Lua, mut multivalue: LuaMultiValue) -> LuaEmptyResult {
    let function_name = "<unsafe> vars.set(key: string, value: string)";
    let key = match multivalue.pop_front() {
        Some(LuaValue::String(s)) => s.to_string_lossy(),
        Some(LuaNil) | None => {
            return wrap_err!("{} was called without required argument 'key' (expected string)");
        },
        Some(other) => {
            return wrap_err!("{}: expected key to be a string, got: {:?}", function_name, other);
        }
    };

    let value = match multivalue.pop_front() {
        Some(LuaValue::String(s)) => s.to_string_lossy(),
        Some(LuaNil) | None => {
            return wrap_err!("{}: missing required argument 'value' (expected string); to remove an environment value use vars.unset instead", function_name);
        },
        Some(other) => {
            return wrap_err!("{}: expected value to be a string, got: {:?}", function_name, other);
        }
    };

    // SAFETY: caller in Luau is responsible to not call this function in multithreaded contexts on *nix
    unsafe {
        std::env::set_var(key, value);
    }

    Ok(())
}

/// # Safety
/// This function should not be called in multithreaded contexts (@std/thread) on unix-like
fn vars_unset(_luau: &Lua, value: LuaValue) -> LuaEmptyResult {
    let function_name = "<unsafe> vars.unset(key: string)";
    let key = match value {
        LuaValue::String(s) => s.to_string_lossy(),
        other => {
            return wrap_err!("{}: expected key to be a string, got: {:?}", function_name, other);
        }
    };

    // SAFETY: caller in Luau is responsible to not call this function in multithreaded contexts on *nix
    unsafe {
        std::env::remove_var(key);
    }
    
    Ok(())
}

fn vars_all(luau: &Lua, _: LuaValue) -> LuaValueResult {
    let result = luau.create_table_with_capacity(0, 12)?;
    for (key, value) in std::env::vars_os() { // vars_os to prevent panicking
        result.raw_set(key.into_lua(luau)?, value.into_lua(luau)?)?;
    }
    result.set_readonly(true);
    ok_table(Ok(result))
}

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_function("get", vars_get)?
        .with_function("set", vars_set)?
        .with_function("unset", vars_unset)?
        .with_function("all", vars_all)?
        .build_readonly()
}