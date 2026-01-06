local env = require("@std/env")
local fs = require("@std/fs")
local process = require("@std/process")

local path = fs.path.join(script:parent(), "inheriting", "runme.luau")

local result = process.run {
    program = `{env.executable_path} {path}`,
    shell = true
}
assert(result.ok and result.out == "meow", `inherit means spawning process that spawns inherit should output meow, got: {result:unwrap()}`)

local child_path = fs.path.join(script:parent(), "inheriting", "child.luau")
local res2 = process.run {
    program = `{env.executable_path} {child_path}`,
    shell = true,
    env = {
        add = {
            PROD = "very much production"
        },
        remove = { "SHELL" }
    }
}
local lines = string.split(res2:unwrap(), "\n")
assert(lines[1] == "very much production", "new environment variable should be inherited")
assert(lines[2] == "nil", "SHELL should be removed")