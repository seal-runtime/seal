local terminal = require("@std/terminal")
local time = require("@std/time")

local contents = ""

if terminal.tty() then
    terminal.rawmode.enable()
    terminal.capture.paste(true)

    for event in terminal.events(time.milliseconds(100)) do
        if event.type == "Empty" then
            continue
        end

        if event.type == "Key" then
            if event.modifiers.ctrl and event.key == "c" then
                break
            end
        end

        if event.type == "Paste" then
            -- print(event.contents)
            contents = event.contents
            break
        end
    end

    terminal.capture.paste(false)
    terminal.rawmode.disable()
    print(`got contents:\n{contents}`)
end


