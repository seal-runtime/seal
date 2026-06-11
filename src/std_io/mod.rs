
use mluau::prelude::*;
use crate::prelude::*;

pub mod colors;
pub mod input;
pub mod output;
pub mod format;
pub mod prompt;
#[macro_use]
pub mod macros;

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_value("input", input::create(luau)?)?
        .with_value("colors", colors::create(luau)?)?
        .with_value("output", output::create(luau)?)?
        .with_value("prompt", prompt::create(luau)?)?
        .build_readonly()
}