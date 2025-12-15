use crate::prelude::*;
use mluau::prelude::*;

use lz4_flex::{compress, decompress};

fn lz4_compress(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "lz4.compress(input: string | buffer)";
    let bytes = match value {
        LuaValue::Buffer(buffy) => {
            buffy.to_vec()
        },
        LuaValue::String(s) => {
            s.as_bytes().to_owned()
        },
        other => {
            return wrap_err!("{}: expected input to be string or buffer, got {:?}", function_name, other);
        }
    };

    let compressed_bytes = compress(&bytes);

    ok_buffy(compressed_bytes, luau)
}

fn lz4_decompress(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
    let function_name = "lz4.decompress(compressed: string | buffer, expected_size: number)";

    let bytes = match multivalue.pop_front() {
        Some(LuaValue::Buffer(buffy)) => {
            buffy.to_vec()
        },
        Some(LuaValue::String(s)) => {
            s.as_bytes().to_owned()
        },
        Some(LuaNil) | None => {
            return wrap_err!("{} was called without required argument 'compressed' (string | buffer)");
        },
        Some(other) => {
            return wrap_err!("{} expected compressed to be string | buffer, got: {:?}", function_name, other);
        }
    };

    let expected_size = match multivalue.pop_front() {
        Some(LuaValue::Number(f)) => float_to_usize(f, function_name, "expected_size")?,
        Some(LuaValue::Integer(i)) => int_to_usize(i, function_name, "expected_size")?,
        Some(LuaNil) | None => {
            return wrap_err!("{} was called without required argument 'expected_size' (the minimum expected size of the uncompressed data)");
        },
        Some(other) => {
            return wrap_err!("{} expected 'expected_size' to be a positive integer (the minimum expected size of the uncompressed data), got: {:?}", function_name, other);
        }
    };

    let decompressed_bytes = match decompress(&bytes, expected_size) {
        Ok(bytes) => bytes,
        Err(err) => {
            return wrap_err!("{}: unable to decompress bytes due to err: {}", function_name, err);
        }
    };

    ok_buffy(decompressed_bytes, luau)
}

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_function("compress", lz4_compress)?
        .with_function("decompress", lz4_decompress)?
        .build_readonly()
}