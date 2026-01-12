local env = require("@std/env")
local process = require("@std/process")

if env.os == "Windows" then
    local child = process.run {
        program = env.executable_path,
        args = { "eval", [[print("meow")]] },
        stdio = "Inherit",
    }
else
    local child = process.run {
        program = [[./target/release/seal eval 'print("meow")']],
        stdio = "Inherit",
        shell = true,
    }
end
