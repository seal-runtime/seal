--!optimize 2
local str = require("@std/str")
local err = require("@std/err")
local env = require("@std/env")
local time = require("@std/time")
local colors = require("@std/io/colors")
local format = require("@std/io/format")
local terminal = require("@std/terminal")
local cursor = require("@std/terminal/cursor")

-- note!! requiring the whole @std/io library here will cause overflow panic
-- cause this is *part* of the @std/io library!!
local input = require("@std/io/input")

local n5 = "\n     "

local prompt = {}

local function df(v: any)
    return if v == nil then "nil" else format.debug(v)
end

local function should_use_colors()
    -- first check if user configured seal not to use colors
    if env.vars.flag("SEAL_NO_COLORS", false) then
        return false
    end

    -- NO_COLOR standard: https://no-color.org/
    if env.vars.flag("NO_COLOR", false) then
        return false
    end

    -- check if we have COLORTERM or TERM environment variables
    local COLORTERM = env.vars.get("COLORTERM")
    if COLORTERM and (COLORTERM == "truecolor" or COLORTERM == "24bit") then
        return true
    end

    local TERM = env.vars.get("TERM")
    if TERM and string.find(TERM, "256color") then
        return true
    elseif TERM and TERM == "dumb" then
        return false
    end

    return true
end

local SHOULD_USE_COLORS = should_use_colors()

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
        return err.wrap(`{function_name}:{n5} expected 'message' to be a string, got: {df(message)}`)
    end

    if typeof(options) ~= "table" then
        return err.wrap(`{function_name}{n5} expected options to be an array-like table (\{ string \}), got: {df(options)}`)
    end

    if #options == 0 then
        return err.wrap(`{function_name}:{n5} passed options table is empty or isn't an array-like table\ngot options = {format.simple(options)}`)
    end

    message = str.trimback(message)
    if not str.ends(message, ":", "?") then
        message ..= ":"
    end

    if default then
        if typeof(default) ~= "number" then
            return err.wrap(`{function_name}:{n5} default value should be a positive whole number or nil/unspecified, got: {df(default)}`)
        elseif default ~= math.round(default) then
            return err.wrap(`{function_name}:{n5} default value should be an positive whole number, got a float ({default})`)
        elseif default <= 0 then
            return err.wrap(`{function_name}:{n5} default value should be positive (got {default})`)
        elseif default > #options then
            return err.wrap(`{function_name}:{n5} default value out of range (options table has {#options} value{if #options > 1 then "s" else ""} but default was {default})`)
        end
    end

    local selected: number = default or 1

    local should_be_interactive =
        terminal.tty()
        and not env.vars.flag("SEAL_SIMPLE_PROMPT", false)

    for index, option in options do
        if string.find(option, "\n") then
            options[index] = string.gsub(option, "\n", " ")
        end
        if string.find(option, "\r") then
            options[index] = string.gsub(options[index], "\r", "")
        end
    end

    if not should_be_interactive then
        print(message)
        for option_index, option in options do
            print(`  {option_index}: {option}`)
        end
        print()
        while true do
            local response = str.trimback(input.rawline("Pick an index from the above: "))
            response = tonumber(response)
            if response and response > 0 and response <= #options then
                selected = response
                break
            else
                print(`Invalid index, try again?`)
            end
        end
        return selected
    end

    -- interactive portion time

    local screen_size = terminal.size()

    --- accounts for message and help rows above options window and scroll indicator under options window
    local extra_rows = 3

    local initial_cursor_position = cursor.position()
    local total_rows_to_allocate = math.min(
        #options + extra_rows, 
        screen_size.y
    )
    
    -- if we can fit all options on the screen but we currently don't
    -- have enough rows between the cursor and the bottom of the screen,
    -- we should print enough rows until we do
    local rows_from_cursor_to_bottom = screen_size.y - initial_cursor_position.y
    if rows_from_cursor_to_bottom < total_rows_to_allocate then
        for i = 1, total_rows_to_allocate do
            print()
        end
        terminal.execute( -- go back up
            cursor.prevline(total_rows_to_allocate),
            terminal.clear("FromCursorDown")
        )
    end

    --- Options window describes which options are currently visible
    --- if we have too many options than can fit on screen,
    --- you can scroll up and down to change which options you see
    --- x is the first option visible and y is the last option visible
    local options_window =
        if (total_rows_to_allocate - extra_rows) < #options then
            vector.create(1, total_rows_to_allocate - extra_rows)
        else
            vector.create(1, #options)

    local normalized_message_length = str.width(format.uncolor(message))

    local top_position = cursor.position()
    local top_update_position = top_position + vector.create(normalized_message_length + 1, 0)
    local options_window_top = top_position + vector.create(0, 2)
    local options_window_bottom = options_window_top + vector.create(0, options_window.y - options_window.x)
    local text_entry_position = options_window_bottom + vector.create(62, 1)

    local submit_on_click = false

    -- SAFETY: terminal.tty() has already been checked
    if not terminal.rawmode.enabled() then
        terminal.rawmode.enable()
    end
    terminal.capture.mouse(true)
    cursor.hide():execute()

    local function truncate_string_middling_to_size(s: string, size: number)
        local offset_for_size = math.max((size // 2) - 3, 0)
        local front_portion = string.sub(s, 1, offset_for_size)
        local back_portion = string.sub(s, #s - offset_for_size, #s)
        return front_portion .. ".." .. back_portion
    end
    local function is_in_window(pos: number, window: vector)
        return pos >= window.x and pos <= window.y
    end
    local function all_options_visible(window: vector, len: number)
        return (window.y - window.x) + 1 >= len
    end

    local function render()
        local actions: { TerminalAction } = {}
        local function push(...: TerminalAction)
            for _, action in {...} do
                table.insert(actions, action)
            end
        end

        local current_size = terminal.size()
        local current_option = options[selected]
        local uncolored_current_option = format.uncolor(current_option)
        local uncolored_message = format.uncolor(message)

        local space_available_for_message = current_size.x - str.width(uncolored_current_option) - 1
        local message_line_horizontal_width = str.width(uncolored_message) + str.width(uncolored_current_option) + 1

        local message_truncated_if_needed = 
            if message_line_horizontal_width > current_size.x then
                truncate_string_middling_to_size(message, space_available_for_message)
            else 
                message
        
        -- adjust the place where the current option is visible dynamically based upon the size of the message
        top_update_position = vector.create(str.width(format.uncolor(message_truncated_if_needed)) + 1, top_update_position.y)
        local help_message =
            if current_size.x > 99 then
                "Scroll, use arrow keys, or type a number to pick an option; double-click or press enter to select."
            elseif current_size.x > 58 then
                "Scroll/Arrow Keys/Number = Move; 2xClick/Enter = Select"
            elseif current_size.x > 24 then
                "pls widen terminal"
            else
                "WIDEN ME"
        push(
            cursor.to(top_position),
            terminal.clear("CurrentLine"),
            terminal.write(message_truncated_if_needed),
            cursor.nextline(),
            terminal.clear("CurrentLine"),
            terminal.write(colors.style.dim(help_message)),
            cursor.to(options_window_top),
            terminal.clear("FromCursorDown")
        )
        for option_index, option in options do
            -- skip options that shouldn't be visible
            if not is_in_window(option_index, options_window) then
                continue
            end
            local index_for_display = colors.bold.blue(tostring(option_index))
            local GREEN_BACKGROUND_WITH_WHITE_TEXT =
                if SHOULD_USE_COLORS then
                    "\27[48;2;0;128;0;38;2;255;255;255m"
                else
                    ""
            -- make sure whole option text stays on same line
            local option_line_width = #tostring(option_index) + 4 + str.width(format.uncolor(option))
            if option_line_width > current_size.x then
                option = truncate_string_middling_to_size(option, current_size.x - 6)
            end
            
            local option_line =
                if option_index == selected then
                    `{" " .. colors.bold.green(">")} {index_for_display} {colors.style.underline(option)}` .. (
                        if submit_on_click then
                            string.rep(" ", 4) .. GREEN_BACKGROUND_WITH_WHITE_TEXT .. " Click again to confirm " .. colors.codes.RESET
                        else
                            ""
                    )
                else
                    `   {index_for_display} {option}`
                
            push(
                terminal.write(option_line),
                cursor.nextline()
            )
        end

        local scroller_message =
            if current_size.x > 60 then
                `Scroll up or down to see more options ({options_window.x}-{options_window.y} of {#options} visible)`
            elseif current_size.x > 20 then
                `({options_window.x}-{options_window.y}/{#options} visible)`
            else
                "pls widen"
            
        if not all_options_visible(options_window, #options) then
            -- jump to bottom to update scroll information
            push(
                cursor.to(options_window_bottom + vector.create(0, 1)),
                terminal.write(colors.style.dim(scroller_message))
            )
        end
        -- now jump to top right to update the currently selected option
        local space_for_current_option = current_size.x - (str.width(format.uncolor(message_truncated_if_needed)) + 4)
        if str.width(uncolored_current_option) > space_for_current_option then
            current_option = truncate_string_middling_to_size(current_option, space_for_current_option)
        end
        push(
            cursor.to(top_update_position),
            terminal.clear("UntilNewLine"),
            terminal.write(current_option)
        )
        terminal.execute(table.unpack(actions))
    end

    render()
    
    local interrupt: interrupt?
    local digits_buffer: { string } = {}
    local scroll_delta = vector.create(1, 1)

    local function get_window_range_where_visible(current: number, window: vector): vector
        if current < window.x then
            local diff = window.x - current
            return window - diff * scroll_delta
        elseif current > window.y then
            local diff = current - window.y
            return window + diff * scroll_delta
        end
        return window
    end

    local function update_text_entry_pos()
        local position = 
            if all_options_visible(options_window, #options) then
                vector.create(0, text_entry_position.y)
            else
                text_entry_position
        terminal.execute(
            cursor.save(),
            cursor.to(position),
            terminal.clear("UntilNewLine"),
            terminal.write("#: " .. table.concat(digits_buffer)),
            cursor.restore()
        )
    end

    for event in terminal.events(time.milliseconds(10)) do
        if event.type == "Key" then
            interrupt = terminal.interrupt.check(event)
            if interrupt then
                break
            end
            if event.key == "Up" then
                if selected > 1 then
                    if selected == options_window.x then
                        -- window should follow currently selected option
                        options_window -= scroll_delta
                    end
                    selected -= 1
                    submit_on_click = false
                    render()
                end
            elseif event.key == "Down" then
                if selected < #options then
                    if selected == options_window.y then
                        options_window += scroll_delta
                    end
                    selected += 1
                    submit_on_click = false
                    render()
                end
            elseif event.key == "Enter" then
                break
            elseif event.key == "Backspace" then
                table.remove(digits_buffer, #digits_buffer)
                update_text_entry_pos()
            else
                local digit = tonumber(event.key)
                if digit then
                    table.insert(digits_buffer, event.key)
                    local new_index = tonumber(table.concat(digits_buffer))
                    assert(new_index, "number concatenated from table of %d strings must be a number")
                    if new_index > 0 and new_index <= #options then
                        selected = new_index
                    else
                        selected = math.clamp(digit, 1, #options)
                        table.clear(digits_buffer)
                        table.insert(digits_buffer, tostring(selected))
                    end
                    submit_on_click = false
                    -- if we've jumped to an option out of window we need to move
                    -- the window so that we see the option
                    if not is_in_window(selected, options_window) then
                        options_window = get_window_range_where_visible(selected, options_window)
                    end
                    render()
                    -- we should inform the user what numbers we're tracking
                    update_text_entry_pos()
                end
            end
        elseif event.type == "Resize" then
            -- screen size changed, we need to adjust options_window size according to new space
            local options_rows = math.max(1, math.min(event.size.y - options_window_top.y, #options))
            local bottom_row = if options_rows <#options then 1 else 0
            options_window = vector.create(1, options_rows - bottom_row)
            options_window_bottom = options_window_top + vector.create(0, options_window.y - options_window.x)
            -- resizing can cause selected option to go out of bounds, let's make sure that doesn't happen
            if not is_in_window(selected, options_window) then
                options_window = get_window_range_where_visible(selected, options_window)
            end
            render()
        elseif event.type == "Mouse" then
            if event.kind == "ScrollUp" then
                if selected > 1 then
                    if selected == options_window.x then
                        options_window -= scroll_delta
                    end
                    selected -= 1
                    submit_on_click = false
                end
                render()
            elseif event.kind == "ScrollDown" then
                if selected < #options then
                    if selected == options_window.y then
                        options_window += scroll_delta
                    end
                    selected += 1
                    submit_on_click = false
                end
                render()
            elseif event.kind == "Up(Left)" then
                if submit_on_click then
                    break
                else
                    submit_on_click = true
                end
                render()
            end
        end
    end

    --- prompt.pick specific functionality to always reset
    local function reset()
        terminal.capture.mouse(false)
        terminal.rawmode.disable()
        terminal.execute(
            cursor.to(top_position),
            cursor.show(),
            cursor.nextline(),
            terminal.clear("CurrentLine"),
            terminal.clear("FromCursorDown")
        )
    end

    if interrupt then
        reset()
        return err.wrap(`{function_name}: prompt interrupted with {interrupt}`)
    end

    reset()
    return selected
end

return prompt