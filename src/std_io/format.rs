use crate::prelude::*;
use mluau::prelude::*;

use std::sync::{LazyLock, RwLock};
use regex::Regex;

static DEFAULT_FORMAT_OPTIONS: LazyLock<RwLock<Option<FormatOptions>>> =
    LazyLock::new(|| RwLock::new(None));

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

const FORMATTER_SRC: &str = include_str!("./formatter.luau");

pub fn cached_formatter(luau: &Lua) -> LuaResult<LuaTable> {
    let f = luau.named_registry_value::<Option<LuaTable>>("format.formatter")?;
    if let Some(resolve) = f {
        Ok(resolve)
    } else {
        let chunk = Chunk::src(FORMATTER_SRC);
        let LuaValue::Table(formatter) = luau.load(chunk).set_name("@std/io/formatter.luau").eval()? else {
            panic!("formatter.luau didnt return table??");
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
    let re_colors = Regex::new(r"\x1b\[[0-9;]*m").expect("this is a valid regex");
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

#[derive(Clone)]
#[derive(Default)]
struct FormatOptions {
    indent_spaces: Option<u32>,
    max_depth: Option<u32>,
    max_elements_in_array: Option<u32>,
    show_array_indices: Option<bool>,
    show_metatables: Option<bool>,
    guidelines: Option<bool>,
    show_array_length: Option<bool>,
}
impl FormatOptions {
    fn from_value(value: &LuaValue, function_name: &'static str) -> LuaResult<Option<Self>> {
        let t = match value {
            LuaValue::Table(t) => t,
            LuaNil => {
                return Ok(None);
            },
            other => {
                return wrap_err!("{}: expected options to be a FormatOptions table, got something else: {:?}", function_name, other);
            }
        };

        fn check_number(t: &LuaTable, parameter_name: &'static str, function_name: &'static str) -> LuaResult<Option<u32>> {
            let u = match t.raw_get(parameter_name)? {
                LuaValue::Number(f) => float_to_u32(f, function_name, parameter_name)?,
                LuaValue::Integer(i) => int_to_u32(i, function_name, parameter_name)?,
                LuaNil => return Ok(None),
                other => return wrap_err!("{}: expected {} to be a number, got: {:?}", function_name, parameter_name, other),
            };
            Ok(Some(u))
        }

        fn check_boolean(t: &LuaTable, parameter_name: &'static str, function_name: &'static str) -> LuaResult<Option<bool>> {
            match t.raw_get(parameter_name)? {
                LuaValue::Boolean(b) => Ok(Some(b)),
                LuaNil => Ok(None),
                other => wrap_err!("{}: expected {} to be a boolean or nil, got: {:?}", function_name, parameter_name, other),
            }
        }

        let indent_spaces = check_number(t, "indent_spaces", function_name)?;
        let max_depth = check_number(t, "max_depth", function_name)?;
        let max_elements_in_array = check_number(t, "max_elements_in_array", function_name)?;
        let show_array_indices = check_boolean(t, "show_array_indices", function_name)?;
        let show_metatables = check_boolean(t, "show_metatables", function_name)?;
        let guidelines = check_boolean(t, "guidelines", function_name)?;
        let show_array_length = check_boolean(t, "show_array_length", function_name)?;

        Ok(Some(Self {
            indent_spaces,
            max_depth,
            max_elements_in_array,
            show_array_indices,
            show_metatables,
            guidelines,
            show_array_length,
        }))
    }
}

fn format_defaults(_luau: &Lua, value: LuaValue) -> LuaValueResult {
    let options = FormatOptions::from_value(&value, "format.defaults")?;
    let mut defaults = DEFAULT_FORMAT_OPTIONS.write().expect("writer should not panic");
    *defaults = options;
    Ok(LuaNil)
}

pub fn pretty(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaResult<String> {
    let function_name = "format.pretty(value: any, options: FormatOptions?, current_depth: number?)";
    let formatter = cached_formatter(luau)?;
    let format_pretty: LuaFunction = formatter.raw_get("pretty")?;
    let Some(value) = multivalue.pop_front() else {
        return wrap_err!("{} got nothing to format", function_name);
    };

    let options = if let Some(options) = multivalue.pop_front() {
        FormatOptions::from_value(&options, function_name)?
    } else {
        DEFAULT_FORMAT_OPTIONS.read().expect("writer should not panic").clone()
    };

    let current_depth: LuaValue = match multivalue.pop_front() {
        Some(LuaValue::Number(f)) => float_to_usize(f, function_name, "current_depth")?.into_lua(luau)?,
        Some(LuaValue::Integer(i)) => int_to_usize(i, function_name, "current_depth")?.into_lua(luau)?,
        None => LuaNil,
        other => {
            return wrap_err!("{}: expected current_depth to be a number or nil/unspecified, got: {:?}", function_name, other);
        }
    };

    let FormatOptions {
        indent_spaces,
        max_depth,
        max_elements_in_array,
        show_array_indices,
        show_metatables,
        guidelines,
        show_array_length,
    } = options.unwrap_or_default();
    let result = format_pretty.call::<LuaString>((
        value,            // value
        LuaNil,           // seen_tables
        current_depth,    // depth
        LuaNil,           // current_table_path
        indent_spaces,
        max_depth,
        max_elements_in_array,
        show_array_indices,
        show_metatables,
        guidelines,
        show_array_length,
    ));

    let formatted = match result {
        Ok(text) => text.to_string_lossy(),
        Err(err) => {
            return wrap_err!("{}: unable to format value due to err: {}", function_name, err);
        }
    };

    Ok(formatted)
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
    let function_name = "format(item: any, options: FormatOptions?)";
    pop_self(&mut multivalue, function_name)?;
    pretty(luau, multivalue)
}

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_function_and_signature("pretty", pretty, signatures::STD_IO_FORMAT_PRETTY)?
        .with_function_and_signature("defaults", format_defaults, signatures::STD_IO_FORMAT_DEFAULTS)?
        .with_function_and_signature("simple", simple, signatures::STD_IO_FORMAT_SIMPLE)?
        .with_function_and_signature("debug", debug, signatures::STD_IO_FORMAT_DEBUG)?
        .with_function_and_signature("uncolor", uncolor, signatures::STD_IO_FORMAT_UNCOLOR)?
        .with_function_and_signature("hexdump", hexdump, signatures::STD_IO_FORMAT_HEXDUMP)?
        .with_metatable(TableBuilder::create(luau)?
            .with_function("__call", __call_format)?
            .build_readonly()?
        )?
        .build_readonly()
}