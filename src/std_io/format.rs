use crate::prelude::*;
use mluau::prelude::*;

use regex::Regex;

pub fn process_debug_values(result: &mut String, value: &LuaValue, depth: usize) -> LuaResult<()> {
    let left_padding = " ".repeat(2 * depth);
    match value {
        LuaValue::Table(t) => {
            if depth < 10 {
                result.push_str("{\n");
                for pair in t.pairs::<LuaValue, LuaValue>() {
                    let (k, v) = pair?;
                    result.push_str(&format!("  {left_padding}{:#?} = ", k));
                    process_debug_values(result, &v, depth + 1)?;
                    result.push('\n');
                }
                result.push_str(&format!("{left_padding}}}"));
            }
        },
        LuaValue::String(s) => {
            let formatted_string = format!("{:?}", s);
            result.push_str(&formatted_string);
        },
        LuaValue::Buffer(buffy) => {
            let hex_cfg = pretty_hex::HexConfig {title: true, width: 8, group: 0, ..pretty_hex::HexConfig::default() };
            result.push_str(&pretty_hex::config_hex(&buffy.to_vec(), hex_cfg));
        },
        LuaValue::UserData(data) => {
            match data.call_method::<LuaString>("__dp", ()) {
                Ok(dp_output) => {
                    result.push_str(&dp_output.to_string_lossy());
                },
                Err(_) => {
                    // __dp isn't defined or fails
                    result.push_str(&format!("{:?}", data));
                }
            }
        },
        _ => {
            result.push_str(&format!("{:?}", value));
        }
    }
    if depth > 0 {
        result.push(',');
    }
    Ok(())
}

fn debug(luau: &Lua, stuff: LuaMultiValue) -> LuaResult<LuaString> {
    let mut result = String::from("");
    let mut multi_values = stuff.clone();

    while let Some(value) = multi_values.pop_front() {
        process_debug_values(&mut result, &value, 0)?;
        if !multi_values.is_empty() {
            result += ", ";
        }
    }

    luau.create_string(&result)
}

const OUTPUT_FORMATTER_SRC: &str = include_str!("./output_formatter.luau");

pub fn cached_formatter(luau: &Lua) -> LuaResult<LuaTable> {
    let f = luau.named_registry_value::<Option<LuaTable>>("format.formatter")?;
    if let Some(resolve) = f {
        Ok(resolve)
    } else {
        let chunk = Chunk::Src(OUTPUT_FORMATTER_SRC.to_owned());
        let LuaValue::Table(formatter) = luau.load(chunk).eval()? else {
            panic!("output_formatter.luau didnt return table??");
        };

        luau.set_named_registry_value("format.formatter", &formatter)?;

        Ok(formatter)
    }
}

pub fn simple(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let formatter = cached_formatter(luau)?;
    let format_simple: LuaFunction = formatter.raw_get("simple")?;
    let result = match format_simple.call::<LuaString>(value) {
        Ok(text) => text.to_string_lossy(),
        Err(err) => {
            return wrap_err!("format.simple: error formatting: {}", err);
        }
    };

    let result = luau.create_string(&result)?;
    Ok(LuaValue::String(result))
}

pub fn strip_colors(input: &str) -> String {
    #[allow(clippy::unwrap_used, reason = "this is a valid regex")]
    let re_colors = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    let without_colors = re_colors.replace_all(input, "");
    without_colors.to_string()
}

fn uncolor(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let input = match value {
        LuaValue::String(s) => s.to_string_lossy(),
        other => {
            return wrap_err!("expected string to strip formatting of, got: {:#?}", other)
        }
    };
    let input = strip_colors(&input);
    Ok(LuaValue::String(
        luau.create_string(input.as_str())?
    ))
}

pub fn pretty(luau: &Lua, value: LuaValue) -> LuaResult<String> {
    let formatter = cached_formatter(luau)?;
    let format_pretty: LuaFunction = formatter.raw_get("pretty")?;
    let result = match format_pretty.call::<LuaString>(value) {
        Ok(text) => text.to_string_lossy(),
        Err(err) => {
            return wrap_err!("format: error formatting: {}", err);
        }
    };
    Ok(result)
}

pub fn hexdump(_luau: &Lua, value: LuaValue) -> LuaResult<String> {
    let hex_cfg = pretty_hex::HexConfig {title: true, width: 8, group: 0, ..pretty_hex::HexConfig::default() };
    let bytes = match value {
        LuaValue::String(s) => s.as_bytes().to_owned(),
        LuaValue::Buffer(buffy) => buffy.to_vec(),
        other => {
            return wrap_err!("format.hexdump(data: string | buffer) expected data to be a string or buffer, got: {:?}", other);
        }
    };
    Ok(pretty_hex::config_hex(&bytes, hex_cfg))
}

pub fn __call_format(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaResult<String> {
    let function_name = "io.format(item: unknown)";
    pop_self(&mut multivalue, function_name)?;
    let value = match multivalue.pop_front() {
        Some(value) => value,
        None => {
            return wrap_err!("{} expected an item to format, got nothing at all (not even nil)", function_name);
        }
    };
    pretty(luau, value)
}

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_function("pretty", pretty)?
        .with_function("simple", simple)?
        .with_function("debug", debug)?
        .with_function("uncolor", uncolor)?
        .with_function("hexdump", hexdump)?
        .with_metatable(TableBuilder::create(luau)?
            .with_function("__call", __call_format)?
            .build_readonly()?
        )?
        .build_readonly()
}