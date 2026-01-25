use std::ffi::{CString, c_int};
use mluau::ffi;
use bstr::{BStr, BString};
use uuid::Uuid;

unsafe fn expect_string(state: *mut ffi::lua_State) -> BString {
    let mut len = 0_usize;
    let ptr = unsafe { ffi::luaL_checklstring(state, 1, &mut len) };
    // SAFETY: clones bytes of passed string so we don't free bytes owned by Luau
    let bytes = unsafe { std::slice::from_raw_parts(ptr as *const u8, len) }.to_owned();
    BString::new(bytes)
}

/// Pushes a wrapped error object from @std/err onto the Luau stack.
/// After this returns, the stack top is the wrapped error.
///
/// # Safety
/// - Error message should not contain any NUL bytes.
unsafe fn wrapped_error(state: *mut ffi::lua_State, msg: &str) {
    // just use seal's @std/err library to construct the error
    unsafe {
        // - push require to stack
        ffi::lua_getglobal(state, c"require".as_ptr());
        // stack: [ require ]
    
        // push "@std/err"
        ffi::lua_pushstring(state, c"@std/err".as_ptr());
        // stack: [ require, "@std/err" ]
    
        // Step 3: call require("@std/err")
        ffi::lua_call(state, 1, 1);
        // stack: [ err_table ]
    
        // Step 4: get err.wrap
        ffi::lua_getfield(state, -1, c"wrap".as_ptr());
        // stack: [ err_table, err.wrap ]
    
        let error_message = CString::new(msg).expect("error message contains internal NUL bytes");
        ffi::lua_pushstring(state, error_message.as_ptr());
        // stack: [ err_table, err.wrap, msg ]
    
        // Step 6: call wrap(msg)
        ffi::lua_call(state, 1, 1);
        // stack: [ err_table, wrapped_error ]
    
        // Step 7: remove err_table, leave wrapped_error
        ffi::lua_remove(state, -2);
        // stack: [ wrapped_error ]
    }
}

/// Calls the global `ecall` function with a given C function pointer.
/// This allows wrapped errors returned by the C function to be thrown like seal errors
/// After this returns, the stack top is the function returned by ecall.
/// Caller should `return 1` or continue stack manipulation.
///
/// # Safety
/// - Leaves exactly 1 value on the stack.
/// - Does not panic.
/// - Does not free any Luau-owned memory.
pub unsafe fn ecall_c_function(
    state: *mut ffi::lua_State,
    func: ffi::lua_CFunction,
) {
    unsafe {
        // Step 1: push global ecall
        ffi::lua_getglobal(state, c"ecall".as_ptr());
        // stack: [ ecall ]
    
        // Step 2: push the C function to wrap
        ffi::lua_pushcfunction(state, func);
        // stack: [ ecall, func ]
    
        // Step 3: call ecall(func)
        // Pops ecall + func, pushes return value
        ffi::lua_call(state, 1, 1);
        // stack: [ wrapped_function ]
    }
}


/// # Safety
/// Expects to be passed a string on the luau stack
/// Returns a string on the luau stack
pub unsafe extern "C-unwind" fn uuid_new_v4(state: *mut ffi::lua_State) -> c_int {
    let uuid = Uuid::new_v4();

    let s = unsafe { expect_string(state) };
    let representation = if s.eq_ignore_ascii_case(b"simple") {
        Some(format!("{}", uuid.simple()))
    } else if s.eq_ignore_ascii_case(b"urn") {
        Some(format!("{}", uuid.as_urn()))
    } else if s.eq_ignore_ascii_case(b"braced") {
        Some(format!("{}", uuid.as_braced()))
    } else {
        unsafe {
            wrapped_error(state, &format!("uuid.new_v4: expected 'mode' to be \"simple\" or \"urn\" or \"braced\", got {:?}", s));
        }
        None
    };

    if let Some(representation) = representation {
        unsafe {
            ffi::lua_pushstring(state, CString::new(representation).unwrap().as_ptr());
        }
    }

    1
}

/// # Safety
/// Returns 1 value on the Luau stack
pub unsafe extern "C-unwind" fn uuid(state: *mut ffi::lua_State) -> c_int {
    unsafe {
        ffi::lua_createtable(state, 0, 2);

        ecall_c_function(state, uuid_new_v4);
        ffi::lua_setfield(state, -2, c"new_v4".as_ptr());
    }
    1
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

/// Function must return one value on the Luau stack,
/// usually a table (usually of functions) exposed by this library
/// 
/// # Safety
/// This library should NOT use mluau::Lua::get_or_init_from_ptr, 
/// reconstructing the mluau::Lua state can only work if *seal* and this library
/// were compiled at the same time by the exact same version of the Rust compiler.
/// Rust does NOT have a stable ABI so that's not possible to rely upon.
/// 
/// This library *must* be kept alive by *seal* (or the caller), otherwise calling functions returned
/// by this library WILL cause segfaults. Use `std::mem::ManuallyDrop` for this.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn seal_open_extern(state: *mut mluau::ffi::lua_State) -> c_int {
    unsafe {
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

        // library table left on stack
    }
    1
}