use crate::prelude::*;
use mluau::prelude::*;

use crossterm::terminal::ClearType;
use crossterm::execute;

mod events;

fn terminal_clear(_luau: &Lua, value: LuaValue) -> LuaResult<()> {
    let function_name = "terminal.clear(mode: ClearMode)";
    let clear_type = match value {
        LuaValue::String(mode) => {
            match mode.to_str()?.as_ref() {
                "All" => ClearType::All,
                "Purge" => ClearType::Purge,
                "FromCursorDown" => ClearType::FromCursorDown,
                "FromCursorUp" => ClearType::FromCursorUp,
                "CurrentLine" => ClearType::CurrentLine,
                "UntilNewLine" => ClearType::UntilNewLine,
                other => {
                    return wrap_err!("{}: expected mode to be a ClearMode string (\"All\" | \"Purge\" | \"FromCursorDown\" | \"FromCursorUp\" | \"CurrentLine\" | \"UntilNewLine\"), got {}", function_name, other)
                }
            }
        },
        LuaNil => ClearType::All,
        other => {
            return wrap_err!("{}: expected mode to be a ClearMode or nil, got: {:?}", function_name, other);
        }
    };

    let mut stdout = std::io::stdout();
    if let Err(err) = execute!(stdout, crossterm::terminal::Clear(clear_type)) {
        return wrap_err!("{}: unable to clear terminal due to err: {}", function_name, err);
    }

    Ok(())
}

fn terminal_size(_luau: &Lua, _: LuaValue) -> LuaValueResult {
    let function_name = "terminal.size()";
    let (cols, rows) = match crossterm::terminal::size() {
        Ok(size) => size,
        Err(err) => {
            return wrap_err!("{}: unable to get terminal size due to err: {}", function_name, err);
        }
    };
    
    Ok(LuaValue::Vector(mluau::Vector::new(cols as f32, rows as f32, 0.0)))
}

fn terminal_switch(_luau: &Lua, value: LuaValue) -> LuaEmptyResult {
    let function_name = "terminal.switch(screen: \"Alternate\" | \"Main\")";

    let alternate = match value {
        LuaValue::String(s) if s.as_bytes().eq_ignore_ascii_case(b"Alternate") => {
            true
        },
        LuaValue::String(s) if s.as_bytes().eq_ignore_ascii_case(b"Main") => {
            false
        },
        other => {
            return wrap_err!("{} expected screen to be \"Alternate\" or \"Main\", got: {:?}", function_name, other);
        }
    };

    if alternate {
        if let Err(err) = execute!(std::io::stdout(), crossterm::terminal::EnterAlternateScreen) {
            return wrap_err!("can't switch to Alternate screen due to err: {}", err);
        }
    } else {
        if let Err(err) = execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen) {
            return wrap_err!("can't switch to Main screen due to err: {}", err);
        }
    }

    Ok(())
}

fn terminal_linewrap(_luau: &Lua, value: LuaValue) -> LuaResult<()> {
    let function_name = "terminal.linewrap(enabled: boolean)";
    let should = match value {
        LuaValue::Boolean(b) => b,
        other => {
            return wrap_err!("{} expected enabled to be a boolean, got: {:?}", function_name, other);
        }
    };

    let result = if should {
        execute!(std::io::stdout(), crossterm::terminal::EnableLineWrap)
    } else {
        execute!(std::io::stdout(), crossterm::terminal::DisableLineWrap)
    };
    if let Err(err) = result {
        return wrap_err!("{}: unable to enable or disable linewrap due to err: {}", function_name, err);
    }
    
    Ok(())
}

enum ScrollDirection {
    Up(u16),
    Down(u16),
    None
}
impl ScrollDirection {
    fn from_i64(i: i64) -> Self {
        if i.is_positive() {
            Self::Down(i.unsigned_abs() as u16)
        } else if i.is_negative() {
            Self::Up(i.unsigned_abs() as u16)
        } else {
            Self::None
        }
    }
}

fn terminal_scroll(_luau: &Lua, value: LuaValue) -> LuaResult<()> {
    let function_name = "terminal.scroll(lines: number)";
    
    let delta = match value {
        LuaValue::Number(f) => ScrollDirection::from_i64(f.round() as i64), // as cast should safely saturate i64 not panic here
        LuaValue::Integer(i) => ScrollDirection::from_i64(i),
        other => {
            return wrap_err!("{} expected lines to be a number; negative whole numbers scroll up, positive whole numbers scroll down; got {:?}", function_name, other);
        }
    };

    let result = match delta {
        ScrollDirection::Up(delta) => {
            execute!(std::io::stdout(), crossterm::terminal::ScrollUp(delta))
        },
        ScrollDirection::Down(delta) => {
            execute!(std::io::stdout(), crossterm::terminal::ScrollDown(delta))
        },
        ScrollDirection::None => {
            return Ok(())
        }
    };

    if let Err(err) = result {
        return wrap_err!("{}: unable to scroll up or down due to err: {}", function_name, err);
    }

    Ok(())
}

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_function("clear", terminal_clear)?
        .with_function("size", terminal_size)?
        .with_function("linewrap", terminal_linewrap)?
        .with_function("scroll", terminal_scroll)?
        .with_function("switch", terminal_switch)?
        .with_function("events", events::events)?
        .with_value("capture", events::create_capture_table(luau)?)?
        .build_readonly()
}