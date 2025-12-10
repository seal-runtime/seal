mod prelude;

use std::ffi::c_int;

use mluau::prelude::*;
use crate::prelude::*;
use std::process::exit;

fn create_webview(luau: &Lua, config: LuaTable) -> LuaResult<LuaValue> {
    let LuaValue::String(title) = config.raw_get("title")? else {
        display_error_message("missing title");
        return wrap_err!("missing title");
    };
    let LuaValue::String(content) = config.raw_get("content")? else {
        display_error_message("missing content");
        return wrap_err!("missing content");
    };

    let mut x = 600;
    let mut y = 420;
    if let LuaValue::Table(size) = config.raw_get("size")? {
        match size.raw_get("x")? {
            LuaValue::Integer(i) => {
                x = i as i32;
            },
            LuaValue::Number(f) => {
                x = f as i32;
            },
            other => {
                display_error_message(&format!("expected size.x to be number, got: {:?}", other));
                return wrap_err!("bad value for size.x");
            }
        }
        match size.raw_get("y")? {
            LuaValue::Integer(i) => {
                y = i as i32;
            },
            LuaValue::Number(f) => {
                y = f as i32;
            },
            other => {
                display_error_message(&format!("expected size.y to be number, got: {:?}", other));
                return wrap_err!("bad value for size.y");
            }
        }
    }

    let LuaValue::Function(message_handler) = config.raw_get("on_message")? else {
        display_error_message("missing handler callback 'on_message'");
        return wrap_err!("missing handler");
    };

    web_view::builder()
        .title(&title.to_string_lossy())
        .content(web_view::Content::Html(&content.to_string_lossy()))
        .size(x, y)
        .user_data(())
        .invoke_handler(|view, event| {
            if event == "close-webview" {
                view.exit();
            } else {
                let result = match message_handler.call::<LuaValue>(event) {
                    Ok(value) => value,
                    Err(_err) => {
                        eprintln!("webview: exiting... your handler threw an error: {}", luau.traceback().unwrap());
                        exit(1);
                    }
                };
                match result {
                    LuaNil => {},
                    LuaValue::Table(t) => {
                        if let LuaValue::String(content) = t.raw_get("content").unwrap() {
                            view.set_html(&content.to_string_lossy().to_string())?;
                        }
                    },
                    other => {
                        eprintln!("webview: exiting... expected you to return a table with field content: string or nil, got: {:?}", other);
                        exit(1);
                    }
                }
            }
            Ok(())
        })
        .debug(true)
        .run()
        .unwrap();

    Ok(LuaNil)
}

fn display_error_message(message: &str) {
    eprintln!("{}[ERR]{}{} webview:{} {}", colors::BOLD_RED, colors::RESET, colors::BOLD_BLUE, colors::RESET, message);
}

fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_function("create", create_webview)?
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