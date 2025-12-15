use mluau::prelude::*;
use crate::prelude::*;

fn url_encode(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "url.encode(text: string)";
    let s = match value {
        LuaValue::String(s) => s.to_string_lossy(),
        other => {
            return wrap_err!("{} expected text to be a string, got: {:?}", function_name, other);
        }
    };

    let encoded = urlencoding::encode(&s).into_owned();

    ok_string(encoded, luau)
}

fn url_decode(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "url.decode(encoded: string)";
    let encoded = match value {
        LuaValue::String(s) => s.to_string_lossy(),
        other => {
            return wrap_err!("{} expected text to be a string, got: {:?}", function_name, other);
        }
    };

    let decoded = match urlencoding::decode(&encoded) {
        Ok(decoded) => decoded.into_owned(),
        Err(err) => {
            return wrap_err!("{}: unable to decode url-encoded string due to err: {}", function_name, err);
        }
    };

    ok_string(decoded, luau)
}

fn url_binary_encode(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "url.binary.encode(data: string | buffer)";
    let bytes = match value {
        LuaValue::String(s) => s.as_bytes().to_owned(),
        LuaValue::Buffer(buffy) => buffy.to_vec(),
        other => {
            return wrap_err!("{} expected data to be a string or buffer, got: {:?}", function_name, other);
        }
    };

    let encoded = urlencoding::encode_binary(&bytes).into_owned();
    ok_string(encoded, luau)
}


fn url_binary_decode(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "url.binary.decode(encoded: string)";
    let bytes = match value {
        LuaValue::String(s) => s.as_bytes().to_owned(),
        other => {
            return wrap_err!("{} expected encoded to be a string, got: {:?}", function_name, other);
        }
    };

    let decoded = urlencoding::decode_binary(&bytes).into_owned();
    ok_buffy(decoded, luau)
}

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_function("encode", url_encode)?
        .with_function("decode", url_decode)?
        .with_value("binary", TableBuilder::create(luau)?
            .with_function("encode", url_binary_encode)?
            .with_function("decode", url_binary_decode)?
            .build_readonly()?
        )?
        .build_readonly()
}