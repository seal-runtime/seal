use mluau::prelude::*;
use crate::prelude::*;

use crate::globals::warn;
use crate::std_err::WrappedError;
use crate::std_terminal::events::Interrupt;
use rustyline::error::ReadlineError;

use atty::Stream::{Stdout, Stderr};

use std::io::{self, Write};
use std::sync::OnceLock;

#[derive(Debug)]
pub struct ExpectSaneOutputStream {
    inner: OnceLock<(bool, bool)>,
}
impl ExpectSaneOutputStream {
    pub const fn uninitialized() -> Self {
        Self {
            inner: OnceLock::new()
        }
    }
    pub fn initialize_and_check(&self) {
        let stdout_sane = atty::is(Stdout);
        let stderr_sane = atty::is(Stderr);

        self.inner.set((stdout_sane, stderr_sane)).expect("ExpectSaneOutputStream already initialized smhmh");
    }
    pub fn stdout(&self) -> bool {
        self.inner.get().expect("ExpectSaneOutputStream not initialized").0
    }
    pub fn stderr(&self) -> bool {
        self.inner.get().expect("ExpectSaneOutputStream not initialized").1
    }
}

pub static EXPECT_OUTPUT_STREAMS: ExpectSaneOutputStream = ExpectSaneOutputStream::uninitialized();

fn input_get(luau: &Lua, raw_prompt: Option<String>) -> LuaValueResult {
    if let Some(prompt) = raw_prompt {
        put!("{}", prompt)?;
        match io::stdout().flush() {
            Ok(_) => {},
            Err(err) => {
                return wrap_err!("input.get: unable to flush stdout due to err: {}", err);
            }
        }
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim_end().to_string();

    Ok(LuaValue::String(luau.create_string(&input)?))
}


pub fn input_rawline(_: &Lua, raw_prompt: Option<String>) -> LuaResult<String> {
    if let Some(prompt) = raw_prompt {
        put!("{}", prompt)?;
        io::stdout().flush()?;
    }

    let mut input= String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim_end().to_string();

    Ok(input)
}

pub fn input_readline(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
    let function_name = "input.readline(prompt: string)";
    let prompt = match multivalue.pop_front() {
        Some(LuaValue::String(s)) => s.to_string_lossy(),
        Some(LuaNil) | None => {
            return wrap_err!("{} expected message to be a string, got nothing or nil", function_name);
        },
        Some(other) => {
            return wrap_err!("{} expected message to be a string, got: {:?}", function_name, other);
        }
    };

    if atty::isnt(atty::Stream::Stdin) || atty::isnt(atty::Stream::Stdout) {
        match input_rawline(luau, if prompt.is_empty() { None } else { Some(prompt) }) {
            Ok(s) => {
                return ok_string(s, luau);
            },
            Err(err) => {
                return wrap_err!("{}: unable to fallback to non-tty due to err: {}", function_name, err);
            }
        }
    }

    let mut rl = match rustyline::DefaultEditor::new() {
        Ok(editor) => editor,
        Err(err) => {
            return wrap_err!("{}: unable to make rustyline DefaultEditor due to ReadlineError: {}", function_name, err);
        }
    };

    let line = match rl.readline(&prompt) {
        Ok(line) => {
            if let Err(err) = rl.add_history_entry(line.as_str()) {
                warn(luau, ok_string(format!("error adding prompt history: {}", err), luau)?)?;
            }
            line
        },
        Err(ReadlineError::Interrupted) => {
            return Interrupt::ctrlc().get_userdata(luau);
        },
        Err(ReadlineError::Eof) => {
            return Interrupt::ctrld().get_userdata(luau);
        },
        Err(ReadlineError::Io(err)) => {
            return WrappedError::from_message(format!("terminal io error: {}", err)).get_userdata(luau);
        }
        Err(err) => {
            return wrap_err!("{}: encountered unexpected ReadlineError: {}", function_name, err);
        }
    };

    ok_string(line, luau)
}

pub fn input_editline(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
    let function_name = "input.editline(prompt: string, left: string, right: string?)";

    let prompt = match multivalue.pop_front() {
        Some(LuaValue::String(s)) => s.to_string_lossy(),
        Some(LuaNil) | None => {
            return wrap_err!("{} expected message to be a string, got nothing or nil", function_name);
        },
        Some(other) => {
            return wrap_err!("{} expected message to be a string, got: {:?}", function_name, other);
        }
    };

    let left = match multivalue.pop_front() {
        Some(LuaValue::String(left)) => left.to_string_lossy(),
        Some(LuaNil) | None => {
            return wrap_err!("{} expected string to the left of cursor to be a string, got nothing or nil", function_name);
        },
        Some(other) => {
            return wrap_err!("{} expected string to the left of cursor to be a string, got: {:?}", function_name, other);
        }
    };

    let right = match multivalue.pop_front() {
        Some(LuaValue::String(right)) => Some(right.to_string_lossy()),
        Some(LuaNil) | None => None,
        Some(other) => {
            return wrap_err!("{} expected string to the right of cursor to be a string or nil/unspecified, got: {:?}", function_name, other);
        }
    };

    if atty::isnt(atty::Stream::Stdin) || atty::isnt(atty::Stream::Stdout) {
        // not an amazing solution because program will have to copy/read from stdout, gsub, and send left + response + right back
        let combined = left.clone() + "<CURSOR>" + &right.unwrap_or_default();
        puts!("{}", &combined)?;
        match input_rawline(luau, Some(combined)) {
            Ok(s) => {
                return ok_string(s, luau);
            },
            Err(err) => {
                return wrap_err!("{}: unable to fallback to non-tty due to err: {}", function_name, err);
            }
        }
    }

    let mut rl = match rustyline::DefaultEditor::new() {
        Ok(editor) => editor,
        Err(err) => {
            return wrap_err!("{}: unable to make rustyline DefaultEditor due to ReadlineError: {}", function_name, err);
        }
    };

    let line = match rl.readline_with_initial(&prompt, (&left, &right.unwrap_or_default())) {
        Ok(line) => {
            if let Err(err) = rl.add_history_entry(line.as_str()) {
                warn(luau, ok_string(format!("error adding prompt history: {}", err), luau)?)?;
            }
            line
        },
        Err(ReadlineError::Interrupted) => {
            return Interrupt::ctrlc().get_userdata(luau);
        },
        Err(ReadlineError::Eof) => {
            return Interrupt::ctrld().get_userdata(luau);
        },
        Err(ReadlineError::Io(err)) => {
            return WrappedError::from_message(format!("terminal io error: {}", err)).get_userdata(luau);
        }
        Err(err) => {
            return wrap_err!("{}: encountered unexpected ReadlineError: {}", function_name, err);
        }
    };

    ok_string(line, luau)
}

fn input_interrupt(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "input.interrupt(code: \"CtrlC\" | \"CtrlD\")";
    match value {
        LuaValue::String(c) => {
            let bytes = c.as_bytes();
            if bytes == &b"CtrlC"[..] {
                Interrupt::ctrlc().get_userdata(luau)
            } else if bytes == &b"CtrlD"[..] {
                Interrupt::ctrld().get_userdata(luau)
            } else {
                wrap_err!("{} expected code to be \"CtrlC\" or \"CtrlD\", got some other string: {}", function_name, c.display())
            }
        },
        other => {
            wrap_err!("{} expected code to be \"CtrlC\" or \"CtrlD\", got: {:?}", function_name, other)
        }
    }
}



pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_function("get", input_get)?
        .with_function("readline", input_readline)?
        .with_function("editline", input_editline)?
        .with_function("rawline", input_rawline)?
        .with_function("interrupt", input_interrupt)?
        .build_readonly()
}