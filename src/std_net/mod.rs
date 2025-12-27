use mluau::prelude::*;

pub mod http;
pub mod serve;
pub mod websocket;

use crate::prelude::*;

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_value("http", self::http::create(luau)?)?
        .with_value("socket", websocket::create(luau)?)?
        .build_readonly()
}