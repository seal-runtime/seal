use mluau::prelude::*;

pub mod http;
pub mod serve;
pub mod socket;

use crate::prelude::*;

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_value("http", self::http::create(luau)?)?
        .with_value("socket", socket::create(luau)?)?
        .build_readonly()
}
