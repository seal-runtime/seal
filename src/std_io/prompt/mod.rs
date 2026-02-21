use mluau::prelude::*;
use crate::std_err::ecall;
use crate::{globals::warn, prelude::*};
use crate::std_io;

use std::io::{self, Write};
use crossterm::event::KeyCode;
use crossterm::terminal;

use rustyline::error::ReadlineError;

fn prompt_line(luau: &Lua, message: &str, function_name: &'static str) -> LuaResult<String> {
    if atty::isnt(atty::Stream::Stdin) || atty::isnt(atty::Stream::Stdout) {
        match std_io::input::input_rawline(luau, if message.is_empty() { None } else { Some(message.to_owned()) }) {
            Ok(s) => {
                return Ok(s);
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

    let line = match rl.readline(message) {
        Ok(line) => {
            if let Err(err) = rl.add_history_entry(line.as_str()) {
                warn(luau, ok_string(format!("error adding prompt history: {}", err), luau)?)?;
            }
            line
        },
        Err(ReadlineError::Interrupted) => {
            return wrap_err!("Prompt interrupted with Ctrl-C; use io.input.readline to intercept without erroring");
        },
        Err(ReadlineError::Eof) => {
            return wrap_err!("Prompt interrupted with Ctrl-D; use io.input.readline to intercept without erroring");
        },
        Err(err) => {
            return wrap_err!("{}: encountered a ReadlineError: {}", function_name, err);
        }
    };

    Ok(line)
}

pub fn prompt_text(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaResult<String> {
    let function_name = "prompt.text(message: string)";

    let mut message = match multivalue.pop_front() {
        Some(LuaValue::String(s)) => s.to_string_lossy(),
        Some(LuaNil) | None => {
            return wrap_err!("{} expected message to be a string, got nothing or nil", function_name);
        },
        Some(other) => {
            return wrap_err!("{} expected message to be a string, got: {:?}", function_name, other);
        }
    };

    if !message.is_empty() && !message.contains(": ") {
        message.push_str(": ");
    }

    prompt_line(luau, &message, function_name)
}

pub fn prompt_edit(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
    let function_name = "prompt.edit(prompt: string, left: string, right: string?)";

    let mut prompt = match multivalue.pop_front() {
        Some(LuaValue::String(s)) => s.to_string_lossy(),
        Some(LuaNil) | None => {
            return wrap_err!("{} expected prompt to be a string, got nothing or nil", function_name);
        },
        Some(other) => {
            return wrap_err!("{} expected prompt to be a string, got: {:?}", function_name, other);
        }
    };

    if !prompt.is_empty() && !prompt.contains(": ") {
        prompt.push_str(": ");
    }

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
        match std_io::input::input_rawline(luau, Some(combined)) {
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
            return wrap_err!("Prompt interrupted with Ctrl-C; use io.input.editline to intercept without erroring");
        },
        Err(ReadlineError::Eof) => {
            return wrap_err!("Prompt interrupted with Ctrl-D; use io.input.editline to intercept without erroring");
        },
        Err(err) => {
            return wrap_err!("{}: encountered unexpected ReadlineError: {}", function_name, err);
        }
    };

    ok_string(line, luau)
}

fn prompt_confirm(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaResult<bool> {
    let function_name = "prompt.confirm(message: string, default: boolean?)";

    let mut message = match multivalue.pop_front() {
        Some(LuaValue::String(s)) => s.to_string_lossy(),
        Some(LuaNil) | None => {
            return wrap_err!("{} expected message to be a string, got nothing or nil", function_name);
        },
        Some(other) => {
            return wrap_err!("{} expected message to be a string, got: {:?}", function_name, other);
        }
    };

    let default = match multivalue.pop_front() {
        Some(LuaValue::Boolean(b)) => b,
        Some(LuaNil) | None => true,
        Some(other) => {
            return wrap_err!("{} expected default to be a boolean (default true), got: {:?}", function_name, other);
        }
    };

    if !message.is_empty() && !message.contains(": ") {
        message.push_str(if default {
            " [Y/n]: "
        } else {
            " [y/N]: "
        });
    }

    loop {
        let line = prompt_line(luau, &message, function_name)?;
        match line.trim().to_lowercase().as_str() {
            "y" => {
                return Ok(true);
            },
            "n" => {
                return Ok(false);
            },
            "" => {
                return Ok(default);
            },
            _ => {
                warn(luau, ok_string("That's not a y or n, try again?", luau)?)?;
            }
        }
    }
}

enum PasswordStyle {
    Hidden,
    Astricks,
}
impl PasswordStyle {
    fn from_luau(s: LuaString, function_name: &'static str) -> LuaResult<Self> {
        let bytes = s.as_bytes();
        if bytes == &b"Hidden"[..] {
            Ok(Self::Hidden)
        } else if bytes == &b"*"[..] {
            Ok(Self::Astricks)
        } else {
            wrap_err!("{} expected style to be either \"Hidden\" or \"*\", got: {}", function_name, s.display())
        }
    }
}

/// Prompts password masked by astricks with crossterm
pub fn prompt_password_masked(message: &str, function_name: &str) -> LuaResult<String> {
    // print the prompt without newline
    put!("{}", message)?;
    if let Err(err) = io::stdout().flush() {
        return wrap_err!("{}: failed to flush stdout: {}", function_name, err);
    }

    // switch terminal to raw mode so we can read keys directly
    if let Err(err) = terminal::enable_raw_mode() {
        return wrap_err!("{}: failed to enable raw mode: {}", function_name, err);
    }

    let mut buffer = String::new();

    loop {
        // read one key event at a time
        let event = match crossterm::event::read() {
            Ok(ev) => ev,
            Err(err) => {
                let _ = terminal::disable_raw_mode(); // best effort cleanup if read fails
                return wrap_err!("{}: error reading key event: {}", function_name, err);
            }
        };

        // only care about key events, ignore mouse or resize
        if let crossterm::event::Event::Key(key) = event {
            match key.code {
                KeyCode::Enter => break, // user pressed enter, we're done
                KeyCode::Char(c) => {
                    buffer.push(c); // store the typed character
                    put!("*")?; // show asterisk instead of actual character
                    if let Err(err) = io::stdout().flush() {
                        let _ = terminal::disable_raw_mode();
                        return wrap_err!("{}: failed to flush stdout: {}", function_name, err);
                    }
                }
                KeyCode::Backspace => {
                    if buffer.pop().is_some() {
                        // move cursor back, overwrite with space, move back again
                        put!("\x08 \x08")?;
                        if let Err(err) = io::stdout().flush() {
                            let _ = terminal::disable_raw_mode();
                            return wrap_err!("{}: failed to flush stdout: {}", function_name, err);
                        }
                    }
                }
                _ => {} // ignore other keys like arrows, tab, etc
            }
        }
    }

    // restore terminal mode before returning
    if let Err(err) = terminal::disable_raw_mode() {
        return wrap_err!("{}: failed to disable raw mode: {}", function_name, err);
    }

    puts!()?; // move to next line after password entry

    Ok(buffer)
}

fn prompt_password(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
    let function_name = "prompt.password(message: string, style: (\"Hidden\" | \"*\")?";

    let mut message = match multivalue.pop_front() {
        Some(LuaValue::String(s)) => s.to_string_lossy(),
        Some(LuaNil) | None => {
            return wrap_err!("{} expected message to be a string, got nothing or nil", function_name);
        },
        Some(other) => {
            return wrap_err!("{} expected message to be a string, got: {:?}", function_name, other);
        }
    };

    let password_style = match multivalue.pop_front() {
        Some(LuaValue::String(s)) => PasswordStyle::from_luau(s, function_name)?,
        Some(LuaNil) | None => PasswordStyle::Hidden,
        Some(other) => {
            return wrap_err!("{} expected style to be either \"Hidden\" or \"*\", got: {:?}", function_name, other);
        }
    };

    if !message.is_empty() && !message.contains(": ") {
        message.push_str(match password_style {
            PasswordStyle::Hidden => " (hidden): ",
            PasswordStyle::Astricks => ": ",
        });
    }

    let password = match password_style {
        PasswordStyle::Astricks => prompt_password_masked(&message, function_name)?,
        PasswordStyle::Hidden => match rpassword::prompt_password(message) {
            Ok(pass) => pass,
            Err(err) => {
                return wrap_err!("{}: unable to read hidden password due to err: {}", function_name, err);
            }
        },
    };

    ok_string(password, luau)
}

const PROMPT_DOT_LUAU_SRC: &str = include_str!("./prompt.luau");

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    let t = TableBuilder::create(luau)?
        .with_function("text", prompt_text)?
        .with_function("edit", prompt_edit)?
        .with_function("password", prompt_password)?
        .with_function("confirm", prompt_confirm)?
        .build()?;

    let chunk = Chunk::Src(PROMPT_DOT_LUAU_SRC.to_owned());
    let prompt_table = match luau.load(chunk).eval::<LuaTable>() {
        Ok(t) => t,
        Err(err) => {
            panic!("std/io/prompt's prompt.luau did a bad: {}", err);
        }
    };

    for pair in prompt_table.pairs() {
        let (key, value): (String, LuaFunction) = pair?;
        t.raw_set(key, ecall(luau, value)?)?;
    }

    t.set_readonly(true);

    Ok(t)
}