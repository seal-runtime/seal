use mluau::prelude::*;
use crate::prelude::*;
use crate::std_err::WrappedError;

use crossterm::event::{Event, KeyEvent, KeyModifiers, MouseEvent};
use crossterm::execute;

use std::borrow::Cow;
use std::time::Duration;
use crate::std_time::duration::TimeDuration;

fn canonicalize_keycodes(code: crossterm::event::KeyCode) -> Cow<'static, str> {
    use crossterm::event::KeyCode;
    match code {
        KeyCode::Backspace => Cow::Borrowed("Backspace"),
        KeyCode::Enter => Cow::Borrowed("Enter"),
        KeyCode::Left => Cow::Borrowed("Left"),
        KeyCode::Right => Cow::Borrowed("Right"),
        KeyCode::Up => Cow::Borrowed("Up"),
        KeyCode::Down => Cow::Borrowed("Down"),
        KeyCode::Home => Cow::Borrowed("Home"),
        KeyCode::End => Cow::Borrowed("End"),
        KeyCode::PageUp => Cow::Borrowed("Page Up"),
        KeyCode::PageDown => Cow::Borrowed("Page Down"),
        KeyCode::Tab => Cow::Borrowed("Tab"),
        KeyCode::BackTab => Cow::Borrowed("Back Tab"),
        KeyCode::Delete => Cow::Borrowed("Delete"),
        KeyCode::Insert => Cow::Borrowed("Insert"),
        KeyCode::F(n) => Cow::Owned(format!("F{}", n)),
        KeyCode::Char(' ') => Cow::Borrowed("Space"),
        KeyCode::Char(c) => Cow::Owned(c.to_lowercase().to_string()),
        KeyCode::Null => Cow::Borrowed("Null"),
        KeyCode::Esc => Cow::Borrowed("Esc"),
        KeyCode::CapsLock => Cow::Borrowed("Caps Lock"),
        KeyCode::ScrollLock => Cow::Borrowed("Scroll Lock"),
        KeyCode::NumLock => Cow::Borrowed("Num Lock"),
        KeyCode::PrintScreen => Cow::Borrowed("Print Screen"),
        KeyCode::Pause => Cow::Borrowed("Pause"),
        KeyCode::Menu => Cow::Borrowed("Menu"),
        KeyCode::KeypadBegin => Cow::Borrowed("Begin"),
        KeyCode::Media(media) => canonicalize_media_code(media),
        KeyCode::Modifier(modifier) => canonicalize_modifier_code(modifier),
    }
}

fn canonicalize_media_code(code: crossterm::event::MediaKeyCode) -> Cow<'static, str> {
    use crossterm::event::MediaKeyCode;
    match code {
        MediaKeyCode::Play => Cow::Borrowed("Play"),
        MediaKeyCode::Pause => Cow::Borrowed("Pause"),
        MediaKeyCode::PlayPause => Cow::Borrowed("Play/Pause"),
        MediaKeyCode::Reverse => Cow::Borrowed("Reverse"),
        MediaKeyCode::Stop => Cow::Borrowed("Stop"),
        MediaKeyCode::FastForward => Cow::Borrowed("Fast Forward"),
        MediaKeyCode::Rewind => Cow::Borrowed("Rewind"),
        MediaKeyCode::TrackNext => Cow::Borrowed("Next Track"),
        MediaKeyCode::TrackPrevious => Cow::Borrowed("Previous Track"),
        MediaKeyCode::Record => Cow::Borrowed("Record"),
        MediaKeyCode::LowerVolume => Cow::Borrowed("Lower Volume"),
        MediaKeyCode::RaiseVolume => Cow::Borrowed("Raise Volume"),
        MediaKeyCode::MuteVolume => Cow::Borrowed("Mute Volume"),
    }
}

fn canonicalize_modifier_code(code: crossterm::event::ModifierKeyCode) -> Cow<'static, str> {
    use crossterm::event::ModifierKeyCode;
    match code {
        ModifierKeyCode::LeftShift => Cow::Borrowed("Left Shift"),
        ModifierKeyCode::RightShift => Cow::Borrowed("Right Shift"),
        ModifierKeyCode::LeftControl => Cow::Borrowed("Left Ctrl"),
        ModifierKeyCode::RightControl => Cow::Borrowed("Right Ctrl"),
        ModifierKeyCode::LeftAlt => Cow::Borrowed("Left Alt"),
        ModifierKeyCode::RightAlt => Cow::Borrowed("Right Alt"),
        ModifierKeyCode::LeftSuper => Cow::Borrowed("Left Super"),
        ModifierKeyCode::RightSuper => Cow::Borrowed("Right Super"),
        ModifierKeyCode::LeftHyper => Cow::Borrowed("Left Hyper"),
        ModifierKeyCode::RightHyper => Cow::Borrowed("Right Hyper"),
        ModifierKeyCode::LeftMeta => Cow::Borrowed("Left Meta"),
        ModifierKeyCode::RightMeta => Cow::Borrowed("Right Meta"),
        ModifierKeyCode::IsoLevel3Shift => Cow::Borrowed("Iso Level 3 Shift"),
        ModifierKeyCode::IsoLevel5Shift => Cow::Borrowed("Iso Level 5 Shift"),
    }
}

fn create_event_table(luau: &Lua, event: Event) -> LuaResult<LuaTable> {
    let t = create_table_with_capacity(luau, 0, 3)?;

    fn table_from_modifiers(luau: &Lua, modifiers: KeyModifiers) -> LuaResult<LuaTable> {
        let t: LuaTable = luau.named_registry_value("InputKeyModifiers")?;
        t.raw_set("shift", modifiers.contains(KeyModifiers::SHIFT))?;
        t.raw_set("ctrl", modifiers.contains(KeyModifiers::CONTROL))?;
        t.raw_set("alt", modifiers.contains(KeyModifiers::ALT))?;
        Ok(t)
    }

    match event {
        Event::Key(KeyEvent { code, modifiers, kind, .. }) => {
            t.raw_set("type", "Key")?;
            t.raw_set("key", canonicalize_keycodes(code).as_ref())?;
            t.raw_set("kind", match kind {
                crossterm::event::KeyEventKind::Press => "Press",
                crossterm::event::KeyEventKind::Release => "Release",
                crossterm::event::KeyEventKind::Repeat => "Repeat",
            })?;
            t.raw_set("modifiers", table_from_modifiers(luau, modifiers)?)?;
        },
        Event::Mouse(MouseEvent { kind, column, row, modifiers }) => {
            use crossterm::event::{MouseButton, MouseEventKind};
            t.raw_set("type", "Mouse")?;
            t.raw_set("kind", match kind {
                MouseEventKind::Down(MouseButton::Left) => "Down(Left)",
                MouseEventKind::Down(MouseButton::Right) => "Down(Right)",
                MouseEventKind::Down(MouseButton::Middle) => "Down(Middle)",
                MouseEventKind::Up(MouseButton::Left) => "Up(Left)",
                MouseEventKind::Up(MouseButton::Right) => "Up(Right)",
                MouseEventKind::Up(MouseButton::Middle) => "Up(Middle)",
                MouseEventKind::Drag(MouseButton::Left) => "Drag(Left)",
                MouseEventKind::Drag(MouseButton::Right) => "Drag(Right)",
                MouseEventKind::Drag(MouseButton::Middle) => "Drag(Middle)",
                MouseEventKind::Moved => "Moved",
                MouseEventKind::ScrollDown => "ScrollDown",
                MouseEventKind::ScrollUp => "ScrollUp",
                MouseEventKind::ScrollLeft => "ScrollLeft",
                MouseEventKind::ScrollRight => "ScrollRight",
            })?;
            t.raw_set("position", LuaValue::Vector(LuaVector::new(column as f32, row as f32, 0.0_f32)))?;
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
            t.raw_set("size", LuaValue::Vector(LuaVector::new(columns as f32, rows as f32, 0.0_f32)))?;
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

enum InterruptCode {
    CtrlC,
    CtrlD,
}
pub struct Interrupt {
    code: InterruptCode
}

impl Interrupt {
    pub fn ctrlc() -> Self {
        Self {
            code: InterruptCode::CtrlC
        }
    }
    pub fn ctrld() -> Self {
        Self {
            code: InterruptCode::CtrlD
        }
    }
    pub fn get_userdata(self, luau: &Lua) -> LuaValueResult {
        ok_userdata(self, luau)
    }
}

impl LuaUserData for Interrupt {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_meta_field("__type", "interrupt"); // allow users to typeof check
        fields.add_field_method_get("code", |luau: &Lua, this: &Interrupt| {
            match this.code {
                InterruptCode::CtrlC => "CtrlC".into_lua(luau),
                InterruptCode::CtrlD => "CtrlD".into_lua(luau),
            }
        });
    }
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, | luau: &Lua, this: &Interrupt, _: LuaValue| -> LuaValueResult {
            match this.code {
                InterruptCode::CtrlC => "CtrlC (SIGINT)".into_lua(luau),
                InterruptCode::CtrlD => "CtrlD (EOF)".into_lua(luau),
            }
        });
    }
}

fn interrupt_sigint(luau: &Lua, _: LuaValue) -> LuaValueResult {
    Interrupt::ctrlc().get_userdata(luau)
}

fn interrupt_eof(luau: &Lua, _: LuaValue) -> LuaValueResult {
    Interrupt::ctrld().get_userdata(luau)
}

const INTERRUPT_CHECK_SRC: &str = include_str!("interrupt_check.luau");

fn interrupt_check(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "interrupt.check(event: TerminalEvent)";
    
    let function = if let Some(function) = luau.named_registry_value::<Option<LuaFunction>>("terminal/interrupt_check")? {
        function
    } else {
        let function = luau.load(INTERRUPT_CHECK_SRC).eval::<LuaFunction>()?;
        luau.set_named_registry_value("terminal/interrupt_check", &function)?;
        function
    };

    let interrupt = match function.call::<LuaValue>(value) {
        Ok(LuaValue::Boolean(b)) if b => Interrupt::ctrlc(),
        Ok(LuaValue::Boolean(b)) if !b => Interrupt::ctrld(),
        Ok(LuaNil) => {
            return Ok(LuaNil);
        },
        Ok(LuaValue::UserData(ud)) if let Ok(err) = ud.borrow::<WrappedError>() => {
            return wrap_err!("{}: {}", function_name, err.format());
        },
        Ok(other) => {
            panic!("{} returned an unexpected type of value: {:?}", function_name, other);
        },
        Err(err) => {
            panic!("{} errored at runtime: {}", function_name, err);
        }
    };
    interrupt.get_userdata(luau)
}

pub(super) fn create_interrupt_table(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_function("sigint", interrupt_sigint)?
        .with_function("eof", interrupt_eof)?
        .with_function("check", interrupt_check)?
        .build_readonly()
}

pub(super) fn create_capture_table(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_function("mouse", capture_mouse)?
        .with_function("focus", capture_focus)?
        .with_function("paste", capture_paste)?
        .build_readonly()
}