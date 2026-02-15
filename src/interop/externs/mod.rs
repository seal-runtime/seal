use std::path::Path;

use mluau::prelude::*;
use mluau::ffi::{self, lua_State};
use crate::prelude::*;

use libloading::Library;
use std::mem::ManuallyDrop;

pub mod ffi_api;

type SealOpenExtern = unsafe extern "C-unwind" fn(*mut lua_State, *const ffi_api::LuauApi) -> i32;

/// Calls the symbol `seal_open_extern` in the dynamic library provided by the caller
/// with a mutable pointer to the Luau state.
/// 
/// # Safety
/// The caller in Luau is responsible for upholding that the loaded library
/// - is valid for the caller's platform
/// - uses the Luau stack correctly
/// - does NOT deallocate any memory owned by Luau
/// - upholds memory safety requirements
/// - uses sealbindings to interact with the lua_State, and does not separately bind to Luau
pub fn extern_load(luau: &Lua, path: String) -> LuaValueResult {
    let function_name = "<unsafe> extern.load(path: string)";

    let library_path = Path::new(&path);

    // SAFETY: ensure we do NOT drop the loaded library, otherwise calling functions
    // returned by the library WILL segfault/ub
    let library = unsafe { 
        let library = match Library::new(library_path) {
            Ok(lib) => lib,
            Err(err) => {
                return wrap_err!("{}: unable to open library at '{}' due to err {}", function_name, &path, err);
            }
        };
        ManuallyDrop::new(library)
    };

    // resolve symbol to seal_open_extern in dynamic library
    let seal_open_extern: libloading::Symbol<SealOpenExtern> = unsafe {
        match library.get(b"seal_open_extern") {
            Ok(symbol) => symbol,
            Err(err) => {
                return wrap_err!("{}: can't find symbol 'seal_open_extern' in library at '{}' due to reason {}", function_name, &path, err);
            }
        }
    };

    let mut error_message: Option<String> = None;
    // SAFETY:
    // should always push 1 value to Luau stack
    // Caller in Luau is responsible for safety of loaded external library/plugin.
    let v: LuaValue = unsafe {
        luau.exec_raw::<LuaValue>((), |state| {
            // number elements on the luau stack before calling seal_open_extern
            let before = ffi::lua_gettop(state);
            // call seal_open_extern with raw luau state and ffi api
            let api = ffi_api::get();
            let number_of_returns = seal_open_extern(state, api);

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
        return wrap_err!("{}: unable to load library at '{}' due to err: {}", function_name, &path, error_message);
    }

    Ok(v)
}

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_function("load", extern_load)?
        .build_readonly()
}