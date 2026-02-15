# External Libraries/Plugins

If you'd like to extend *seal*'s functionality, bind to native libraries, or write highly
performant code that bypasses mluau and the Luau VM's overhead, you can write an external
library (a *seal* plugin) and load native code directly from *seal*.

This is *seal*'s alternative to FFI. If you feel a binding is useful enough,
general enough, cross-platform enough, and doesn't drastically increase *seal*'s size,
please make a PR to *seal* itself instead of implementing the feature as a plugin.

For a (very basic) example of writing an external library, see [extern-example](/tests/rust/crates/example/).

To load a *seal* plugin within Luau, use `@interop/extern`'s `extern.load` function.
Keep in mind that if you want your library to support multiple platforms,
you need to ship binaries for each platform and use conditional checks to ensure you
`extern.load` the correct shared library for that platform.

The binary should be compiled as a `cdylib` (`.so`, `.dll`, etc.).

To facilitate linking to the *correct* version of `mluau::ffi`, *seal* exports the entire
Luau C-stack API. Ensure you use [sealbindings](https://github.com/seal-runtime/sealbindings)
or an equivalent in your preferred language that binds directly to the symbols exported in *seal* and not mluau or Luau.

## The symbol `seal_open_extern`

Your plugin should have a single point of entry, an exported function named `seal_open_extern` that's visible
and not mangled.

It should have the signature `fn seal_open_extern(state: *mut Luau::lua_State, ptr: *const LuauApi) -> i32`:

- The first argument is a mutable pointer to a Luau `lua_State`.
- The second argument is a const pointer to seal's C-Stack Luau API, which you use to initialize `sealbindings::initialize`.

- It should return an integer `c_int` representing the number of returns (on the Luau stack)
the function should return. In our case, we always return 1 value, so it should always be 1.

- You should call `sealbindings::initialize()` with the provided `ptr` before using any functions in `sealbindings::ffi::*`.

In Rust:

```rs
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn seal_open_extern(state: *mut sealbindings::ffi::lua_State, ptr: *const sealbindings::ffi::api::LuauApi) -> c_int
```

## Stack usage

Unlike with *seal* itself, you need to use the Luau C Stack API directly (or abstractions around the Luau C Stack)
when writing a *seal* plugin, even if you're writing it in Rust.

Unfortunately, the C Stack API is not well documented, so you might have to look at the old Lua C Stack API documentation or Luau source code. For example, `luaL_typename` is a Luau-only API that is the backend for the `typeof` function, and returns the `__type` field of a host-defined userdata if it is set, or `"no value"` if the stack index is out-of-bounds, and differs from `lua_typename` which is the backend for `type`.

## Making a library

You most likely want to create a table, put a few functions inside it, and return it as the library.

To do this, you need to call

- `sealbindings::initialize(ptr)`
- `ffi::lua_createtable(state, 0, 1)`
- `ffi::lua_pushcfunction(state, yourfunction)`
- `ffi::lua_setfield(state, -2, c"your function's name".as_ptr())`
, etc., and leave one value on the stack.

All C functions should take in a `*mut lua_State` as an argument and return a `c_int` representing the
number of values the function returns on the Luau stack. You should ensure that C functions are
exported/public, but their names don't need to be no-mangled if you're passing them via pointer.

## Error messages and handling *seal* userdata (extern types)

If you want to be consistent with *seal*, you want to return or throw nominally-typed `error` userdatas (extern types)
instead of `luaL_error` (runtime errors). These come from *seal*'s `@std/err` library, which can be accessed from plugins. To facilitate this, I've added helper functions in `sealbindings` to wrap *seal*'s errors, including `sealbindings::wrap_c_function` and `sealbindings::push_wrapped_error`.

If you want to deal with userdatas defined *in* *seal*, you'll probably have to do something similar to what I do
in `extern-example` for accessing and calling methods on `Duration`.

## Safety

Please keep in mind you're responsible for maintaining memory and thread safety in your external libraries.
This includes, but is not limited to using the Luau stack correctly, not freeing memory owned by Luau, etc.

Keep in mind if you run into `illegal hardware instruction` crashes, you're probably just using the Luau stack incorrectly! Don't forget to `checkstack` and `gettop`.

The Luau VM is not thread safe so you should not attempt to use the VM in multiple OS threads.

To copy and paste the Safety docs from `extern.load`:

⚠️ Safety

This function (`extern.load`) is extremely unsafe.

External libraries can easily bypass Rust's and mluau's safety guarantees, including but not limited to:

- causing memory safety vulnerabilities,
- incorrectly using the Luau stack (causing hard crashes or UB),
- modifying the Luau state in cursed ways,
- execute arbitrary code.

The caller is responsible for ensuring the library:

- is compatible with the caller's operating system, platform, and architecture,
- is a *seal* extern library, not any other shared/dynamic library,
- contains a *not mangled* function symbol `seal_open_extern` that:
  - is in the C ABI (use `"C-unwind"` in Rust),
  - takes in a Luau `lua_State` pointer as its first argument,
  - returns `1` as an `i32 (c_int)`.

Library/plugin maintainers are responsible for ensuring the library:

- uses the Luau stack correctly,
- does NOT share the passed `lua_State` between multiple OS threads,
- does NOT free memory owned by Luau,
- correctly links to *seal* via sealbindings or similar, and does not directly bind to Luau or mluau.
- should not throw an uncaught foreign exception or Rust panic.
