
--[=[
    Load external libraries at runtime.

    External libraries (also called plugins), can be used to expand *seal*'s functionality,
    add bindings to C and Rust libraries, and allow performance-critical code execution without mluau or Luau VM overhead.
]=]
export type extern = {
    --[=[
        Loads the native dynamic library at `path`, calling the symbol `seal_open_extern` in that library with a pointer to the Luau state.

        It is expected that calling `seal_open_extern` pushes 1 value directly onto the Luau stack. This function returns that value unchanged.

        This function does not handle platform-specific behavior; it is expected the caller loads the correct dynamic library for
        supported platforms/architectures.

        # ⚠️ Safety
        
        This function is extremely unsafe.
            
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
            - takes in a pointer to seal's LuauAPi as its second argument,
            - returns `1` as an `i32 (c_int)`.

        Library/plugin maintainers are responsible for ensuring the library:

        - uses the Luau stack correctly,
        - does NOT share the passed `lua_State` between multiple OS threads,
        - does NOT free memory owned by Luau,
        - correctly exposes seal's C-Stack API via sealbindings or similar, and does not directly bind to Luau or mluau.
        - should not throw an uncaught foreign exception or Rust panic.

        # Errors
        
        Throws an error if:

        - The dynamic library cannot be found (usually with a message like "dlopen failed"),
        - The dynamic library is missing the symbol `seal_open_extern`,
        - The dynamic library could not be loaded for a different reason,
        - `seal_open_extern` does not return 1 (c_int),
        - `seal_open_extern` returns 1 but does not push exactly 1 value to the Luau stack.
    ]=]
    load: (path: string) -> unknown,
}

return {} :: extern