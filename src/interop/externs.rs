use std::path::Path;

use mluau::prelude::{*};
use mluau::ffi::{self, lua_State};
use crate::prelude::{*};

use libloading::Library;
use std::mem::ManuallyDrop;

type SealOpenExtern = unsafe extern "C-unwind" fn(*mut lua_State) -> i32;

/// Opens a dynamic library,
/// 
/// ### Arguments (on the Luau stack) 
/// - An absolute path to the Rust/C library (on the Luau stack)
/// 
/// ### Returns (on the Luau stack)
/// - (true, LuaValue) if loading the library worked
/// - (false, string) if loading the library didn't work
///   the string should be the error message
/// 
/// ### Returns 
/// - an i32 that should always be 2 (number of returns on the Luau stack).
/// 
/// # Safety
/// Passed Luau state must be valid and alive and outlive this call.
/// Returns the LuaValue returned by the external library
pub fn extern_load(luau: &Lua, path: String) -> LuaValueResult {
    let library_path = Path::new(&path);

    let library = unsafe { 
        match Library::new(library_path) {
            Ok(lib) => lib,
            Err(err) => {
                return wrap_err!("_LOAD_EXTERN: unable to open library at '{}' due to err {}", &path, err);
            }
        }
    };
    let library = ManuallyDrop::new(library);

    // resolve symbol to seal_open_extern in dynamic library
    let seal_open_extern: libloading::Symbol<SealOpenExtern> = unsafe {
        match library.get(b"seal_open_extern") {
            Ok(symbol) => symbol,
            Err(err) => {
                return wrap_err!("can't find symbol 'seal_open_extern' in library at '{}' due to reason {}", &path, err);
            }
        }
    };

    let mut error_message: Option<String> = None;
    let v: LuaValue = unsafe {
        luau.exec_raw::<LuaValue>((), |state| {
            // number elements on the luau stack before calling seal_open_extern
            let before = ffi::lua_gettop(state);
            // call seal_open_extern with raw luau state
            let number_of_returns = seal_open_extern(state);

            // ensure seal_open_extern says it returned 1 value on the stack
            if number_of_returns != 1 {
                error_message = Some(format!("'seal_open_extern' expected to say it returns 1 value, got {}", number_of_returns));
                ffi::lua_pushnil(state);
                return;
            }
            
            // ensure seal_open_extern actually returned 1 value on the stack
            let after = ffi::lua_gettop(state);
            let change = after - before;
            if change != 1 {
                error_message = Some(format!("'seal_open_extern' expected to return 1 value but it actually put {} value(s) onto the stack", change));
                ffi::lua_pushnil(state);
            }
        })?
    };

    if let Some(error_message) = error_message {
        return wrap_err!("_LOAD_EXTERN: unable to load library at '{}' due to err: {}", &path, error_message);
    }

    Ok(v)
}

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_function("load", extern_load)?
        .build_readonly()
}