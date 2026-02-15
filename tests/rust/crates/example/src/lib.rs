use std::ffi::{CStr, CString, c_int};
use bstr::{BStr, BString};
use uuid::Uuid;

use seal::{ffi, push_wrapped_error, push_wrapped_c_function};

/// Checks if the function argument `arg` (by argument index) is a Luau string.
/// If it is, returns it as a BString (cloning the passed data), otherwise throws a runtime error.
/// # Safety
/// - Luau state must be non-null
pub unsafe fn args_expect_string(state: *mut ffi::lua_State, arg: c_int) -> BString {
    let mut len = 0_usize;
    let ptr = unsafe { ffi::luaL_checklstring(state, arg, &mut len) };
    // SAFETY: clones bytes of passed string so we don't free bytes owned by Luau
    let bytes = unsafe { std::slice::from_raw_parts(ptr as *const u8, len) }.to_owned();
    BString::new(bytes)
}

pub trait BStringFromPtr {
    /// Takes a pointer to a Luau/C string (owned by Luau),
    /// clones the relevant bytes and returns a BString (owned by Rust).
    /// This avoids us from freeing bytes owned by Luau.
    /// # Safety
    /// - ptr must be interpretable as CStr
    unsafe fn clone_from_ptr(ptr: *const i8) -> BString;
}
impl BStringFromPtr for BString {
    unsafe fn clone_from_ptr(ptr: *const i8) -> BString {
        // need to cstr it first cus NUL
        let cstr = unsafe { CStr::from_ptr(ptr) };
        // ensure we clone and not borrow; we do NOT want to free bytes owned by Luau
        BString::from(cstr.to_bytes().to_owned())
    }
}

#[allow(unused, reason = "only needed for debugging")]
/// # Safety
/// - state must be a non-null pointer to a lua_State
/// - `idx` must be on the luau stack
unsafe fn type_of(state: *mut ffi::lua_State, idx: c_int) -> BString {
    let ptr = unsafe { ffi::luaL_typename(state, idx) };
    unsafe { BString::clone_from_ptr(ptr) }
}

/// # Safety
/// - Luau state must be non-null
/// - stack space
/// - expects to be passed a string on the luau stack
pub unsafe extern "C-unwind" fn uuid_new_v4(state: *mut ffi::lua_State) -> c_int {
    let uuid = Uuid::new_v4();

    let s = unsafe { args_expect_string(state, 1) };
    let representation = if s.eq_ignore_ascii_case(b"simple") {
        Some(format!("{}", uuid.simple()))
    } else if s.eq_ignore_ascii_case(b"urn") {
        Some(format!("{}", uuid.as_urn()))
    } else if s.eq_ignore_ascii_case(b"braced") {
        Some(format!("{}", uuid.as_braced()))
    } else {
        push_wrapped_error(state, &format!("uuid.new_v4: expected 'mode' to be \"simple\" or \"urn\" or \"braced\", got {:?}", s));
        None
    };
    
    if let Some(representation) = representation {
        let Ok(cstring) = CString::new(representation) else {
            push_wrapped_error(state, "can't convert the string you passed into a CString (why NUL bytes hmm?)");
            return 1;
        };
        unsafe {
            ffi::lua_pushstring(state, cstring.as_ptr());
        }
    }

    1
}

/// Constructs the 'uuid' sub-library
/// # Safety
/// - Luau state must be non-null
pub unsafe extern "C-unwind" fn uuid(state: *mut ffi::lua_State) -> c_int {
    unsafe {
        ffi::lua_createtable(state, 0, 1);

        push_wrapped_c_function(state, uuid_new_v4);
        ffi::lua_setfield(state, -2, c"new_v4".as_ptr());
    }
    1
}

/// All this does is take a Duration userdata from seal and print duration:display()
/// # Safety
/// - Luau state must be non-null
/// - Check stack space
pub unsafe extern "C-unwind" fn takes_a_duration(state: *mut ffi::lua_State) -> c_int {
    unsafe { ffi::luaL_checkstack(state, 3, c"not enough stack slots to handle datetime stuff bruv".as_ptr()) };

    // ensure user actually passed a "Duration"
    let ptr = unsafe { ffi::luaL_typename(state, -1) };
    let b = unsafe { BString::clone_from_ptr(ptr) };
    if !b.eq_ignore_ascii_case(b"Duration") {
        push_wrapped_error(state, &format!("dur: expected d to be a Duration, got: {}", b));
        return 1;
    }

    let s = unsafe {
        // grab duration.display method (getfield follows metamethod __index)
        ffi::lua_getfield(state, -1, c"display".as_ptr());
        // put duration back onto the stack
        ffi::lua_pushvalue(state, -2);
        // [display fn, duration]
        // lua_call expects args to be between it and the function to call
        // self is 1 argument
        ffi::lua_call(state, 1, 1);
        let ptr = ffi::lua_tostring(state, -1);
        BString::clone_from_ptr(ptr)
    };

    println!("{}", s);
    0
}

/// # Safety
/// check luau stack usage
pub unsafe extern "C-unwind" fn say_hi(state: *mut ffi::lua_State) -> c_int {
    unsafe {
        let mut len = 0_usize;
        let s = ffi::luaL_checklstring(state, 1, &mut len);

        // bytes in the string are owned by Luau, do NOT free them
        let bytes = std::slice::from_raw_parts(s as *const u8, len);
        let s = BStr::new(bytes);
        
        println!("hi {}", s);
    }
    0
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
pub unsafe extern "C-unwind" fn seal_open_extern(state: *mut ffi::lua_State, ptr: *const seal::ffi::api::LuauApi) -> c_int {
    unsafe {
        seal::initialize(ptr);

        // libary return table
        ffi::lua_createtable(state, 0, 1);

        // put function hi in library return table
        ffi::lua_pushcfunction(state, say_hi);
        ffi::lua_setfield(state, -2, c"hi".as_ptr());

        // we need to call the c function uuid to put the uuid table on the stack
        ffi::lua_pushcfunction(state, uuid);
        // pops the c function, calls it, puts its results on Luau stack
        ffi::lua_call(state, 0, 1);

        // now we have [library, uuid] left on the stack
        ffi::lua_setfield(state, -2, c"uuid".as_ptr());

        ffi::lua_pushcfunction(state, takes_a_duration);
        ffi::lua_setfield(state, -2, c"see_duration".as_ptr());

        // library table left on stack
    }
    1
}