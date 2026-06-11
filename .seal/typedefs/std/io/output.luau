--[=[
    Write to the terminal's stdout/stderr.
]=]
export type output = {
    --[=[
        Writes `contents` to stdout without any intermediate utf-8 validation.

        The stream is flushed immediately after each write. Manual control over when flushing occurs is not available.
        If you need explicit control over flushing or want to batch multiple writes before flushing, use `@std/terminal` with `terminal.write()` and `terminal.execute()` instead.

        ## Windows Portability Concerns

        On Windows, this function may return an error if `contents` is invalid utf-8.

        Additionally, on Windows it may fail silently when used in a child process.

        ## Returns

        - An `error` instance if writing to the stream failed or flushing failed.

        ## Usage

        ```luau
        local err = output.write("idk")
        if err then
            warn(`error writing to stdout: {err}`)
        end
        ```
    ]=]
    write: (contents: string | buffer) -> error?,
    --[=[
        Writes `contents` to stdout without any intermediate utf-8 validation, adding a trailing newline.

        The stream is flushed immediately after writing.

        ## Windows Portability Concerns

        On Windows, this function may return an error if `contents` is invalid utf-8.

        Additionally, on Windows it may fail silently when used in a child process.

        ## Returns

        - An `error` instance if writing to the stream failed or flushing failed.
    ]=]
    writeln: (contents: string | buffer) -> error?,
    --[=[
        Writes `contents` to stderr without any intermediate utf-8 validation.

        The stream is flushed immediately after each write. Manual control over when flushing occurs is not available.
        If you need explicit control over flushing or want to batch multiple writes before flushing, consider buffering your output or use a file handle with `@std/fs` instead.

        ## Windows Portability Concerns

        On Windows, this function may return an error if `contents` is invalid utf-8.

        Additionally, on Windows it may fail silently when used in a child process.

        ## Returns

        - An `error` instance if writing to the stream failed or flushing failed.

        ## Usage

        ```luau
        local err = output.ewrite("error message\n")
        ```
    ]=]
    ewrite: (contents: string | buffer) -> error?,
    --[=[
        Writes `contents` to stderr without any intermediate utf-8 validation, adding a trailing newline.

        The stream is flushed immediately after writing.

        ## Windows Portability Concerns

        On Windows, this function may return an error if `contents` is invalid utf-8.

        Additionally, on Windows it may fail silently when used in a child process.

        ## Returns

        - An `error` instance if writing to the stream failed or flushing failed.
    ]=]
    ewriteln: (contents: string | buffer) -> error?,
    --- clears stdout akin to `cls` or `clear`.
    clear: () -> (),
}
return {} :: output