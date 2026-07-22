use std::panic;
use mluau::prelude::*;
use crate::globals;
use crate::prelude::*;
use crate::std_err::WrappedError;

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
    match luau_for_traceback.load(parse_traceback).set_name("seal/src/err/parse_traceback.luau").eval() {
        Ok(LuaValue::Function(parse_traceback)) => {
            match parse_traceback.call::<LuaValue>(raw_traceback) {
                Ok(LuaValue::String(s)) => s.to_string_lossy(),
                Ok(LuaValue::UserData(ud)) => {
                    match WrappedError::borrowed(ud) {
                        Ok(err) => {
                            panic!("parse_traceback.luau (error formatter) errored at runtime:\n\n{}", err.message());
                        },
                        Err(name) => {
                            panic!("parse_traceback.luau should return string or return an error, got userdata of type: {}", name);
                        }
                    }
                }
                Ok(other) => {
                    panic!("parse_traceback.luau should return string or return an error, got: {:?}", other);
                },
                Err(err) => {
                    panic!("parse_traceback.luau should not fail unexpectedly; got error message: {}", err);
                },
            }
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
            let nc = colors::are_disabled();
            
            let (bold_red, yellow, red, reset) = if nc { ("", "", "", "") } else { (colors::BOLD_RED, colors::YELLOW, colors::RED, colors::RESET) };
            let payload_line = format!("{}[PANIC! IN THE SEAL INTERNALS]: {}{}{}{}", bold_red, reset, yellow, payload, reset);
            let panic_occurred_at = format!("panic occurred at: [\"{}\"]:{}:{}", location.file(), location.line(), location.column());
            let rust_backtrace: Option<String> = if let Ok(var) = std::env::var("RUST_BACKTRACE") && !var.eq_ignore_ascii_case("0") {
                Some(std::backtrace::Backtrace::capture().to_string())
            } else {
                None
            };

            let save_path = {
                let dt = jiff::Zoned::now().strftime("%Y-%m-%dT%H:%M:%S%z");
                format!("PANIC_IN_THE_SEAL_INTERNALS_{dt}.log")
            };

            let seal_panicked_message = format!("seal panicked. seal is not supposed to panic, so you've found a bug.\nContext and traceback written to {}.\n\nPlease report it here: https://github.com/seal-runtime/seal/issues", save_path);

            let mut stderr = std::io::stderr().lock();
            
            let we_should_have_stderr_stream = crate::std_io::input::EXPECT_OUTPUT_STREAMS.stderr();
            if we_should_have_stderr_stream {
                // we should have access to a sane stderr to print on so let's just report to it
                let _ = writeln!(stderr, "{}\n{}", payload_line, panic_occurred_at);
                let _ = writeln!(stderr, "{r}\n{seal_panicked_message}{res}",
                    r=red,
                    res=reset
                );
                if let Some(ref bt) = rust_backtrace {
                    let _ = writeln!(stderr, "\nRUST BACKTRACE:\n{}", bt);
                }
            }

            let context = std::env::current_exe().ok().map(|env| {
                let mut displayable = format!("executable: {}",  env.display());
                let args = std::env::args()
                    .collect::<Vec<String>>()
                    .join(" ");
                displayable.push('\n');
                displayable.push_str("args: ");
                displayable.push_str(&args);
                displayable
            });


            // even if seal is running headless/without visible stderr we want to see panics so we report to a log file
            let mut payload_lines = format!(
                "{pan}\n\nmessage: {pay}\n{wher}\n{cont}\n",
                pan=seal_panicked_message,
                pay=payload,
                wher=panic_occurred_at,
                cont=context.unwrap_or_default()
            );
            payload_lines.push_str("\nRUST BACKTRACE:\n");
            payload_lines += &rust_backtrace.unwrap_or(std::backtrace::Backtrace::force_capture().to_string());

            // we purposefully omit an option for disabling panic logfiles to encourage (annoy)
            // users into reporting seal bugs to the github repository
            if let Err(err) = std::fs::write(&save_path, payload_lines)
                && we_should_have_stderr_stream
            {
                let _ = writeln!(stderr, "unable to write panic log file to ./{} due to err: {}", save_path, err);
            };
        }
    }));
}

pub fn display_error_and_exit(err: LuaError) -> ! {
    let nc = colors::are_disabled();
    let (bold_red, bold_yellow, reset) = if nc { ("", "", "") } else { (colors::BOLD_RED, colors::BOLD_YELLOW, colors::RESET) };
    let err = parse_traceback(err.to_string());
    let error_message = format!("{}[ERR]{} {}", bold_red, reset, err);

    let mut stderr = std::io::stderr().lock();

    // if Luau code hits an error while in rawmode it can lock out the entire terminal.
    // we need to ensure seal gives user back control over their terminal so they're not
    // stuck in rawmode in an alternate screen; this also ensures we display the actual error
    // message correctly and legibly instead of having it scattered across all different columns.
    if let Ok(is_raw) = crossterm::terminal::is_raw_mode_enabled() && is_raw {
        let _ = crossterm::terminal::disable_raw_mode();
        let _  = crossterm::execute!(
            std::io::stdout(),
            crossterm::event::DisableMouseCapture,
            crossterm::event::DisableFocusChange,
            crossterm::event::DisableBracketedPaste,
            crossterm::terminal::LeaveAlternateScreen,
            crossterm::terminal::EnableLineWrap,
            crossterm::cursor::SetCursorStyle::DefaultUserShape,
            crossterm::cursor::MoveToColumn(0),
            crossterm::cursor::Show,
            crossterm::terminal::Clear(crossterm::terminal::ClearType::FromCursorDown),
        );
        let warning_message = format!("{}[WARN]{} the program errored in terminal raw mode; switching back to cooked mode and returning control to the user.", bold_yellow, reset);
        let _ = writeln!(stderr, "{}", warning_message);
    }

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
    ($msg:expr) => {{
        let msg = $msg.to_string();
        Err(LuaError::external(if colors::are_disabled() { msg } else { format!("{}{}{}", colors::RED, msg, colors::RESET) }))
    }};
    ($msg:expr, $($arg:tt)*) => {{
        let msg = format!($msg, $($arg)*);
        Err(LuaError::external(if colors::are_disabled() { msg } else { format!("{}{}{}", colors::RED, msg, colors::RESET) }))
    }};
}