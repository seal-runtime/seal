use core::str;
use std::cell::RefCell;
use std::io::{self, Write};
use std::process::{self, Command, Output};
use std::rc::Rc;

use crate::{prelude::*, std_err};
use crate::std_env;
use mluau::prelude::*;

mod stream;
mod options;
use stream::Stream;
use options::{RunOptions, SpawnOptions, Shell};

fn run_result_unwrap_or(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
    let function_name = "RunResult:unwrap_or(default: string | (result: RunResult) -> string)";
    let run_result = match multivalue.pop_front() {
        Some(LuaValue::Table(run_result)) => run_result,
        Some(other) => {
            return wrap_err!("{} expected self to be a RunResult table, got: {:?}", function_name, other);
        }
        None => {
            return wrap_err!("{} expected to be called with self; did you forget methodcall syntax (:)?", function_name);
        }
    };

    let default_value: Option<LuaValue> = match multivalue.pop_front() {
        Some(LuaValue::String(default)) => Some(LuaValue::String(default)),
        Some(LuaValue::Function(f)) => Some(LuaValue::Function(f)),
        Some(LuaNil) => Some(LuaNil),
        Some(other) => {
            return wrap_err!("{}: expected default value to be a string (or a function that returns one), got: {:?}", function_name, other);
        }
        None => None,
    };

    let is_ok = match run_result.raw_get("ok")? {
        LuaValue::Boolean(b) => b,
        other => {
            return wrap_err!("{}: expected RunResult.ok to be a boolean, got: {:?}", function_name, other);
        }
    };

    let stdout = if is_ok {
        match run_result.raw_get("stdout")? {
            LuaValue::String(s) => {
                let Ok(s) = s.to_str() else {
                    return wrap_err!(
                        "{}: stdout is not a valid utf-8 encoded string, use RunResult.stdout to get the raw stdout without attempting to trim/clean it",
                        function_name
                    );
                };
                let s = s.trim_end();
                LuaValue::String(luau.create_string(s)?)
            }
            other => {
                return wrap_err!("{} RunResult.stdout is not a string??: {:?}", function_name, other);
            }
        }
    } else if let Some(default_value) = default_value {
        match default_value {
            LuaValue::String(d) => LuaValue::String(d),
            LuaValue::Function(f) => match f.call::<LuaValue>(run_result) {
                Ok(LuaValue::String(default)) => LuaValue::String(default),
                Ok(other) => {
                    return wrap_err!("{}: expected default value function to return string, got: {:?}", function_name, other);
                }
                Err(err) => {
                    return wrap_err!("{}: default value function unexpectedly errored: {}", function_name, err);
                }
            },
            other => {
                return wrap_err!("{}: default value expected to be a string (or a function that returns one), got: {:?}", function_name, other);
            }
        }
    } else {
        return wrap_err!("Attempt to {} an unsuccessful RunResult without a default value!", function_name);
    };
    Ok(stdout)
}

fn trim_end_or_return(vec: &[u8]) -> &[u8] {
    match str::from_utf8(vec) {
        Ok(s) => s.trim_end().as_bytes(),
        Err(_) => vec,
    }
}

fn create_run_result_table(luau: &Lua, output: Output) -> LuaValueResult {
    let ok = output.status.success();
    let stdout = output.stdout.clone();
    let stderr = output.stderr.clone();

    let run_result = TableBuilder::create(luau)?
        .with_value("ok", ok)?
        .with_value("out", {
            if ok {
                let s = trim_end_or_return(&stdout);
                LuaValue::String(luau.create_string(s)?)
            } else {
                LuaNil
            }
        })?
        .with_value("err", {
            if !ok {
                let s = trim_end_or_return(&stderr);
                LuaValue::String(luau.create_string(s)?)
            } else {
                LuaNil
            }
        })?
        .with_value("stdout", luau.create_string(&stdout)?)?
        .with_value("stderr", luau.create_string(&stderr)?)?
        .with_function("unwrap", {
            move |luau: &Lua, _value: LuaMultiValue| -> LuaValueResult {
                if ok {
                    let s = trim_end_or_return(&stdout);
                    Ok(LuaValue::String(luau.create_string(s)?))
                } else {
                    let readable_stderr = String::from_utf8_lossy(&stderr);
                    wrap_err!("Attempt to :unwrap() a failed RunResult! Use :unwrap_or to specify a default value.\n  Process terminated with stderr:\n  {}", readable_stderr)
                }
            }
        })?
        .with_function("unwrap_or", run_result_unwrap_or)?
        .build_readonly();

    ok_table(run_result)
}

fn run_command(options: RunOptions) -> io::Result<Output> {
    let shell_switches = match options.shell {
        Some(ref shell) => shell.get_switches(),
        None => Vec::new(),
    };

    let mut command = if let Some(ref shell) = options.shell {
        let mut command = Command::new(shell.program_name());
        command.args(shell_switches).arg(options.program);
        if let Some(args) = options.args {
            command.arg(args.join(" "));
        }
        command
    } else {
        let mut command = Command::new(options.program);
        if let Some(args) = options.args {
            command.args(args);
        }
        command
    };

    if let Some(cwd) = options.cwd {
        command.current_dir(cwd);
    }

    if let Some(extra) = options.extra_envs {
        extra.apply(&mut command);
    }

    options.stdio.apply(&mut command);

    command.output()
}

fn process_run(luau: &Lua, run_options: LuaValue) -> LuaValueResult {
    let function_name = "process.run(options: RunOptions)";
    let options = match run_options {
        LuaValue::Table(run_options) => RunOptions::from_table(luau, &run_options, function_name, "Run")?,
        LuaValue::Nil => {
            return wrap_err!("{} expected RunOptions table, got nil.", function_name);
        }
        other => {
            return wrap_err!("{} expected RunOptions table, got: {:?}", function_name, other);
        }
    };

    let program_to_run = options.program.clone();

    match run_command(options) {
        Ok(output) => create_run_result_table(luau, output),
        Err(err) => {
            // we want to throw an error if the program was unable to spawn at all
            // this is because when a user calls process.run/shell, they expect their program to actually run
            // and we don't want the 'ok' or 'err' value to serve two purposes (program failed to execute vs program executed with error)
            wrap_err!("{} was unable to run the program '{}' due to err {}; did you forget to set shell = true?", function_name, program_to_run, err)
        }
    }
}

fn process_shell(luau: &Lua, shell_command: LuaValue) -> LuaValueResult {
    let function_name = "process.shell(command: string)";
    let shell_name = std_env::get_current_shell();
    let shell_command = match shell_command {
        LuaValue::String(command) => command.to_str()?.to_string(),
        other => {
            return wrap_err!("{} expected command to be a string, got: {:?}", function_name, other);
        }
    };

    let run_options = RunOptions {
        program: shell_command.clone(),
        args: None,
        shell: Some(Shell::from(shell_name.clone())),
        cwd: None,
        stdio: options::StdioTriple::default(),
        extra_envs: None
    };

    match run_command(run_options) {
        Ok(output) => create_run_result_table(luau, output),
        Err(err) => {
            wrap_err!("{} unable to run shell command '{}' with shell '{}' because of err: {}", function_name, shell_command, shell_name, err)
        }
    }
}

fn process_spawn(luau: &Lua, spawn_options: LuaValue) -> LuaValueResult {
    let function_name = "process.spawn(options: SpawnOptions)";
    let options = match spawn_options {
        LuaValue::Table(spawn_options) => SpawnOptions::from_table(luau, spawn_options, function_name)?,
        LuaValue::Nil => {
            return wrap_err!("{} expected a SpawnOptions table, got nil", function_name);
        }
        other => {
            return wrap_err!("{} expected SpawnOptions table, got: {:?}", function_name, other);
        }
    };

    let shell_switches = match options.shell {
        Some(ref shell) => shell.get_switches(),
        None => Vec::new(),
    };

    let mut child = {
        let mut command = if let Some(ref shell) = options.shell {
            let mut command = Command::new(shell.program_name());
            command.args(shell_switches).arg(options.program);
            if let Some(args) = options.args {
                command.arg(args.join(" "));
            }
            command
        } else {
            let mut command = Command::new(options.program);
            if let Some(args) = options.args {
                command.args(args);
            }
            command
        };

        if let Some(cwd) = options.cwd {
            command.current_dir(cwd);
        }

        if let Some(extra) = options.extra_envs {
            extra.apply(&mut command);
        }

        options.stdio.apply(&mut command);

        match command.spawn() {
            Ok(child) => child,
            Err(err) => {
                return wrap_err!("process.spawn failed to execute process: {} (did you forget to set shell = true | string)", err);
            }
        }
    };

    let child_id = child.id();
    let stdout = child.stdout.take();
    let stderr = child.stderr.take();
    let stdin = child.stdin.take();

    let child_cell = Rc::new(RefCell::new(child));

    let child_process_handle = {
        let stdout_handle = if let Some(stdout) = stdout {
            let stdout_stream = Stream::new(
                function_name,
                stdout,
                stream::StreamType::Stdout,
                options.stdout_capacity,
                options.stdout_truncate,
            )?;
            let stdout_cell = Rc::new(RefCell::new(stdout_stream));
            Some(ok_table(Stream::create_handle(stdout_cell, luau))?)
        } else {
            None
        };

        let stderr_handle = if let Some(stderr) = stderr {
            let stderr_stream = Stream::new(
                function_name,
                stderr,
                stream::StreamType::Stderr,
                options.stderr_capacity,
                options.stderr_truncate,
            )?;
            let stderr_cell = Rc::new(RefCell::new(stderr_stream));
            Some(ok_table(Stream::create_handle(stderr_cell, luau))?)
        } else {
            None
        };

        let stdin_handle = if let Some(stdin) = stdin {
            let stdin_cell_write = Rc::new(RefCell::new(Some(stdin)));
            let stdin_cell_close = Rc::clone(&stdin_cell_write);
    
            Some(TableBuilder::create(luau)?
                .with_function_mut("write", {
                    move |luau: &Lua, mut multivalue: LuaMultiValue| -> LuaValueResult {
                        let function_name = "child.stdin:write(data: string)";
                        pop_self(&mut multivalue, function_name)?;
                        let data_to_write = match multivalue.pop_front() {
                            Some(LuaValue::String(data)) => data.as_bytes().to_vec(),
                            Some(LuaValue::Buffer(b)) => b.to_vec(),
                            Some(other) => {
                                return wrap_err!("{} expected data to be a string or buffer, got: {:?}", function_name, other);
                            }
                            None => {
                                return wrap_err!("{} expected data to be string or buffer, unexpectedly got nothing (not even nil)", function_name);
                            }
                        };
    
                        let mut stdin = match stdin_cell_write.try_borrow_mut() {
                            Ok(mut cell) => match cell.take() {
                                Some(stdin) => stdin,
                                None => {
                                    return wrap_err!("{}: attempt to write to closed stdin", function_name);
                                }
                            },
                            Err(_) => {
                                unreachable!("{}: stdin already borrowed; this shouldn't happen as Luau VM is single threaded and multithreaded code should never touch this???", function_name);
                            }
                        };
    
                        match stdin.write_all(&data_to_write) {
                            Ok(_) => Ok(LuaNil),
                            Err(err) => {
                                std_err::WrappedError::from_message(format!("{} can't write to stdin due to err: {}", function_name, err)).get_userdata(luau)
                            }
                        }
                    }
                })?
                .with_function_mut("close", {
                    move |_luau: &Lua, mut multivalue: LuaMultiValue| -> LuaEmptyResult {
                        let function_name = "child.stdin:close()";
                        pop_self(&mut multivalue, function_name)?;
    
                        let mut stdin = match stdin_cell_close.try_borrow_mut() {
                            Ok(mut cell) => match cell.take() {
                                Some(stdin) => stdin,
                                None => {
                                    return Ok(())
                                }
                            },
                            Err(_) => {
                                unreachable!("{}: stdin already borrowed; this shouldn't happen as Luau VM is single threaded and multithreaded code should never touch this???", function_name);
                            }
                        };
    
                        if stdin.flush().is_err() {
                            return wrap_err!("{}: unable to flush stdin", function_name);
                        }
    
                        drop(stdin);
                        Ok(())
                    }
                })?
                .build_readonly()?.into_lua(luau)?)
        } else {
            None
        };

        TableBuilder::create(luau)?
            .with_value("id", child_id)?
            .with_value("stdout", stdout_handle.unwrap_or(LuaNil))?
            .with_value("stderr", stderr_handle.unwrap_or(LuaNil))?
            .with_value("stdin", stdin_handle.unwrap_or(LuaNil))?
            .with_function("alive", {
                let child_cell = Rc::clone(&child_cell);
                move |_luau: &Lua, _value: LuaValue| -> LuaValueResult {
                    let function_name = "ChildProcess:alive()";
                    let Ok(mut child) = child_cell.try_borrow_mut() else {
                        unreachable!("{}: child (pid {}) already borrowed; this is likely a seal bug because there isn't anything multithreaded that should touch this.", function_name, child_id);
                    };

                    match child.try_wait() {
                        Ok(Some(_status)) => Ok(LuaValue::Boolean(false)),
                        Ok(None) => Ok(LuaValue::Boolean(true)),
                        Err(err) => {
                            wrap_err!("{}: (heisenseal's child) cannot determine whether child (pid {}) is dead or alive due to err: {}", function_name, child_id, err)
                        }
                    }
                    
                }
            })?
            .with_function("kill", {
                let function_name = "ChildProcess:kill()";
                let child_cell = Rc::clone(&child_cell);
                move |_luau: &Lua, _value: LuaValue| -> LuaEmptyResult {
                    match child_cell.try_borrow_mut() {
                        Ok(ref mut child) => match child.kill() {
                            Ok(_) => Ok(()),
                            Err(err) => {
                                wrap_err!("{} could not murder child due to err: {}", function_name, err)
                            }
                        },
                        Err(_) => {
                            wrap_err!("{}: child already borrowed", function_name)
                        }
                    }
                }
            })?
            .build_readonly()
    };

    ok_table(child_process_handle)
}

fn set_exit_callback(luau: &Lua, f: Option<LuaValue>) -> LuaValueResult {
    if let Some(f) = f {
        match f {
            LuaValue::Function(f) => {
                let globals = luau.globals();
                globals.set("_process_exit_callback_function", f)?;
                Ok(LuaNil)
            }
            _ => {
                let err_message = format!("process.setexitcallback expected to be called with a function, got {:?}", f);
                Err(LuaError::external(err_message))
            }
        }
    } else {
        let err_message = format!("process.setexitcallback expected to be called with a function, got {:?}", f);
        Err(LuaError::external(err_message))
    }
}

pub fn _handle_exit_callback(luau: &Lua, exit_code: i32) -> LuaResult<()> {
    match luau.globals().get("_process_exit_callback_function")? {
        LuaValue::Function(f) => {
            let _ = f.call::<i32>(exit_code);
        }
        LuaValue::Nil => {}
        _ => {
            unreachable!("what did you put into _process_exit_callback_function???");
        }
    }
    Ok(())
}

fn exit(luau: &Lua, exit_code: Option<LuaValue>) -> LuaResult<()> {
    let exit_code = if let Some(exit_code) = exit_code {
        match exit_code {
            LuaValue::Integer(i) => i,
            _ => {
                return wrap_err!("process.exit expected exit_code to be a number (integer) or nil, got {:?}", exit_code);
            }
        }
    } else {
        0
    };
    // if we have custom callback function let's call it
    let globals = luau.globals();
    match globals.get("_process_exit_callback_function")? {
        LuaValue::Function(f) => {
            f.call::<i64>(exit_code)?;
        }
        LuaValue::Nil => {}
        other => {
            unreachable!("wtf is in _process_exit_callback_function other than a function or nil?: {:?}", other)
        }
    }
    if let Ok(exit_code) = i32::try_from(exit_code) {
        process::exit(exit_code);
    } else {
        wrap_err!("process.exit: your exit code is too big ({}), we can't convert it to i32.", exit_code)
    }
}

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_function("run", process_run)?
        .with_function("spawn", process_spawn)?
        .with_function("shell", process_shell)?
        .with_function("setexitcallback", set_exit_callback)?
        .with_function("exit", exit)?
        .build_readonly()
}