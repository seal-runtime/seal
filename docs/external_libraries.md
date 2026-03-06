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

## Linking

To facilitate linking to the *exact* same Luau within *seal*, *seal* passes
its entire Luau C-Stack API to your external library as the second argument to `seal_open_extern`.

Ensure you use [sealbindings](https://github.com/seal-runtime/sealbindings) or
write an equivalent in your preferred language that understands the exact structure
and function ordering of the struct passed as `*const LuauApi`.

Keep in mind that the Luau C-Stack API will change in later versions as more APIs are added.
I will try to keep sealbindings backwards-compatible, but it's not a guarantee
that any version of *seal* will work with any build of *sealbindings* or vice versa.

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
pub unsafe extern "C-unwind" fn seal_open_extern(state: *mut sealbindings::ffi::lua_State, ptr: *const sealbindings::LuauApi) -> c_int
```

## Stack usage

Unlike with *seal* itself, you need to use the Luau C-Stack API directly (or abstractions around the Luau C-Stack)
when writing a *seal* plugin, even if you're writing it in Rust.

Unfortunately, the C-Stack API is not well documented, so you might have to look at
the old Lua C-Stack API documentation or Luau source code. The C-Stack APIs can be confusingly named; for example, `luaL_typename` is a Luau-only API that backs the Luau global function `typeof`, and returns the `__type` field of a host-defined userdata if it is set, or `"no value"` if the stack index is out-of-bounds, and differs from the similarly-named `lua_typename` which backs the Luau global function `type`.

Since the C-Stack API doesn't directly protect you like mluau does, you'll likely
run into hard crashes causing coredumps while you develop and test your plugins.
If you're on Linux, I recommend (at least) using `gdb` backtrace with `bt` on the resulting coredumps.

Keep in mind if you run into `illegal hardware instruction` crashes, you're probably just using the Luau stack incorrectly!
Check your stack indices, and don't forget to `checkstack` and `gettop`.

## Making a library

You most likely want to create a table, put a few functions inside it, and return it as the library.

To do this, you need to call

- `sealbindings::initialize(ptr)`
- `ffi::lua_createtable(state, 0, 1)`
- `sealbindings::push_wrapped_c_function(state, yourfunction)` or `ffi::lua_pushcfunction(state, yourfunction)`
- `ffi::lua_setfield(state, -2, c"your function's name".as_ptr())`
, etc., and leave one value on the stack.

All C functions should take in a `*mut lua_State` as an argument and return a `c_int` representing the
number of values the function returns on the Luau stack. You should ensure that C functions are
exported/public, but their names don't need to be no-mangled if you're passing them via pointer.

## Error messages and handling *seal* userdata (extern types)

If you want to be consistent with *seal*, you want to return or throw nominally-typed `error` userdatas (extern types)
instead of `luaL_error` (runtime errors). These come from *seal*'s `@std/err` library, which can be accessed from plugins. To facilitate this, I've added helper functions in `sealbindings` to wrap *seal*'s errors, including
`sealbindings::push_wrapped_error` and `sealbindings::push_wrapped_c_function`.

If you want to deal with userdatas defined *in* *seal*, you'll probably have to do something similar to what I do
in `extern-example` for accessing and calling methods on `Duration`.

## Safety

Please keep in mind you're responsible for maintaining memory and thread safety in your external libraries.

These requirements include but are not limited to:

- Using the Luau stack correctly, passing the correct stack indices to Luau C-Stack APIs, etc.
- Never freeing memory owned by Luau, including strings and buffers.
  - In Rust, this includes accidentally dropping memory owned by Luau. To avoid this, ensure you copy data owned by Luau into data structures owned by Rust.
- Ensuring memory passed into Luau C-Stack APIs lives long enough to be read/copied by Luau. Be careful where
  your `CString`s get dropped.
- Not accessing the Luau VM in multiple OS threads at once; Luau is not thread-safe.
- Ensure pointers from the embedder stored in Luau (userdata) are not cleaned-up before being used.

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
