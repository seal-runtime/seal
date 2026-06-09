local fs = require("@std/fs")
local env = require("@std/env")
local process = require("@std/process")
local time = require("@std/time")
local cheese = require("@tests/cheese")

local READTOEND_SRC = [[
local input = require("@std/io/input")
local result = input.read()
if result == nil then
    print("NIL")
else
    print(result)
end
]]

local CHILD_SCRIPT_PATH = fs.path.join(fs.path.home(), ".cache", "seal_test_input.read_child.luau")
fs.writefile(CHILD_SCRIPT_PATH, READTOEND_SRC)

local function spawn_child(): process.PipedChild
    return process.spawn {
        program = env.executable_path,
        args = { CHILD_SCRIPT_PATH },
    } :: process.PipedChild
end

local function wait_for(child: process.PipedChild)
    while child:alive() do
        time.wait(0.05)
    end
end

local function basic()
    local child = spawn_child()
    child.stdin:write("hello\nworld\n")
    child.stdin:close()
    wait_for(child)

    local output = child.stdout:read()
    assert(output ~= nil, "input.read: expected output, got nil")
    assert(output:match("hello"), `input.read: expected "hello" in output, got: {output}`)
    assert(output:match("world"), `input.read: expected "world" in output, got: {output}`)
end

local function empty_stdin()
    local child = spawn_child()
    child.stdin:close()
    wait_for(child)

    local stderr = child.stderr:read()
    assert(stderr == nil or stderr == "", `input.read: empty stdin should not error, got stderr: {tostring(stderr)}`)
    local output = child.stdout:read()
    assert(output ~= nil and output:match("NIL"), `input.read: expected nil return for empty stdin, got: {tostring(output)}`)
end

local function preserves_content()
    local expected = "line1\nline2\nline3\n"
    local child = spawn_child()
    child.stdin:write(expected)
    child.stdin:close()
    wait_for(child)

    local output = child.stdout:read()
    assert(output ~= nil, "input.read: expected output, got nil")
    assert(output:match("line1") and output:match("line2") and output:match("line3"),
        `input.read: all lines should appear in output, got: {tostring(output)}`)
end

cheese.retry(basic, 2)
cheese.retry(empty_stdin, 2)
cheese.retry(preserves_content, 2)

fs.file.try_remove(CHILD_SCRIPT_PATH)
