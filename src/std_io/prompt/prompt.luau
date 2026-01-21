local str = require("@std/str")
local err = require("@std/err")
local time = require("@std/time")
local colors = require("@std/colors")
local format = require("@std/io/format")

-- note!! requiring the whole @std/io library here will cause overflow panic
-- cause this is *part* of the @std/io library!!
local input = require("@std/io/input")
local output = require("@std/io/output")

local n5 = "\n     "

local prompt = {}

local function df(v: any)
    return if v == nil then "nil" else format.debug(v)
end

function prompt.validate(message: string, v: (response: string) -> true | string): err.Result<string>
    local function_name = "prompt.validate(message: string, v: (response: string) -> true | string)"

    if typeof(message) ~= "string" then
        return err.wrap(`{function_name}{n5} expected message to be a string, got: {df(message)}`)
    end

    if typeof(v) ~= "function" then
        return err.wrap(`{function_name}{n5} expected validator function, got: {df(v)}`)
    end

    if not string.find(message, ": ", 1, true) then
        message ..= ": "
    end

    local result = input.readline(message)

    if typeof(result) == "string" then
        local _, validation = pcall(v, result)
        if validation == true then
            return result
        elseif typeof(validation) == "string" then
            warn(validation)
            if not str.startswith(message, "Try again?") then
                message = "Try again? " .. message
            end
            return prompt.validate(message, v)
        elseif typeof(validation) == "error" then
            return err.wrap(`{function_name}:{n5} validator function errored at runtime: {tostring(validation)}`)
        else
            return err.wrap(`{function_name}:{n5} validator function returned an invalid type, expected true or a string, got: {df(validation)}`)
        end
    elseif typeof(result) == "interrupt" then
        return err.wrap(`Prompt interrupted with {if result.code == "CtrlC" then "Ctrl-C" else "Ctrl-D"}; use io.input.readline to intercept without erroring`)
    elseif typeof(result) == "error" then
        return err.wrap(`{function_name}: unexpected ReadlineError: {tostring(result)}`)
    end

    error("unreachable")
end

function prompt.pick(message: string, options: { string }, default: number?): err.Result<number>
    local function_name = "prompt.pick(message: string, options: { string }, default: number?)"

    if typeof(message) ~= "string" then
        return err.wrap(`{function_name}{n5} expected message to be a string, got: {df(message)}`)
    end

    if typeof(options) ~= "table" then
        return err.wrap(`{function_name}{n5} expected options to be an array-like table (\{ string \}), got: {df(options)}`)
    end

    if #options == 0 then
        return err.wrap(`{function_name}:{n5} passed options table is empty or isn't an array-like table\ngot options = {format.simple(options)}`)
    end

    if not string.find(message, ": ", 1, true) then
        message ..= ": "
    end

    if default then
        if typeof(default) ~= "number" then
            return err.wrap(`{function_name}:{n5} default value should be nil/unspecified or an integer number, got: {df(default)}`)
        elseif default ~= math.round(default) then
            return err.wrap(`{function_name}:{n5} default value should be an integer number, got a float ({default})`)
        elseif default <= 0 then
            return err.wrap(`{function_name}:{n5} default value should be positive (got {default})`)
        elseif default > #options then
            return err.wrap(`{function_name}:{n5} default value out of range (options table has {#options} value{if #options > 1 then "s" else ""} but default was {default})`)
        end
    end

    local index: number
    
    if input.tty() then
        print(`{message} {colors.style.dim("(up/down arrow keys to move, enter = select, r = redraw, numbers = jump)")}`)

        input.rawmode(true)

        output.write("\27[?25l") -- hide cursor

        local current = default or 1
        -- write all options down putting green > next to default
        for option_index, option in options do
            output.write(`{if option_index == current then colors.bold.green(">") else " "}  {colors.bold.blue(tostring(option_index))} {option}\r\n`)
        end
        -- move cursor back up to default option
        for i = #options, 1, -1 do
            output.write("\27[A") -- go up 1
            if i == current then
                break 
            end
        end

        local interrupt_message: string?

        local digits_buffer: { string } = {}

        local function go_up(count: number, change_state: boolean)
            for i = 1, math.abs(count) do
                if change_state and (current - 1) == 0 then
                    break
                end
                if change_state then
                    output.write("\r ") -- erase the > at the start of the current line
                end
                output.write("\27[A") -- go up 1 visually
                if change_state then
                    current -= 1
                    output.write("\r" .. colors.bold.green(">")) -- write a new > at the current line
                end
            end
        end

        local function go_down(count: number, change_state: boolean)
            for i = 1, math.abs(count) do
                if change_state and (current + 1) > #options then
                    break
                end
                if change_state then
                    output.write("\r ")
                end
                output.write("\27[B") -- go down 1 visually
                if change_state then
                    current += 1
                    output.write("\r" .. colors.bold.green(">"))
                end
            end
        end

        local function show()
            go_up(current, false)
            output.write("\27[2K\27[G")
            output.write(`{message}{options[current]} ({current}) buffer: {`[{table.concat(digits_buffer, ", ")}]`}`)
            go_down(current, false)
        end

        for event in input.events(time.milliseconds(20)) do
            if event.is == "Key" then
                if event.key == "Up" and current - 1 > 0 then
                    go_up(1, true)
                    table.clear(digits_buffer)
                    show()
                elseif event.key == "Down" and current < #options then
                    go_down(1, true)
                    table.clear(digits_buffer)
                    show()
                elseif event.key == "Enter" then
                    index = current
                    break
                elseif event.key == "c" and event.modifiers.ctrl then
                    interrupt_message = "Prompt interrupted with Ctrl+C"
                    index = current
                    break
                elseif event.key == "d" and event.modifiers.ctrl then
                    interrupt_message = "Prompt interrupted with Ctrl+D"
                    index = current
                    break
                elseif event.key == "." then
                    show()
                elseif event.key == "r" then
                    output.write("\r")
                    input.rawmode(false)
                    output.clear()
                    return prompt.pick(message, options, default)
                elseif event.key == "Backspace" and #digits_buffer > 0 then
                    table.remove(digits_buffer, #digits_buffer)
                else
                    local number_pressed = string.match(event.key, "([%d]+)")
                    if not number_pressed then
                        continue
                    end

                    table.insert(digits_buffer, number_pressed)

                    local index_target = tonumber(table.concat(digits_buffer))
                    assert(index_target ~= nil, "concating a bunch of %d strings should result in a valid number")

                    -- if index_target > options, we assume the user meant to navigate to the number they just pressed
                    -- instead of the combined one in the buffer
                    if index_target == 0 or index_target > #options then
                        table.clear(digits_buffer)
                        table.insert(digits_buffer, number_pressed)

                        local pressed_index = tonumber(number_pressed)
                        assert(pressed_index ~= nil, "number from %d must not be nil")
                        -- index_target = math.max(index_target, 1)
                        index_target = math.clamp(pressed_index, 1, #options)
                    end

                    local diff = index_target - current
                    if diff > 0 then
                        go_down(diff, true)
                    else
                        go_up(diff, true)
                    end

                    show()
                end
            end
        end

        -- output.write("\x1b[999B\x1b[G")   -- move to absolute bottom left of terminal
        -- move down from current to bottom of options
        go_down((#options - current) + 1, false)
        output.write("\r") -- make sure we're on the left before starting to erase

        for i = 1, (#options + 2) do
            output.write("\27[A")         -- move up one line
            output.write("\27[2K\27[G")   -- erase line and go to start
        end

        output.write(message .. options[index] .. "\n")
        output.write("\r") -- return cursor to LHS

        output.write("\27[?25h") -- show cursor

        input.rawmode(false)
    
        if interrupt_message then
            return err.wrap(interrupt_message)
        end
    else
        print(message)

        print()

        for option_index, option in options do
            print(`  {option_index}: {option}`)
        end

        print()

        while true do
            local response = str.trimback(input.rawline("Pick an index from the above: "))
            response = tonumber(response)
            if response and response > 0 and response < #options then
                index = response
                break
            else
                print(`Invalid index, try again?`)
            end
        end
    end

    return index
end

return prompt