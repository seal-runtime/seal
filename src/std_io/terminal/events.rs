use mluau::prelude::*;
use crate::prelude::*;

use crossterm::event::{Event, KeyEvent, KeyModifiers, MouseEvent};
use crossterm::execute;

use std::time::Duration;
use crate::std_time::duration::TimeDuration;

fn create_event_table(luau: &Lua, event: Event) -> LuaResult<LuaTable> {
    let t = create_table_with_capacity(luau, 0, 3)?;

    fn table_from_modifiers(luau: &Lua, modifiers: KeyModifiers) -> LuaResult<LuaTable> {
        let t: LuaTable = luau.named_registry_value("InputKeyModifiers")?;
        t.raw_set("shift", modifiers.contains(KeyModifiers::SHIFT))?;
        t.raw_set("ctrl", modifiers.contains(KeyModifiers::CONTROL))?;
        t.raw_set("alt", modifiers.contains(KeyModifiers::ALT))?;
        // t.raw_set("meta", modifiers.contains(KeyModifiers::META))?;
        Ok(t)
    }

    match event {
        Event::Key(KeyEvent { code, modifiers, .. }) => {
            t.raw_set("type", "Key")?;
            t.raw_set("key", code.to_string())?;
            t.raw_set("modifiers", table_from_modifiers(luau, modifiers)?)?;
        },
        Event::Mouse(MouseEvent { kind, column, row, modifiers }) => {
            // return ok_string(format!("Mouse: {:?}", kind), luau);
            t.raw_set("type", "Mouse")?;
            t.raw_set("kind", format!("{:?}", kind))?;
            t.raw_set("column", column)?;
            t.raw_set("row", row)?;
            t.raw_set("modifiers", table_from_modifiers(luau, modifiers)?)?;
        },
        Event::FocusLost => {
            t.raw_set("type", "FocusLost")?;
        },
        Event::FocusGained => {
            t.raw_set("type", "FocusGained")?;
        },
        Event::Resize(columns, rows) => {
            t.raw_set("type", "Resize")?;
            t.raw_set("columns", columns)?;
            t.raw_set("rows", rows)?;
        },
        Event::Paste(s) => {
            t.raw_set("type", "Paste")?;
            t.raw_set("contents", s)?;
        },
    }
    t.set_readonly(true);
    Ok(t)
}

fn capture_mouse(_luau: &Lua, value: LuaValue) -> LuaEmptyResult {
    let function_name = "input.capture.mouse(enabled: boolean)";
    let should_capture = match value {
        LuaValue::Boolean(b) => b,
        other => {
            return wrap_err!("{} expected enabled to be a boolean, got: {:?}", function_name, other);
        }
    };

    if let Err(err) = if should_capture {
        execute!(
            std::io::stdout(),
            crossterm::event::EnableMouseCapture,
        )
    } else {
        execute!(
            std::io::stdout(),
            crossterm::event::DisableMouseCapture,
        )
    } {
        return wrap_err!("{}: cannot {} terminal mouse capture due to err: {}", function_name, if should_capture { "enable" } else { "disable" }, err);
    }

    Ok(())
}

fn capture_focus(_luau: &Lua, value: LuaValue) -> LuaEmptyResult {
    let function_name = "terminal.capture.focus(enabled: boolean)";
    let should_capture = match value {
        LuaValue::Boolean(b) => b,
        other => {
            return wrap_err!("{} expected enabled to be a boolean, got: {:?}", function_name, other);
        }
    };

    if let Err(err) = if should_capture {
        execute!(
            std::io::stdout(),
            crossterm::event::EnableFocusChange,
        )
    } else {
        execute!(
            std::io::stdout(),
            crossterm::event::DisableFocusChange,
        )
    } {
        return wrap_err!("{}: cannot {} terminal focus capture due to err: {}", function_name, if should_capture { "enable" } else { "disable" }, err);
    }

    Ok(())
}
fn capture_paste(_luau: &Lua, value: LuaValue) -> LuaEmptyResult {
    let function_name = "terminal.capture.paste(enabled: boolean)";
    let should_capture = match value {
        LuaValue::Boolean(b) => b,
        other => {
            return wrap_err!("{} expected enabled to be a boolean, got: {:?}", function_name, other);
        }
    };

    if let Err(err) = if should_capture {
        execute!(
            std::io::stdout(),
            crossterm::event::EnableBracketedPaste,
        )
    } else {
        execute!(
            std::io::stdout(),
            crossterm::event::DisableBracketedPaste,
        )
    } {
        return wrap_err!("{}: cannot {} terminal bracketed paste capture due to err: {}", function_name, if should_capture { "enable" } else { "disable" }, err);
    }

    Ok(())
}

pub(super) fn events(luau: &Lua, value: LuaValue) -> LuaResult<LuaFunction> {
    let function_name = "input.events(poll: Duration?)";
    let poll_duration = match value {
        LuaValue::UserData(ud) => {
            if let Ok(duration) = ud.borrow::<TimeDuration>() {
                if duration.inner.is_negative() {
                    return wrap_err!("{}: cannot poll for a negative duration", function_name);
                } else {
                    duration.inner.unsigned_abs()
                }
            } else {
                let type_name = ud.type_name()?.unwrap_or(String::from("userdata"));
                return wrap_err!("{} expected poll to be a Duration, got a different kind of userdata: {}", function_name, type_name);
            }
        },
        LuaNil => {
            Duration::from_millis(50)
        },
        other => {
            return wrap_err!("{} expected poll to be a Duration, got: {:?}", function_name, other);
        }
    };

    let empty_event_table = TableBuilder::create(luau)?
        .with_value("type", "Empty")?
        .build_readonly()?;

    let modifiers_table = create_table_with_capacity(luau, 0, 3)?;
    luau.set_named_registry_value("InputKeyModifiers", modifiers_table)?;

    let empty_event_registry_key = luau.create_registry_value(empty_event_table)?;

    let f = luau.create_function(move | luau: &Lua, _: LuaValue | -> LuaValueResult {
        if let Ok(b) = crossterm::event::poll(poll_duration) && b {
            let event =  match crossterm::event::read() {
                Ok(event) => event,
                Err(err) => {
                    return wrap_err!("event not eventing due to err: {}", err);
                }
            };
            ok_table(create_event_table(luau, event))
        } else {
            let empty_event_table: LuaTable = luau.registry_value(&empty_event_registry_key)?;
            ok_table(Ok(empty_event_table))
        }
    })?;

    Ok(f)
}

pub(super) fn create_capture_table(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_function("mouse", capture_mouse)?
        .with_function("focus", capture_focus)?
        .with_function("paste", capture_paste)?
        .build_readonly()
}