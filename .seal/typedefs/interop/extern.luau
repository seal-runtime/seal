
--[=[
    Load external libraries at runtime.
]=]
export type extern = {
    --[=[
    Load the dynamic library at `path`, returning what the library's `seal_open_extern` function returns.
    
    # ⚠️ Safety

    This function is extremely unsafe.
    
    - You have to ensure the loaded binary is compatible with the expected operating system.
    - The loaded library must have a symbol named `seal_open_extern` in the C-unwind ABI that returns `c_int`.
    - The loaded library must return a single value directly on the Luau stack.
    ]=]
    load: (path: string) -> unknown,
}

return {} :: extern