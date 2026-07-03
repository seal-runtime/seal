local fs = require("@std/fs")
local env = require("@std/env")
local process = require("@std/process")
local time = require("@std/time")
local cheese = require("@tests/cheese")

local CACHE_DIR = fs.path.join(fs.path.home(), ".cache")
fs.dir.ensure(CACHE_DIR)

local written_scripts: { string } = {}

-- writes `src` to a uniquely named child script and returns its path
local function make_child(name: string, src: string): string
    local path = fs.path.join(CACHE_DIR, `seal_test_input_read_{name}.luau`)
    fs.writefile(path, src)
    table.insert(written_scripts, path)
    return path
end

local function spawn(path: string, add: { [string]: string }?): process.PipedChild
    return process.spawn {
        program = env.executable_path,
        args = { path },
        env = if add then { add = add } else nil,
    } :: process.PipedChild
end

local function wait_for(child: process.PipedChild)
    while child:alive() do
        time.wait(0.05)
    end
end

-- ── input.read() to EOF ───────────────────────────────────────────────────────

local READTOEND = make_child("readtoend", [[
local input = require("@std/io/input")
local result, more = input.read()
if result == nil then
    print(`NIL more={tostring(more)}`)
else
    print(`{result} more={tostring(more)}`)
end
]])

local function basic()
    local child = spawn(READTOEND)
    child.stdin:write("hello\nworld\n")
    child.stdin:close()
    wait_for(child)

    local output = child.stdout:read()
    assert(output ~= nil, "input.read: expected output, got nil")
    assert(output:match("hello"), `input.read: expected "hello" in output, got: {output}`)
    assert(output:match("world"), `input.read: expected "world" in output, got: {output}`)
    -- reading to EOF means there's nothing more to read
    assert(output:match("more=false"), `input.read: expected more=false after EOF, got: {output}`)
end

local function empty_stdin()
    local child = spawn(READTOEND)
    child.stdin:close()
    wait_for(child)

    local stderr = child.stderr:read()
    assert(stderr == nil or stderr == "", `input.read: empty stdin should not error, got stderr: {tostring(stderr)}`)
    local output = child.stdout:read()
    assert(output ~= nil and output:match("NIL"), `input.read: expected nil return for empty stdin, got: {tostring(output)}`)
    assert(output ~= nil and output:match("more=false"), `input.read: expected more=false for empty stdin, got: {tostring(output)}`)
end

local function preserves_content()
    local expected = "line1\nline2\nline3\n"
    local child = spawn(READTOEND)
    child.stdin:write(expected)
    child.stdin:close()
    wait_for(child)

    local output = child.stdout:read()
    assert(output ~= nil, "input.read: expected output, got nil")
    assert(output:match("line1") and output:match("line2") and output:match("line3"),
        `input.read: all lines should appear in output, got: {tostring(output)}`)
end

-- ── input.read { bytes = ... } ────────────────────────────────────────────────

-- reports the number of bytes read, whether there's more, and the exact data.
local READBYTES = make_child("readbytes", [[
local input = require("@std/io/input")
local filesize = require("@std/fs/filesize")
local vars = require("@std/env/vars")

local limit = tonumber(vars.get("SEAL_TEST_READ_LIMIT") or "5") :: number
local bytes: number | FileSize = if vars.get("SEAL_TEST_READ_FILESIZE") == "1"
    then filesize.bytes(limit)
    else limit

local data, more = input.read({ bytes = bytes })
print(`LEN={#(data or "")} MORE={tostring(more)} DATA={data or "<nil>"}`)
]])

local function reads_up_to_byte_limit()
    local child = spawn(READBYTES, { SEAL_TEST_READ_LIMIT = "5" })
    child.stdin:write("hello world, there's plenty more here\n")
    child.stdin:close()
    wait_for(child)

    local output = child.stdout:read()
    assert(output ~= nil, "input.read(bytes): expected output, got nil")
    assert(output:match("LEN=5"), `input.read(bytes): expected exactly 5 bytes read, got: {output}`)
    assert(output:match("MORE=true"), `input.read(bytes): expected more=true when limit reached, got: {output}`)
    assert(output:match("DATA=hello"), `input.read(bytes): expected "hello", got: {output}`)
end

local function byte_limit_accepts_filesize()
    local child = spawn(READBYTES, { SEAL_TEST_READ_LIMIT = "3", SEAL_TEST_READ_FILESIZE = "1" })
    child.stdin:write("abcdefg\n")
    child.stdin:close()
    wait_for(child)

    local output = child.stdout:read()
    assert(output ~= nil, "input.read(FileSize): expected output, got nil")
    assert(output:match("LEN=3"), `input.read(FileSize): expected exactly 3 bytes read, got: {output}`)
    assert(output:match("MORE=true"), `input.read(FileSize): expected more=true when limit reached, got: {output}`)
    assert(output:match("DATA=abc"), `input.read(FileSize): expected "abc", got: {output}`)
end

local function byte_limit_eof_before_limit()
    -- limit larger than the input: we should hit EOF first and report more=false
    local child = spawn(READBYTES, { SEAL_TEST_READ_LIMIT = "100" })
    child.stdin:write("hi")
    child.stdin:close()
    wait_for(child)

    local output = child.stdout:read()
    assert(output ~= nil, "input.read(bytes eof): expected output, got nil")
    assert(output:match("LEN=2"), `input.read(bytes eof): expected 2 bytes read, got: {output}`)
    assert(output:match("MORE=false"), `input.read(bytes eof): expected more=false when EOF reached before limit, got: {output}`)
end

-- ── input.read { timeout = ... } ──────────────────────────────────────────────

-- reads with a short timeout while stdin stays open; reports what came through.
local READTIMEOUT = make_child("readtimeout", [[
local input = require("@std/io/input")
local time = require("@std/time")
local data, more = input.read({ timeout = time.milliseconds(200) })
print(`LEN={#(data or "")} MORE={tostring(more)} DATA={data or "<nil>"}`)
]])

local function timeout_returns_when_no_input()
    -- stdin left open with nothing written; read should give up after the timeout
    local child = spawn(READTIMEOUT)
    wait_for(child)

    local output = child.stdout:read()
    assert(output ~= nil, "input.read(timeout): expected output, got nil")
    assert(output:match("LEN=0"), `input.read(timeout): expected 0 bytes on timeout, got: {output}`)
    assert(output:match("MORE=true"), `input.read(timeout): expected more=true on timeout, got: {output}`)
    pcall(function()
        child.stdin:close()
        return nil
    end)
end

local function timeout_returns_partial_data()
    -- write some data but keep stdin open; read should return that data then time out
    local child = spawn(READTIMEOUT)
    child.stdin:write("partial")
    wait_for(child)

    local output = child.stdout:read()
    assert(output ~= nil, "input.read(timeout partial): expected output, got nil")
    assert(output:match("DATA=partial"), `input.read(timeout partial): expected "partial", got: {output}`)
    assert(output:match("MORE=true"), `input.read(timeout partial): expected more=true, got: {output}`)
    pcall(function()
        child.stdin:close()
        return nil
    end)
end

local function timeout_reads_to_eof()
    -- if the stream closes before the timeout, we should report more=false
    local child = spawn(READTIMEOUT)
    child.stdin:write("done")
    child.stdin:close()
    wait_for(child)

    local output = child.stdout:read()
    assert(output ~= nil, "input.read(timeout eof): expected output, got nil")
    assert(output:match("DATA=done"), `input.read(timeout eof): expected "done", got: {output}`)
    assert(output:match("MORE=false"), `input.read(timeout eof): expected more=false when stream closes, got: {output}`)
end

cheese.retry(basic, 2)
cheese.retry(empty_stdin, 2)
cheese.retry(preserves_content, 2)
cheese.retry(reads_up_to_byte_limit, 2)
cheese.retry(byte_limit_accepts_filesize, 2)
cheese.retry(byte_limit_eof_before_limit, 2)
cheese.retry(timeout_returns_when_no_input, 3)
cheese.retry(timeout_returns_partial_data, 3)
cheese.retry(timeout_reads_to_eof, 3)

for _, path in written_scripts do
    fs.file.try_remove(path)
end
