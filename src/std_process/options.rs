use std::path::{Path, PathBuf};
use std::collections::HashMap as Map;

use crate::std_env::get_current_shell;
use crate::std_io::format::hexdump;
use crate::{prelude::*, std_process::stream::TruncateSide};
use mluau::prelude::*;

#[derive(Debug)]
pub enum Shell {
    #[allow(clippy::enum_variant_names)]
    WindowsPowerShell,
    Pwsh,
    Bash,
    Sh,
    Zsh,
    Fish,
    CmdDotExe,
    Other(String),
}

impl From<String> for Shell {
    fn from(s: String) -> Self {
        let shell_name = Path::new(&s)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(&s); // If file_name fails, fall back to the original

        match shell_name {
            "pwsh" => Shell::Pwsh,
            "powershell" => Shell::WindowsPowerShell,
            "bash" => Shell::Bash,
            "sh" => Shell::Sh,
            "zsh" => Shell::Zsh,
            "fish" => Shell::Fish,
            "cmd" | "cmd.exe" => Shell::CmdDotExe,
            other => Shell::Other(other.to_string()),
        }
    }
}

impl Shell {
    pub fn current() -> Self {
        Shell::from(get_current_shell())
    }
    pub fn program_name(&self) -> &str {
        match self {
            Shell::Pwsh => "pwsh",
            Shell::WindowsPowerShell => "powershell",
            Shell::Bash => "bash",
            Shell::Sh => "sh",
            Shell::Zsh => "zsh",
            Shell::Fish => "fish",
            Shell::CmdDotExe => "cmd.exe",
            Shell::Other(name) => name.as_str(),
        }
    }
    pub fn get_switches(&self) -> Vec<&str> {
        match self {
            Shell::Pwsh | Shell::WindowsPowerShell => vec!["-Command", "-NonInteractive"],
            Shell::CmdDotExe => vec!["/C"],
            _other => vec!["-c"],
        }
    }
}

#[derive(Copy, Clone)]
pub enum Stdio {
    Piped,
    Inherit,
    None,
}
impl Stdio {
    fn from_luau_string(s: LuaString, function_name: &'static str) -> LuaResult<Self> {
        Ok(match s.as_bytes().as_ref() {
            b"Piped" | b"piped" | b"Pipe" | b"pipe" => Self::Piped,
            b"Inherit" | b"inherit" => Self::Inherit,
            b"None" | b"none" => Self::None,
            _ => {
                return wrap_err!("{}: stdio option expected to be \"Pipe\" | \"Inherit\" | \"Ignore\", got: {:?}", function_name, s)
            }
        })
    }
    fn from_value(v: LuaValue, function_name: &'static str, stream_name: &'static str) -> LuaResult<Self> {
        Ok(match v {
            LuaValue::String(s) => Self::from_luau_string(s, function_name)?,
            LuaNil => Self::Piped,
            other => {
                return wrap_err!("{}: expected Process/RunOptions.stdio.{} to be one of \"Pipe\" | \"Inherit\" | \"Ignore\", got {:?}", function_name, stream_name, other);
            }
        })
    }
    fn into_std_stdio(self) -> std::process::Stdio {
        match self {
            Self::Piped => std::process::Stdio::piped(),
            Self::Inherit => std::process::Stdio::inherit(),
            Self::None => std::process::Stdio::null(),
        }
    }
}

pub struct StdioTriple(Stdio, Stdio, Stdio);
impl StdioTriple {
    pub fn default() -> Self {
        Self(Stdio::Piped, Stdio::Piped, Stdio::Piped)
    }
    fn from_value(value: LuaValue, function_name: &'static str) -> LuaResult<Self> {
        let v = match value {
            LuaValue::String(s) => {
                let which = Stdio::from_luau_string(s, function_name)?;
                StdioTriple(which, which, which)
            },
            LuaValue::Table(t) => {
                let stdout_variant = Stdio::from_value(t.raw_get("stdout")?, function_name, "stdout")?;
                let stderr_variant = Stdio::from_value(t.raw_get("stderr")?, function_name, "stderr")?;
                let stdin_variant = Stdio::from_value(t.raw_get("stdin")?, function_name, "stdin")?;
                StdioTriple(stdout_variant, stderr_variant, stdin_variant)
            },
            LuaNil => StdioTriple::default(),
            other => {
                return wrap_err!("{}: stdio expected to be \"Pipe\" | \"Inherit\" or \"Ignore\" or a struct-like table mapping stdout, stderr, and stdin to any of those, or nil, got: {:?}", function_name, other);
            }
        };
        Ok(v)
    }
    fn into_std_stdio(self) -> (std::process::Stdio, std::process::Stdio, std::process::Stdio) {
        (self.0.into_std_stdio(), self.1.into_std_stdio(), self.2.into_std_stdio())
    }
    pub fn apply(self, command: &mut std::process::Command) {
        let (stdout_stdio, stderr_stdio, stdin_stdio) = self.into_std_stdio();
        command.stdout(stdout_stdio).stderr(stderr_stdio).stdin(stdin_stdio);
    }
}

pub struct ExtraEnvs {
    pub clear: bool,
    pub add: Option<Map<String, String>>,
    pub remove: Option<Vec<String>>,
}
impl ExtraEnvs {
    pub fn apply(self, command: &mut std::process::Command) {
        if self.clear {
            command.env_clear();
        }
        if let Some(vars) = self.add {
            command.envs(vars);
        }
        if let Some(rem) = self.remove {
            for var in rem {
                command.env_remove(var);
            }
        }
    }
    fn map_from_table(luau: &Lua, vars: LuaTable, function_name: &'static str, what: &'static str) -> LuaResult<Map<String, String>> {
        let mut map = Map::new();
        for pair in vars.pairs::<LuaValue, LuaValue>() {
            let (key, value) = pair?;
            match (key, value) {
                (LuaValue::String(key), LuaValue::String(value)) => {
                    if key.to_str().is_err() {
                        return wrap_err!("{}: environment key bad utf8 (smh): {}", function_name, hexdump(luau, key.into_lua(luau)?)?);
                    }
                    if value.to_str().is_err() {
                        return wrap_err!("{}: environment value bad utf8 (smh): {}", function_name, hexdump(luau, value.into_lua(luau)?)?);
                    }
                    map.insert(key.to_string_lossy(), value.to_string_lossy());
                },
                (key, value) => {
                    return wrap_err!("{}: {}Options.env.add: key/value not pair of string to string, got ({:?}, {:?})", function_name, what, key, value);
                }
            }
        }
        Ok(map)
    }
    fn list_from_table(luau: &Lua, table: LuaTable, function_name: &'static str, what: &'static str) -> LuaResult<Vec<String>> {
        let cap = table.len()?;
        let mut list = Vec::with_capacity(int_to_usize(cap, function_name, "list capacity")?);
        for pair in table.pairs::<LuaValue, LuaValue>() {
            let (index, value) = pair?;
            match (index, value) {
                (LuaValue::Integer(i), LuaValue::String(s)) => {
                    if s.to_str().is_err() {
                        return wrap_err!("{}: env variable to remove at index {} is bad utf8: {}", function_name, i, hexdump(luau, s.into_lua(luau)?)?)
                    }
                    list.push(s.to_string_lossy());
                },
                (LuaValue::Integer(i), other) => {
                    return wrap_err!("{}: env variable to remove at index {} should be a string, got: {:?}", function_name, i, other);
                },
                (key, value) => {
                    return wrap_err!("{}: {}Options.env.remove: index/value pair not integer to string, got: ({:?}, {:?})", function_name, what, key, value);
                }
            }
        }
        Ok(list)
    }
    fn from_options(luau: &Lua, options: &LuaTable, function_name: &'static str, what: &'static str) -> LuaResult<Option<Self>> {
        let env_table = match options.raw_get("env")? {
            LuaValue::Table(t) => t,
            LuaNil => {
                return Ok(None);
            },
            other => {
                return wrap_err!("{}: expected {}Options.env to be nil or a table with propertiesgot: {:?}", function_name, what, other);
            }
        };
        let clear = match env_table.raw_get("clear")? {
            LuaValue::Boolean(b) => b,
            LuaNil => false,
            other => {
                return wrap_err!("{}: expected {}Options.env.clear to be a boolean or nil (default false), got: {:?}", function_name, what, other);
            }
        };

        let add = match env_table.raw_get("add")? {
            LuaValue::Table(add) => {
                Some(Self::map_from_table(luau, add, function_name, what)?)
            },
            LuaNil => None,
            other => {
                return wrap_err!("{}: expected {}Options.env.add to be a map of strings to strings or nil, got: {:?}", function_name, what, other);
            }
        };

        let remove = match env_table.raw_get("remove")? {
            LuaValue::Table(rem) => {
                Some(Self::list_from_table(luau, rem, function_name, what)?)
            },
            LuaNil => None,
            other => {
                return wrap_err!("{}: expected {}Options.env.remove to be {{ string }} or nil, got: {:?}", function_name, what, other);
            }
        };

        Ok(Some(Self {
            clear,
            add,
            remove
        }))
    }
}

pub struct RunOptions {
    pub program: String,
    pub args: Option<Vec<String>>,
    pub shell: Option<Shell>,
    pub cwd: Option<PathBuf>,
    pub stdio: StdioTriple,
    pub extra_envs: Option<ExtraEnvs>
}

impl RunOptions {
    pub fn from_table(luau: &Lua, run_options: &LuaTable, function_name: &'static str, what: &'static str) -> LuaResult<Self> {
        let program = match run_options.raw_get("program")? {
            LuaValue::String(program) => program.to_string_lossy(),
            LuaValue::Nil => {
                return wrap_err!("{}Options missing field `program`; expected string, got nil", what);
            }
            other => {
                return wrap_err!("{}Options.program expected to be a string, got: {:#?}", what, other);
            }
        };

        let args = match run_options.raw_get("args")? {
            LuaValue::Table(args) => {
                let mut rust_vec: Vec<String> = Vec::from_lua(LuaValue::Table(args), luau)?;
                // let's trim the whitespace just to make sure we pass valid args (untrimmed args might explode)
                for s in rust_vec.iter_mut() {
                    *s = s.trim().to_string();
                }
                Some(rust_vec)
            }
            LuaValue::Nil => None,
            other => {
                return wrap_err!("{}Options.args expected to be {{string}} or nil, got: {:#?}", what, other);
            }
        };

        let shell = match run_options.raw_get("shell")? {
            LuaValue::String(shell) => Some(Shell::from(shell.to_string_lossy())),
            LuaValue::Boolean(b) if b => Some(Shell::current()),
            LuaValue::Nil => None,
            other => {
                return wrap_err!("{}Options.shell expected to be a string or true or nil, got: {:?}", what, other);
            }
        };

        let cwd = match run_options.raw_get("cwd")? {
            LuaValue::String(cwd) => {
                let cwd = cwd.as_bytes();
                let cwd_str = str::from_utf8(&cwd)?;
                let cwd_pathbuf = PathBuf::from(cwd_str);
                let canonicalized_cwd = match cwd_pathbuf.canonicalize() {
                    Ok(pathbuf) => pathbuf,
                    Err(err) => {
                        return wrap_err!(
                            "{}Options.cwd must be able to be canonicalized as an absolute path that currently exists on the filesystem; canonicalization failed with err: {}",
                            what,
                            err
                        );
                    }
                };
                Some(canonicalized_cwd)
            }
            LuaNil => None,
            other => {
                return wrap_err!("{}Options.cwd expected to be a string or nil, got: {:?}", what, other);
            }
        };

        let triple = StdioTriple::from_value(run_options.raw_get("stdio")?, function_name)?;
        let extra_envs = ExtraEnvs::from_options(luau, run_options, function_name, what)?;

        Ok(Self {
            program,
            args,
            shell,
            cwd,
            stdio: triple,
            extra_envs
        })
    }
}

pub struct SpawnOptions {
    // common to spawn and run
    pub program: String,
    pub args: Option<Vec<String>>,
    pub shell: Option<Shell>,
    pub cwd: Option<PathBuf>,
    pub stdio: StdioTriple,
    pub extra_envs: Option<ExtraEnvs>,
    
    pub stdout_capacity: usize,
    pub stderr_capacity: usize,
    pub stdout_truncate: TruncateSide,
    pub stderr_truncate: TruncateSide,
}

impl SpawnOptions {
    fn extract_stream_fields(value: LuaValue) -> LuaResult<(usize, usize, TruncateSide, TruncateSide)> {
        Ok(match value {
            LuaValue::Table(stream_table) => (
                match stream_table.raw_get("stdout_capacity")? {
                    LuaValue::Number(f) => float_to_usize(f, "SpawnOptions.capacity.stdout", "stdout")?,
                    LuaValue::Integer(i) => int_to_usize(i, "SpawnOptions.capacity.stdout", "stdout")?,
                    LuaNil => 2048_usize,
                    other => {
                        return wrap_err!("SpawnOptions.stream.stdout expected to be number or nil, got: {:?}", other);
                    }
                },
                match stream_table.raw_get("stderr_capacity")? {
                    LuaValue::Number(f) => float_to_usize(f, "SpawnOptions.capacity.stderr", "stderr")?,
                    LuaValue::Integer(i) => int_to_usize(i, "SpawnOptions.capacity.stderr", "stderr")?,
                    LuaNil => 1024_usize,
                    other => {
                        return wrap_err!("SpawnOptions.stream.stdout expected to be number or nil, got: {:?}", other);
                    }
                },
                TruncateSide::from_value(stream_table.raw_get("stdout_truncate")?, "stdout_truncate")?,
                TruncateSide::from_value(stream_table.raw_get("stderr_truncate")?, "stderr_truncate")?,
            ),
            LuaNil => (2048_usize, 1024_usize, TruncateSide::Front, TruncateSide::Front),
            other => {
                return wrap_err!("SpawnOptions.capacity expected to be a table or nil, got: {:?}", other);
            }
        })
    }

    pub fn from_table(luau: &Lua, spawn_options: LuaTable, function_name: &'static str) -> LuaResult<Self> {
        // SpawnOptions is basically just RunOptions + a few extra fields
        let RunOptions { 
            program, 
            args, 
            shell, 
            cwd, 
            stdio ,
            extra_envs
        } = RunOptions::from_table(luau, &spawn_options, function_name, "Spawn")?;

        let (
            stdout_capacity, 
            stderr_capacity, 
            stdout_truncate, 
            stderr_truncate
        ) = Self::extract_stream_fields(spawn_options.raw_get("stream")?)?;

        // let detached = match spawn_options.raw_get("detached")? {
        //     LuaValue::Boolean(b) => b,
        //     LuaNil => false,
        //     other => {
        //         return wrap_err!("{} expected SpawnOptions.detached to be a boolean or nil (default false), got: {:?}", function_name, other);
        //     }
        // };

        Ok(Self {
            program,
            args,
            shell,
            cwd,
            stdio,
            extra_envs,
            stdout_capacity,
            stderr_capacity,
            stdout_truncate,
            stderr_truncate,
        })
    }
}