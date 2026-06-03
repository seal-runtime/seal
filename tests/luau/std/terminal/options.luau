local process = require("@std/process")
local terminal = require("@std/terminal")
local cursor = require("@std/terminal/cursor")
local time = require("@std/time")

local animals = { "seal", "monkey", "rhino", "dolphin", "crab", "gopher" }

local function draw(selected: number, start_row: number)
    local actions = { cursor.to(0, start_row) }
    for i, animal in animals do
        if i == selected then
            table.insert(actions, terminal.write("> " .. animal))
        else
            table.insert(actions, terminal.write("  " .. animal))
        end
        if i < #animals then
            table.insert(actions, cursor.down(1))
            table.insert(actions, cursor.column(0))
        end
    end
    terminal.execute(table.unpack(actions))
end

if terminal.tty() then
    if not terminal.rawmode.enabled() then
        terminal.rawmode.enable()
    end

    terminal.execute(
        terminal.switch("Alternate"),
        cursor.hide(),
        cursor.to(0, 0),
        terminal.write("Pick an animal:")
    )

    local selected: (string | interrupt)?
    local current_selection = 1
    local start_row = 1

    draw(current_selection, start_row)

    while selected == nil do
        for event in terminal.events(time.milliseconds(50)) do
            local interrupt = terminal.interrupt.check(event)
            if interrupt then
                selected = interrupt
                break
            end

            if event.type == "Key" then
                if event.key == "Up" then
                    current_selection = if current_selection > 1 then current_selection - 1 else #animals
                    draw(current_selection, start_row)
                elseif event.key == "Down" then
                    current_selection = if current_selection < #animals then current_selection + 1 else 1
                    draw(current_selection, start_row)
                elseif event.key == "Enter" then
                    selected = animals[current_selection]
                    break
                end
            end
        end
    end

    if typeof(selected) == "interrupt" then
        terminal.reset()
        warn(`exiting with {selected.code}`)
        process.exit(1)
    end

    terminal.reset()
    print(`You selected: {selected}`)
end
