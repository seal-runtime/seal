local env = require("@std/env")
local process = require("@std/process")

local child = process.run {
    program = (
        if env.os == "Windows" then
            [[.\target\release\seal.exe eval 'print("meow")']]
        else
            [[./target/release/seal eval 'print("meow")']]
    ),
    stdio = "Inherit",
    shell = true
}