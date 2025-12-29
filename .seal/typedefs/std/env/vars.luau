--[=[
    Library for interacting with the environment variables of the current process.
]=]
export type vars = {
    --[=[
        Get an environment variable from the current process's running environment.

        If the environment variable is not present, returns `nil`.

        If the environment variable is not valid utf-8, returns the invalid utf-8 string (without coercion)
    ]=]
    get: (key: string) -> string?,
    --[=[
        Set an environment variable for the currently-running program.

        These variables are not persistent and will disappear after the program terminates; use a shell configuration file for persistent variables.

        # Safety
        
        - This function may cause undefined behavior when used in multithreaded programs (`@std/thread`) on Unix-like operating systems.
    ]=]
    set: (key: string, value: string) -> (),
    --[=[
        Remove an environment variable for the currently-running program.

        # Safety

        - This function may cause undefined behavior when used in multithreaded programs (`@std/thread`) on Unix-like operating systems.
    ]=]
    unset: (key: string) -> (),
    --[=[
        Returns a map-like table that represents the snapshot of the process's environment variables
        at the time this function was called.

        If you want to modify these variables, use `vars.set`; modifying this table will cause an error.
    ]=]
    all: () -> { [string]: string },
}

return {} :: vars