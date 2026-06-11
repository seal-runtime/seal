local process = require("@std/process")
local str = require("@std/str")
local terminal = require("@std/terminal")
local cursor = require("@std/terminal/cursor")
local time = require("@std/time")

if terminal.tty() then
    if not terminal.rawmode.enabled() then
        terminal.rawmode.enable()
    end

    terminal.execute(
        terminal.switch("Alternate"),
        cursor.to(0, 0),
        cursor.right(2),
        terminal.write("Name:"),
        cursor.left(5),
        cursor.down(1),
        cursor.right(2),
        cursor.style("BlinkingBar")
    )

    local name_result: string | interrupt

    while name_result == nil do
        local characters: { string } = {}
        for event in terminal.events(time.milliseconds(10)) do
            if event.type == "Key" then
                local interrupt = terminal.interrupt.check(event)
                if interrupt then
                    name_result = interrupt
                    break
                end

                if event.key == "Enter" then
                    name_result = table.concat(characters)
                    break
                elseif event.key == "Backspace" then
                    if #characters > 0 then
                        table.remove(characters)
                        terminal.execute(
                            cursor.left(1),
                            terminal.write(" "),
                            cursor.left(1)
                        )
                    end
                else
                    local key = if event.key == "Space" then " " else event.key
                    local char =
                        if event.modifiers.shift then
                            string.upper(key)
                        else
                            string.lower(key)
                    if str.width(key) == 1 then
                        table.insert(characters, char)
                        terminal.write(char):execute()
                    end
                end
            end
        end
    end

    if typeof(name_result) == "interrupt" then
        terminal.reset()
        warn(`exiting with {name_result}`)
        process.exit(1)
    end

    assert(typeof(name_result) == "string", "must be string here")

    terminal.execute(
        terminal.clear("CurrentLine"),
        cursor.left(#name_result),
        terminal.write(`Hello, {name_result}\n`),
        cursor.hide(),
        cursor.nextline(),
        terminal.write("Loading... ")
    )

    local keyframes = { "|", "/", "\\" }

    local start_time = os.clock()
    local current_keyframe = 1
    while os.clock() - start_time < 1 do
        time.wait(1/6)
        terminal.execute(
            terminal.write(keyframes[current_keyframe]),
            cursor.left(1)
        )
        if current_keyframe >= 3 then
            current_keyframe = 1
        else
            current_keyframe += 1
        end
    end

    terminal.reset()
end