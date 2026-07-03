--- input handling lib
local input = {}
export type input = typeof(input)

--- gets input with an optional `raw_prompt` to display before getting said input.
@[deprecated { use = "io.prompt.text OR io.input.readline OR io.input.rawline" }]
function input.get(raw_prompt: string): string
    return nil :: any
end

--> io.input.rawline(prompt: string?): string
--[=[
    Gets a line directly from stdout in a way that doesn't properly handle editing text (going back and forward with arrow keys), etc.

    But works with stdin in a child process/works while piped, making it a fallback for automated solutions or cursed ancient terminals.
]=]
function input.rawline(prompt: string?): string
    return nil :: any
end

--> io.input.readline(prompt: string): string | interrupt | error
--[=[
    Prompts the user for one line of text with proper text buffer handling and error handling.

    For a higher-level prompting API, use `@std/io/prompt` instead.

    Falls back to `io.input.rawline` if called in a non-TTY (like a piped child process)

    ## Returns

    - A `string` once the user types something in and presses <kbd>Enter</kbd>.
    - An `interrupt` userdata, which contains a `code` (`"CtrlC"` or `"CtrlD"`) if the user presses either.
    - An `error` userdata, if some other IO error occurs (like a broken pipe)

    ## Errors

    - *Throws* an error if some weird low level Errno code occurs.

    ## Usage

    ```luau
    local line: string?
    local result = input.readline("what's your name?: ")
    if typeof(result) == "string" then
        line = result
    elseif typeof(result) == "interrupt" then
        if result.code == "CtrlC" then
            process.exit(0)
        elseif result.code == "CtrlD" then
            line = ""
        end
    elseif typeof(result) == "error" then
        print(`got error {result}`)
    end
    ```
]=]
function input.readline(prompt: string): string | interrupt | error
    return nil :: any
end

--[=[
    Prompts the user to edit one line of text, where the cursor is between the `left` and `right` strings.

    If `right` is not provided, the user will start editing from the back of the message.

    If a TTY is not detected, outputs `left .. "<CURSOR>" .. right`; the user/program will have
    to respond with the full, edited message, not just the text to insert.

    This function shares most semantics with `input.readline`, including the following:

    ## Returns

    - A `string` once the user types something in and presses <kbd>Enter</kbd>.
    - An `interrupt` userdata, which contains a `code` (`"CtrlC"` or `"CtrlD"`) if the user presses either.
    - An `error` userdata, if some other IO error occurs (like a broken pipe)

    ## Errors

    - *Throws* an error if some weird low level Errno code occurs or writing output to stdout fails.
    
]=]
function input.editline(prompt: string, left: string, right: string?): string | interrupt | error
    return nil :: any
end

export type InputReadOptions = {
    --- The maximum number of bytes to read before returning; reading stops once this many bytes
    --- have been read, even if the stream is still open. Accepts a plain `number` or a `FileSize`.
    bytes: (number | FileSize)?,
    --- A `Duration` (from `@std/time`) to wait for input before returning; reading stops once the
    --- timeout elapses, even if the stream is still open.
    timeout: Duration?,
}

--[=[
    Reads from stdin, returning the bytes read and whether there may be more left to read.

    With no `options`, reads all of stdin until EOF and blocks until the stream closes — either a
    pipe closes or the user presses <kbd>Ctrl+D</kbd> in a TTY.

    Useful for consuming piped input in scripts, e.g. `echo "hello" | seal script.luau`.

    ## Parameters

    - `options.bytes`: the maximum number of bytes to read before returning. Accepts a plain
      `number` or a `FileSize` (from `@std/fs/filesize`). Reading stops once this many bytes have
      been read, even if the stream is still open.
    - `options.timeout`: a `Duration` (from `@std/time`) to wait for input before returning.
      Reading stops once the timeout elapses, even if the stream is still open.

    ## Returns

    Returns two values:

    - A `string` containing the bytes read from stdin, or `nil` if nothing was read.
    - A `boolean` that is `true` if reading stopped before EOF (i.e. because the `bytes` limit was
      reached or the `timeout` elapsed and there may be more to read), and `false` if the stream
      reached EOF.

    ## Errors

    - *Throws* an error if reading from stdin fails (e.g. a broken pipe or IO error).

    ## Usage

    ```luau
    -- read everything until EOF
    local contents = input.read()
    print(`got {#(contents or "")} bytes from stdin`)

    -- read at most 1 KB, or give up after 5 seconds
    local chunk, more = input.read {
        bytes = filesize.kilobytes(1),
        timeout = time.seconds(5),
    }
    if more then
        print("there's still more to read!")
    end
    ```
]=]
function input.read(options: InputReadOptions?): (string?, boolean)
    return nil :: any
end

--[=[
    Returns an `interrupt` userdata object. For reasons. Maybe control flow.
]=]
function input.interrupt(key: "CtrlC" | "CtrlD"): interrupt
    return nil :: any
end

return input :: input