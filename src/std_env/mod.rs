use std::env;
use std::io;
use std::path::PathBuf;
use std::sync::OnceLock;

use mluau::prelude::*;
use crate::compile;
use crate::prelude::*;

pub mod vars;

/// The shell `process.shell` and `shell = true` use: `$SHELL` when it's set, otherwise the system's default shell.
/// `$SHELL` is read on every call so a script can change it; the default is looked up once per process.
pub fn get_current_shell() -> String {
    if let Ok(shell_path) = env::var("SHELL") {
        return shell_path;
    }
    static DEFAULT_SHELL: OnceLock<String> = OnceLock::new();
    DEFAULT_SHELL.get_or_init(find_default_shell).clone()
}

/// Finds `executable` in the directories on `$PATH` with filesystem checks only. Running `where` instead started a
/// process on every `require("@std/env")` and cost up to a second on Windows.
#[cfg(target_os = "windows")]
fn find_on_path(executable: &str) -> Option<String> {
    let path = env::var_os("PATH")?;
    env::split_paths(&path)
        .map(|directory| directory.join(executable))
        .find(|candidate| candidate.is_file())
        .map(|found| found.to_string_lossy().into_owned())
}

#[cfg(target_os = "windows")]
fn find_default_shell() -> String {
    // Windows PowerShell before pwsh: pwsh 7 can blow up with a threading error
    if let Some(powershell) = find_on_path("powershell.exe") {
        return powershell;
    }
    if let Some(pwsh) = find_on_path("pwsh.exe") {
        return pwsh;
    }
    use std::io::Write;
    let _ = writeln!(std::io::stderr().lock(), "get_current_shell falling back to cmd.exe; please set $SHELL");
    env::var("ComSpec").unwrap_or_else(|_| String::from("cmd.exe"))
}

#[cfg(not(target_os = "windows"))]
fn find_default_shell() -> String {
    String::from("/bin/sh")
}

pub fn get_cwd(function_name: &str) -> LuaResult<PathBuf> {
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(err) => {
            match err.kind() {
                io::ErrorKind::NotFound => { // yes this happened in testing
                    return wrap_err!("{}: your current directory does not exist (try reloading your terminal/editor?)", function_name);
                },
                io::ErrorKind::PermissionDenied => {
                    return wrap_err!("{}: insufficient permissions to access your current directory", function_name);
                },
                other => {
                    return wrap_err!("{}: error getting your current directory: {}", function_name, other);
                }
            }
        }
    };
    Ok(cwd)
}

fn env_cwd(luau: &Lua, _: LuaValue) -> LuaValueResult {
    let function_name = "env.cwd()";
    let cwd = match get_cwd(function_name) {
        Ok(cwd) => {
            cwd.to_string_lossy().to_string()
        },
        Err(err) => {
            return wrap_err!("{}", err.to_string())
        }
    };
    ok_string(cwd, luau)
}

fn env_environment_getvar(luau: &Lua, value: LuaValue) -> LuaValueResult {
    deprecate("env.getvar", "env.vars.get", luau)?;
    let var_name = match value {
        LuaValue::String(var) => var.to_string_lossy(),
        other => {
            return wrap_err!("env.getvar expected a string, got: {:#?}", other);
        }
    };

    match env::var(&var_name) {
        Ok(var) => Ok(LuaValue::String(luau.create_string(&var)?)),
        Err(env::VarError::NotPresent) => {
            Ok(LuaNil)
        },
        Err(env::VarError::NotUnicode(_nonunicode_var)) => {
            wrap_err!("env.getvar: requested environment variable '{}' has invalid unicode value", var_name)
        }
    }
}

fn env_environment_setvar(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
    deprecate("env.setvar", "env.vars.set", luau)?;
    let key = match multivalue.pop_front() {
        Some(LuaValue::String(key)) => key.to_string_lossy(),
        Some(other) => {
            return wrap_err!("env.setvar(key: string, value: string) expected key to be a string, got: {:#?}", other);
        },
        None => {
            return wrap_err!("env.setvar(key: string, value: string) expected 2 arguments, got none")
        }
    };

    let value = match multivalue.pop_back() {
        Some(LuaValue::String(value)) => value.to_string_lossy(),
        Some(other) => {
            return wrap_err!("env.setvar(key: string, value: string) expected value to be a string, got: {:#?}", other);
        },
        None => {
            return wrap_err!("env.setvar(key: string, value: string) was called with only one argument");
        }
    };

    // safety: setting/removing environment unsafe in multithreaded programs on linux
    // this could be possibly unsafe if the same variable gets set in scripts from multiple thread.spawns on linux
    unsafe { env::set_var(&key, value); }

    match env::var(&key) {
        Ok(_value) => Ok(LuaNil),
        Err(err) => {
            wrap_err!("env.setvar: unable to set environment variable '{}': {}", key, err)
        }
    }
}

fn env_environment_removevar(luau: &Lua, value: LuaValue) -> LuaValueResult {
    deprecate("env.removevar", "env.vars.unset", luau)?;
    let key = match value {
        LuaValue::String(key) => key.to_string_lossy(),
        other => {
            return wrap_err!("env.removevar(key: string) expected key to be a string, got: {:#?}", other);
        }
    };

    // SAFETY: removing env variable unsafe in multithreaded linux
    // this could cause ub if mixed with thread.spawns
    unsafe { env::remove_var(&key); }

    match env::var(&key) {
        Ok(key) => {
            wrap_err!("env.removevar: unable to remove environment variable '{}'", key)
        },
        Err(_err) => {
            Ok(LuaNil)
        },
    }
}

fn env_where(luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "env.where(name: string)";
    let name = match value {
        LuaValue::String(s) => s.to_string_lossy(),
        other => {
            return wrap_err!("{}: expected application name to find within $PATH to be a string, got: {:#?}", function_name, other);
        }
    };

    if name.starts_with("/") {
        return wrap_err!("{}: application name to find within $PATH starts with absolute path separator '/'; this is most likely user error", function_name);
    }

    let mut found_paths: Vec<String> = Vec::new();

    if let Some(path_var) = env::var_os("PATH") {
        #[cfg(target_family = "unix")]
        {
            use std::os::unix::fs::PermissionsExt;
            for dir in env::split_paths(&path_var) {
                #[allow(clippy::disallowed_methods, reason = "Path::join is safe here; we've validated name doesn't start with /")]
                let candidate = dir.join(&name);
                if let Ok(metadata) = std::fs::metadata(&candidate)
                    && metadata.is_file()
                    && metadata.permissions().mode() & 0o111 != 0 {
                        found_paths.push(candidate.to_string_lossy().to_string());
                    }
            }
        }

        #[cfg(target_os = "windows")]
        {
            let pathext = env::var("PATHEXT").unwrap_or_else(|_| String::from(".COM;.EXE;.BAT;.CMD"));
            let extensions: Vec<&str> = pathext.split(';').filter(|ext| !ext.is_empty()).collect();
            let has_extension = std::path::Path::new(&name).extension().is_some();

            for dir in env::split_paths(&path_var) {
                if has_extension {
                    #[allow(clippy::disallowed_methods, reason = "Path::join is safe here; we've validated name doesn't start with /")]
                    let candidate = dir.join(&name);
                    if candidate.is_file() {
                        found_paths.push(candidate.to_string_lossy().to_string());
                    }
                } else {
                    for extension in &extensions {
                        #[allow(clippy::disallowed_methods, reason = "Path::join is safe here; we've validated name doesn't start with /")]
                        let candidate = dir.join(format!("{}{}", name, extension));
                        if candidate.is_file() {
                            found_paths.push(candidate.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }
    }

    let result = luau.create_sequence_from(found_paths)?;
    result.set_readonly(true);
    ok_table(Ok(result))
}

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    let formatted_os = match env::consts::OS {
        "linux" => String::from("Linux"),
        "windows" => String::from("Windows"),
        "android" => String::from("Android"),
        "macos" => String::from("MacOS"),
        other => other[0..1].to_uppercase() + &other[1..],
    };

    let executable_path = env::current_exe().ok().unwrap_or_default().to_owned();
    let executable_path = executable_path.to_string_lossy();
    let luau_args = luau.create_table_with_capacity(4, 0)?;
    for (index, arg) in env::args_os().enumerate() {
        if index == 0 {
            continue; // skip 'seal' argument
        }
        if compile::is_standalone(None) {
            let arg_bytes = arg.as_encoded_bytes();
            luau_args.raw_push(luau.create_string(arg_bytes)?)?;
        } else if index == 1 {
            continue; // file name for seal ./filename.luau or r in seal r, either way not useful
        } else {
            let arg_bytes = arg.as_encoded_bytes();
            luau_args.raw_push(luau.create_string(arg_bytes)?)?;
        }
    }

    TableBuilder::create(luau)?
        .with_value("os", formatted_os)?
        .with_value("arch", env::consts::ARCH)?
        .with_value("args", luau_args)?
        .with_value("executable_path", executable_path)?
        .with_value("shell", get_current_shell())?
        .with_value("vars", vars::create(luau)?)?
        .with_function_and_signature("getvar", env_environment_getvar, c"env.getvar is deprecated, use env.vars.get instead")?
        .with_function_and_signature("setvar", env_environment_setvar, c"env.setvar is deprecated, use env.vars.set instead")?
        .with_function_and_signature("removevar", env_environment_removevar, c"env.removevar is deprecated, use env.vars.unset instead")?
        .with_function_and_signature("cwd", env_cwd, signatures::STD_ENV_CWD)?
        .with_function_and_signature("where", env_where, signatures::STD_ENV_WHERE)?
        .build_readonly()
}