--[=[
    Library for interacting with the environment variables of the current process, including handling `.env` files.
    Refer to [dotenvy](https://github.com/allan2/dotenvy) for the specific syntax allowed.

    *seal* automatically loads environment variables from the `.env` file in your cwd (or up).

    Environment variables change this library's behavior:
    - `SEAL_LOAD_DOTENV=FALSE` disables loading environment variables from `.env`
    - `SEAL_LOAD_DOTENV=TRUE` causes *seal* to error out if it can't find a `.env` file near your cwd.
    - if `SEAL_LOAD_DOTENV` is unset or missing, *seal* will load a `.env` file if one is present.
]=]
export type vars = {
    --[=[
        Get an environment variable from the current process's running environment, your `.env` file, or additional
        environment files loaded via `env.vars.load`.

        - If the environment variable is not present, returns `nil`.
        - If the environment variable is not valid utf-8, returns the invalid utf-8 string (without coercion)

        ## Usage
        
        ```luau
        local vars = require("@std/env/vars")
        local CONFIG_PATH = vars.get("APP_CONFIG_PATH") or DEFAULT_PATH
        local API_KEY = vars.get("API_KEY") or error("missing API_KEY")
        ```
    ]=]
    get: (key: string) -> string?,
    --[=[
        Handles boolean-style environment flags in a case-insensitive manner.

        - any variations of `"TRUE" | "YES" | "ON" | "1" | "Y" | "T"`  => `true`
        - any variations of `"FALSE" | "NO" | "OFF" | "0" | "N" | "F"` => `false`
        - if the environment variable is not set, returns `default`.

        ## Errors

        - throws an error if the environment variable contains an unexpected string,
        is not valid Unicode, or cannot be interpreted as a boolean flag.

        ## Usage

        ```luau
        local PROD = vars.flag("PROD", false)
        ```
    ]=]
    flag: (name: string, default: boolean) -> boolean,
    --[=[
        Validate or transform an environment variable into another type `T`.

        ## Usage
        
        ```luau
        local PORT = env.vars.validate("PORT", function(s)
            if s == nil then
                error("missing PORT env variable")
            end

            local port = tonumber(s)
            if port == nil then
                error(`PORT can't be converted to number, got: {s}`)
            elseif port < 0 then
                error(`PORT cannot be negative, got {port}`)
            end

            return port
        end)
        ```
    ]=]
    validate: <T>(key: string, f: (value: string?) -> T) -> T,
    --[=[
        Set an environment variable for the currently-running program.

        These variables are not persistent and will disappear after the program terminates; use a shell configuration file for persistent variables.

        # Safety

        - This function should only be called before initializing any threads (`thread.spawn` or `process.spawn`), 
        otherwise it may cause undefined behavior when used in multithreaded programs on Unix-like operating systems.
    ]=]
    set: (key: string, value: string) -> (),
    --[=[
        Remove an environment variable for the currently-running program.

        # Safety

        - This function should only be called before initializing any threads (`thread.spawn` or `process.spawn`), 
        otherwise it may cause undefined behavior when used in multithreaded programs on Unix-like operating systems.
    ]=]
    unset: (key: string) -> (),
    --[=[
        Returns a map-like table that represents the snapshot of the process's environment variables
        at the time this function was called.

        If you want to modify these variables, use `vars.set`; modifying this table will cause an error.
    ]=]
    all: () -> { [string]: string },
    --[=[
        Load additional environment variables from the environment file at `path` into the running process's environment.
        See [dotenvy](https://github.com/allan2/dotenvy) for the specific syntax allowed.
        
        Note that any `.env` file in your cwd (or up) will already be loaded by *seal*, so you don't need to load it explicitly.
        To disable this default behavior, set `SEAL_LOAD_DOTENV=FALSE`.

        Set `override` to replace any existing environment variables of the same name; defaults to `false`.

        # Safety

        - You should only call this function before you spawn any threads via `@std/thread` or `process.spawn`. 
        Calling this function in multithreaded contexts on Unix-like operating systems may cause undefined behavior.
    ]=]
    load: (path: string, override: boolean?) -> (),
}

return {} :: vars