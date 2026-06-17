-- A simple interactive task runner: pick a task from a menu and run it.
-- Copy and adapt the `tasks` table for your own project.

local process = require("@std/process")
local prompt = require("@std/io/prompt")
local colors = require("@std/io/colors")

type Task = {
    label: string,
    cmd: string,
}

local tasks: { Task } = {
    { label = "run tests", cmd = "cargo test" },
    { label = "build (debug)", cmd = "cargo build" },
    { label = "build (release)", cmd = "cargo build --release" },
    { label = "check", cmd = "cargo check" },
    { label = "clean", cmd = "cargo clean" },
    { label = "format", cmd = "cargo fmt" },
}

local options = {}
for _, task in tasks do
    table.insert(options, task.label)
end

local picked = prompt.pick("Pick a task to run:", options)
local task = tasks[picked]

print(colors.style.dim(`$ {task.cmd}`))
process.shell(task.cmd):unwrap()
