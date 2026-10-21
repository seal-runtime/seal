
use mluau::prelude::*;
use crate::{prelude::*, std_err::WrappedError, userdata::SealLock};

pub mod externs;
pub mod standalone;

fn interop_mlua_isint(_luau: &Lua, n: LuaValue) -> LuaValueResult {
    match n {
        LuaValue::Integer(_i) => {
            Ok(LuaValue::Boolean(true))
        },
        LuaValue::Number(_n) => {
            Ok(LuaValue::Boolean(false))
        },
        other => {
            wrap_err!("interop.mlua.isint(n: number) expected n to be a number, got: {:#?}", other)
        }
    }
}

fn interop_mlua_iserror(_luau: &Lua, value: LuaValue) -> LuaValueResult {
    match value {
        LuaValue::UserData(v) => {
            Ok(LuaValue::Boolean(v.borrow::<SealLock<WrappedError>>().is_some()))
        },
        _ => Ok(LuaValue::Boolean(false))
    }
}

pub fn create_mlua(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_function("isint", interop_mlua_isint)?
        .with_function("iserror", interop_mlua_iserror)?
        .build_readonly()
}

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_value("mlua", create_mlua(luau)?)?
        .with_value("standalone", standalone::create(luau)?)?
        .with_value("extern", externs::create(luau)?)?
        .build_readonly()
}