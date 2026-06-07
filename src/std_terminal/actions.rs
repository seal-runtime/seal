use mluau::prelude::*;
use crate::{prelude::*};
use std::io::Write;

use std::fmt::Display;

use super::cursor::{CursorStyle, MoveDirection};
use super::{ScrollDirection, WhichScreen};
use crossterm::execute;
use crossterm::QueueableCommand;

pub(super) fn queue_and_execute(actions: Vec<TerminalAction>, function_name: &'static str) -> LuaResult<()> {
    let mut stdout = std::io::stdout();
    if let Err(err) = stdout.queue(crossterm::terminal::BeginSynchronizedUpdate) {
        return wrap_err!("{}: failed to queue BeginSynchronizedUpdate: {}", function_name, err);
    }
    for command in actions {
        command.queue(&mut stdout, function_name)?;
    }
    if let Err(err) = stdout.queue(crossterm::terminal::EndSynchronizedUpdate) {
        return wrap_err!("{}: failed to queue EndSynchronizedUpdate: {}", function_name, err);
    }
    if let Err(err) = stdout.flush() {
        return wrap_err!("{}: unable to flush stdout due to err: {}", function_name, err);
    }

    Ok(())
}

#[derive(Clone)]
pub enum TerminalAction {
    Write(String),
    Title(String),
    Scroll(ScrollDirection),
    Clear(crossterm::terminal::ClearType),
    Switch(WhichScreen),
    Linewrap(bool),

    // cursor actions
    Save,
    Restore,
    SetStyle(CursorStyle),
    Show,
    Hide,
    MoveCursor(MoveDirection)
}
impl LuaUserData for TerminalAction {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_meta_field("__type", "TerminalAction");
    }
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("execute", |_luau, this, _: LuaValue| -> LuaEmptyResult {
            let function_name = "TerminalAction:execute()";
            let mut stdout = std::io::stdout();

            let result = match this {
                TerminalAction::Write(content) => execute!(stdout, crossterm::style::Print(content)),
                TerminalAction::Title(title) => execute!(stdout, crossterm::terminal::SetTitle(title)),
                TerminalAction::Clear(clear_type) => execute!(stdout, crossterm::terminal::Clear(*clear_type)),
                TerminalAction::Scroll(direction) => match direction {
                    ScrollDirection::Up(lines) => execute!(stdout, crossterm::terminal::ScrollUp(*lines)),
                    ScrollDirection::Down(lines) => execute!(stdout, crossterm::terminal::ScrollDown(*lines)),
                    ScrollDirection::None => Ok(()),
                },
                TerminalAction::Switch(screen) => match screen {
                    WhichScreen::Alternate => execute!(stdout, crossterm::terminal::EnterAlternateScreen),
                    WhichScreen::Main => execute!(stdout, crossterm::terminal::LeaveAlternateScreen),
                },
                TerminalAction::Linewrap(enabled) => if *enabled {
                    execute!(stdout, crossterm::terminal::EnableLineWrap)
                } else {
                    execute!(stdout, crossterm::terminal::DisableLineWrap)
                },
                TerminalAction::Save => execute!(stdout, crossterm::cursor::SavePosition),
                TerminalAction::Restore => execute!(stdout, crossterm::cursor::RestorePosition),
                TerminalAction::SetStyle(style) => execute!(stdout, style.clone().into_crossterm()),
                TerminalAction::Show => execute!(stdout, crossterm::cursor::Show),
                TerminalAction::Hide => execute!(stdout, crossterm::cursor::Hide),
                TerminalAction::MoveCursor(direction) => direction.clone().execute(&mut stdout),
            };

            if let Err(err) = result {
                return wrap_err!("{}: unable to execute command due to err: {}", function_name, err);
            }

            Ok(())
        });
    }
}

impl TerminalAction {
    pub fn get_userdata(self, luau: &Lua) -> LuaValueResult {
        Ok(LuaValue::UserData(luau.create_userdata(self)?))
    }
    pub fn from_value<S: Display>(value: &LuaValue, function_name: &'static str, what: S) -> LuaResult<Self> {
        match value {
            LuaValue::UserData(ud) if let Ok(ud) = ud.borrow::<TerminalAction>() => {
                Ok((*ud).clone())
            },
            other => {
                wrap_err!("{}: {} is not a TerminalAction from @std/terminal; got: {:#?}", function_name, what, other)
            }
        }
    }
    pub fn queue(self, stdout: &mut std::io::Stdout, function_name: &'static str) -> LuaResult<()> {
        let result = match self {
            Self::Write(content) => stdout.queue(crossterm::style::Print(content)),
            Self::Title(title) => stdout.queue(crossterm::terminal::SetTitle(title)),
            Self::Clear(clear_type) => stdout.queue(crossterm::terminal::Clear(clear_type)),
            Self::Scroll(direction) => match direction {
                ScrollDirection::Up(lines) => stdout.queue(crossterm::terminal::ScrollUp(lines)),
                ScrollDirection::Down(lines) => stdout.queue(crossterm::terminal::ScrollDown(lines)),
                ScrollDirection::None => Ok(stdout),
            },
            Self::Switch(screen) => match screen {
                WhichScreen::Alternate => stdout.queue(crossterm::terminal::EnterAlternateScreen),
                WhichScreen::Main => stdout.queue(crossterm::terminal::LeaveAlternateScreen),
            },
            Self::Linewrap(enabled) => if enabled {
                stdout.queue(crossterm::terminal::EnableLineWrap)
            } else {
                stdout.queue(crossterm::terminal::DisableLineWrap)
            },
            Self::Save => stdout.queue(crossterm::cursor::SavePosition),
            Self::Restore => stdout.queue(crossterm::cursor::RestorePosition),
            Self::SetStyle(style) => stdout.queue(style.into_crossterm()),
            Self::Show => stdout.queue(crossterm::cursor::Show),
            Self::Hide => stdout.queue(crossterm::cursor::Hide),
            Self::MoveCursor(direction) => direction.queue(stdout),
        };
        
        if let Err(err) = result {
            return wrap_err!("{}: failed to queue cursor command: {}", function_name, err);
        }

        Ok(())
    }
}
