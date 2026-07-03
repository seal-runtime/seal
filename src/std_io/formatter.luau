--!optimize 2
--!nolint LocalShadow
--[[
    doing highly recursive operations + lots of table iteration is faster directly in luau
    rather than through mlua's interop layer, which, among other things, has additional safety costs
    associated with iterating through tables
]]

local mlua = require("@interop/mlua")
local colors = require("@std/io/colors")
local vars = require("@std/env/vars")
local format = require("@std/io/format")
local terminal = require("@std/terminal")

local VECTOR_FOUR_WIDE_MODE = (function()
    local s, _ = pcall(function()
        local testvec = (vector.create :: any)(1, 2, 3, 4)
        return testvec.w
    end)
    return s
end)()

local function get_table_id(t: { [any]: any }): string
    if typeof(t) ~= "table" then
        error("[INTERNAL] output_process_values.get_table_id got passed something that isn't a table")
    end
    local tostringed_t = tostring(t)
    -- remove the `table: ` part of `table: 0x000062524e10a0d8`
    local tableid = string.sub(tostringed_t, 8, #tostringed_t)
    return tableid
end

local IDENTIFIER_PATTERN = "^[%a_][%w_]*$"

local function process_raw_values(
    value: any,
    seen_tables: { [string]: string }?,
    depth: number?
): string
    seen_tables = seen_tables or {}
    local result: { string } = {}
    local depth = depth or 0
    local current_indent = string.rep("    ", depth)
    if typeof(value) == "table" then
        local value = value :: { [any]: any }
        local table_id = get_table_id(value)
        local seen_name = seen_tables[table_id]
        if seen_name then
            table.insert(result, "cyclic(" .. seen_name .. ")")
        else
            if depth == 0 then
                seen_tables[table_id] = "self" -- so we can recognize top level cyclic tables
            end
            table.insert(result, "{\n")
            for key, val in value do
                table.insert(result, current_indent)
                table.insert(result, "    ")
                -- handle key stringification
                if typeof(key) == "string" then
                    -- we can omit brackets if key is an identifier-like key (like "meow1234" which is vs "cats are cool" which isn't)
                    if string.match(key, IDENTIFIER_PATTERN) then
                        table.insert(result, key)
                    else
                        table.insert(result, "[\"" .. key :: string .. "\"]") -- LUAU FIXME: error type regression here
                    end
                elseif typeof(key) == "number" then
                    if mlua.isint(key) then
                        table.insert(result, "[" .. tostring(key) .. "]")
                    else -- is a LuaNumber
                        if math.round(key) == key then
                            table.insert(result, "[" .. tostring(key) .. ".0]")
                        else
                            table.insert(result, "[" .. tostring(key) .. "]")
                        end
                    end
                elseif typeof(key) == "table" then
                    local key = key :: { [any]: any }
                    local key_tableid = get_table_id(key)
                    local seen_key_name = seen_tables[key_tableid]
                    if seen_key_name then
                        table.insert(result, "[cyclic(" .. seen_key_name .. ")]")
                    else
                        -- according to ariel et al, people probably want to see addresses in debug mode
                        local stringified_tableid = string.gsub(table_id, "0x0*", "")
                        table.insert(result, "[table(0x" .. stringified_tableid .. ")]")
                        -- but pretty print should show full table
                    end
                else
                    table.insert(result, "[" .. process_raw_values(key, seen_tables, depth + 1) .. "]")
                end
                table.insert(result, " = ")
                -- handle val stringification
                if typeof(val) == "table" then
                    local val = val :: { [any]: any }
                    local val_tableid = get_table_id(val)
                    local seen_val_name = seen_tables[val_tableid]
                    if seen_val_name then
                        table.insert(result, "cyclic(" .. seen_val_name .. ")")
                    else
                        table.insert(result, process_raw_values(val, seen_tables, depth + 1))
                    end
                elseif typeof(val) == "string" then -- no need for extra recursive call for common case
                    table.insert(result, '"' .. val :: string .. '"') -- LUAU FIXME: error type on val
                else
                    table.insert(result, process_raw_values(val, seen_tables, depth + 1))
                end
                table.insert(result, ",\n")
            end
        end
        table.insert(result, current_indent ..  "}")
    elseif typeof(value) == "string" then
        -- LUAU FIXME: need to cast to string here to fix error type
        table.insert(result, (if depth == 0 then value else  '"' .. value .. '"') :: string)
    elseif typeof(value) == "boolean" then
        table.insert(result, tostring(value))
    elseif typeof(value) == "number" then
        if mlua.isint(value) then
            table.insert(result, tostring(value))
        else
            if math.round(value) == value then
                table.insert(result, tostring(value) .. ".0")
            else
                table.insert(result, tostring(value))
            end
        end
    elseif typeof(value) == "function" then
        local function_id = tostring(value)
        function_id = string.gsub(function_id, "function: 0x0*", "")
        return "function<0x" .. function_id .. ">"
    elseif typeof(value) == "buffer" then
        local buffer_id = tostring(value)
        buffer_id = string.gsub(buffer_id, "buffer: 0x0*", "")
        return "buffer<0x" .. buffer_id .. ">"
    elseif type(value) == "vector" then
        local vec = "vector<" .. value.x .. ", " .. value.y .. ", " .. value.z
        if VECTOR_FOUR_WIDE_MODE then
            vec ..= ", " ..  value.z
        end
        vec ..= ">"
        return vec
    elseif mlua.iserror(value) then
        local stringified_error = tostring(value)
        local errsplit = string.split(stringified_error, "\n")
        local errmsglist: { string } = {}
        for _, m in errsplit do
            if string.find(m, "stack traceback") then
                break
            else
                table.insert(errmsglist, m)
            end
        end
        return "error<" .. format.uncolor(table.concat(errmsglist)) .. ">"
    elseif type(value) == "userdata" then
        local typeof_value = typeof(value)
        if typeof_value ~= "userdata" then
            return "userdata<" .. typeof_value .. ">"
        else
            local userdata_id = tostring(value)
            userdata_id = string.gsub(userdata_id, "userdata: 0x0*", "")
            return "userdata<0x" .. userdata_id .. ">"
        end
    elseif type(value) == "nil" then
        return "nil"
    elseif type(value) == "thread" then
        local coro_id = tostring(value)
        coro_id = string.gsub(coro_id, "thread: 0x0*", "")
        return "coroutinethread<0x" .. coro_id .. ">"
    else
        return tostring(value)
    end
    return table.concat(result, "")
end

local RESET = colors.codes.RESET
local BOLD_WHITE = colors.codes.BOLD_WHITE
local RED = colors.codes.RED
local BOLD_RED = colors.codes.BOLD_RED
local CYAN = colors.codes.CYAN
local MAGENTA = colors.codes.MAGENTA
local BOLD_MAGENTA = colors.codes.BOLD_MAGENTA
local BLUE = colors.codes.BLUE
local BOLD_BLUE = colors.codes.BOLD_BLUE
local GREEN = colors.codes.GREEN
local YELLOW = colors.codes.YELLOW
local BOLD_YELLOW = colors.codes.BOLD_YELLOW
local BOLD = colors.codes.BOLD
local DIM = colors.codes.DIM

local is_tty = terminal.tty()
local is_truecolor = (function()
    local colorterm = vars.get("COLORTERM")
    return colorterm == "truecolor" or colorterm == "24bit"
end)()

-- Each entry is a (dr, dg, db) deviation from a neutral base to give the color its hue identity.
-- The base luminance shifts so guide lines stay readable but muted against any background.
local GUIDE_HUE_OFFSETS = {
    vector.create(-25, 10, 10),  -- cyan
    vector.create( 10, -25, 10),  -- magenta
    vector.create(-25, -10, 35),  -- blue
    vector.create( 20, 10, -35),  -- yellow
    vector.create(-30, 15, -15),  -- green
    vector.create( 20, -30, -30),  -- red
}

local function compute_guide_colors(): { string }
    if not is_truecolor then
        return { CYAN, MAGENTA, BLUE, YELLOW, GREEN, RED }
    end
    local bg = terminal.background()
    local bg_lum = if bg then (0.299 * bg.x + 0.587 * bg.y + 0.114 * bg.z) / 255 else 0.12
    local base = math.round(110 - 50 * bg_lum)
    local result: { string } = {}
    for _, offset in GUIDE_HUE_OFFSETS do
        local r = math.clamp(base + offset.x, 0, 255)
        local g = math.clamp(base + offset.y, 0, 255)
        local b = math.clamp(base + offset.z, 0, 255)
        table.insert(result, colors.rgb(vector.create(r, g, b)))
    end
    return result
end

local GUIDE_COLORS: { string } = compute_guide_colors()

local colors_enabled_state: boolean = is_tty and vars.get("NO_COLOR") == nil

local function apply_color_state(enabled: boolean)
    if enabled then
        RESET = colors.codes.RESET
        BOLD_WHITE = colors.codes.BOLD_WHITE
        RED = colors.codes.RED
        BOLD_RED = colors.codes.BOLD_RED
        CYAN = colors.codes.CYAN
        MAGENTA = colors.codes.MAGENTA
        BOLD_MAGENTA = colors.codes.BOLD_MAGENTA
        BLUE = colors.codes.BLUE
        BOLD_BLUE = colors.codes.BOLD_BLUE
        GREEN = colors.codes.GREEN
        YELLOW = colors.codes.YELLOW
        BOLD_YELLOW = colors.codes.BOLD_YELLOW
        BOLD = colors.codes.BOLD
        DIM = colors.codes.DIM
        GUIDE_COLORS = compute_guide_colors()
    else
        RESET = ""
        BOLD_WHITE = ""
        RED = ""
        BOLD_RED = ""
        CYAN = ""
        MAGENTA = ""
        BOLD_MAGENTA = ""
        BLUE = ""
        BOLD_BLUE = ""
        GREEN = ""
        YELLOW = ""
        BOLD_YELLOW = ""
        BOLD = ""
        DIM = ""
        GUIDE_COLORS = { "", "", "", "", "", "" }
    end
end

if not colors_enabled_state then
    apply_color_state(false)
end

local LEFT_BRACKET = DIM .. "[" .. RESET
local RIGHT_BRACKET = DIM .. "]" .. RESET

local LEFT_ANGLED = BOLD .. DIM .. "<" .. RESET
local RIGHT_ANGLED = BOLD .. DIM .. ">" .. RESET

local ID_CATCHER_PATTERN = "[.]*0x0*([%d%w]+)"

local function stringify_simple_type(value: any, key_function_name: string?): string
    if type(value) == "number" then
        -- mlua Numbers are BOLD_BLUE and Integers are regular BLUE
        if mlua.isint(value) then
            return BLUE .. tostring(value) .. RESET
        else
            return BOLD_BLUE .. tostring(value) .. (if math.round(value) == value then ".0" else "") .. RESET
        end
    elseif type(value) == "string" then
        return GREEN .. '"' .. value .. '"' .. RESET
    elseif type(value) == "boolean" then
        return YELLOW .. tostring(value) .. RESET
    elseif type(value) == "vector" then
        return BOLD_RED .. "vector" .. RESET .. LEFT_ANGLED
            .. BOLD_BLUE .. value.x .. CYAN .. ", "
            .. BOLD_BLUE .. value.y .. CYAN .. ", "
            .. BOLD_BLUE .. value.z .. CYAN ..
            (if VECTOR_FOUR_WIDE_MODE then ", " ..
                BOLD_BLUE .. value.w
            else "") 
            .. RED .. RIGHT_ANGLED .. RESET
    elseif type(value) == "function" then
        -- function ids look like 0x562311bd6640, we bold the last 5 digits for ez identification
        local function_id = string.match(tostring(value), ID_CATCHER_PATTERN)
        local last_five = string.sub(function_id, #function_id - 4, # function_id)
        local first_part = string.sub(function_id, 1, #function_id - 5)
        -- apparently we can get the actual function name with debug.info if it was defined in luau or rust/c functions with debugnames
        -- debug.info returns "" not nil if function has no debug info
        local function_name = (debug.info :: any)(value, "n")
        local is_seal_fn_with_signature = false
        if key_function_name and key_function_name == function_name then
            function_name = ""
        elseif string.find(function_name, ".", 1, true) or string.find(function_name, ":", 1, true) then
            -- it's probably a seal standard library function with a full signature, let's format it differently
            is_seal_fn_with_signature = true
        end
        if #function_name > 0 then
            function_name = " " .. function_name
        end
        if not is_seal_fn_with_signature then
            return BOLD_RED .. "function" .. function_name .. RESET .. RED .. LEFT_ANGLED .. RED .. "0x" ..  first_part .. colors.bold.red(last_five) .. RESET .. RED .. RIGHT_ANGLED
        else
            return BOLD_RED .. "function" ..  BLUE .. function_name .. RESET
        end
    elseif type(value) == "buffer" then
        if buffer.len(value) > 512 then
            local buffer_id = string.match(tostring(value), ID_CATCHER_PATTERN)
            local last_five = string.sub(buffer_id, #buffer_id - 4, # buffer_id)
            local first_part = string.sub(buffer_id, 1, #buffer_id - 5)
            return BOLD_MAGENTA .. "buffer" .. RESET .. LEFT_ANGLED .. MAGENTA .. "0x" .. first_part .. colors.bold.magenta(last_five) .. RESET .. MAGENTA .. RIGHT_ANGLED
        else
            return BOLD_MAGENTA .. "buffer" .. RESET .. LEFT_ANGLED .. GREEN .. '"' .. buffer.tostring(value) .. '"' .. RESET .. RESET .. MAGENTA .. RIGHT_ANGLED
        end
    elseif mlua.iserror(value) or typeof(value) == "error" then
        local stringified_error = tostring(value)
        local errsplit = string.split(stringified_error, "\n")
        local errmsglist = {}
        for _, m in errsplit do
            if string.find(m, "stack traceback") then
                break
            else
                table.insert(errmsglist, m)
            end
        end
        return BOLD_RED .. "error" .. RESET .. RED .. LEFT_ANGLED .. table.concat(errmsglist) .. RIGHT_ANGLED
    elseif type(value) == "thread" then
        local thread_id = string.match(tostring(value), ID_CATCHER_PATTERN)
        return "corothread" .. LEFT_ANGLED .. "0x" .. thread_id .. RIGHT_ANGLED
    elseif type(value) == "table" then -- treat a table as a simple value
        local tostringed = tostring(value)
        local table_id = string.match(tostringed, ID_CATCHER_PATTERN)
        if table_id then
            local last_five = string.sub(table_id, #table_id - 4, #table_id)
            local first_part = string.sub(table_id, 1, #table_id - 5)
            return BOLD_WHITE .. "table" .. RESET .. LEFT_ANGLED .. "0x" .. first_part .. BOLD_WHITE .. last_five .. RESET .. RIGHT_ANGLED
        else
            return tostringed
        end
    elseif type(value) == "userdata" then
        local tostringed = tostring(value)
        local userdata_id: string? = string.match(tostringed, ID_CATCHER_PATTERN)
        local typeof_userdata = typeof(value)
        if userdata_id and userdata_id == "0" then -- mluau's json null uses address 0x0000000000000000 which gets caught as "0"
            return colors.bold.red("null")
        elseif typeof_userdata ~= "userdata" then -- userdatas with custom names (__type)
            return colors.bold.magenta(tostringed) -- we most likely have a custom tostring implementation for it
        elseif userdata_id then
            return BOLD_MAGENTA .. "userdata" .. RESET .. LEFT_ANGLED .. "0x" .. userdata_id .. RIGHT_ANGLED
        else
            return colors.bold.magenta(tostringed)
        end
    else
        return tostring(value)
    end
end

local function stringify_table_path(current_table_path: { any }): string
    local table_path = BOLD_YELLOW .. "self" .. RESET
    for _, level in current_table_path do
        if typeof(level) == "string" then
            if string.match(level, IDENTIFIER_PATTERN) then
                table_path ..= "." .. level
            else
                table_path ..= LEFT_BRACKET .. GREEN .. '"' .. level .. '"' .. RESET .. RIGHT_BRACKET
            end
        elseif typeof(level) == "number" then
            table_path ..= LEFT_BRACKET .. BLUE .. level .. RESET .. RIGHT_BRACKET
        else
            table_path ..= LEFT_BRACKET .. stringify_simple_type(level) .. RIGHT_BRACKET
        end
    end
    return table_path
end

type TablePair = {
    key: any,
    val: any,
}

local function sort_table_pairs(unsorted_pairs: { TablePair }): { TablePair }
    local sorted_pairs: { TablePair } = table.create(#unsorted_pairs) :: { TablePair }
    local seen_indices: { [number]: boolean } = {} -- indices seen/visited in unsorted_pairs

    local numerical_pairs: { TablePair } = {} do
        for index, pair in unsorted_pairs do
            if type(pair.key) == "number" then
                seen_indices[index] = true
                table.insert(numerical_pairs, pair)
            end
        end
        table.sort(numerical_pairs, function(a: TablePair, b: TablePair)
            local isint_a = mlua.isint(a.key)
            local isint_b = mlua.isint(b.key)
            if isint_a and isint_b then
                return a.key < b.key
            elseif isint_a and not isint_b then
                return true
            elseif isint_b and not isint_a then
                return false
            else
                return a.key < b.key
            end
        end)
    end

    for _, v in numerical_pairs do
        table.insert(sorted_pairs, v)
    end

    local string_pairs: { TablePair } = {} do
        for index, pair in unsorted_pairs do
            if type(pair.key) == "string" then
                seen_indices[index] = true
                table.insert(string_pairs, pair)
            end
        end
        table.sort(string_pairs, function(a: TablePair, b: TablePair)
            local a_byte, b_byte = string.byte(a.key), string.byte(b.key)
            -- string.byte can return 0 values, which isn't documented in hover docs or return type
            if a_byte and b_byte then
                return a_byte < b_byte
            elseif a_byte and not b_byte then
                return false
            elseif b_byte and not a_byte then
                return true
            else
                return false
            end
        end)
    end

    for _, v in string_pairs do
        table.insert(sorted_pairs, v)
    end

    for index, pair in unsorted_pairs do
        if not seen_indices[index] then
            table.insert(sorted_pairs, pair)
        end
    end

    return sorted_pairs
end

local function indents(guidelines: boolean, depth: number, indent_spaces: number): (string, string)
    local current_indent: string
    local element_indent: string
    if guidelines then
        local parts: { string } = {}
        -- truecolor colors already encode the desired muting; DIM would double-dim them
        local dim = if is_truecolor then "" else DIM
        for d = 0, depth - 1 do
            local color = GUIDE_COLORS[(d % #GUIDE_COLORS) + 1]
            local bar = if d % 2 == 0 then "┆" else "╎"
            table.insert(parts, dim .. color .. bar .. RESET .. string.rep(" ", indent_spaces - 1))
        end
        current_indent = table.concat(parts)
        local next_color = GUIDE_COLORS[(depth % #GUIDE_COLORS) + 1]
        local next_bar = if depth % 2 == 0 then "┆" else "╎"
        element_indent = current_indent .. dim .. next_color .. next_bar .. RESET .. string.rep(" ", indent_spaces - 1)
    else
        local one_indent = string.rep(" ", indent_spaces)
        current_indent = string.rep(one_indent, depth)
        element_indent = current_indent .. one_indent
    end
    return current_indent, element_indent
end

local function process_pretty_values(
    value: any,
    --- seen_tables' keys are tables since they get uniquely hashed, value is stringified path
    seen_tables: { [{ [any]: any }]: string? }?,
    --- recursion depth of table we're processing
    depth: number?,
    --- used to put table paths in seen_tables
    current_table_path: { any }?,

    -- config parameters

    --- how many spaces, defaults to 4
    indent_spaces: number?,
    --- how deep inside a table before we say nope
    max_depth: number?,
    --- how many array elements to print before putting ..., defaults to 100 when not tty or everything
    max_elements_in_array: number?,
    show_array_indices: boolean?,
    show_metatables: boolean?,
    guidelines: boolean?,
    --- show the total item count at the opening brace, defaults to true when not tty
    show_array_length: boolean?
): string
    seen_tables = seen_tables or {}
    depth = depth or 0
    max_depth = if not is_tty then math.huge elseif max_depth then max_depth else 6
    max_elements_in_array = if not is_tty then math.huge elseif max_elements_in_array then max_elements_in_array else 42
    indent_spaces = indent_spaces or 4
    show_array_indices = if show_array_indices == nil or show_array_indices == true then true else false
    show_metatables = if show_metatables == nil or show_metatables == true then true else false
    guidelines = if guidelines ~= nil then guidelines else indent_spaces < 3
    show_array_length = if show_array_length ~= nil then show_array_length else is_tty

    local current_indent, element_indent = indents(guidelines, depth, indent_spaces)
    
    local current_table_path: { any } = current_table_path or {}
    if type(value) == "table" then
        local value = value :: { [any]: any }
        local value_meta = getmetatable(value :: any) :: { [any]: any }?

        -- if we have a metatable and the metatable has custom working __display implementation
        -- we want to call it and return whatever string it returns.
        -- we don't use __tostring for this because a __tostring implementation is not necessarily for displaying
        -- a user can call tostring(value) explicitly if they want that.
        if value_meta and value_meta.__display and typeof(value_meta.__display) == "function" then
            value_meta = value_meta :: {
                -- we want to allow downstream users to be able to customize how their data structure
                -- gets formatted based upon current level of indent and relevant formatting config
                __display: (
                    self: any,
                    depth: number,
                    config: {
                        max_depth: number,
                        indent_spaces: number,
                        max_elements_in_array: number,
                        show_array_indices: boolean,
                        show_metatables: boolean,
                        show_array_length: boolean,
                    }?
                ) -> unknown
            }
            local success, custom_display = pcall(
                value_meta.__display,
                value,
                depth,
                {
                    max_depth = max_depth,
                    indent_spaces = indent_spaces,
                    max_elements_in_array = max_elements_in_array,
                    show_array_indices = show_array_indices,
                    show_metatables = show_metatables,
                    guidelines = guidelines,
                    show_array_length = show_array_length,
                }
            )
            if success and typeof(custom_display) == "string" then
                return custom_display
            end
            -- this fallthrough is intentional if meta.__display() doesn't work or returns an incorrect result
        end

        local result: { string } = {}

        local function push(s: string)
            table.insert(result, s)
        end

        local is_frozen = table.isfrozen(value)

        -- pretty printed tables should be printed in numerical, then alphabetical order (if possible)
        -- built before the opening brace so we can display the item count
        local unsorted_pairs: { TablePair } = {}
        if value_meta then
            for k, v in pairs(value) do -- use pairs to avoid unexpectedly calling __call on tables w/ metatable
                table.insert(unsorted_pairs, { key = k, val = v })
            end
        else
            for k, v in value do
                table.insert(unsorted_pairs, { key = k, val = v })
            end
        end

        local sorted_pairs: { TablePair } = sort_table_pairs(unsorted_pairs)
        if #sorted_pairs == 0 then
            return "{}" -- special case empty tables
        end

        local length_suffix = if show_array_length and #value > 0 then DIM .. ` ({#sorted_pairs})` .. RESET else ""

        push(
            if is_frozen and is_tty then
                "{❄" .. length_suffix .. "\n"
            elseif is_frozen then
                "{<frozen>" .. length_suffix .. "\n"
            else
                "{" .. length_suffix .. "\n"
        )

        if depth == 0 then
            seen_tables[value] = BOLD_YELLOW .. "self" .. RESET -- track top level cyclic tables
        end

        local array_elements_seen = 0
        for _, pair in sorted_pairs do
            local key, val = pair.key, pair.val
            push(element_indent)

            -- stringify keys
            local key_type = type(key)
            if key_type == "number" then
                array_elements_seen += 1
                if array_elements_seen > max_elements_in_array then
                    local omitted = #sorted_pairs - max_elements_in_array
                    push(DIM .. `[{omitted}…{#sorted_pairs}] omitted` .. RESET .. "\n")
                    break
                end
                if show_array_indices ~= false then
                    -- left pad array keys by prepending spaces in front of [
                    -- so the = signs are aligned
                    if #value > 10 and #value < 100 then
                        if key < 10 then
                            push(" ")
                        end
                    elseif #value > 100 and #value < 1000 then
                        if key < 10 then
                            push("  ")
                        elseif key < 100 then
                            push(" ")
                        end
                    elseif #value > 1000 then
                        if key < 10 then
                            push("   ")
                        elseif key < 100 then
                            push("  ")
                        elseif key < 1000 then
                            push(" ")
                        end
                    end
                    push(
                        LEFT_BRACKET ..
                        (if mlua.isint(key) then
                            BLUE .. tostring(key) .. RESET
                        else
                            BOLD_BLUE .. tostring(key) .. RESET
                        ) ..
                        RIGHT_BRACKET
                    )
                end
            elseif key_type == "string" then
                if string.match(key, IDENTIFIER_PATTERN) then
                    push(key)
                else
                    push(LEFT_BRACKET .. GREEN .. '"' .. key .. '"' .. RESET .. RIGHT_BRACKET)
                end
            elseif key_type == "table" then
                local seen_name: string? = seen_tables[key]
                if seen_name then
                    push(LEFT_BRACKET .. seen_name .. RIGHT_BRACKET)
                elseif depth + 1 > max_depth then
                    local tostringed = tostring(key)
                    local tableid = string.match(tostringed, ID_CATCHER_PATTERN) or tostringed
                    push(LEFT_BRACKET .. colors.cyan(`table<0x{tableid}>`) .. RIGHT_BRACKET)
                else
                    push(
                        LEFT_BRACKET .. process_pretty_values(key, seen_tables, depth + 1, current_table_path, indent_spaces, max_depth, max_elements_in_array, show_array_indices, show_metatables, guidelines, show_array_length) .. RIGHT_BRACKET
                    )
                end
            else
                push(
                    LEFT_BRACKET .. process_pretty_values(key, seen_tables, depth + 1, current_table_path, indent_spaces, max_depth, max_elements_in_array, show_array_indices, show_metatables, guidelines, show_array_length) .. RIGHT_BRACKET
                )
            end

            if key_type ~= "number" or show_array_indices ~= false then
                push(" = ")
            end

            -- stringify values
            if type(val) == "table" then
                local val = val :: { [any]: any }
                local seen_name: string? = seen_tables[val]
                if seen_name then
                    push(seen_name)
                elseif depth + 1 > max_depth then
                    if next(val) == nil then -- table is empty
                        push("{}")
                    else
                        push("{" .. colors.codes.BOLD_WHITE .. "..." .. RESET .. "}")
                    end
                else
                    -- push key to current_table_path before recursion
                    table.insert(current_table_path, key)
                    seen_tables[val] = stringify_table_path(current_table_path) -- tostring(key)
                    push(process_pretty_values(val, seen_tables, depth + 1, current_table_path, indent_spaces, max_depth, max_elements_in_array, show_array_indices, show_metatables, guidelines, show_array_length))
                    -- pop key from current_table_path after recursion
                    table.remove(current_table_path, #current_table_path)
                end
            elseif type(val) == "string" then -- don't recurse in extremely common case
                if string.find(val, "\n") then
                    if #val < 42 then
                        push(GREEN .. '"' .. string.gsub(val, "\n", "\\n") .. '"' .. RESET)
                    else
                        push("(newlined string starts on next line) " ..  GREEN .. "\n" .. val .. RESET .. "\n(end newlined string)")
                    end
                else
                    push(GREEN .. '"' .. val .. '"' .. RESET)
                end
            elseif type(val) == "number" or type(val) == "boolean" then
                push(stringify_simple_type(val))
            elseif type(val) == "function" and key_type == "string" then
                -- pass table key in so we can skip displaying the function_name twice if key == debug.info(val)
                push(stringify_simple_type(val, key))
            else
                push(process_pretty_values(val, seen_tables, depth + 1, current_table_path, indent_spaces, max_depth, max_elements_in_array, show_array_indices, show_metatables, guidelines, show_array_length))
            end
            
            push(DIM .. "," .. RESET .. "\n")
        end
        if value_meta and show_metatables then
            push(
                element_indent .. RED .. "@metatable" .. RESET .. " = " .. process_pretty_values(value_meta, seen_tables, depth + 1, current_table_path, indent_spaces, max_depth, max_elements_in_array, show_array_indices, show_metatables, guidelines, show_array_length) .. "\n"
            )
        end
        push(current_indent .. (
            if is_frozen and is_tty then
                "}❄"
            elseif is_frozen then
                "<frozen>}"
            else
                "}"
        ))
        -- return result
        return table.concat(result, "")
    elseif type(value) == "string" and depth == 0 then -- calling print("hello world") shouldn't colorize
        return value
    else
        return stringify_simple_type(value)
    end
end

local function pretty(value: any, ...: any): string
    local now_enabled = colors.enabled()
    if now_enabled ~= colors_enabled_state then
        colors_enabled_state = now_enabled
        apply_color_state(now_enabled)
        LEFT_BRACKET = DIM .. "[" .. RESET
        RIGHT_BRACKET = DIM .. "]" .. RESET
        LEFT_ANGLED = BOLD .. DIM .. "<" .. RESET
        RIGHT_ANGLED = BOLD .. DIM .. ">" .. RESET
    end
    return process_pretty_values(value, ...)
end

return {
    simple = process_raw_values,
    pretty = pretty,
}