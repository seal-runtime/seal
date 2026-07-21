use crate::prelude::*;
use mluau::prelude::*;

use crate::globals;
use crate::{Args, LuauLoadResult};

use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use mluau::Compiler;

pub mod options;
use options::CompileOptions;

/// handles bundling and compiling depending on `args` and `only_luau_please`
pub fn run(args: Args, only_luau_please: bool) -> LuauLoadResult {
    let function_name = if only_luau_please {
        "seal bundle"
    } else {
        "seal compile"
    };

    let CompileOptions { 
        input_path, 
        mut output_path, 
        should_transform 
    } = CompileOptions::from_args(args, function_name)?;

    let mut src = if should_transform { 
        bundle(&input_path)? 
    } else {
        match fs::read_to_string(&input_path) {
            Ok(contents) => contents,
            Err(err) => {
                return wrap_err!("{}: unable to read input file at '{}' due to err: {}", function_name, input_path.display(), err);
            }
        }
    };

    if only_luau_please && output_path.extension().is_none() {
        output_path.set_extension("luau");
    } else if only_luau_please 
        && let Some(extension) = output_path.extension()
        && extension != "luau"
    {
        return wrap_err!("{} should only be used to bundle to Luau; use `seal compile` for standalone executables", function_name);
    }

    if let Some(extension) = output_path.extension() 
        && extension == "luau" 
    {
        match fs::write(&output_path, src) {
            Ok(_) => {
                puts!("{} - bundled project sourcecode to '{}'", function_name, &output_path.display())?;
            },
            Err(err) => {
                return wrap_err!("{} - unable to write to file '{}' due to err: {}", function_name, &output_path.display(), err);
            }
        }
        return Ok(None);
    }

    // handle shebangs by stripping first line from \n
    if src.starts_with("#!") && let Some(first_newline_pos) = src.find('\n') {
        src = src[first_newline_pos + 1..].to_string();
    }

    let compiled_standalone_bytes = standalone(&src)?;

    #[cfg(windows)]
    {
        if output_path.extension().is_none() {
            output_path.set_extension("exe");
        }
    }
    
    let mut file = match OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&*output_path)
    {
        Ok(f) => f,
        Err(err) => {
            return wrap_err!("{} - error creating output file: {}", function_name, err);
        }
    };

    if let Err(err) = file.write_all(&compiled_standalone_bytes) {
        return wrap_err!("{} - error writing compiled program to file: {}", function_name, err);
    }

    puts!("{} - compiled to standalone program '{}'!", function_name, output_path.display())?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        // give it executable permissions on unix (chmod +x file) rwxr-xr-x (0o755)
        if let Err(err) = file.set_permissions(PermissionsExt::from_mode(0o755)) {
            return wrap_err!("{} - error setting executable permissions: {}", function_name, err);
        }
    }

    Ok(None)
}

const BUNDLER_SRC: &str = include_str!("./bundle.luau");

pub fn bundle(project_path: &Path) -> LuaResult<String> {
    let luau = Lua::new();
    globals::set_globals(&luau, "bundler")?;
    
    let chunk = Chunk::src(BUNDLER_SRC);
    let bundle = match luau.load(chunk).set_name("bundle.luau").eval::<LuaFunction>() {
        Ok(bundle) => bundle,
        Err(err) => {
            panic!("loading seal bundle function broke due to err: {}", err);
        }
    };

    let res = match bundle.call::<LuaValue>(project_path.into_lua(&luau)?) {
        Ok(LuaValue::String(bundled)) => bundled.to_string_lossy(),
        Ok(LuaValue::UserData(ud)) => {
            return wrap_err!("seal bundle - {}", ud.to_string()?)
        },
        Ok(other) => {
            panic!("wtf did seal bundle return? expected string | error, got: {:?}", other);
        }
        Err(err) => {
            panic!("seal bundle errored at runtime: {}", err);
        }
    };

    Ok(res)
}

pub fn is_standalone(bin: Option<PathBuf>) -> bool {
    const MAGIC: &[u8] = b"ASEALB1N";

    let Some(executable_path) = bin.or(std::env::current_exe().ok()) else {
        return false;
    };
    let Some(executable_bytes) = fs::read(&executable_path).ok() else {
        return false;
    };

    let file_len = executable_bytes.len();
    // check the back of the file (last 12 bytes) to see if we're a standalone exe
    let magic_start = file_len - MAGIC.len() - 4;
    let extracted_magic = &executable_bytes[magic_start..magic_start + MAGIC.len()];

    extracted_magic == MAGIC
}

/// if this seal executable is standalone, returns its compiled bytecode;
/// if it's not standalone, returns None
pub fn extract_bytecode(bin: Option<PathBuf>) -> Option<Vec<u8>> {
    const MAGIC: &[u8] = b"ASEALB1N";

    let executable_path = bin.unwrap_or(std::env::current_exe().ok()?);
    let executable_bytes = fs::read(&executable_path).ok()?;
    let file_len = executable_bytes.len();

    // check the back of the file (last 12 bytes) to see if we're a standalone exe
    let magic_start = file_len - MAGIC.len() - 4;
    let extracted_magic = &executable_bytes[magic_start..magic_start + MAGIC.len()];
    if extracted_magic != MAGIC {
        // we are not a standalone bin, early return None
        return None;
    }

    // read bytecode length (exactly 4 bytes from end of magic header)
    let bytecode_len = {
        let len_start = magic_start + MAGIC.len();
        let len_end = len_start + 4;

        if len_end > executable_bytes.len() {
            return None;
        }

        let len_bytes = &executable_bytes[len_start..len_end];
        u32::from_le_bytes(len_bytes.try_into().ok()?) as usize
    };

    // extract bytecode
    let bytecode_start = file_len.checked_sub(MAGIC.len() + 4 + bytecode_len)?;
    let bytecode_end = bytecode_start + bytecode_len;

    if bytecode_end > executable_bytes.len() {
        return None;
    }

    Some(executable_bytes[bytecode_start..bytecode_end].to_vec())
}

/// returns a compiled binary of <normal seal machine code><bytecode><magic><len>
/// so we only need to check the end of the bin to see if it's a standalone exec or not
pub fn standalone(src: &str) -> LuaResult<Vec<u8>> {
    let comp = Compiler::new();
    let bytecode = match comp.compile(src) {
        Ok(bytecode) => bytecode,
        Err(err) => {
            return wrap_err!("seal compile - unable to compile standalone due to err: {}", err);
        }
    };

    // need to read the current seal executable into memory so we can append magic header and bytecode
    let executable_path = match std::env::current_exe() {
        Ok(exe) => exe,
        Err(err) => {
            return wrap_err!("seal compile - cannot get this seal executable path due to err: {}", err);
        }
    };

    let mut standalone_bytes = Vec::new();
    match fs::File::open(&executable_path)
        .and_then(|mut f| f.read_to_end(&mut standalone_bytes))
    {
        Ok(_) => {},
        Err(err) => {
            return wrap_err!("seal compile - error reading current executable path: {}", err);
        }
    };

    // append magic 8 byte header + length prefix + bytecode
    const MAGIC: &[u8] = b"ASEALB1N";
    let bytecode_len = (bytecode.len() as u32).to_le_bytes();
    standalone_bytes.extend_from_slice(&bytecode);
    standalone_bytes.extend_from_slice(MAGIC);
    standalone_bytes.extend_from_slice(&bytecode_len);

    Ok(standalone_bytes)
}