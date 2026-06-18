-- Two-column interactive picker on the 'Alternate' screen.
-- Left column selects a category; right column selects an option within it.
-- Arrow keys navigate, Left/Right switches columns, Enter (in right column) confirms.

local process = require("@std/process")
local terminal = require("@std/terminal")
local cursor = require("@std/terminal/cursor")
local colors = require("@std/io/colors")
local format = require("@std/io/format")
local str = require("@std/str")
local time = require("@std/time")

export type ColumnItem = {
    group: string,
    options: { string },
}

export type Selection = {
    group: string,
    option: string,
}

local MARGIN = 2
local CONTENT_ROW = 2

--[=[
    Prompt the user to pick from a two-column layout of Group/Option, where each Group
    can contain multiple options. Users can use arrow keys and Enter to switch between
    columns and options and pick an option. Returns a `Selection` table containing
    the Group name and Option picked.
]=]
local function pick_double_columns(message: string, items: { ColumnItem }): Selection
    if not terminal.tty() then
        error("pick_double_columns requires a TTY terminal")
    end

    terminal.rawmode.enable() -- so we can listen for raw arrow keys/keystrokes
    terminal.capture.mouse(true) -- so we can listen for mouse scroll
    terminal.execute(
        terminal.switch("Alternate"), -- prevent erasing existing terminal contents
        terminal.linewrap(false),
        cursor.hide()
    )

    local size = terminal.size()

    local function divider_col(): number
        return MARGIN + math.floor((size.x - MARGIN) * 0.33)
    end

    local function right_col(): number
        return divider_col() + 2
    end

    --- always make sure to handle unicode non ascii graphemes properly
    local function display_width(text: string): number
        return str.width(format.uncolor(text))
    end

    --- Trims `text` to fit within `max_width` display columns, appending `…` if truncated.
    local function truncate(text: string, max_width: number): string
        if display_width(text) <= max_width then return text end
        local result = ""
        for _, grapheme in str.split(text) do
            if display_width(result .. grapheme) > max_width - 1 then break end
            result ..= grapheme
        end
        return result .. "…"
    end

    local active_column = "left"
    local left_selected = 1
    local right_selected = 1

    local function draw()
        local divider_x = divider_col()
        local right_x = right_col()
        local left_col_max_width = divider_x - MARGIN - 2
        local right_col_max_width = size.x - right_x - 2

        local actions: { TerminalAction } = {
            cursor.to(0, 0),
            terminal.clear("All"),
            cursor.to(MARGIN, 0),
            terminal.write(message),
            cursor.to(MARGIN, 1),
            terminal.write(string.rep("─", divider_x - MARGIN) .. "┬" .. string.rep("─", size.x - right_x - 1)),
        }

        local function push(...: TerminalAction)
            for _, action in { ... } do
                table.insert(actions, action)
            end
        end

        for index, item in items do
            local row = CONTENT_ROW + index - 1
            if row >= size.y then break end
            local group = truncate(item.group, left_col_max_width)
            local line: string
            if index == left_selected then
                local highlighted = "> " .. group
                line = if active_column == "left" then colors.bold.green(highlighted) else colors.style.dim(highlighted)
            else
                line = "  " .. group
            end
            push(
                cursor.to(MARGIN, row),
                terminal.write(line)
            )
        end

        for row_offset = 0, size.y - CONTENT_ROW - 1 do
            push(
                cursor.to(divider_x, CONTENT_ROW + row_offset),
                terminal.write("│")
            )
        end

        local current_options = items[left_selected].options
        for index, option in current_options do
            local row = CONTENT_ROW + index - 1
            if row >= size.y then break end
            local truncated_option = truncate(option, right_col_max_width)
            local line: string
            if index == right_selected then
                local highlighted = "> " .. truncated_option
                line = if active_column == "right" then colors.bold.green(highlighted) else colors.style.dim(highlighted)
            else
                line = "  " .. truncated_option
            end
            push(
                cursor.to(right_x, row),
                terminal.write(line)
            )
        end

        terminal.execute(table.unpack(actions))
    end

    draw()

    local result: Selection

    while result == nil do
        -- setting a consistent poll duration allows for a 'heartbeat'
        -- so your program keeps running even when no events are reported
        for event in terminal.events(time.milliseconds(50)) do
            if terminal.interrupt.check(event) then
                terminal.reset()
                process.exit(1)
            end

            if event.type == "Key" and (event.kind == "Press" or event.kind == "Repeat") then
                if event.key == "Esc" then
                    terminal.reset()
                    process.exit(0)
                elseif active_column == "left" then
                    if event.key == "Up" and left_selected > 1 then
                        left_selected -= 1
                        right_selected = 1
                        draw()
                    elseif event.key == "Down" and left_selected < #items then
                        left_selected += 1
                        right_selected = 1
                        draw()
                    elseif event.key == "Right" or event.key == "Enter" then
                        active_column = "right"
                        draw()
                    end
                else
                    local current_options = items[left_selected].options
                    if event.key == "Up" and right_selected > 1 then
                        right_selected -= 1
                        draw()
                    elseif event.key == "Down" and right_selected < #current_options then
                        right_selected += 1
                        draw()
                    elseif event.key == "Left" then
                        active_column = "left"
                        draw()
                    elseif event.key == "Enter" then
                        result = {
                            group = items[left_selected].group, 
                            option = current_options[right_selected]
                        }
                        break
                    end
                end
            elseif event.type == "Mouse" then
                if event.kind == "ScrollUp" then
                    if active_column == "left" and left_selected > 1 then
                        left_selected -= 1
                        right_selected = 1
                        draw()
                    elseif active_column == "right" and right_selected > 1 then
                        right_selected -= 1
                        draw()
                    end
                elseif event.kind == "ScrollDown" then
                    if active_column == "left" and left_selected < #items then
                        left_selected += 1
                        right_selected = 1
                        draw()
                    elseif active_column == "right" and right_selected < #items[left_selected].options then
                        right_selected += 1
                        draw()
                    end
                end
            elseif event.type == "Resize" then
                size = event.size
                draw()
            end
        end
    end

    terminal.reset()
    return result
end

local response = pick_double_columns("Pick a Linux distro:", {
    {
        group = "Arch",
        options = { "Arch Linux", "Manjaro", "EndeavourOS", "Garuda" },
    },
    {
        group = "Debian",
        options = { "Debian", "Ubuntu", "Linux Mint", "Pop!_OS", "Kali Linux" },
    },
    {
        group = "Fedora",
        options = { "Fedora Workstation", "Nobara", "Bazzite", "Ultramarine" },
    },
    {
        group = "openSUSE",
        options = { "openSUSE Tumbleweed", "openSUSE Leap", "Aeon" },
    },
    {
        group = "Gentoo",
        options = { "Gentoo", "Funtoo", "Calculate Linux" },
    },
    {
        group = "NixOS",
        options = { "NixOS", "Guix" },
    },
})
print(`Selected: {response.group}/{response.option}`)

-- return {
--     pick_double_columns = pick_double_columns,
-- }
