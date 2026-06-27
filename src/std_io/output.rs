use std::process::Command;
use std::io::{self, Write};

use crate::prelude::*;
use mluau::prelude::*;

use super::format;
use crate::std_err::WrappedError;

pub fn debug_print(luau: &Lua, multivalue: LuaMultiValue) -> LuaMultiResult {
    let function_name = "dp(...: any)";
    let mut result = String::new();

    for (index, value) in multivalue.iter().enumerate() {
        format::process_debug_values(&mut result, value, 0)?;
        if index + 1 < multivalue.len() {
            result.push_str(", ");
        }
    }

    let debug_info = DebugInfo::from_caller(luau, function_name)?;
    puts!(
        "{}[DEBUG]{} {}:{} in {}{}\n{}",
        colors::BOLD_RED, colors::RESET, debug_info.source.replace("string ", ""), debug_info.line, debug_info.function_name, colors::RESET,
        &result
    )?;

    Ok(multivalue)
}

pub fn simple_print_and_return(luau: &Lua, multivalue: LuaMultiValue) -> LuaMultiResult {
    let formatter: LuaTable = format::cached_formatter(luau)?;
    let format_simple: LuaFunction = formatter.raw_get("simple")?;

    let mut output = String::from("");

    for (index, value) in multivalue.iter().enumerate() {
        let formatted = match format_simple.call::<LuaValue>(value) {
            Ok(LuaValue::String(text)) => text.to_string_lossy(),
            Ok(other) => {
                panic!("p: format.simple returned a non-string, got: {:?}", other);
            },
            Err(err) => {
                return wrap_err!("p: error printing: {}", err);
            }
        };

        output.push_str(&formatted);

        if index + 1 < multivalue.len() {
            output.push_str(", ");
        }
    }

    puts!("{}", &output)?;

    Ok(multivalue)
}

pub fn pretty_print_and_return(luau: &Lua, multivalue: LuaMultiValue) -> LuaMultiResult {
    let mut output = String::from("");

    for (index, value) in multivalue.iter().enumerate() {
        let formatted = match format::pretty(luau, value.into_lua_multi(luau)?) {
            Ok(text) => text,
            Err(err) => {
                return wrap_err!("pp: error printing: {}", err);
            }
        };

        output.push_str(&formatted);

        if index + 1 < multivalue.len() {
            output.push_str(", ");
        }
    }

    puts!("{}", &output)?;

    Ok(multivalue)
}

pub fn pretty_print(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaResult<()> {
    let mut output = String::from("");
    let values: Vec<LuaValue> = multivalue.drain(..).collect();

    for (index, value) in values.iter().enumerate() {
        match format::pretty(luau, value.into_lua_multi(luau)?) {
            Ok(text) => output += &text,
            Err(err) => {
                return wrap_err!("print: error printing: {}", err);
            }
        };
        if index + 1 < values.len() {
            output.push_str(", ");
        }
    }
    puts!("{}", &output)?;
    Ok(())
}

pub fn output_clear(_luau: &Lua, _value: LuaValue) -> LuaValueResult {
    let mut clear_command = if cfg!(target_os = "windows") {
        // use "cmd.exe /C cls" for Windows
        let mut com = Command::new("cmd");
        com.args(["/C", "cls"]);
        com
    } else {
        // use "clear" for Unix-like systems
        Command::new("clear")
    };
    match clear_command.spawn() {
        Ok(_) => {
            // this is pretty cursed, but yields long enough for the clear to have been completed
            // otherwise the next print() calls get erased
            std::thread::sleep(std::time::Duration::from_millis(20));
            Ok(LuaNil)
        },
        Err(err) => {
            wrap_err!("output.clear: unable to clear the terminal: {}", err)
        }
    }
}

pub fn output_write(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "output.write(contents: string | buffer)";
    let contents = match value {
        LuaValue::String(text) => {
            text.as_bytes().to_owned()
        },
        LuaValue::Buffer(buffy) => {
            buffy.to_vec()
        },
        LuaNil => {
            return wrap_err!("{} expected contents to be a string or buffer, got nothing or nil", function_name);
        },
        other => {
            return wrap_err!("{} expected contents to be a string or buffer, got {:?}", function_name, other);
        }
    };

    // we can't give users explicit flush control because stdout/stderr flushes anyway
    // when it goes out of scope and it's not worth it to keep stdout/stderr around for multiple calls
    let should_flush = !contents.ends_with(b"\n");

    let mut stdout = io::stdout();
    if let Err(err) = stdout.write_all(&contents) {
        return WrappedError::from_message(err.to_string()).get_userdata(luau);
    }

    if should_flush && let Err(err) = stdout.flush() {
        return WrappedError::from_message(err.to_string()).get_userdata(luau);
    }

    Ok(LuaNil)
}

pub fn output_ewrite(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "output.ewrite(contents: string | buffer)";
    let contents = match value {
        LuaValue::String(text) => {
            text.as_bytes().to_owned()
        },
        LuaValue::Buffer(buffy) => {
            buffy.to_vec()
        },
        LuaNil => {
            return wrap_err!("{} expected contents to be a string or buffer, got nothing or nil", function_name);
        },
        other => {
            return wrap_err!("{} expected contents to be a string or buffer, got {:?}", function_name, other);
        }
    };

    // we can't give users explicit flush control because stdout/stderr flushes anyway
    // when it goes out of scope and it's not worth it to keep stdout/stderr around for multiple calls
    let should_flush = !contents.ends_with(b"\n");

    let mut stderr = io::stderr();
    if let Err(err) = stderr.write_all(&contents) {
        return WrappedError::from_message(err.to_string()).get_userdata(luau);
    }

    if should_flush && let Err(err) = stderr.flush() {
        return WrappedError::from_message(err.to_string()).get_userdata(luau);
    }

    Ok(LuaNil)
}

pub fn output_writeln(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "output.writeln(contents: string | buffer)";
    let mut contents = match value {
        LuaValue::String(text) => {
            text.as_bytes().to_owned()
        },
        LuaValue::Buffer(buffy) => {
            buffy.to_vec()
        },
        LuaNil => {
            return wrap_err!("{} expected contents to be a string or buffer, got nothing or nil", function_name);
        },
        other => {
            return wrap_err!("{} expected contents to be a string or buffer, got {:?}", function_name, other);
        }
    };

    contents.push(b'\n');

    let mut stdout = io::stdout();
    if let Err(err) = stdout.write_all(&contents) {
        return WrappedError::from_message(err.to_string()).get_userdata(luau);
    }

    if let Err(err) = stdout.flush() {
        return WrappedError::from_message(err.to_string()).get_userdata(luau);
    }

    Ok(LuaNil)
}

pub fn output_ewriteln(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "output.ewriteln(contents: string | buffer)";
    let mut contents = match value {
        LuaValue::String(text) => {
            text.as_bytes().to_owned()
        },
        LuaValue::Buffer(buffy) => {
            buffy.to_vec()
        },
        LuaNil => {
            return wrap_err!("{} expected contents to be a string or buffer, got nothing or nil", function_name);
        },
        other => {
            return wrap_err!("{} expected contents to be a string or buffer, got {:?}", function_name, other);
        }
    };

    contents.push(b'\n');

    let mut stderr = io::stderr();
    if let Err(err) = stderr.write_all(&contents) {
        return WrappedError::from_message(err.to_string()).get_userdata(luau);
    }

    if let Err(err) = stderr.flush() {
        return WrappedError::from_message(err.to_string()).get_userdata(luau);
    }

    Ok(LuaNil)
}

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_function_and_signature("clear", output_clear, signatures::STD_IO_OUTPUT_CLEAR)?
        .with_function_and_signature("write", output_write, signatures::STD_IO_OUTPUT_WRITE)?
        .with_function_and_signature("writeln", output_writeln, signatures::STD_IO_OUTPUT_WRITELN)?
        .with_function_and_signature("ewrite", output_ewrite, signatures::STD_IO_OUTPUT_EWRITE)?
        .with_function_and_signature("ewriteln", output_ewriteln, signatures::STD_IO_OUTPUT_EWRITELN)?
        .with_function("sprint", simple_print_and_return)?
        .build_readonly()
}