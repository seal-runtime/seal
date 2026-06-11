local time = require("@std/time")
local terminal = require("@std/terminal")
local output = require("@std/io/output")

terminal.rawmode.enable()

for event in terminal.events(time.milliseconds(20)) do
    if event.type == "Key" then
        if event.key == "c" and event.modifiers.ctrl then
            print("\r\n")
            break
        end
        if #event.key == 1 then
            output.write(event.key)
        elseif event.key == "Space" then
            output.write(" ")
        elseif event.key == "Enter" then
            output.write("\r\n")
        elseif event.key == "Backspace" then
            output.write("\8 \8")
        elseif event.key == "Left" then
            output.write("\27[D")
        elseif event.key == "Right" then
            output.write("\27[C")
        elseif event.key == "Up" then
            output.write("\27[A")
        elseif event.key == "Down" then
            output.write("\27[B")
        end
    end
end

terminal.rawmode.disable()