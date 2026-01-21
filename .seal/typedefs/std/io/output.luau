--[=[
    Write to the terminal's stdout/stderr.
]=]
export type output = {
    --[=[
        Writes `contents` to stdout without any intermediate utf-8 validation, flushing the stream immediately.

        ## Windows Portability Concerns

        On Windows, this function may return an error if `contents` is invalid utf-8.

        Additionally, on Windows it may fail silently when used in a child process.
        
        ## Returns

        - An `error` instance if either writing to the stream failed or flushing the stream failed.

        ## Usage

        ```luau
        local err = output.write("idk")
        if err then
            warn(`error writing to stdout: {err}`)
        end
        ```
    ]=]
    write: (contents: string | buffer, flush: boolean?) -> error?,
    --[=[
        Writes `contents` to stderr without any intermediate utf-8 validation, flushing the stream immediately.

        ## Windows Portability Concerns

        On Windows, this function may return an error if `contents` is invalid utf-8.

        Additionally, on Windows it may fail silently when used in a child process.

        ## Returns

        - An `error` instance if either writing to the stream failed or flushing the stream failed.

        ## Usage

        ```luau
        local err = output.ewrite("error message\n")
        ```
    ]=]
    ewrite: (contents: string | buffer, flush: boolean?) -> error?,
    --- clears stdout akin to `cls` or `clear`.
    clear: () -> (),
    --- Returns the size of the terminal as a vector of (column, rows, 0). Errors if terminal unavailable.
    size: () -> vector,
    --- Switch to crossterm's "Alternate" or "Main" terminal screen of stdout.
    switch: (screen: "Alternate" | "Main") -> (),
    resize: (cols: number, rows: number) -> (),
    --- returns a vector of (column, rows, 0); might block if you're polling on unix
    cursor: () -> vector,
}
return {} :: output