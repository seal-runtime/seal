local process = require("@std/process")

local child = process.run {
    program = [[./target/release/seal eval 'print("meow")']],
    stdio = "Inherit",
    shell = true
}