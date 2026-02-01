local input = require("@std/io/input")
local env = require("@std/env")
local success, cwd = pcall(function()
    return env.cwd() -- pcalling here to prevent downstream panic in err.rs if cwd doesn't exist
end)

if not success or typeof(cwd) == "error" then
    warn("Your current working directory might've been removed! Try reloading your editor.")
    cwd = "nonexisting/removed directory"
end

--- the 3 lines surrounding context
local function get_line_context(path: string, context_line_number: number): { string }
    local fs = require("@std/fs")
    local lines: { string } = {}
    for line_number, line in fs.readlines(path) do
        if 
            line_number == context_line_number 
            or line_number == (context_line_number - 1) 
            or line_number == (context_line_number + 1)
        then
            table.insert(lines, line)
        end
        if #lines == 3 then
            break
        end
    end
    return lines
end

local function parse_traceback(traceback: string): string
    local is_tty = input.tty()
    local pretty: boolean do
        local pretty_var = env.vars.get("SEAL_PRETTY_ERRORS")
        local seal_no_colors = env.vars.get("SEAL_NO_COLORS")

        if pretty_var and string.lower(pretty_var) == "true" then
            pretty = true
        elseif seal_no_colors and string.lower(seal_no_colors) == "true" then
            local format = require("@std/io/format")
            traceback = format.uncolor(traceback)
            pretty = false
        elseif pretty_var and string.lower(pretty_var) == "false" then
            pretty = false
        else
            pretty = true
        end
    end

    local new_traceback: string = traceback
        :gsub("%[string ", "[")
        :gsub("%s*%[C%]: in %?", "")
        :gsub(': in function <%[.+', ": in top level")
        :gsub(cwd :: string, ".")
    if is_tty and pretty then
        local str = require("@std/str")
        local colors = require("@std/colors")

        -- syntax highlighting stuff
        local tokenmap: { [string]: (string) -> string } = {
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

        local function get_string_delim(token: string): string?
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
                else nil
        end

        local function syntax(line: string): string
            local splits = str.splitaround(line, " ", "--", ":", ".", "+", "-", "/", "(", ")", '"', "'", "`", "[[", "]]", ",", "[=[", "]=]", "[", "]")
            local result: { string } = {}
            local last = ""

            local in_string = false
            local string_delim: string? = nil

            local in_comment = false
            local just_set_indent_next_line = false

            for _, token in splits do
                local colorizer = tokenmap[token]
                local colorized = 
                    if colorizer then 
                        colorizer(token)
                    elseif tonumber(token) then
                        colors.blue(token)
                    elseif in_string then
                        colors.green(token)
                    else 
                        token

                local new_string_delim = get_string_delim(token)
                if in_string and new_string_delim and new_string_delim == string_delim then
                    in_string = false
                    string_delim = nil
                elseif not in_string and new_string_delim then
                    in_string = true
                    string_delim = new_string_delim
                    colorized = colors.green(token)
                end

                if token == "(" and last ~= " " then -- it's a fncall
                    local popped = table.remove(result, #result) :: string
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

        -- new_traceback = new_traceback:gsub("stack traceback", colors.bold.yellow("stack traceback"))
        local lines = str.splitlines(new_traceback)
        local is_runtime_error = false
        local is_error_error = false
        local first_path: string?, first_line_number: string?

        local passed_stack_traceback = false
        local current_frame = 0
        --- if a chunk name ends in *.rs we might have an issue in seal
        local might_be_rust_error = false

        for index, line in lines do
            if index == 1 then
                is_runtime_error = string.match(line, "runtime error: ") ~= nil
                if is_runtime_error then
                    local path, line_number = string.match(line, `%["([%w%s%p]+)"%]:(%d+)`)
                    if path and line_number then
                        first_path = path
                        first_line_number = line_number
                        local fixed_line = line
                            -- :gsub("runtime error:", "")
                            :gsub(`[%s]+%["[%w%s%p]+"%]:%d+:[%s]+`, " ")
                        lines[index] = fixed_line
                    end
                else
                    -- some seal stdlib error messages are pretty long, let's try giving them newlines
                    if #line > 100 then
                        line = string.gsub(line, "%):", "):\n  ")
                    end
                    lines[index] = line
                end
                continue
            end

            if not passed_stack_traceback and string.match(line, "stack traceback") then
                passed_stack_traceback = true
                continue
            end

            if passed_stack_traceback then
                if string.match(line, "%.rs") then
                    might_be_rust_error = true
                end
                if string.match(line, "%[C%]: in function 'error'") then
                    is_error_error = true
                    line = line:gsub("%[C%]: in function 'error'", "code called error()")
                end
                current_frame += 1
                line = string.gsub(line, '%["', "["):gsub('"%]', "]")
                line = `{current_frame} ` .. line
                lines[index] = line
            end
        end

        local succ, err = pcall(function()
            if is_runtime_error and first_path and first_line_number and tonumber(first_line_number) then
                local line_number = tonumber(first_line_number) :: number
                local context = get_line_context(first_path, line_number)
                local trimmed = str.trim(context[2])
                local highlighted = syntax(trimmed)
                table.insert(lines, 2, colors.style.underline(`{first_path}:{first_line_number}`))
                table.insert(lines, 3, ` {colors.style.dim(tostring(line_number - 1))} | ` .. colors.style.dim(str.trim(context[1])))
                table.insert(lines, 4, ` {colors.blue(tostring(line_number))} | ` .. highlighted)
                table.insert(lines, 5, ` {string.rep("." , #tostring(line_number + 1))} | ` .. colors.red(string.rep("^", #trimmed)))
            end
            return nil
        end)

        if might_be_rust_error then
            table.insert(lines, `\nError appears to have come from a {colors.red("Rust")} file; this might indicate a bad error message or bug in seal.\nConsider reporting this to {colors.blue("https://github.com/seal-runtime/seal/issues")}`)
        end

        if not is_runtime_error and not is_error_error then
            local seal_need_help = env.vars.get("SEAL_NEED_HELP")
            if seal_need_help == nil or (seal_need_help and string.lower(seal_need_help) == "true") then
                table.insert(lines, `\nNeed help? {colors.style.dim("(set SEAL_NEED_HELP=FALSE to hide)")}\z
                \n  Check the docs:   {colors.blue("https://github.com/seal-runtime/seal/tree/main/docs/reference")}\z
                \n  Join the Discord: {colors.blue("https://discord.gg/3MJ37CFNWh")}`)
            end
        end

        if succ then
            new_traceback = table.concat(lines, "\n")
        end
    end

    return new_traceback
end

return parse_traceback