local terminal = require("@std/terminal")
local cursor = require("@std/terminal/cursor")
local time = require("@std/time")

local keyframes = { "⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏" }

local function spinner(message: string, duration_seconds: number)
    local pos = cursor.position()

    -- Hide cursor during animation
    cursor.hide():execute()

    local start = os.clock()
    local frame = 0

    while (os.clock() - start) < duration_seconds do
        frame = (frame % #keyframes) + 1

        -- Move back to start position and redraw frame
        terminal.execute(
            cursor.to(pos.x, pos.y),
            terminal.write(keyframes[frame] .. " " .. message)
        )

        time.wait(0.08)
    end

    -- Clear the spinner and restore cursor
    terminal.execute(
        cursor.to(pos.x, pos.y),
        terminal.write(string.rep(" ", #message + 2)),
        cursor.show(),
        cursor.column(0)
    )
end

if terminal.tty() then
    spinner("Processing", 3)
end
