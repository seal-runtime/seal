use std::fs;

use mluau::prelude::*;
use crate::prelude::*;
use crate::std_fs::{entry::{wrap_io_read_errors, wrap_io_read_errors_empty}, validate_path};
use serde_json_lenient as serde_json;

pub struct EncodeOptions {
    pub pretty: bool,
    pub sorted: bool,
}
impl EncodeOptions {
    pub fn from_table(options_table: LuaTable) -> LuaResult<Self> {
        let pretty = match options_table.raw_get::<LuaValue>("pretty")? {
            LuaValue::Boolean(pretty) => pretty,
            LuaNil => true,
            other => {
                return wrap_err!("EncodeOptions.pretty expected to be a boolean or nil, got: {:#?}", other);
            }
        };
        let sorted = match options_table.raw_get::<LuaValue>("sorted")? {
            LuaValue::Boolean(ordered) => ordered,
            LuaNil => false,
            other => {
                return wrap_err!("EncodeOptions.sorted expected to be a boolean or nil, got: {:#?}", other);
            },
        };

        Ok(Self {
            pretty,
            sorted,
        })
    }
    pub fn default() -> Self {
        Self {
            pretty: true,
            sorted: false,
        }
    }
}

pub fn encode(luau: &Lua, table_to_encode: LuaTable, encode_options: EncodeOptions) -> LuaResult<String> {
    match if encode_options.pretty && !encode_options.sorted {
        serde_json::to_string_pretty(&table_to_encode)
    } else if !encode_options.pretty && !encode_options.sorted {
        serde_json::to_string(&table_to_encode)
    } else {
        let mut json_value: serde_json::Value = luau.from_value(LuaValue::Table(table_to_encode))?;
        if encode_options.sorted {
            json_value.sort_all_objects();
        }
        if encode_options.pretty {
            serde_json::to_string_pretty(&json_value)
        } else {
            serde_json::to_string(&json_value)
        }
    } {
        Ok(s) => Ok(s),
        Err(err) => wrap_err!("json.encode: {}", err)
    }
}

pub fn json_encode(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaResult<String> {
    let table_to_encode = match multivalue.pop_front() {
        Some(LuaValue::Table(table)) => table,
        Some(other) => {
            return wrap_err!("json.encode expected the value to encode to be a table, got: {:#?}", other);
        }
        None => {
            return wrap_err!("json.encode expected a value to encode, got nothing");
        }
    };

    let encode_options = {
        let options_table = match multivalue.pop_front() {
            Some(LuaValue::Table(table)) => Some(table),
            Some(LuaNil) => None,
            Some(other) => {
                return wrap_err!("json.encode(value: any, options: EncodeOptions) expected options to be a table, got: {:#?}", other);
            },
            None => None,
        };
        if let Some(options_table) = options_table {
            EncodeOptions::from_table(options_table)?
        } else {
            EncodeOptions::default()
        }
    };

    encode(luau, table_to_encode, encode_options)
}

pub fn json_raw_encode(_luau: &Lua, table: LuaValue) -> LuaResult<String> {
    let table_to_encode = match table {
        LuaValue::Table(t) => t,
        other => {
            return wrap_err!("json.raw expected any json-serializable table, got: {:#?}", other);
        }
    };
    match serde_json::to_string(&table_to_encode) {
        Ok(t) => Ok(t),
        Err(err) => {
            wrap_err!("json.raw: unable to encode table: {}", err)
        }
    }
}

/// rust-internal entry point for decoding json: takes `&str` (borrowed, no alloc for callers who already
/// have their json as a `&str`/`String`) rather than `json_decode`'s owned `String`, which only exists
/// because that's what Luau calling `json.decode` gets bound to
pub fn decode(luau: &Lua, json: &str) -> LuaValueResult {
    let json_value: serde_json::Value = match serde_json::from_str(json) {
        Ok(json) => json,
        Err(err) => {
            return wrap_err!("json: unable to decode json. serde_json error: {}", err.to_string());
        }
    };

    let luau_value = match luau.to_value::<serde_json_lenient::Value>(&json_value) {
        Ok(deserialized) => deserialized,
        Err(err) => {
            return wrap_err!("json.decode: unable to convert serde_json_lenient::Value into LuaValue due to err: {}", err);
        }
    };

    Ok(luau_value)
}

pub fn json_decode(luau: &Lua, json: String) -> LuaValueResult {
    decode(luau, &json)
}

fn json_readfile(luau: &Lua, file_path: LuaValue) -> LuaValueResult {
    let function_name = "json.readfile(path: string)";
    let path = match file_path {
        LuaValue::String(path) => validate_path(&path, function_name)?,
        other => {
            return wrap_err!("{} expected path to be a string, got: {:?}", function_name, other);
        }
    };

    let bytes = match fs::read(&path) {
        Ok(bytes) => bytes,
        Err(err) => {
            return wrap_io_read_errors(err, function_name, &path);
        }
    };

    // files can be utf-8, utf-16 (with or without a bom), etc; json.decode assumes utf-8 for speed,
    // but readfile reads arbitrary files off disk so it's worth the encoding detection here
    let utf8 = crate::std_str::bytes_to_utf8(&bytes, function_name)?;
    decode(luau, &utf8)
}

fn json_writefile(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaEmptyResult {
    let function_name = "json.writefile(path: string, json: JsonData, options: EncodeOptions?)";
    let path = match multivalue.pop_front() {
        Some(LuaValue::String(path)) => {
            validate_path(&path, function_name)?
        },
        Some(other) => {
            return wrap_err!("{} expected path to be a string, got: {:?}", function_name, other);
        },
        None => {
            return wrap_err!("{} expected path, got nothing", function_name);
        }
    };
    let encoded_data = match json_encode(luau, multivalue) {
        Ok(encoded) => encoded,
        Err(err) => {
            return wrap_err!("{}: encoding error: {}", function_name, err);
        }
    };
    match fs::write(&path, &encoded_data) {
        Ok(_) => Ok(()),
        Err(err) => {
            wrap_io_read_errors_empty(err, function_name, &path)
        }
    }
}

fn json_writefile_raw(_luau: &Lua, mut multivalue: LuaMultiValue) -> LuaEmptyResult {
    let function_name = "json.writefile_raw(path: string, json: JsonData)";
    let path = match multivalue.pop_front() {
        Some(LuaValue::String(path)) => {
            validate_path(&path, function_name)?
        },
        Some(other) => {
            return wrap_err!("{} expected path to be a string, got: {:?}", function_name, other);
        },
        None => {
            return wrap_err!("{} expected path, got nothing", function_name);
        }
    };
    let encoded_data = match multivalue.pop_front() {
        Some(LuaValue::Table(t)) => {
            match serde_json::to_string(&t) {
                Ok(data) => data,
                Err(err) => {
                    return wrap_err!("{}: unable to encode table: {}", function_name, err)
                }
            }
        },
        Some(other) => {
            return wrap_err!("{} expected json to be any json-serializable table, got: {:?}", function_name, other);
        },
        None => {
            return wrap_err!("{} missing second argument json", function_name);
        }
    };
    match fs::write(&path, &encoded_data) {
        Ok(_) => Ok(()),
        Err(err) => {
            wrap_io_read_errors_empty(err, function_name, &path)
        }
    }
}

fn json_null(luau: &Lua, _: LuaValue) -> LuaValueResult {
    Ok(luau.null())
}

fn json_array(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
    let t = match multivalue.pop_front() {
        Some(LuaValue::Table(t)) => t,
        Some(other) => {
            return wrap_err!("json.array(t: {{ T }}?) expected t to be an array-like table or nil, got: {:?}", other);
        },
        None => luau.create_table_with_capacity(10, 0)?
    };
    t.set_metatable(Some(luau.array_metatable()))?;
    ok_table(Ok(t))
}

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_function_and_signature("encode", json_encode, signatures::STD_JSON_ENCODE)?
        .with_function_and_signature("raw", json_raw_encode, signatures::STD_JSON_RAW)?
        .with_function_and_signature("decode", json_decode, signatures::STD_JSON_DECODE)?
        .with_function_and_signature("readfile", json_readfile, signatures::STD_JSON_READFILE)?
        .with_function_and_signature("writefile", json_writefile, signatures::STD_JSON_WRITEFILE)?
        .with_function_and_signature("writefile_raw", json_writefile_raw, signatures::STD_JSON_WRITEFILE_RAW)?
        .with_function_and_signature("null", json_null, signatures::STD_JSON_NULL)?
        .with_function_and_signature("array", json_array, signatures::STD_JSON_ARRAY)?
        .build_readonly()
}