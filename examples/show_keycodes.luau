-- show all pressed keys on linux

local env = require("@std/env")
local process = require("@std/process")

if env.os ~= "Linux" then
    error("not supported")
end

local child = process.spawn {
    program = "libinput debug-events --show-keycodes",
    shell = "sh",
}

-- ensure seal's running in sudo (administrator mode) so we can see keycodes
local err = child.stderr:read(128, 0.25)
if err and err:match("Permission denied") then
    error("must run as sudo")
end

for line in child.stdout:lines() do
    print(line)
end
