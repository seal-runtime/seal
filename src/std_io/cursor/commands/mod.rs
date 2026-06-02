use std::io::Write;

use mluau::prelude::*;
use crate::prelude::*;

pub(super) mod command;
use command::CursorCommand;

pub(super) fn queue_and_execute(commands: Vec<CursorCommand>, function_name: &'static str) -> LuaResult<()> {
    let mut stdout = std::io::stdout();
    for command in commands {
        command.queue(&mut stdout, function_name)?;
    }

    if let Err(err) = stdout.flush() {
        return wrap_err!("{}: unable to flush stdout due to err: {}", function_name, err);
    }

    Ok(())
}

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

fn commands_save(luau: &Lua, _value: LuaValue) -> LuaValueResult {
    let _function_name = "cursor.commands.save()";
    CursorCommand::Save.get_userdata(luau)
}

fn commands_restore(luau: &Lua, _value: LuaValue) -> LuaValueResult {
    let _function_name = "cursor.commands.restore()";
    CursorCommand::Restore.get_userdata(luau)
}

fn commands_up(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "cursor.commands.up(lines: number)";
    let lines = convert_number_param_to_u16(value, function_name, "lines")?;

    CursorCommand::Up(lines).get_userdata(luau)
}

fn commands_down(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "cursor.commands.down(lines: number)";
    let lines = convert_number_param_to_u16(value, function_name, "lines")?;

    CursorCommand::Down(lines).get_userdata(luau)
}

fn commands_left(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "cursor.commands.left(cols: number)";
    let cols = convert_number_param_to_u16(value, function_name, "cols")?;

    CursorCommand::Left(cols).get_userdata(luau)
}

fn commands_right(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "cursor.commands.right(cols: number)";
    let cols = convert_number_param_to_u16(value, function_name, "cols")?;

    CursorCommand::Right(cols).get_userdata(luau)
}

enum MoveToOverload {
    ColumnsAndRows,
    Vector
}

fn commands_to(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
    // this function is overloaded, users can pass in a number or a vector and we have to decide which one to use
    let function_name_1 = "cursor.commands.to(column: number, row: number)";
    let function_name_2 = "cursor.commands.to(position: vector)";

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

    CursorCommand::To(command::ColumnOrRow::Both { column, row }).get_userdata(luau)
}

fn commands_column(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "cursor.commands.column(c: number)";
    let column = convert_number_param_to_u16(value, function_name, "c")?;

    CursorCommand::To(command::ColumnOrRow::Column(column)).get_userdata(luau)
}

fn commands_row(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "cursor.commands.row(r: number)";
    let row = convert_number_param_to_u16(value, function_name, "r")?;

    CursorCommand::To(command::ColumnOrRow::Row(row)).get_userdata(luau)
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

fn commands_nextline(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "cursor.commands.nextline(n: number?)";
    let n = convert_optional_number_param_to_u16(value, function_name, "n")?;

    CursorCommand::NextLine(n).get_userdata(luau)
}

fn commands_prevline(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "cursor.commands.prevline(n: number?)";
    let n = convert_optional_number_param_to_u16(value, function_name, "n")?;

    CursorCommand::PreviousLine(n).get_userdata(luau)
}

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_function("save", commands_save)?
        .with_function("restore", commands_restore)?
        .with_function("up", commands_up)?
        .with_function("down", commands_down)?
        .with_function("left", commands_left)?
        .with_function("right", commands_right)?
        .with_function("to", commands_to)?
        .with_function("column", commands_column)?
        .with_function("row", commands_row)?
        .with_function("nextline", commands_nextline)?
        .with_function("prevline", commands_prevline)?
        .build_readonly()

}