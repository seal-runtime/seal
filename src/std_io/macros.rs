
/// rust in its infinite wisdom decided println! should panic
/// if stdout disappears before we've finished writing
/// (which happens when piping output to another process like `less`)
/// so we need to replace every instance of println! with this `puts` macro 
/// that behaves identically to println! except won't explode in our face 
/// if someone uses seal as a normal cli tool.
/// 
/// `puts!` silently exits seal if we hit `io::ErrorKind::BrokenPipe`
/// and returns a LuaResult version of the io error if we encounter any other issue writing to stdout
#[macro_export]
macro_rules! puts {
    () => {
        $crate::put!("\n")
    };
    ($($arg:tt)*) => {{
        use std::io::{self, Write};
        use std::process;

        let s = format!($($arg)*);

        // (avoid partial writes)
        let mut out = io::stdout().lock();

        match writeln!(out, "{}", s) {
            Ok(_) => Ok(()),
            Err(e) => {
                if e.kind() == io::ErrorKind::BrokenPipe {
                    use $crate::std_io::input::EXPECT_OUTPUT_STREAMS;
                    if EXPECT_OUTPUT_STREAMS.stdout() {
                        return wrap_err!("BrokenPipe: stdout access was cut off unexpectedly");
                    } else {
                        process::exit(0);
                    }
                }
                wrap_err!(e)
            }
        }
    }};
}

/// like puts but doesn't print a newline
#[macro_export]
macro_rules! put {
    ($($arg:tt)*) => {{
        use std::io::{self, Write};
        use std::process;

        let s = format!($($arg)*);

        // (avoid partial writes)
        let mut out = io::stdout().lock();

        match write!(out, "{}", s) {
            Ok(_) => Ok(()),
            Err(e) => {
                if e.kind() == io::ErrorKind::BrokenPipe {
                    use $crate::std_io::input::EXPECT_OUTPUT_STREAMS;
                    if EXPECT_OUTPUT_STREAMS.stdout() {
                        return wrap_err!("BrokenPipe: stdout access was cut off unexpectedly");
                    } else {
                        process::exit(0);
                    }
                }
                wrap_err!(e)
            }
        }
    }};
}

/// like eputs but doesn't print a newline
#[macro_export]
macro_rules! eput {
    ($($arg:tt)*) => {{
        use std::io::{self, Write};
        use std::process;

        let s = format!($($arg)*);

        // (avoid partial writes)
        let mut stderr = io::stderr().lock();

        match write!(stderr, "{}", s) {
            Ok(_) => Ok(()),
            Err(e) => {
                if e.kind() == io::ErrorKind::BrokenPipe {
                    use $crate::std_io::input::EXPECT_OUTPUT_STREAMS;
                    if EXPECT_OUTPUT_STREAMS.stderr() {
                        return wrap_err!("BrokenPipe: stderr access was cut off unexpectedly");
                    } else {
                        process::exit(0);
                    }
                }
                wrap_err!(e)
            }
        }
    }};
}

/// like puts but for stderr
#[macro_export]
macro_rules! eputs {
    ($($arg:tt)*) => {{
        use std::io::{self, Write};
        use std::process;

        let s = format!($($arg)*);

        // (avoid partial writes)
        let mut stderr = io::stderr().lock();

        match writeln!(stderr, "{}", s) {
            Ok(_) => Ok(()),
            Err(e) => {
                if e.kind() == io::ErrorKind::BrokenPipe {
                    use $crate::std_io::input::EXPECT_OUTPUT_STREAMS;
                    if EXPECT_OUTPUT_STREAMS.stderr() {
                        return wrap_err!("BrokenPipe: stderr access was cut off unexpectedly");
                    } else {
                        process::exit(0);
                    }
                }
                wrap_err!(e)
            }
        }
    }};
}