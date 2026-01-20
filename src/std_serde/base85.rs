use mluau::prelude::*;
use crate::prelude::*;

// This Base85 implementation utilizes RFC 1924 encoding;
// It's similar to Z85, with a slightly different character set.

/// encodes luau string | buffer to base85
fn encode(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "base85.encode(content: string | buffer)";
    let value_to_serialize = match value {
        LuaValue::String(s) => s.as_bytes().to_owned(),
        LuaValue::Buffer(buffy) => buffy.to_vec(),
        other => {
            return wrap_err!("{} expected content to be a string or buffer, got: {:?}", function_name, other);
        }
    };
    let encoded = base85::encode(&value_to_serialize);
    ok_string(encoded, luau)
}

fn decode(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "base85.decode(encoded: string)";
    let encoded = match value {
        LuaValue::String(s) => s.to_string_lossy(),
        other => {
            return wrap_err!("{} expected encoded to be a string, got: {:?}", function_name, other);
        }
    };
    let decoded = match base85::decode(&encoded) {
        Ok(dec) => dec,
        Err(err) => {
            return wrap_err!("{}: error decoding base85: {}", function_name, err);
        }
    };
    ok_buffy(decoded, luau)
}

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_function("encode", encode)?
        .with_function("decode", decode)?
        .build_readonly()
}
