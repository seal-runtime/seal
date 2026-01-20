use std::panic;
use mluau::prelude::*;
use crate::globals;
use crate::prelude::*;

use std::io::Write;
use std::env::VarError;

#[cfg(unix)]
use libc::{signal, SIGABRT, exit};

#[cfg(unix)]
unsafe extern "C" fn handle_sigabrt(_: i32) {
    unsafe {
        exit(1);
    }
}

/// Intercepts SIGABRT on Unix-like systems to prevent core dumps and keep `seal` as nonblocking as possible.
pub fn setup_sigabrt_handler() {
    // Intercepts SIGABRT to prevent core dumps when `seal` is used as a child process.
    // This occurs because `ChildProcessStream` readers are fully nonblocking;
    // if the parent process exits while either `ChildProcessStream` reader is `reader.read()`ing to the inner buffer,
    // the child process may exit with a SIGABRT, causing a core dump--even though `seal` itself is functioning correctly.
    // We intercept these core dumps to prevent leaking sensitive memory.
    //
    // SAFETY:
    // - This signal is registered as early as possible in `main`.
    // - It is the only signal handler registered by `seal`; registering any other signal handlers may cause UB.
    // - Only async-signal-safe functions (such as `exit()`) are called in the signal handler.
    // - Behavior has been tested on x86_64 Arch Linux.
    #[cfg(unix)]
    unsafe {
        signal(SIGABRT, handle_sigabrt as *const () as usize);
    }
}

pub fn parse_traceback(raw_traceback: String) -> String {
    let parse_traceback = include_str!("./scripts/parse_traceback.luau");
    let luau_for_traceback = Lua::new();
    if let Err(err) = globals::set_globals(&luau_for_traceback, String::default()) {
        unreachable!("setting globals for parse_traceback.luau shouldn't fail; err: {}", err);
    }
    match luau_for_traceback.load(parse_traceback).eval() {
        Ok(LuaValue::Function(parse_traceback)) => {
            parse_traceback.call::<LuaString>(raw_traceback)
                .expect("calling parse_traceback.luau should not fail unexpectedly.")
                .to_string_lossy()
                .to_string()
        },
        Ok(other) => {
            panic!("parse_traceback.luau should return a function??, got: {:#?}", other);
        },
        Err(err) => {
            panic!("parse_traceback.luau broke with err: {err}");
        },
    }
}

pub fn setup_panic_hook() {
    if let Ok(v) = std::env::var("SEAL_DEBUG_PANICS") && v.eq_ignore_ascii_case("true") {
        return;
    }
    panic::set_hook(Box::new(|info| {
        let payload = info.payload().downcast_ref::<&str>().map(|s| s.to_string())
            .or_else(|| info.payload().downcast_ref::<String>().cloned())
            .unwrap_or_else(|| "Unknown error running the custom panic hook, please report this to the manager (deviaze)".to_string());

        if let Some(location) = info.location() {
            let payload_line = format!("{}[PANIC! IN THE SEAL INTERNALS]: {}{}{}{}", colors::BOLD_RED, colors::RESET, colors::YELLOW, payload, colors::RESET);
            let panic_top_line = format!("panic occurred at: [\"{}\"]:{}:{}", location.file(), location.line(), location.column());
            let rust_backtrace: Option<String> = if let Ok(var) = std::env::var("RUST_BACKTRACE") && !var.eq_ignore_ascii_case("0") {
                Some(std::backtrace::Backtrace::capture().to_string())
            } else {
                None
            };

            let mut stderr = std::io::stderr().lock();
            
            let we_should_have_stderr_stream = crate::std_io::input::EXPECT_OUTPUT_STREAMS.stderr();
            if we_should_have_stderr_stream {
                // we should have access to a sane stderr to print on so let's just report to it
                let _ = writeln!(stderr, "{}\n{}", payload_line, panic_top_line);
                let _ = writeln!(stderr,
                    "{}\nseal panicked. seal is not supposed to panic, so you've found a bug.\nPlease report it here: https://github.com/seal-runtime/seal/issues{}",
                    colors::RED, colors::RESET
                );
                if let Some(ref bt) = rust_backtrace {
                    let _ = writeln!(stderr, "\nRUST BACKTRACE:\n{}", bt);
                }
            }

            // even if seal is running headless/without visible stderr we want to see panics so we report to a log file
            let mut payload_lines = "seal panicked. seal is not supposed to panic, so you've found a bug.\nPlease report it here: https://github.com/seal-runtime/seal/issues"
                .to_string() + "\n" + &payload + "\n" + &panic_top_line + "\n";
            payload_lines.push_str("RUST BACKTRACE:\n");
            payload_lines += &rust_backtrace.unwrap_or(std::backtrace::Backtrace::force_capture().to_string());

            // we purposefully omit an option for disabling panic logfiles to encourage (annoy)
            // users into reporting seal bugs to the github repository
            if let Err(err) = std::fs::write("PANIC_IN_THE_SEAL_INTERNALS.log", payload_lines)
                && we_should_have_stderr_stream
            {
                let _ = writeln!(stderr, "unable to write panic log file to ./PANIC_IN_THE_SEAL_INTERNALS.log due to err: {}", err);
            };
        }
    }));
}

pub fn display_error_and_exit(err: LuaError) -> ! {
    let err = parse_traceback(err.to_string());
    let error_message = format!("{}[ERR]{} {}", colors::BOLD_RED, colors::RESET, err);

    let mut stderr = std::io::stderr().lock();
    if writeln!(stderr, "{}", error_message).is_err() {
        // welp we can't even write to stderr, ig best we can do is throw the error message in a log file?
        match std::env::var("SEAL_SKIP_BACKUP_LOGFILE") {
            Ok(var) if var.eq_ignore_ascii_case("true") => {},
            Ok(_) | Err(VarError::NotPresent) => {
                // if this fails, user has more issues going on and we can't help them atp
                let _ = std::fs::write("error.backup.seal.log", &error_message);
            },
            _ => {},
        }
    }

    if let Ok(var) = std::env::var("SEAL_LOG_ERRORS") && var.eq_ignore_ascii_case("true") {
        let file_name = format!("error.{}.seal.log", jiff::Timestamp::now());
        if let Err(err) = std::fs::write(&file_name, &error_message) {
            let _ = writeln!(stderr, "can't write error message to log file '{}' due to err: {}", file_name, err);
        };
    }

    std::process::exit(1);
}

#[macro_export]
macro_rules! wrap_err {
    ($msg:expr) => {
        Err(LuaError::external(format!("{}{}{}", colors::RED, $msg, colors::RESET)))
    };
    ($msg:expr, $($arg:tt)*) => {
        Err(LuaError::external(format!("{}{}{}", colors::RED, format!($msg, $($arg)*), colors::RESET)))
    };
}