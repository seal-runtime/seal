local process = require("@std/process")

-- old/current behavior
local child = process.spawn {
    program = [[seal eval 'print("meow")']],
    stdio = "Inherit"
} :: process.PipedChild

local options = { out = "" }

