const terminal = require("@std/terminal")
const env = require("@std/env")
const fs = require("@std/fs")
local success, cwd = pcall(function()
    return env.cwd() -- pcalling here to prevent downstream panic in err.rs if cwd doesn't exist
end)

if not success or typeof(cwd) == "error" or typeof(cwd) == "err" then
    warn("Your current working directory might've been removed! Try reloading your editor.")
    cwd = "nonexisting/removed directory"
end

--- The line an error was raised on, and the line before it when there is one.
const function get_line_context(path: string, context_line_number: number): (string?, string?)
    local previous: string? = nil
    for line_number, line in fs.readlines(path) do
        if line_number == context_line_number then
            return previous, line
        end
        previous = line
    end
    return nil, nil
end

--- Where an error was raised.
type Location = {
    path: string,
    line_number: number,
}

--- Reads the `["path"]:12:` that starts a traceback frame or a position in an error message.
const function read_location(text: string): Location?
    const path, line_number = string.match(text, '%["(.-)"%]:(%d+):')
    if not path or not line_number then
        return nil
    end
    const line_number_n = tonumber(line_number)
    if not line_number_n then
        return nil
    end
    return {
        path = path,
        line_number = line_number_n,
    }
end

--- Splits text with traceback after a "stack traceback:" line, like `error("message")` and Rust errors.
const function split_traceback(text: string): (string, string?)
    const start = string.find(text, "\nstack traceback:", 1, true)
    if not start then
        return text, nil
    end
    return string.sub(text, 1, start - 1), string.sub(text, start + 1)
end

-- derived err location from traceback frames
type Frames = {
    --- The frames worth showing, innermost first.
    shown: { string },
    --- The innermost frame running Luwu/Luau code.
    location: Location?,
    --- The function named by the innermost frame when it's a C function, like `error` or a seal API.
    raised_by: string?,
    might_be_rust_error: boolean,
}

const function read_frames(traceback: string?): Frames
    const str = require("@std/str")
    const frames: Frames = {
        shown = {},
        location = nil,
        raised_by = nil,
        might_be_rust_error = false,
    }
    for _, line in str.splitlines(traceback or "") do
        const frame = string.gsub(str.trim(line), ": in function <%[.+", ": in top level")
        const worth_showing = frame ~= "" and frame ~= "stack traceback:" and frame ~= "[C]: in ?"
        if not worth_showing then
            continue
        end

        if #frames.shown == 0 then
            frames.raised_by = string.match(frame, "^%[C%]: in function '(.-)'$")
        end
        if string.find(frame, "%.rs") then
            frames.might_be_rust_error = true
        end
        frames.location = frames.location or read_location(frame)
        table.insert(frames.shown, frame)
    end
    return frames
end


const RAISING_GLOBALS: { [string]: boolean } = {
    ["error"] = true,
    ["assert"] = true,
    ["unreachable"] = true,
    ["todo"] = true,
    -- `pcall(error, message)` raises with `pcall` as the caller
    ["pcall"] = true,
    ["xpcall"] = true,
}

const function should_be_pretty(): boolean
    local pretty_errors = env.vars.get("SEAL_PRETTY_ERRORS")
    if pretty_errors then
        pretty_errors = string.lower(pretty_errors)
    end

    local seal_colors = env.vars.get("SEAL_COLORS")
    if seal_colors then
        seal_colors = string.lower(seal_colors)
    end

    const no_color = env.vars.get("NO_COLOR")

    -- SEAL_COLORS=true or SEAL_PRETTY_ERRORS=true overrides NO_COLOR
    if 
        seal_colors and seal_colors == "true"
        or (pretty_errors and pretty_errors == "true")
    then
        return true
    end

    if
        -- NO_COLOR standard says to avoid coloring if variable is present at all
        no_color ~= nil
        or (seal_colors and seal_colors == "false")
        or (pretty_errors and pretty_errors == "false")
    then
        return false
    end

    return true
end

const function parse_traceback(message: string, traceback: string?): string
    const is_tty = terminal.tty()

    const pretty = should_be_pretty()
    if not pretty then
        const format = require("@std/io/format")
        message = format.uncolor(message)
        if traceback then
            traceback = format.uncolor(traceback)
        end
    end

    const str = require("@std/str")
    const colors = require("@std/io/colors")

    const escaped_cwd = string.gsub(cwd :: string, "%p", "%%%0")
    const function tidy(text: string): string
        const without_string = string.gsub(text, "%[string ", "[")
        return (string.gsub(without_string, escaped_cwd, "."))
    end

    -- `error("message")` and Rust errors
    const own_message, own_traceback = split_traceback(tidy(message))
    message = string.gsub(own_message, "^runtime error: ", "")
    traceback = own_traceback or (if traceback then tidy(traceback) else nil)

    const read = read_frames(traceback)
    const frames = read.shown
    -- syntax and compile errors (without a Lua frame) can still contain positions
    const location = read.location or read_location(message)

    const raised_by_seal = read.raised_by ~= nil and not RAISING_GLOBALS[read.raised_by]
    const lines: { string } = {}

    if is_tty and pretty then
        -- syntax highlighting stuff
        const tokenmap: { [string]: (string) -> string } = {
            ["local"] = colors.red,
            ["function"] = colors.red,
            ["if"] = colors.red,
            ["then"] = colors.red,
            ["else"] = colors.red,
            ["elseif"] = colors.red,
            ["end"] = colors.red,
            ["while"] = colors.red,
            ["repeat"] = colors.red,
            ["return"] = colors.red,
            ["until"] = colors.red,
            ["nil"] = colors.bold.red,
            ["for"] = colors.red,
            ["in"] = colors.red,
            ["do"] = colors.red,
            ["or"] = colors.yellow,
            ["and"] = colors.yellow,
            ["export"] = colors.magenta,
            ["type"] = colors.magenta,
            ["="] = colors.style.dim,
            ["."] = colors.style.dim,
            [":"] = colors.style.dim,
            ["{"] = colors.style.dim,
            ["}"] = colors.style.dim,
            ["["] = colors.style.dim,
            ["]"] = colors.style.dim,
            ["true"] = colors.bold.yellow,
            ["false"] = colors.bold.yellow,
            ["@native"] = colors.bold.red,
            ["@deprecated"] = colors.bold.red,
        }

        const function get_string_delim(token: string): string?
            return 
                if token == "'" then
                    "'"
                elseif token == '"' then
                    '"'
                elseif token == "`" then
                    '`'
                elseif token == "[[" then
                    "[["
                elseif token == "]]" then
                    "[[" -- on purpose
                elseif token == "[=[" then
                    "[=["
                elseif token == "]=]" then
                    "[=[" -- on purpose
                else
                    nil
        end

        const function syntax(line: string): string
            const splits = str.splitaround(line, " ", "--", ":", ".", "+", "-", "/", "(", ")", '"', "'", "`", "[[", "]]", ",", "[=[", "]=]", "[", "]")
            const result: { string } = {}
            local last = ""

            local in_string = false
            local string_delim: string? = nil

            local in_comment = false

            for _, token in splits do
                const colorizer = tokenmap[token]
                local colorized = 
                    if colorizer then 
                        colorizer(token)
                    elseif tonumber(token) then
                        colors.blue(token)
                    elseif in_string then
                        colors.green(token)
                    else 
                        token

                const new_string_delim = get_string_delim(token)
                if in_string and new_string_delim and new_string_delim == string_delim then
                    in_string = false
                    string_delim = nil
                elseif not in_string and new_string_delim then
                    in_string = true
                    string_delim = new_string_delim
                    colorized = colors.green(token)
                end

                if token == "(" and last ~= " " then -- it's a fncall
                    const popped = table.remove(result, #result) :: string
                    table.insert(result, colors.blue(popped))
                end

                if token == "--" then
                    in_comment = true
                end

                if in_comment then
                    colorized = colors.style.dim(token)
                end

                table.insert(result, colorized)
                last = token
            end
            
            return table.concat(result)
        end
        -- syntax highlighting stuff end

        --- The raising line under a `path:line` header, with the line before it dimmed and carets under it.
        const function show_location(at: Location): { string }
            const previous, line = get_line_context(at.path, at.line_number)
            if not line then
                return {}
            end

            const number = tostring(at.line_number)
            const trimmed = str.trim(line)
            const shown = { colors.style.underline(`{at.path}:{number}`) }
            if previous then
                const previous_number = tostring(at.line_number - 1)
                const padding = string.rep(" ", #number - #previous_number)
                table.insert(shown, ` {padding}{colors.style.dim(previous_number)} | {colors.style.dim(str.trim(previous))}`)
            end
            table.insert(shown, ` {colors.blue(number)} | {syntax(trimmed)}`)
            table.insert(shown, ` {string.rep(".", #number)} | {colors.red(string.rep("^", #trimmed))}`)
            return shown
        end

        -- the message doesn't need a `["path"]:line:` prefix if there's a location below that would repeat it
        local shown_message = if location then (string.gsub(message, '%[".-"%]:%d+:%s*', "", 1)) else message
        -- some seal stdlib error messages are pretty long, let's try giving them newlines
        if raised_by_seal and #shown_message > 100 then
            shown_message = string.gsub(shown_message, "%):", "):\n  ")
        end
        table.insert(lines, colors.red(shown_message))

        if location then
            const shown_ok, shown = pcall(show_location, location)
            if shown_ok and typeof(shown) ~= "error" then
                table.move(shown, 1, #shown, #lines + 1, lines)
            end
        end

        if #frames > 0 then
            table.insert(lines, "stack traceback:")
        end
        for index, frame in frames do
            const name = string.match(frame, "^%[C%]: in function '(.-)'$")
            const raising = name ~= nil and name ~= "pcall" and name ~= "xpcall" and RAISING_GLOBALS[name]
            const called = if raising then `code called {name}()` else frame
            const unquoted = string.gsub(string.gsub(called, '%["', "["), '"%]', "]")
            table.insert(lines, `{index} \t{unquoted}`)
        end
    else
        table.insert(lines, message)
        if #frames > 0 then
            table.insert(lines, "stack traceback:")
        end
        for _, frame in frames do
            table.insert(lines, `\t{frame}`)
        end
    end

    local new_traceback = table.concat(lines, "\n")

    if read.might_be_rust_error then
        new_traceback ..= `\n\nError appears to have come from a {colors.red("Rust")} file; this might indicate a bad error message or bug in seal.\n\z
        Consider reporting this to {colors.blue("https://github.com/seal-runtime/seal/issues")}`
    end

    -- only show 'Need help?' on seal stdlib errosr messages, not on any error messages purposely raised by user code
    -- like 'error', 'assert', 'unreachable', etc.
    const raised_by_user = #frames > 0 and not raised_by_seal
    if not raised_by_user then
        const seal_need_help = env.vars.get("SEAL_NEED_HELP")
        if seal_need_help == nil or (seal_need_help and string.lower(seal_need_help) == "true") then
            new_traceback ..= `\n\nNeed help? {colors.style.dim("(set SEAL_NEED_HELP=FALSE to hide)")}\z
            \n  Check the docs:   {colors.blue("https://github.com/seal-runtime/seal/tree/main/docs/reference")}\z
            \n  Join the Discord: {colors.blue("https://discord.gg/3MJ37CFNWh")}`
        end
    end

    return new_traceback
end

const function parse(message: string, traceback: string?): string | error
    const success, result = pcall(parse_traceback, message, traceback)
    if success and type(result) == "string" then
        return result
    end
    -- if the error formatter throws we don't want to clobber what actually caused the error
    const err = require("@std/err")
    const format = require("@std/io/format")
    const str = require("@std/str")
    const original = if traceback then `{message}\n{traceback}` else message
    const indented = str.indent(original, "Spaces", 2)
    return err.message(`original error traceback:\n{format.uncolor(indented)}\n\nerror in error formatter:\n  {result}`)
end

return parse
