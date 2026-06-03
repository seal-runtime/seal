use crossterm::QueueableCommand;
use mluau::prelude::*;
use crate::prelude::*;

pub(super) mod cursor_style;
pub use cursor_style::CursorStyle;

use super::TerminalAction;
use std::io::Stdout;

fn convert_number_param_to_u16(value: LuaValue, function_name: &'static str, parameter_name: &'static str) -> LuaResult<u16> {
    let converted = match value {
        LuaValue::Number(f) => float_to_u16(f, function_name, parameter_name)?,
        LuaValue::Integer(i) => int_to_u16(i, function_name, parameter_name)?,
        other => {
            return wrap_err!("{}: expected '{}' to be a positive whole number, got: {:?}", function_name, parameter_name, other);
        }
    };
    Ok(converted)
}

fn convert_optional_number_param_to_u16(value: LuaValue, function_name: &'static str, parameter_name: &'static str) -> LuaResult<u16> {
    let converted = match value {
        LuaValue::Number(f) => float_to_u16(f, function_name, parameter_name)?,
        LuaValue::Integer(i) => int_to_u16(i, function_name, parameter_name)?,
        LuaNil => 1_u16,
        other => {
            return wrap_err!("{}: expected '{}' to be a positive whole number or nil (default 1), got: {:?}", function_name, parameter_name, other);
        }
    };
    Ok(converted)
}

fn cursor_position(_luau: &Lua, _value: LuaValue) -> LuaValueResult {
    let function_name = "cursor.position()";
    let (column, row) = match crossterm::cursor::position() {
        Ok(pos) => pos,
        Err(err) => {
            return wrap_err!("{}: unable to determine cursor position due to err {}", function_name, err);
        }
    };

    Ok(LuaValue::Vector(LuaVector::new(column as f32, row as f32, 0.0_f32)))
}

fn cursor_save(luau: &Lua, _value: LuaValue) -> LuaValueResult {
    TerminalAction::Save.get_userdata(luau)
}

fn cursor_restore(luau: &Lua, _value: LuaValue) -> LuaValueResult {
    TerminalAction::Restore.get_userdata(luau)
}

fn cursor_show(luau: &Lua, _value: LuaValue) -> LuaValueResult {
    TerminalAction::Show.get_userdata(luau)
}

fn cursor_hide(luau: &Lua, _value: LuaValue) -> LuaValueResult {
    TerminalAction::Hide.get_userdata(luau)
}

fn cursor_style(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "cursor.style(mode: CursorStyle)";
    let style = CursorStyle::from_luau(value, function_name)?;
    TerminalAction::SetStyle(style).get_userdata(luau)
}

#[derive(Clone)]
pub enum MoveDirection {
    Up(u16),
    Down(u16),
    Left(u16),
    Right(u16),
    ToPosition { column: u16, row: u16 },
    Column(u16),
    Row(u16),
    NextLine(u16),
    PreviousLine(u16),
}
impl MoveDirection {
    pub fn queue(self, stdout: &mut Stdout) -> Result<&mut Stdout, std::io::Error> {
        match self {
            Self::Up(rows) => stdout.queue(crossterm::cursor::MoveUp(rows)),
            Self::Down(rows) => stdout.queue(crossterm::cursor::MoveDown(rows)),
            Self::Left(columns) => stdout.queue(crossterm::cursor::MoveLeft(columns)),
            Self::Right(columns) => stdout.queue(crossterm::cursor::MoveRight(columns)),
            Self::Row(row) => stdout.queue(crossterm::cursor::MoveToRow(row)),
            Self::Column(column) => stdout.queue(crossterm::cursor::MoveToColumn(column)),
            Self::ToPosition { column, row } => stdout.queue(crossterm::cursor::MoveTo(column, row)),
            Self::NextLine(n) => stdout.queue(crossterm::cursor::MoveToNextLine(n)),
            Self::PreviousLine(n) => stdout.queue(crossterm::cursor::MoveToPreviousLine(n)),
        }
    }
    pub fn execute(self, stdout: &mut Stdout) -> Result<(), std::io::Error> {
        use crossterm::execute;
        match self {
            Self::Up(rows) => execute!(stdout, crossterm::cursor::MoveUp(rows)),
            Self::Down(rows) => execute!(stdout, crossterm::cursor::MoveDown(rows)),
            Self::Left(columns) => execute!(stdout, crossterm::cursor::MoveLeft(columns)),
            Self::Right(columns) => execute!(stdout, crossterm::cursor::MoveRight(columns)),
            Self::Row(row) => execute!(stdout, crossterm::cursor::MoveToRow(row)),
            Self::Column(column) => execute!(stdout, crossterm::cursor::MoveToColumn(column)),
            Self::ToPosition { column, row } => execute!(stdout, crossterm::cursor::MoveTo(column, row)),
            Self::NextLine(n) => execute!(stdout, crossterm::cursor::MoveToNextLine(n)),
            Self::PreviousLine(n) => execute!(stdout, crossterm::cursor::MoveToPreviousLine(n)),
        }
    }
}

fn cursor_up(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "cursor.up(rows: number)";
    let rows = convert_number_param_to_u16(value, function_name, "rows")?;

    TerminalAction::MoveCursor(MoveDirection::Up(rows)).get_userdata(luau)
}

fn cursor_down(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "cursor.down(rows: number)";
    let rows = convert_number_param_to_u16(value, function_name, "rows")?;

    TerminalAction::MoveCursor(MoveDirection::Down(rows)).get_userdata(luau)
}

fn cursor_left(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "cursor.left(cols: number)";
    let cols = convert_number_param_to_u16(value, function_name, "cols")?;

    TerminalAction::MoveCursor(MoveDirection::Left(cols)).get_userdata(luau)
}

fn cursor_right(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "cursor.right(cols: number)";
    let cols = convert_number_param_to_u16(value, function_name, "cols")?;

    TerminalAction::MoveCursor(MoveDirection::Right(cols)).get_userdata(luau)
}

enum MoveToOverload {
    ColumnsAndRows,
    Vector
}

fn cursor_to(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
    // this function is overloaded, users can pass in a number or a vector and we have to decide which one to use
    let function_name_1 = "cursor.to(column: number, row: number)";
    let function_name_2 = "cursor.to(position: vector)";

    if multivalue.is_empty() {
        return wrap_err!("{} & {}: expected to be called with 1 or 2 arguments but was called with zero arguments", function_name_1, function_name_2);
    }

    let first_argument = multivalue.pop_front().expect("argument list cannot be empty");

    let (function_name, which) = if matches!(first_argument, LuaValue::Number(_) | LuaValue::Integer(_)) {
        (function_name_1, MoveToOverload::ColumnsAndRows)
    } else if matches!(first_argument, LuaValue::Vector(_)) {
        (function_name_2, MoveToOverload::Vector)
    } else {
        return wrap_err!("{} & {}: expected the first argument (column or position) to be a number or vector, got: {:?}", function_name_1, function_name_2, first_argument);  
    };

    fn column_or_row_to_u16(argument: LuaValue, first: bool, function_name: &'static str, parameter_name: &'static str) -> LuaResult<u16> {
        let converted = match argument {
            LuaValue::Number(f) => float_to_u16(f, function_name, parameter_name)?,
            LuaValue::Integer(i) => int_to_u16(i, function_name, parameter_name)?,
            LuaValue::Vector(_) => {
                if first {
                    unreachable!("if first_argument is a vector we would've gone with the other overloaded")
                } else {
                    return wrap_err!("{}: you accidentally passed (number, vector) and therefore picked the wrong overload", function_name);
                }
            },
            other => {
                return wrap_err!("{}: expected '{}' to be a positive whole number, got: {:?}", function_name, parameter_name, other);
            }
        };
        Ok(converted)
    }

    let (column, row) = match which {
        MoveToOverload::ColumnsAndRows => {
            let column = column_or_row_to_u16(first_argument, true, function_name, "column")?;

            let second_argument = multivalue.pop_front();
            if second_argument.is_none() {
                return wrap_err!("{} called without required argument 'row'", function_name);
            }

            let second_argument = second_argument.expect("we just checked .is_none()");
            let row = column_or_row_to_u16(second_argument, false, function_name, "row")?;

            (column, row)
        },
        MoveToOverload::Vector => {
            let vec = match first_argument {
                LuaValue::Vector(vecc) => vecc,
                LuaValue::Number(_) | LuaValue::Integer(_) => unreachable!("if first_argument is a number we would've gone with the other overload"),
                other => {
                    return wrap_err!("{}: expected 'position' to be a vector (created with vector.create), got: {:?}", function_name, other);
                }
            };

            let column = float_to_u16(vec.x() as f64, function_name, "columns")?;
            let row = float_to_u16(vec.y() as f64, function_name, "rows")?;

            (column, row)
        }
    };

    TerminalAction::MoveCursor(MoveDirection::ToPosition { column, row }).get_userdata(luau)
}

fn cursor_column(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "cursor.column(c: number)";
    let column = convert_number_param_to_u16(value, function_name, "c")?;

    TerminalAction::MoveCursor(MoveDirection::Column(column)).get_userdata(luau)
}

fn cursor_row(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "cursor.row(r: number)";
    let row = convert_number_param_to_u16(value, function_name, "r")?;

    TerminalAction::MoveCursor(MoveDirection::Row(row)).get_userdata(luau)
}

fn cursor_nextline(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "cursor.nextline(n: number?)";
    let n = convert_optional_number_param_to_u16(value, function_name, "n")?;

    TerminalAction::MoveCursor(MoveDirection::NextLine(n)).get_userdata(luau)
}

fn cursor_prevline(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "cursor.prevline(n: number?)";
    let n = convert_optional_number_param_to_u16(value, function_name, "n")?;

    TerminalAction::MoveCursor(MoveDirection::PreviousLine(n)).get_userdata(luau)
}

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_function("position", cursor_position)?
        .with_function("save", cursor_save)?
        .with_function("restore", cursor_restore)?
        .with_function("show", cursor_show)?
        .with_function("hide", cursor_hide)?
        .with_function("style", cursor_style)?
        .with_function("up", cursor_up)?
        .with_function("down", cursor_down)?
        .with_function("left", cursor_left)?
        .with_function("right", cursor_right)?
        .with_function("to", cursor_to)?
        .with_function("column", cursor_column)?
        .with_function("row", cursor_row)?
        .with_function("nextline", cursor_nextline)?
        .with_function("prevline", cursor_prevline)?
        .build_readonly()

}