-- This script is run by seal's git pre-commit hook to keep README.md up to date.
-- It updates version badges and the handcrafted error message count in-place,
-- and installs the hook itself on first run.

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

local seal_version = _RUNTIME.version.display
local luau_version = _LUAU.version.display

local function find_plain(s: string, pattern: string, init: number?): number?
    return (string.find(s, pattern, init or 1, true))
end

local function get_between(line: string, start_tag: string, end_tag: string): string?
    local s = find_plain(line, start_tag)
    if not s then return nil end
    local cs = s + #start_tag
    local e = find_plain(line, end_tag, cs)
    if not e then return nil end
    return string.sub(line, cs, e - 1)
end

local function replace_between(line: string, start_tag: string, end_tag: string, replacement: string): string
    local s = find_plain(line, start_tag)
    if not s then return line end
    local cs = s + #start_tag
    local e = (find_plain(line, end_tag, cs) :: number) - 1
    return string.sub(line, 1, cs - 1) .. replacement .. string.sub(line, e + 1)
end

local lines = {}
for _, line in fs.readlines("./README.md") do
    local m1 = get_between(line, "<!-- Start-Precommit-Marker-1 -->", "<!-- End-Precommit-Marker-1 -->")
    if m1 then
        line = replace_between(line, "<!-- Start-Precommit-Marker-1 -->", "<!-- End-Precommit-Marker-1 -->",
            string.gsub(m1, "%d+", tostring(count), 1)
        )
    end
    line = replace_between(line, "<!-- Start-Precommit-Marker-2 -->", "<!-- End-Precommit-Marker-2 -->", seal_version)
    line = replace_between(line, "<!-- Start-Precommit-Marker-3 -->", "<!-- End-Precommit-Marker-3 -->", luau_version)
    table.insert(lines, line)
end
table.insert(lines, "") -- satisfy MD047 single-trailing-newline
fs.writefile("./README.md", table.concat(lines, "\n"))

-- create a commit hook to automatically run this script every time a commit is made
local hook_path = ".git/hooks/pre-commit"
if not fs.path.exists(hook_path) then
    local SCRIPT_SRC = str.unindent([[
    #!/bin/sh
    if command -v seal > /dev/null 2>&1; then
        SEAL=seal
    elif [ -x "./target/release/seal" ]; then
        SEAL="./target/release/seal"
    elif [ -x "./target/debug/seal" ]; then
        SEAL="./target/debug/seal"
    else
        echo "warning: seal not found in PATH or ./target/, skipping handcrafted error message count update"
        exit 0
    fi
    $SEAL examples/precommit_readme.luau
    git add README.md
    ]], "Spaces", 4)
    
    fs.writefile(hook_path, SCRIPT_SRC)
    if env.os ~= "Windows" then
        process.shell(`chmod +x {hook_path}`):unwrap()
    end
    print(`installed pre-commit hook at {hook_path}`)
end