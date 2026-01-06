use std::path::{Path, PathBuf};

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
    pub fn into_std_stdio(self) -> (std::process::Stdio, std::process::Stdio, std::process::Stdio) {
        (self.0.into_std_stdio(), self.1.into_std_stdio(), self.2.into_std_stdio())
    }
}

pub struct RunOptions {
    pub program: String,
    pub args: Option<Vec<String>>,
    pub shell: Option<Shell>,
    pub cwd: Option<PathBuf>,
    pub stdio: StdioTriple,
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
            LuaValue::Nil => None,
            other => {
                return wrap_err!("{}Options.shell expected to be a string or nil, got: {:#?}", what, other);
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

        Ok(Self {
            program,
            args,
            shell,
            cwd,
            stdio: triple
        })
    }
}

pub struct SpawnOptions {
    // common to spawn and run
    pub program: String,
    pub args: Option<Vec<String>>,
    pub shell: Option<Shell>,
    pub cwd: Option<PathBuf>,
    
    pub stdout_capacity: usize,
    pub stderr_capacity: usize,
    pub stdout_truncate: TruncateSide,
    pub stderr_truncate: TruncateSide,

    #[allow(dead_code)]
    pub detached: bool,
    pub stdio: StdioTriple,
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
            stdio 
        } = RunOptions::from_table(luau, &spawn_options, function_name, "Spawn")?;

        let (
            stdout_capacity, 
            stderr_capacity, 
            stdout_truncate, 
            stderr_truncate
        ) = Self::extract_stream_fields(spawn_options.raw_get("stream")?)?;

        let detached = match spawn_options.raw_get("detached")? {
            LuaValue::Boolean(b) => b,
            LuaNil => false,
            other => {
                return wrap_err!("{} expected SpawnOptions.detached to be a boolean or nil (default false), got: {:?}", function_name, other);
            }
        };

        Ok(Self {
            program,
            args,
            shell,
            cwd,
            stdio,
            stdout_capacity,
            stderr_capacity,
            stdout_truncate,
            stderr_truncate,
            detached
        })
    }
}