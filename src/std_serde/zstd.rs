use std::io::Cursor;

use crate::prelude::*;
use mluau::prelude::*;

fn zstd_compress(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
    let function_name = "zstd.compress(data: string | buffer)";
    let bytes = match multivalue.pop_front() {
        Some(LuaValue::Buffer(buffy)) => {
            buffy.to_vec()
        },
        Some(LuaValue::String(s)) => {
            s.as_bytes().to_owned()
        },
        Some(LuaNil) | None => {
            return wrap_err!("{} was called without required argument 'data' (expected string | buffer)");
        },
        Some(other) => {
            return wrap_err!("{}: expected data to be string | buffer, got: {:?}", function_name, other);
        }
    };

    let level = match multivalue.pop_front() {
        Some(LuaValue::Number(f)) => f as i32,
        Some(LuaValue::Integer(i)) => i as i32,
        Some(LuaNil) | None => 0,
        Some(other) => {
            return wrap_err!("{}: expected level to be 0, 1, 2, 3, or nil/unspecified, got: {:?}", function_name, other);
        }
    };

    let compressed = match zstd::encode_all(Cursor::new(bytes), level) {
        Ok(compressed) => compressed,
        Err(err) => {
            return wrap_err!("{}: unable to compress bytes: {}", function_name, err);
        }
    };

    ok_buffy(compressed, luau)
}

fn zstd_decompress(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "zstd.decompress(compressed: buffer)";

    let bytes = match value {
        LuaValue::Buffer(buffy) => {
            buffy.to_vec()
        },
        other => {
            return wrap_err!("{}: expected 'compressed' to be a buffer, got: {:?}", function_name, other);
        }
    };

    let decompressed = match zstd::decode_all(Cursor::new(bytes)) {
        Ok(decompressed) => decompressed,
        Err(err) => {
            return wrap_err!("{}: unable to decompress bytes: {}", function_name, err);
        }
    };

    ok_buffy(decompressed, luau)
}

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_function("compress", zstd_compress)?
        .with_function("decompress", zstd_decompress)?
        .build_readonly()
}