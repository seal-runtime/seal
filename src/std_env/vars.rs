use std::env::VarError;

use mluau::prelude::*;
use crate::{prelude::*, std_fs::validate_path};

enum MaybeDotenv {
    ExplicitTrue,
    ImplicitTrue,
    ExplicitFalse,
}

pub fn initialize_dotenv() -> LuaEmptyResult {
    let should_load = match std::env::var("SEAL_LOAD_DOTENV") {
        Ok(s) if s.eq_ignore_ascii_case("true") => MaybeDotenv::ExplicitTrue,
        Ok(s) if s.eq_ignore_ascii_case("false") => MaybeDotenv::ExplicitFalse,
        Ok(other) => {
            return wrap_err!("unexpected SEAL_LOAD_DOTENV environment variable (expected TRUE or FALSE or not present, got {})", other);
        },
        Err(VarError::NotPresent) => MaybeDotenv::ImplicitTrue,
        Err(err) => {
            return wrap_err!("bad SEAL_LOAD_DOTENV variable {}", err);
        }
    };

    if matches!(should_load, MaybeDotenv::ExplicitFalse) {
        return Ok(())
    }

    match dotenvy::dotenv() {
        Ok(_) => Ok(()),
        Err(err) if err.not_found() => match should_load {
            MaybeDotenv::ExplicitTrue => {
                wrap_err!("Your SEAL_LOAD_DOTENV environment variable is explicitly set to TRUE but no .env was found")
            },
            MaybeDotenv::ImplicitTrue => Ok(()),
            _ => unreachable!(),
        },
        Err(err) => {
            wrap_err!("seal was unable to load your .env file due to err: {}", err)
        }
    }
}

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

fn vars_flag(_luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
    let function_name = "vars.flag(name: string, default: boolean)";

    let name = match multivalue.pop_front() {
        Some(LuaValue::String(s)) => s.to_string_lossy(),
        Some(LuaNil) | None => {
            return wrap_err!("{} missing required argument 'name' (expected string)", function_name);
        },
        Some(other) => {
            return wrap_err!("{}: expected name to be a string, got: {:?}", function_name, other);
        }
    };

    let default = match multivalue.pop_front() {
        Some(LuaValue::Boolean(b)) => b,
        Some(LuaNil) | None => {
            return wrap_err!("{} missing required argument 'default' (expected boolean)", function_name);
        },
        Some(other) => {
            return wrap_err!("{}: expected default to be a boolean, got: {:?}", function_name, other);
        }
    };

    let raw = match std::env::var(&name) {
        Ok(s) => s,
        Err(VarError::NotPresent) => {
            return Ok(LuaValue::Boolean(default));
        }
        Err(VarError::NotUnicode(not_unicode)) => {
            return wrap_err!("{}: environment variable '{}' is not valid unicode ({:?})", function_name, name, not_unicode);
        }
    };

    let is_true =
        raw.eq_ignore_ascii_case("true")
            || raw.eq_ignore_ascii_case("yes")
            || raw.eq_ignore_ascii_case("on")
            || raw.eq_ignore_ascii_case("1")
            || raw.eq_ignore_ascii_case("y")
            || raw.eq_ignore_ascii_case("t");

    let is_false =
        raw.eq_ignore_ascii_case("false")
            || raw.eq_ignore_ascii_case("no")
            || raw.eq_ignore_ascii_case("off")
            || raw.eq_ignore_ascii_case("0")
            || raw.eq_ignore_ascii_case("n")
            || raw.eq_ignore_ascii_case("f");

    if is_true {
        return Ok(LuaValue::Boolean(true));
    }

    if is_false {
        return Ok(LuaValue::Boolean(false));
    }

    wrap_err!(
        "{}: unexpected value for environment flag '{}'; expected TRUE/YES/ON or FALSE/NO/OFF, got: {:?}",
        function_name,
        name,
        raw
    )
}

fn vars_validate(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
    let function_name = "vars.validate<T>(key: string, f: (value: string?) -> T)";

    let key = match multivalue.pop_front() {
        Some(LuaValue::String(s)) => s.to_string_lossy(),
        Some(LuaNil) | None => {
            return wrap_err!("{} missing required argument 'key' (expected string)", function_name);
        }
        Some(other) => {
            return wrap_err!("{}: expected key to be a string, got: {:?}", function_name, other);
        }
    };

    let callback = match multivalue.pop_front() {
        Some(LuaValue::Function(f)) => f,
        Some(LuaNil) | None => {
            return wrap_err!("{} missing required argument 'f' (expected function)", function_name);
        }
        Some(other) => {
            return wrap_err!("{}: expected f to be a function, got: {:?}", function_name, other);
        }
    };

    let value_opt = match std::env::var(&key) {
        Ok(s) => Some(LuaValue::String(luau.create_string(&s)?)),
        Err(VarError::NotPresent) => None,
        Err(VarError::NotUnicode(not_unicode)) => Some(not_unicode.into_lua(luau)?),
    };

    let arg = match value_opt {
        Some(v) => v,
        None => LuaNil,
    };

    let result = callback.call::<LuaValue>(arg)?;

    Ok(result)
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

/// # Safety
/// Callers in Luau are responsible to call this function <before> doing anything multithreaded.
fn vars_load(_luau: &Lua, mut multivalue: LuaMultiValue) -> LuaEmptyResult {
    let function_name = "<unsafe> vars.load(path: string, override: boolean?)";
    let path = match multivalue.pop_front() {
        Some(LuaValue::String(path)) => validate_path(&path, function_name)?,
        Some(LuaNil) | None => {
            return wrap_err!("{} missing required argument 'path' (expected string)");
        },
        Some(other) => {
            return wrap_err!("{}: expected path to be string, got: {:?}", function_name, other);
        }
    };

    let r#override = match multivalue.pop_front() {
        Some(LuaValue::Boolean(b)) => b,
        Some(LuaNil) | None => false,
        Some(other) => {
            return wrap_err!("{}: expected override to be a boolean or nil (default false), got: {:?}", function_name, other);
        }
    };

    match if r#override {
        dotenvy::from_path_override(&path)
    } else {
        dotenvy::from_path(&path)
    } {
        Ok(_) => Ok(()),
        Err(err) => {
            wrap_err!("{}: unable to load additional environment variables due to err: {}", function_name, err)
        }
    }
}

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_function("get", vars_get)?
        .with_function("flag", vars_flag)?
        .with_function("validate", vars_validate)?
        .with_function("set", vars_set)?
        .with_function("unset", vars_unset)?
        .with_function("all", vars_all)?
        .with_function("load", vars_load)?
        .build_readonly()
}