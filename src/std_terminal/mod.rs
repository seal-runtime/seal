use crate::prelude::*;
use mluau::prelude::*;

use crossterm::terminal::ClearType;

pub mod actions;
pub mod cursor;
pub mod events;

pub use actions::TerminalAction;

enum WhichTty {
    Stdin,
    Stderr,
    Stdout,
    All,
}
impl WhichTty {
    fn pick(value: LuaValue, function_name: &'static str) -> LuaResult<Self> {
        let s = match value {
            LuaValue::String(ref s) => {
                &*s.as_bytes()
            },
            LuaNil => {
                return Ok(Self::All);
            },
            other => {
                return wrap_err!("{} expected stream to be nil/unspecified or \"Stdout\" | \"Stderr\" | \"Stdin\", got: {:?}", function_name, other);
            }
        };
        match s {
            b"Stdout" => Ok(Self::Stdout),
            b"Stderr" => Ok(Self::Stderr),
            b"Stdin"  => Ok(Self::Stdin),
            _ => {
                wrap_err!("{} expected stream to be nil/unspecified or \"Stdout\" | \"Stderr\" | \"Stdin\", got: {}", function_name, value.as_string().expect("we know it's a string").display())
            }
        }
    }
}

fn terminal_tty(_luau: &Lua, value: LuaValue) -> LuaResult<bool> {
    use atty::Stream::{Stdout, Stderr, Stdin};

    let function_name = "terminal.tty(stream: (\"Stdout\" | \"Stderr\" | \"Stdin\")?)";
    match WhichTty::pick(value, function_name)? {
        WhichTty::All => {
            Ok(atty::is(Stdout) && atty::is(Stderr) && atty::is(Stdin))
        },
        WhichTty::Stdout => Ok(atty::is(Stdout)),
        WhichTty::Stderr => Ok(atty::is(Stderr)),
        WhichTty::Stdin => Ok(atty::is(Stdin)),
    }
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

fn terminal_write(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "terminal.write(content: string)";
    let content = match value {
        LuaValue::String(ref s) if let Ok(content) = s.to_str() => {
            content.to_string()
        },
        LuaValue::String(_) | LuaValue::Buffer(_) => {
            return wrap_err!("{}: content must be valid utf-8 to be displayable; use io.output.write instead", function_name);
        },
        other => {
            return wrap_err!("{}: expected content to be a string, got: {:?}", function_name, other);
        }
    };

    TerminalAction::Write(content).get_userdata(luau)
}

fn terminal_title(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "terminal.title(title: string)";
    let title = match value {
        LuaValue::String(ref s) if let Ok(title) = s.to_str() => {
            title.to_string()
        },
        LuaValue::String(_) => {
            return wrap_err!("{}: title must be valid utf-8", function_name);
        },
        other => {
            return wrap_err!("{}: expected title to be a string, got: {:?}", function_name, other);
        }
    };

    TerminalAction::Title(title).get_userdata(luau)
}

fn terminal_clear(luau: &Lua, value: LuaValue) -> LuaValueResult {
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

    TerminalAction::Clear(clear_type).get_userdata(luau)
}

#[derive(Clone)]
pub enum WhichScreen {
    Main,
    Alternate,
}
impl WhichScreen {
    fn from_luau(value: LuaValue, function_name: &'static str) -> LuaResult<Self> {
        match value {
            LuaValue::String(ref s) => {
                if s.as_bytes().eq_ignore_ascii_case(b"Main") {
                    Ok(Self::Main)
                } else if s.as_bytes().eq_ignore_ascii_case(b"Alternate") {
                    Ok(Self::Alternate)
                } else {
                    wrap_err!("{}: expected 'screen' to be \"Main\" or \"Alternate\", got: {}", function_name, s.display())
                }
            },
            other => {
                wrap_err!("{}: expected 'screen' to be a string (\"Main\" or \"Alternate\"), got: {:?}", function_name, other)
            }
        }
    }
}

fn terminal_switch(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "terminal.switch(screen: \"Alternate\" | \"Main\")";
    let screen = WhichScreen::from_luau(value, function_name)?;
    TerminalAction::Switch(screen).get_userdata(luau)
}

fn terminal_linewrap(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "terminal.linewrap(enabled: boolean)";
    let enable = match value {
        LuaValue::Boolean(b) => b,
        other => {
            return wrap_err!("{} expected enabled to be a boolean, got: {:?}", function_name, other);
        }
    };

    TerminalAction::Linewrap(enable).get_userdata(luau)
}

#[derive(Clone)]
pub enum ScrollDirection {
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

fn terminal_scroll(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "terminal.scroll(lines: number)";
    
    let delta = match value {
        LuaValue::Number(f) => ScrollDirection::from_i64(f.round() as i64), // as cast should safely saturate i64 not panic here
        LuaValue::Integer(i) => ScrollDirection::from_i64(i),
        other => {
            return wrap_err!("{} expected lines to be a number; negative whole numbers scroll up, positive whole numbers scroll down; got {:?}", function_name, other);
        }
    };

    TerminalAction::Scroll(delta).get_userdata(luau)
}

fn terminal_execute(_luau: &Lua, multivalue: LuaMultiValue) -> LuaEmptyResult {
    let function_name = "terminal.execute(actions: TerminalAction...)";
    
    if multivalue.is_empty() {
        return Ok(());
    }

    let mut actions: Vec<TerminalAction> = Vec::with_capacity(multivalue.len());
    for (index, value) in multivalue.iter().enumerate() {
        let description = format!("action at index {}", index);
        let action = TerminalAction::from_value(value, function_name, description)?;
        actions.push(action);
    }

    actions::queue_and_execute(actions, function_name)
}

fn terminal_rawmode_enabled(_luau: &Lua, _value: LuaValue) -> LuaValueResult {
    let function_name = "terminal.rawmode.enabled()";
    match crossterm::terminal::is_raw_mode_enabled() {
        Ok(b) => Ok(LuaValue::Boolean(b)),
        Err(err) => {
            wrap_err!("{}: unable to determine whether terminal is raw or cooked due to err: {}", function_name, err)
        }
    }
}

fn terminal_rawmode_enable(_luau: &Lua, _value: LuaValue) -> LuaEmptyResult {
    let function_name = "terminal.rawmode.enable()";
    if let Err(err) = crossterm::terminal::enable_raw_mode() {
        return wrap_err!("{}: unable to enable terminal raw mode due to err: {}", function_name, err);
    }
    Ok(())
}

fn terminal_rawmode_disable(_luau: &Lua, _value: LuaValue) -> LuaEmptyResult {
    let function_name = "terminal.rawmode.disable()";
    if let Err(err) = crossterm::terminal::disable_raw_mode() {
        return wrap_err!("{}: unable to disable raw mode and switch back to cooked mode due to err: {}", function_name, err);
    }
    Ok(())
}

fn terminal_reset(_luau: &Lua, _value: LuaValue) -> LuaEmptyResult {
    let function_name = "terminal.reset()";

    if let Ok(is_raw) = crossterm::terminal::is_raw_mode_enabled()
        && is_raw
        && let Err(err) = crossterm::terminal::disable_raw_mode()
    {
        return wrap_err!("{}: unable to disable raw mode due to err: {}", function_name, err);
    }

    let result = crossterm::execute!(
        std::io::stdout(),
        crossterm::event::DisableBracketedPaste,
        crossterm::event::DisableFocusChange,
        crossterm::event::DisableMouseCapture,
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::terminal::EnableLineWrap,
        crossterm::cursor::SetCursorStyle::DefaultUserShape,
        crossterm::cursor::MoveToColumn(0),
        crossterm::cursor::Show,
    );

    if let Err(err) = result {
        return wrap_err!("{}: unable to reset terminal due to err: {}", function_name, err);
    }

    Ok(())
}

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_function("tty", terminal_tty)?
        .with_function("size", terminal_size)?
        .with_value("interrupt", events::create_interrupt_table(luau)?)?
        .with_function("write", terminal_write)?
        .with_function("title", terminal_title)?
        .with_function("clear", terminal_clear)?
        .with_function("linewrap", terminal_linewrap)?
        .with_function("scroll", terminal_scroll)?
        .with_function("switch", terminal_switch)?
        .with_function("events", events::events)?
        .with_function("execute", terminal_execute)?
        .with_function("reset", terminal_reset)?
        .with_value("capture", events::create_capture_table(luau)?)?
        .with_value("rawmode", TableBuilder::create(luau)?
            .with_function("enabled", terminal_rawmode_enabled)?
            .with_function("enable", terminal_rawmode_enable)?
            .with_function("disable", terminal_rawmode_disable)?
            .build_readonly()?
        )?
        .with_value("cursor", cursor::create(luau)?)?
        .build_readonly()
}