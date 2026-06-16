local fs = require("@std/fs")
local env = require("@std/env")
local process = require("@std/process")
local str = require("@std/str")

local rust_files = fs.listdir("./src/", true, function(path)
    return str.endswith(path, ".rs")
end)

local count = 0
for _, file in rust_files do
    for _, line in fs.readlines(file) do
        if  -- is a custom error message
            string.match(line, "wrap_err!")
            -- is not a rust comment
            and not str.startswith(str.trimfront(line, " "), "//")
        then
            count += 1
        end
    end
end

print(tostring(count))

local lines = {}
local in_marker = false
for _, line in fs.readlines("./README.md") do
    if string.find(line, "Start-Precommit-Marker-1", 1, true) then
        in_marker = true
    elseif string.find(line, "End-Precommit-Marker-1", 1, true) then
        in_marker = false
    elseif in_marker then
        line = string.gsub(line, "%d+", tostring(count), 1)
    end
    table.insert(lines, line)
end
table.insert(lines, "") -- satisfy MD047 single-trailing-newline
fs.writefile("./README.md", table.concat(lines, "\n"))

-- create a commit hook to automatically run this script every time a commit is made
local hook_path = ".git/hooks/pre-commit"
if not fs.path.exists(hook_path) then
    fs.writefile(hook_path, "#!/bin/sh\nseal examples/handcrafted_error_messages.luau\ngit add README.md\n")
    if env.os ~= "Windows" then
        process.shell(`chmod +x {hook_path}`):unwrap()
    end
    print(`installed pre-commit hook at {hook_path}`)
end