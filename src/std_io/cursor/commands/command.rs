use mluau::prelude::*;
use crate::prelude::*;

use std::fmt::Display;

use super::super::CursorStyle;

#[derive(Clone)]
pub enum ColumnOrRow {
    Column(u16),
    Row(u16),
    Both {
        column: u16,
        row: u16,
    },
}

#[derive(Clone)]
pub enum CursorCommand {
    Save,
    Restore,
    SetStyle(CursorStyle),
    Show,
    Hide,

    // movement commands
    Up(u16),
    Down(u16),
    Left(u16),
    Right(u16),
    To(ColumnOrRow),
    NextLine(u16),
    PreviousLine(u16),

}
impl LuaUserData for CursorCommand {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_meta_field("__type", "CursorCommand");
    }
}

impl CursorCommand {
    pub fn get_userdata(self, luau: &Lua) -> LuaValueResult {
        Ok(LuaValue::UserData(luau.create_userdata(self)?))
    }
    pub fn from_value<S: Display>(value: &LuaValue, function_name: &'static str, what: S) -> LuaResult<Self> {
        match value {
            LuaValue::UserData(ud) if let Ok(ud) = ud.take::<CursorCommand>() => {
                Ok(ud)
            },
            other => {
                wrap_err!("{}: {} is not a CursorCommand from cursor.commands; got: {:?}", function_name, what, other)
            }
        }
    }
    pub fn queue(self, stdout: &mut std::io::Stdout, function_name: &'static str) -> LuaResult<()> {
        use crossterm::QueueableCommand;

        let result = match self {
            Self::Save => stdout.queue(crossterm::cursor::SavePosition),
            Self::Restore => stdout.queue(crossterm::cursor::RestorePosition),
            Self::SetStyle(style) => stdout.queue(style.into_crossterm()),
            Self::Show => stdout.queue(crossterm::cursor::Show),
            Self::Hide => stdout.queue(crossterm::cursor::Hide),
            Self::Up(n) => stdout.queue(crossterm::cursor::MoveUp(n)),
            Self::Down(n) => stdout.queue(crossterm::cursor::MoveDown(n)),
            Self::Left(n) => stdout.queue(crossterm::cursor::MoveLeft(n)),
            Self::Right(n) => stdout.queue(crossterm::cursor::MoveRight(n)),
            Self::To(column_or_row) => match column_or_row {
                ColumnOrRow::Column(col) => stdout.queue(crossterm::cursor::MoveToColumn(col)),
                ColumnOrRow::Row(row) => stdout.queue(crossterm::cursor::MoveToRow(row)),
                ColumnOrRow::Both { column, row } => stdout.queue(crossterm::cursor::MoveTo(column, row)),
            },
            Self::NextLine(n) => stdout.queue(crossterm::cursor::MoveToNextLine(n)),
            Self::PreviousLine(n) => stdout.queue(crossterm::cursor::MoveToPreviousLine(n)),
        };
        
        if let Err(err) = result {
            return wrap_err!("{}: failed to queue cursor command: {}", function_name, err);
        }

        Ok(())
    }
}