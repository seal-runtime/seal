-- Watches ./.seal/typedefs/std for changes and runs reference.luau

local fs = require("@std/fs")
local env = require("@std/env")
local str = require("@std/str")
local time = require("@std/time")
local process = require("@std/process")

local defs_path = fs.dir.project():join(".seal", "typedefs", "std")

--- minimum time between saves so our cpu threads can take a breather
local debounce = 2 -- seconds
local last_ran_time = os.clock()
local running = false

--- would rather run this with the release version of seal in path or ./target/release/seal
local function find_faster_seal(): string?
    local cwd = fs.dir.project()
    local seal_exe = if env.os == "Windows" then "seal.exe" else "seal"
    local target_release_seal = cwd:join("target", "release", seal_exe)

    if fs.is(target_release_seal) == "File" then
        return target_release_seal
    end

    -- check system path seal
    if process.shell("seal --version").ok then
        return if env.os ~= "Windows" then
            str.splitlines(process.shell("where seal"):unwrap())[1]
        else
            process.run {
                program = "powershell",
                args = { "-NoProfile", "-Command", "(Get-Command seal.exe).Source"}
            }:unwrap()
    end

    return nil
end

local seal_path = find_faster_seal() or env.executable_path

for category, event in fs.watch(defs_path, {
    timeout_ms = 60,
}) do
    if event.is_write then
        if running and (os.clock() - last_ran_time > debounce) then
            running = false
        end
        if running then continue end
        time.wait(0.1) -- wait for files to be closed to prevent race condition
        running = true
        local result = process.run {
            program = seal_path,
            args = { fs.path.join(script:parent(), "reference.luau") }
        }
        last_ran_time = os.clock()
        if env.os == "Linux" then
            local notification_result = process.shell(`notify-send -t 5000 "updated - autorun.luau" "./docs/reference updated from typedefs"`)
            if not notification_result.ok then
                error(notification_result.err)
            end
        end
        print("updated")
    end
end
