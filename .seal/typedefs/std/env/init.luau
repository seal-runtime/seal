--[=[
A stdlib to interact with the script's running environment.
]=]
export type env = {
    --- a list of arguments passed to the program
    args: { string },
    --- your operating system
    os: "Windows" | "Linux" | "Android" | "MacOS" | string,
    --- your system architecture, most likely `"x86_64"`
    arch: ARCH,
    --- path to the executable running your code; either *seal* itself or a compiled standalone version of *seal*.
    executable_path: string,
    --- path to the current shell, like `/usr/bin/zsh`. Handles cross-platform edge cases with fallbacks.
    shell: string,
    --[=[ 
        Get the current working directory of the running process.

        Errors if the `cwd` doesn't exist or otherwise isn't accessible (permission denied).
    ]=]
    cwd: () -> string,
    --[=[
        Finds 'where' an executable is in your `$PATH`, similar to the common `where` utility
        on Linux systems. Returns a list of every occurence of `name` in your path
        in order of PATH precedence.

        On Unix-like, also checks the executable bit to make sure the paths returned are executable.

        On Windows, checks `PATH` and `PATHEXT` for you so you don't have to worry about appending `.exe`.
    ]=]
    where: (name: string) -> { string },
    -- below are deprecated; set SEAL_ALLOW_DEPRECATED=TRUE to disable warnings at runtime
    -- --[=[
    -- Gets an environment variable in the current process.
    -- ]=]
    -- getvar: (key: string) -> string?,
    -- --[=[
    -- Sets an environment variable in the current process.

    -- Note, this function is **unsafe** in multithreaded contexts on Linux.
    -- ]=]
    -- setvar: (key: string, value: string) -> string,
    -- --[=[
    -- Removes an environment variable in the current process.

    -- Note, this function is **unsafe** in multithreaded contexts on Linux.
    -- ]=]
    -- removevar: (key: string) -> nil,
    vars: typeof(require("@self/vars"))
}

--- Your CPU processors' architecture; probably `"x86_64"`.
export type ARCH = 
    | "x86"
    | "x86_64"
    | "arm"
    | "aarch64"
    | "m68k"
    | "mips"
    | "mips32r6"
    | "mips64"
    | "mips64r6"
    | "csky"
    | "powerpc"
    | "powerpc64"
    | "riscv32"
    | "riscv64"
    | "s390x"
    | "sparc"
    | "sparc64"
    | "hexagon"
    | "loongarch32"
    | "loongarch64"

return {} :: env