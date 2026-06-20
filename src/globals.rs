use std::path::{Path, PathBuf};

use mluau::prelude::*;
use crate::prelude::*;
use crate::std_err::ecall;
use crate::{require, std_io};

pub fn error(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
    let function_name = "error(message: string | unknown, level: number?)";
    let message = match multivalue.pop_front() {
        Some(LuaValue::String(s)) => s.to_string_lossy(),
        Some(LuaNil) | None => String::default(),
        Some(other) => std_io::format::pretty(luau, other.into_lua_multi(luau)?)?,
    };

    let level = match multivalue.pop_front() {
        Some(LuaValue::Number(f)) => Some(float_to_usize(f, function_name, "level")?),
        Some(LuaValue::Integer(i)) => Some(int_to_usize(i, function_name, "level")?),
        Some(LuaNil) | None => None,
        Some(other) => {
            return wrap_err!("{}: level expected to be number or nil/unspecified, got: {:?}", function_name, other);
        }
    };

    if let Some(level) = level {
        let traceback = luau.traceback(Some(&message), level)?.to_string_lossy();
        Err(LuaError::runtime(traceback))
    } else {
        wrap_err!("{}", message)
    }
}

pub fn warn(luau: &Lua, warn_value: LuaValue) -> LuaValueResult {
    let formatted_text = std_io::format::pretty(luau, warn_value.into_lua_multi(luau)?)?;
    eputs!("{}[WARN]{} {}{}", colors::BOLD_YELLOW, colors::RESET, formatted_text, colors::RESET)?;
    Ok(LuaNil)
}

pub const SEAL_VERSION: &str = env!("CARGO_PKG_VERSION");
const GIT_COMMIT: &str = env!("GIT_COMMIT_HASH");
const GIT_BRANCH: &str = env!("GIT_BRANCH");
const SEAL_URL: &str = "https://github.com/seal-runtime/seal";
const LUAU_URL: &str = "https://github.com/mluau/luau";

pub fn set_globals<S: AsRef<str>>(luau: &Lua, entry_path: S) -> LuaValueResult {
    let globals: LuaTable = luau.globals();
    // must use globals().get instead of globals().raw_get due to safeenv/sandbox (which requires newindex); raw_get incorrectly returns nil when safeenv enabled
    let luau_version: LuaString = globals.get("_VERSION")?;
    globals.raw_set("require", luau.create_function(require::require)?)?;
    globals.raw_set("error", luau.create_function(error)?)?;
    globals.raw_set("p", luau.create_function(std_io::output::simple_print_and_return)?)?;
    globals.raw_set("pp", luau.create_function(std_io::output::pretty_print_and_return)?)?;
    globals.raw_set("dp", luau.create_function(std_io::output::debug_print)?)?;
    globals.raw_set("print", luau.create_function(std_io::output::pretty_print)?)?;
    globals.raw_set("ecall", luau.create_function(ecall)?)?;
    globals.raw_set("warn", luau.create_function(warn)?)?;
    globals.raw_set("_SEAL_VERSION", SEAL_VERSION)?;
    globals.raw_set("_VERSION", format!("seal {}+{}", SEAL_VERSION, luau_version.to_string_lossy().strip_prefix("Luau ").expect("Luau version always has Luau prefix")))?;
    globals.raw_set("_G", TableBuilder::create(luau)?.build()?)?;
    globals.raw_set("_REQUIRE_CACHE", TableBuilder::create(luau)?.build()?)?;
    globals.raw_set("script", TableBuilder::create(luau)?
        .with_value("entry_path", entry_path.as_ref())?
        .with_function("path", get_script_path)?
        .with_function("parent", get_script_parent)?
        .with_function("project", get_script_project)?
        .build_readonly()?
    )?;

    globals.raw_set("_RUNTIME", create_runtime_global(luau)?)?;
    globals.raw_set("_LUAU", create_luau_global(luau, &luau_version.to_string_lossy())?)?;

    Ok(LuaNil)
}

fn parse_semver(version: &str) -> (u64, u64, u64, Option<String>, Option<String>) {
    let (version_core, build) = match version.find('+') {
        Some(i) => (&version[..i], Some(version[i + 1..].to_string())),
        None => (version, None),
    };
    let (version_numbers, prerelease) = match version_core.find('-') {
        Some(i) => (&version_core[..i], Some(version_core[i + 1..].to_string())),
        None => (version_core, None),
    };
    let parts: Vec<&str> = version_numbers.split('.').collect();
    let major = parts.first().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch, prerelease, build)
}

/// Create _RUNTIME global according to the spec at https://gist.github.com/Bottersnike/001470cbbb0cd63d9790a542ed5be1bf
fn create_runtime_global(luau: &Lua) -> LuaResult<LuaTable> {
    let (major, minor, patch, prerelease, build) = parse_semver(SEAL_VERSION);

    let semantic = TableBuilder::create(luau)?
        .with_value("major", major)?
        .with_value("minor", minor)?
        .with_value("patch", patch)?
        .with_value("prerelease", prerelease)?
        .with_value("build", build)?
        .build_readonly()?;

    let git = TableBuilder::create(luau)?
        .with_value("url", SEAL_URL)?
        .with_value("commit", GIT_COMMIT)?
        .with_value("branch", GIT_BRANCH)?
        .build_readonly()?;

    let version = TableBuilder::create(luau)?
        .with_value("display", SEAL_VERSION)?
        .with_value("semantic", semantic)?
        .with_value("git", git)?
        .build_readonly()?;

    let extra = TableBuilder::create(luau)?
        .with_value("jit", cfg!(not(target_os = "android")))?
        .with_value("debug", cfg!(debug_assertions))?
        .build_readonly()?;

    TableBuilder::create(luau)?
        .with_value("name", "seal")?
        .with_value("version", version)?
        .with_value("url", SEAL_URL)?
        .with_value("extra", extra)?
        .build_readonly()
}

/// Create _LUAU global according to the spec at https://gist.github.com/Bottersnike/001470cbbb0cd63d9790a542ed5be1bf
fn create_luau_global(luau: &Lua, raw_luau_version: &str) -> LuaResult<LuaTable> {
    // raw_luau_version is "Luau 0.722" — strip "Luau " prefix
    let display = raw_luau_version.strip_prefix("Luau ").unwrap_or(raw_luau_version);
    let mut parts = display.splitn(2, '.');
    let major: u64 = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor: u64 = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);

    let version = TableBuilder::create(luau)?
        .with_value("display", display)?
        .with_value("major", major)?
        .with_value("minor", minor)?
        .build_readonly()?;

    TableBuilder::create(luau)?
        .with_value("version", version)?
        .with_value("url", LUAU_URL)?
        .build_readonly()
}

const SCRIPT_PATH_SRC: &str = r#"
    requiring_file = ""
    local debug_name: string = (debug :: any).info(3, "s") --[[ this should give us the
        debug name (set by luau.load().set_name) for the chunk that called require(),
        in the format `[string "./src/somewhere.luau"]`
    ]]
    requiring_file = string.sub(debug_name, 10, -3) -- grabs the part between `[string "` and `"]`
    return requiring_file
"#;

pub fn get_debug_name(luau: &Lua) -> LuaResult<String> {
    let chunk = Chunk::Src(SCRIPT_PATH_SRC.to_owned());
    luau.load(chunk).eval::<String>()
}

pub fn get_script_path(luau: &Lua, _multivalue: LuaMultiValue) -> LuaValueResult {
    let requiring_file = get_debug_name(luau)?;
    let requiring_file = luau.create_string(&requiring_file)?;
    Ok(LuaValue::String(requiring_file))
}

pub fn get_script_parent(luau: &Lua, _multivalue: LuaMultiValue) -> LuaValueResult {
    let requiring_parent = {
        let chunk = Chunk::Src(SCRIPT_PATH_SRC.to_owned());
        let result: LuaString = luau.load(chunk).eval()?;
        let script_path = result.to_string_lossy();
        match std::path::PathBuf::from(script_path).parent() {
            Some(parent) => parent.to_string_lossy().to_string(),
            None => {
                return wrap_err!("script:parent(): script does not have a parent :skull 💀:");
            }
        }
    };
    let parent_string = luau.create_string(&requiring_parent)?;
    Ok(LuaValue::String(parent_string))
}

pub fn find_project(path: &str, projects_up: usize) -> Option<PathBuf> {
    let mut current = Path::new(path).to_path_buf();

    if current.is_file() {
        current = current.parent()?.to_path_buf();
    }

    let mut matches = 0;

    loop {
        let seal_dir = current.join(".seal");
        if seal_dir.is_dir() {
            matches += 1;
            if matches == projects_up {
                return Some(current);
            }
        }

        // Move up one directory
        match current.parent() {
            Some(parent) => current = parent.to_path_buf(),
            None => break, // Reached filesystem root
        }
    }

    None
}

pub fn get_script_project(luau: &Lua, mut multivalue: LuaMultiValue) -> LuaValueResult {
    let function_name = "script:project(n: number?)";
    pop_self(&mut multivalue, function_name)?;
    let projects_up = match multivalue.pop_front() {
        Some(LuaValue::Integer(i)) => int_to_usize(i, function_name, "n")?,
        Some(LuaValue::Number(f)) => float_to_usize(f, function_name, "n")?,
        Some(LuaNil) | None => 1,
        Some(other) => {
            return wrap_err!("{} expected number of projects up to be a number or nil/unspecified, got: {:?}", function_name, other);
        }
    };

    let requiring_file = get_debug_name(luau)?;
    match find_project(&requiring_file, projects_up) {
        Some(project) => ok_string(project.to_string_lossy().to_string(), luau),
        None => {
            wrap_err!("{}: project not found relative to '{}'! consider using fs.path.project instead (which doesn't error)", function_name, requiring_file)
        }
    }
}