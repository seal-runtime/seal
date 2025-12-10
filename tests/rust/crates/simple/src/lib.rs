mod table_helpers;

use table_helpers::TableBuilder;
use std::{ffi::c_int};
use mluau::prelude::{*};

// fn some_iterator(luau: &Lua, value: LuaValue) -> LuaResult<LuaValue> {
//     let starting_value = match value {
//         LuaValue::Number(n) => n,
//         LuaValue::Integer(i) => i as f64,
//         other => {
//             return LuaError::external(format!(""))
//         }
//     };
// }

fn hello(luau: &Lua, name: String) -> LuaResult<LuaValue> {
    format!("We say hello to dear {}", name).into_lua(luau)
}

fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_function("hello", hello)?
        .build_readonly()
}

/// # Safety
/// Function *must* return one value a table (usually of functions) exposed by this library
/// This library *must* be kept alive by *seal* (or the caller) otherwise calling functions returned
/// by this library WILL cause segfaults. Use `std::mem::ManuallyDrop` for this.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn seal_open_extern(state: *mut mluau::ffi::lua_State) -> c_int {
    // SAFETY: *seal* must pass a valid lua_State that outlives this external library
    let luau = unsafe { Lua::get_or_init_from_ptr(state) };

    match create(luau) {
        Ok(t) => {
            match unsafe { luau.push_to_stack(t) } {
                Ok(_) => 1,
                Err(err) => {
                    eprintln!("unable to push to luau stack because of err: {}", err);
                    0
                }
            }
        },
        Err(err) => {
            eprintln!("unable to create return table because of err: {}", err);
            0
        }
    }
    // unsafe {
    //     ffi::lua_createtable(state, 0, 1);

    //     ffi::lua_pushcfunction(state, say_hi);
    //     ffi::lua_setfield(state, -2, c"hi".as_ptr());

    //     ffi::lua_pushstring(state, c"simple".as_ptr());
    //     ffi::lua_setfield(state, -2, c"library".as_ptr());
    //     // table left on the stack
    // }

    // 1 // number of return values
}