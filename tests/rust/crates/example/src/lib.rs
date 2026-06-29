use std::{ffi::c_int, fmt};

use sealbindings::{StateExt, LuauState, SealValue};
use uuid::Uuid;

enum UuidV4Representation {
    Simple,
    Urn,
    Braced,
}
impl UuidV4Representation {
    fn from_value(value: SealValue, function_name: &'static str) -> Result<Self, String> {
        let which = match value {
            SealValue::String(rep) => {
                if rep.eq_ignore_ascii_case(b"simple") {
                    Self::Simple
                } else if rep.eq_ignore_ascii_case(b"urn") {
                    Self::Urn
                } else if rep.eq_ignore_ascii_case(b"braced") {
                    Self::Braced
                } else {
                    return Err(format!("{}: expected representation to be a string simple, urn, or braced, got: {}", function_name, rep));
                }
            },
            other => {
                return Err(format!("{}: expected representation to be a string, got: {:?}", function_name, other));
            }
        };

        Ok(which)
    }
}
impl fmt::Display for UuidV4Representation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let uuid = Uuid::new_v4();
        let rep = match self {
            Self::Simple => format!("{}", uuid.as_simple()),
            Self::Braced => format!("{}", uuid.as_braced()),
            Self::Urn => format!("{}", uuid.as_urn()),
        };
        f.write_str(&rep)?;
        Ok(())
    }
}

extern "C-unwind" fn uuid_new_v4(state: *mut LuauState) -> c_int {
    let function_name = "uuid.new_v4(representation: string)";

    let _stack_guard = state.stack_returns(1);

    let value = state.to_seal(-1);
    let representation = match UuidV4Representation::from_value(value, function_name) {
        Ok(rep) => rep,
        Err(err) => {
            return state.push_wrapped_error(err);
        }
    };

    state.push_str(representation.to_string());

    1
}

extern "C-unwind" fn say_hi(state: *mut LuauState) -> c_int {
    let _sg = state.stack_returns_none_or_errs();

    let name = match state.to_seal(-1) {
        SealValue::String(s) => {
            s.to_string()
        },
        other => {
            return state.push_wrapped_error(format!("say_hi expected string, got: {:?}", other));
        }
    };
    
    println!("Hi {} from the dynamic library :3", name);

    0
}

extern "C-unwind" fn how_long(state: *mut LuauState) -> c_int {
    let _sg = state.stack_returns_or_errs(1);

    let time = match state.to_seal(-1) {
        SealValue::UserData { type_name, .. } => {
            if let Some(ref name) = type_name && name.eq_ignore_ascii_case(b"duration") {
                // lua_getfield works on userdatas
                // SAFETY: we know stack idx -1 is a userdata
                let tag = unsafe { state.get_field(-1, c"seconds") };
                if tag != sealbindings::ffi::LUA_TNUMBER() {
                    return state.push_wrapped_error("Duration.seconds should be a number... what did you pass?");
                }

                // SAFETY: Duration.seconds put a number onto stack
                let f = unsafe { state.to_number(-1) };

                // state.to_number doesn't pop; we should remove Duration.seconds
                // SAFETY: Duration.seconds put one value onto stack, we are allowed to pop the value
                unsafe { state.pop(1) };
                f
            } else {
                return state.push_wrapped_error(format!("got unexpected userdata type: {:?}", type_name));
            }
        },
        other => {
            return state.push_wrapped_error(format!("expected time to be a duration, got: {:?}", other));
        }
    };

    if time > 10.0 {
        state.push_str("wow that's a lot of time");
    } else if time.is_sign_negative() && time < 0.0 {
        state.push_str("wow negative time i see");
    } else {
        state.push_str("not very long eh");
    }

    1
}

// example of what happens if you panic in a properly wrapped seal extern
// right now we get "Rust cannot catch foreign exceptions" but at least
// the panic handler catches it and says where the panic came from
// importantly, seal itself doesn't panic
unsafe extern "C-unwind" fn kaboom(state: *mut LuauState) -> c_int {
    match state.to_seal(-1) {
        SealValue::Boolean(should) if should => {
            panic!("kaboom");
        },
        _other => {
            state.push_str("everything ok");
        }
    }
    1
}

/// The entrypoint to an extern library/plugin for the seal runtime.
/// 
/// This function must return one value on the Luau stack,
/// usually a table (usually of functions) exposed by this library.
/// 
/// # Safety
/// - Caller must pass a valid, non-null pointer to a lua_State.
/// - This library must use sealbindings or equivalent to access *seal*'s exposed
///   C-stack API, and should not bind to Luau separately.
/// - This library *must* be kept alive by *seal* (or the caller) for 'static (forever).
///   If the library is prematurely closed, or functions from this library
///   are dropped, subsequent calls to those functions from Luau WILL cause segfaults and/or UB.
///   In Rust, use `std::mem::ManuallyDrop` to keep a libloading Library alive for longer than the function call.
/// - This function must call `sealbindings::initialize()` immediately.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn seal_open_extern(
    state: *mut sealbindings::LuauState,
    api: *const sealbindings::LuauApi,
) -> c_int {
    // SAFETY: seal passes a valid pointer to a lua_State (LuauState) as well as its own C API.
    // the caller is responsible for passing valid pointers.
    unsafe {
        sealbindings::initialize(state, api, |state| {
            // prevents us from accidentally returning too few or too many values on luau stack
            let _stack_guard = state.stack_returns(1);

            state.create_table(0, 4);
            state.set_wrapped_function(c"hi", say_hi, c"simple.hi(name: string)");

            // stack: [ lib ]

            // create uuid table for sublib
            state.create_table(0, 1);
            // stack: [ uuid table, lib ]

            state.set_wrapped_function(
                c"new_v4", 
                uuid_new_v4, 
                c"simple.uuid.new_v4(representation: \"Simple\" | \"Urn\" | \"Braced\") -> string"
            );

            // set lib.uuid = uuid table
            state.set_field(-2, c"uuid");

            // stack: [ lib ]
            state.set_wrapped_function(c"kaboom", kaboom, c"<unsafe> simple.kaboom(b: boolean) -> string | never");
            state.set_wrapped_function(c"how_long", how_long, c"simple.how_long(time: Duration) -> string");

            // stack: [ lib ]
            1
        })
    }
}