use mluau::prelude::*;
use crate::prelude::*;

mod cursor_style;
use cursor_style::CursorStyle;

mod commands;
use commands::command::CursorCommand;

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

fn cursor_show(_luau: &Lua, _value: LuaValue) -> LuaResult<()> {
    let function_name = "cursor.show()";
    let result = crossterm::execute!(
        std::io::stdout(),
        crossterm::cursor::Show,
    );

    if let Err(err) = result {
        return wrap_err!("{}: unable to show cursor due to err: {}", function_name, err);
    }

    Ok(())
}

fn cursor_hide(_luau: &Lua, _value: LuaValue) -> LuaResult<()> {
    let function_name = "cursor.hide()";
    let result = crossterm::execute!(
        std::io::stdout(),
        crossterm::cursor::Hide,
    );

    if let Err(err) = result {
        return wrap_err!("{}: unable to hide cursor due to err: {}", function_name, err);
    }

    Ok(())
}

fn cursor_style(_luau: &Lua, value: LuaValue) -> LuaResult<()> {
    let function_name = "cursor.style(mode: CursorStyle)";
    let style = CursorStyle::from_luau(value, function_name)?;
    let result = crossterm::execute!(
        std::io::stdout(),
        style.into_crossterm()
    );

    if let Err(err) = result {
        return wrap_err!("{}: unable to set cursor style due to err: {}", function_name, err);
    }

    Ok(())
}

fn cursor_move(_luau: &Lua, multivalue: LuaMultiValue) -> LuaResult<()> {
    let function_name = "cursor.move(commands: CursorCommand...)";

    let mut commands: Vec<CursorCommand> = Vec::with_capacity(multivalue.len());
    for (index, value) in multivalue.iter().enumerate() {
        let what = format!("argument at position {}", index);
        commands.push(CursorCommand::from_value(value, function_name, what)?);
    }

    commands::queue_and_execute(commands, function_name)?;

    Ok(())
}


pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_function("position", cursor_position)?
        .with_function("show", cursor_show)?
        .with_function("hide", cursor_hide)?
        .with_function("style", cursor_style)?
        .with_function("move", cursor_move)?
        .with_value("commands", commands::create(luau)?)?
        .build_readonly()
}