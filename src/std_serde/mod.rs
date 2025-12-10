use crate::prelude::*;
use mluau::prelude::*;

pub mod base64;
pub mod hex;
pub mod toml;
pub mod yaml;
pub mod lz4;
pub mod zstd;
pub mod gzip;
pub mod zlib;

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_value("base64", base64::create(luau)?)?
        .with_value("json", crate::std_json::create(luau)?)?
        .with_value("hex", hex::create(luau)?)?
        .with_value("lz4", lz4::create(luau)?)?
        .with_value("zstd", zstd::create(luau)?)?
        .with_value("zlib", zlib::create(luau)?)?
        .build_readonly()
}
