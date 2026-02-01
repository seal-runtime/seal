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

## The symbol `seal_open_extern`

Your plugin should have a single point of entry, an exported function named `seal_open_extern` that's visible
and not mangled.

It should have the signature `fn seal_open_extern(state: *mut Luau::lua_State) -> i32`:

- The first (and currently, only) argument is a mutable pointer to a Luau `lua_State`.

- It should return an integer `c_int` representing the number of returns (on the Luau stack)
the function should return. In our case, we always return 1 value, so it should always be 1.

In Rust:

```rs
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn seal_open_extern(state: *mut mluau::ffi::lua_State) -> c_int
```

You don't have to use `mluau` if you don't want to, anything that binds to Luau and exposes the
Luau C Stack API should be fine.

## Stack usage

Unlike with *seal* itself, you need to use the Luau C Stack API directly (or abstractions around the Luau C Stack)
when writing a *seal* plugin, even if you're writing it in Rust.

This is due to the fact that Rust's ABI is not stable: the mluau compiled within *seal*
may not have the same memory layout as the mluau compiled within your plugin. So we can't rely
on that if we want other people to be able to use your plugin without segfaulting or causing UB.

Unfortunately, the C Stack API is not well documented, so you might have to look at
the old Lua C Stack API documentation or Luau source code.

For example, `luaL_typename` is a Luau-only API that returns the `__type` field of a userdata,
or `"no value"` if `__type` is unset, and differs from `lua_typename` which is more like `type`.

## Making a library

You most likely want to create a table, put a few functions inside it, and return it as the library.

To do this, you need to call `lua_createtable`, `lua_pushcfunction/closure`, `lua_setfield`, etc.

All C functions should take in a `*mut lua_State` as an argument and return a `c_int` representing the
number of values the function returns on the Luau stack. You should ensure that C functions are
exported/public, but their names don't need to be no-mangled if you're passing them via pointer.

## Error messages and handling *seal* userdata (extern types)

If you want to be consistent with *seal*, you want to return or throw nominally-typed `error` userdatas (extern types)
instead of `luaL_error` (runtime errors). You can retrieve `@std/err`'s `err.wrap` and `ecall` from the C Stack API.
I wrote Rust examples of doing so in the extern-example library I linked above. This is also probably the best
way to create extern types like `Duration` defined in *seal*.

## Safety

Please keep in mind you're responsible for maintaining memory and thread safety in your external libraries.
This includes, but is not limited to using the Luau stack correctly, not freeing memory owned by Luau, etc.

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
- correctly links to Luau (not Lua) for using the Luau stack,
- should not throw an uncaught foreign exception or Rust panic.
