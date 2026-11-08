//! `seal check`: type checks and lints Luau and Luwu files with Luwu's type checker (luwu's `luaanalysis.h`).
//!
//! By default it checks against seal's typedefs and resolves requires with seal's own resolver, so what it checks is
//! what `seal` runs. `--defs` checks against other definition files with the Luau require-by-string rules instead.

use crate::prelude::*;
use mluau::ffi;
use mluau::prelude::*;

use crate::{Args, LuauLoadResult, globals, require, setup};

use std::ffi::{CStr, CString, c_char};
use std::fs;
use std::path::{Path, PathBuf};

/// Directories `seal check <dir>` doesn't look in, besides hidden ones (`.seal`, `.git`)
const SKIPPED_DIRECTORIES: [&str; 2] = ["target", "node_modules"];

struct CheckOptions {
    paths: Vec<String>,
    /// Definition files given with `--defs`. Empty means checking against seal's typedefs with seal's resolver.
    definitions: Vec<String>,
}

impl CheckOptions {
    fn from_args(args: Args, function_name: &str) -> LuaResult<Self> {
        let mut paths = Vec::new();
        let mut definitions = Vec::new();
        let mut args = args.into_iter();

        while let Some(arg) = args.next() {
            let Some(arg) = arg.to_str().map(str::to_owned) else {
                return wrap_err!("{}: arguments must be valid utf-8", function_name);
            };
            if arg == "--defs" {
                let Some(path) = args.next().and_then(|path| path.into_string().ok()) else {
                    return wrap_err!("{}: --defs needs the path of a definition file after it", function_name);
                };
                definitions.push(path);
            } else if let Some(path) = arg.strip_prefix("--defs=") {
                definitions.push(path.to_owned());
            } else if arg.starts_with('-') {
                return wrap_err!("{}: unknown option '{}' (see `seal help check`)", function_name, arg);
            } else {
                paths.push(arg);
            }
        }

        if paths.is_empty() {
            if !in_seal_project() {
                return wrap_err!(
                    "{}: pass the files or directories to check (`seal check` checks '.' only inside a seal project)",
                    function_name
                );
            }
            paths.push(String::from("."));
        }

        Ok(Self { paths, definitions })
    }
}

/// Whether the current directory is in a seal project. `~/.seal` holds seal's global setup, so the home directory
/// doesn't count as a project.
fn in_seal_project() -> bool {
    let Ok(cwd) = std::env::current_dir() else {
        return false;
    };
    let Some(project) = globals::find_project(&path_string(&cwd), 1) else {
        return false;
    };
    std::env::home_dir().is_none_or(|home| project != home)
}

struct Diagnostic {
    module: String,
    kind: String,
    message: String,
    severity: i32,
    line: i32,
    column: i32,
}

/// What a check found, or why it couldn't run
struct CheckOutcome {
    diagnostics: Vec<Diagnostic>,
    legend: Option<String>,
    failure: Option<String>,
}

pub fn run(args: Args) -> LuauLoadResult {
    let function_name = "seal check";
    let options = CheckOptions::from_args(args, function_name)?;
    let files = source_files(&options.paths, function_name)?;

    // SAFETY: flags are set before any checker exists. They're process-wide, so the resolver's Lua state runs with them
    // too, which is how the `luwu` CLI runs.
    unsafe { ffi::luwu_analysis_enabledefaultflags() };

    let luau = Lua::default();
    globals::set_globals(&luau, function_name)?;

    let outcome = if options.definitions.is_empty() {
        let typedefs = seal_typedefs(&options.paths[0], function_name)?;
        let reserved_aliases = TableBuilder::create(&luau)?
            .with_value("std", path_string(&typedefs_entry(&typedefs, "std")))?
            .with_value("interop", path_string(&typedefs_entry(&typedefs, "interop")))?
            .build()?;
        let resolver = require::check_resolver(&luau, reserved_aliases)?;
        check(&luau, &[path_string(&typedefs_entry(&typedefs, "globals.d.luau"))], &files, Some(resolver), function_name)?
    } else {
        check(&luau, &options.definitions, &files, None, function_name)?
    };

    report(&outcome, files.len())
}

/// The `.luau` and `.luwu` files at `paths`, sorted: a directory gives every one under it
fn source_files(paths: &[String], function_name: &str) -> LuaResult<Vec<String>> {
    let mut files = Vec::new();
    for path in paths {
        let path = Path::new(path);
        if path.is_dir() {
            collect_source_files(path, &mut files, function_name)?;
        } else if path.is_file() {
            files.push(path_string(path));
        } else {
            return wrap_err!("{}: '{}' doesn't exist", function_name, path.display());
        }
    }
    files.sort();
    files.dedup();
    Ok(files)
}

fn collect_source_files(directory: &Path, files: &mut Vec<String>, function_name: &str) -> LuaResult<()> {
    let entries = match fs::read_dir(directory) {
        Ok(entries) => entries,
        Err(err) => {
            return wrap_err!("{}: can't read the directory '{}': {}", function_name, directory.display(), err);
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.starts_with('.') {
            continue;
        }

        if path.is_dir() {
            if !SKIPPED_DIRECTORIES.contains(&name.as_str()) {
                collect_source_files(&path, files, function_name)?;
            }
        } else if is_source_file(&name) {
            files.push(path_string(&path));
        }
    }
    Ok(())
}

/// Whether `seal check <dir>` checks a file named `name`. Definition files (`*.d.luau`) aren't modules.
fn is_source_file(name: &str) -> bool {
    (name.ends_with(".luau") || name.ends_with(".luwu")) && !name.ends_with(".d.luau")
}

/// The typedefs of the seal project `path` is in, or else this seal's own, unpacked into the temp directory
fn seal_typedefs(path: &str, function_name: &str) -> LuaResult<PathBuf> {
    let absolute = std::path::absolute(path).unwrap_or_else(|_| PathBuf::from(path));
    if let Some(project) = globals::find_project(&path_string(&absolute), 1) {
        let typedefs = typedefs_entry(&typedefs_entry(&project, ".seal"), "typedefs");
        if typedefs_entry(&typedefs, "globals.d.luau").is_file() {
            return Ok(typedefs);
        }
    }

    let typedefs = typedefs_entry(&std::env::temp_dir(), &format!("seal-typedefs-{}", globals::display_version()));
    if !typedefs_entry(&typedefs, "globals.d.luau").is_file() && let Err(err) = setup::extract_typedefs_into(&typedefs) {
        return wrap_err!("{}: can't unpack seal's typedefs into '{}': {}", function_name, typedefs.display(), err);
    }
    Ok(typedefs)
}

/// `directory/name`, for the fixed names `seal check` looks up next to typedefs
#[allow(clippy::disallowed_methods, reason = "`name` is a relative name seal check chose, never user input")]
fn typedefs_entry(directory: &Path, name: &str) -> PathBuf {
    directory.join(name)
}

fn path_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn c_string(value: &str, function_name: &str) -> LuaResult<CString> {
    match CString::new(value) {
        Ok(value) => Ok(value),
        Err(_) => wrap_err!("{}: the path '{}' contains a NUL byte", function_name, value),
    }
}

/// An owned copy of a C string the checker returned, or None for NULL
///
/// # Safety
/// `pointer` is NULL or a NUL-terminated string that stays valid during the call.
unsafe fn owned(pointer: *const c_char) -> Option<String> {
    if pointer.is_null() {
        None
    } else {
        // SAFETY: the caller promises a valid NUL-terminated string
        Some(unsafe { CStr::from_ptr(pointer) }.to_string_lossy().into_owned())
    }
}

/// Checks `files` against `definitions`, resolving requires with `resolver` when there is one.
///
/// The whole check runs inside one `exec_raw` call: the checker calls the resolver on the raw Lua state, which is only
/// safe to use while mluau has handed it out.
fn check(
    luau: &Lua,
    definitions: &[String],
    files: &[String],
    resolver: Option<LuaFunction>,
    function_name: &str,
) -> LuaResult<CheckOutcome> {
    let definitions = definitions.iter().map(|path| c_string(path, function_name)).collect::<LuaResult<Vec<_>>>()?;
    let files = files.iter().map(|path| c_string(path, function_name)).collect::<LuaResult<Vec<_>>>()?;

    let mut outcome = CheckOutcome {
        diagnostics: Vec::new(),
        legend: None,
        failure: None,
    };

    // SAFETY: the state is only used inside the closure, and the resolver ref is released before it returns
    unsafe {
        luau.exec_raw::<()>(resolver, |state| {
            let analysis = ffi::luwu_analysis_new();
            if analysis.is_null() {
                outcome.failure = Some(String::from("couldn't create the type checker"));
                return;
            }

            run_check(analysis, state, &definitions, &files, &mut outcome);

            ffi::luwu_analysis_free(analysis);
        })?;
    }

    Ok(outcome)
}

/// # Safety
/// `analysis` is a live checker and `state` is the Lua state mluau handed to `exec_raw`, with the resolver (or nil) on
/// top of its stack.
unsafe fn run_check(
    analysis: *mut ffi::luwu_Analysis,
    state: *mut ffi::lua_State,
    definitions: &[CString],
    files: &[CString],
    outcome: &mut CheckOutcome,
) {
    unsafe {
        for path in definitions {
            let failed = ffi::luwu_analysis_loaddefinitions(analysis, path.as_ptr()) != 0;
            collect_diagnostics(analysis, outcome);
            if failed {
                outcome.failure = owned(ffi::luwu_analysis_lasterror(analysis));
                return;
            }
        }

        let has_resolver = ffi::lua_isfunction(state, -1) != 0;
        let resolver_ref = if has_resolver { ffi::lua_ref(state, -1) } else { ffi::LUA_NOREF };
        if has_resolver {
            ffi::luwu_analysis_setresolver(analysis, state, resolver_ref);
        }

        for path in files {
            ffi::luwu_analysis_queue(analysis, path.as_ptr());
        }

        if ffi::luwu_analysis_check(analysis) == 0 {
            collect_diagnostics(analysis, outcome);
            outcome.legend = owned(ffi::luwu_analysis_legend(analysis));
        } else {
            outcome.failure = owned(ffi::luwu_analysis_lasterror(analysis));
        }

        if has_resolver {
            ffi::luwu_analysis_setresolver(analysis, std::ptr::null_mut(), ffi::LUA_NOREF);
            ffi::lua_unref(state, resolver_ref);
        }
    }
}

/// # Safety
/// `analysis` is a live checker.
unsafe fn collect_diagnostics(analysis: *mut ffi::luwu_Analysis, outcome: &mut CheckOutcome) {
    unsafe {
        let count = ffi::luwu_analysis_diagnosticcount(analysis);
        for index in 0..count {
            let mut raw = std::mem::MaybeUninit::<ffi::luwu_Diagnostic>::uninit();
            if ffi::luwu_analysis_getdiagnostic(analysis, index, raw.as_mut_ptr()) != 0 {
                continue;
            }
            let raw = raw.assume_init();
            outcome.diagnostics.push(Diagnostic {
                module: owned(raw.module).unwrap_or_default(),
                kind: owned(raw.kind).unwrap_or_default(),
                message: owned(raw.message).unwrap_or_default(),
                severity: raw.severity,
                line: raw.beginline,
                column: raw.begincolumn,
            });
        }
    }
}

fn severity_color(severity: i32) -> &'static str {
    match severity {
        ffi::LUWU_DIAGNOSTIC_ERROR => colors::BOLD_RED,
        ffi::LUWU_DIAGNOSTIC_WARNING => colors::BOLD_YELLOW,
        _ => colors::BOLD_BLUE,
    }
}

fn plural(count: usize, word: &str) -> String {
    if count == 1 {
        format!("{} {}", count, word)
    } else {
        format!("{} {}s", count, word)
    }
}

/// Prints the diagnostics as `path:line:column: Kind: message` and exits with 1 when any is an error
fn report(outcome: &CheckOutcome, file_count: usize) -> LuauLoadResult {
    let use_colors = !colors::are_disabled();

    for diagnostic in &outcome.diagnostics {
        let location = format!("{}:{}:{}", diagnostic.module, diagnostic.line + 1, diagnostic.column + 1);
        if use_colors {
            puts!(
                "{}: {}{}{}: {}",
                location,
                severity_color(diagnostic.severity),
                diagnostic.kind,
                colors::RESET,
                diagnostic.message
            )?;
        } else {
            puts!("{}: {}: {}", location, diagnostic.kind, diagnostic.message)?;
        }
    }

    if let Some(legend) = &outcome.legend {
        puts!("\n{}", legend)?;
    }

    if let Some(failure) = &outcome.failure {
        return wrap_err!("seal check: {}", failure);
    }

    let errors = outcome.diagnostics.iter().filter(|diagnostic| diagnostic.severity == ffi::LUWU_DIAGNOSTIC_ERROR).count();
    let warnings = outcome.diagnostics.iter().filter(|diagnostic| diagnostic.severity == ffi::LUWU_DIAGNOSTIC_WARNING).count();

    if errors == 0 && warnings == 0 {
        puts!("seal check: no problems in {}", plural(file_count, "file"))?;
    } else {
        puts!()?;
        puts!("seal check: {} and {} in {}", plural(errors, "error"), plural(warnings, "warning"), plural(file_count, "file"))?;
    }

    if errors > 0 {
        std::process::exit(1);
    }
    Ok(None)
}
