local fs = require("@std/fs")
local env = require("@std/env")
local process = require("@std/process")
local time = require("@std/time")
local cheese = require("@tests/cheese")

local CACHE_DIR = fs.path.join(fs.path.home(), ".cache")
fs.dir.ensure(CACHE_DIR)

local written_scripts: { string } = {}
local function make_child(name: string, src: string): string
	local path = fs.path.join(CACHE_DIR, `seal_test_stdin_write_{name}.luau`)
	fs.writefile(path, src)
	table.insert(written_scripts, path)
	return path
end

local function spawn(path: string): process.PipedChild
	return process.spawn {
		program = env.executable_path,
		args = { path },
	} :: process.PipedChild
end

local function wait_for(child: process.PipedChild)
	while child:alive() do
		time.wait(0.05)
	end
end

-- reads two lines from stdin and echoes them back joined, proving both writes arrived
local ECHO_TWO = make_child("echo_two", [[
local input = require("@std/io/input")
local a = input.rawline()
local b = input.rawline()
print(`GOT:{a}|{b}`)
]])

-- stays alive long enough for us to kill it
local SLEEPER = make_child("sleeper", [[
require("@std/time").wait(30)
]])

-- stdin should be writable more than once (regression: write used to close the pipe after one call)
local function multiple_writes_are_delivered()
	local child = spawn(ECHO_TWO)
	child.stdin:write("hello\n")
	child.stdin:write("world\n")
	child.stdin:close()
	wait_for(child)

	local output = child.stdout:read()
	assert(output ~= nil, "stdin multi-write: expected output, got nil")
	assert(output:match("GOT:hello|world"), `stdin multi-write: expected both lines, got: {output}`)
end

-- killing a child should release its stdin fd promptly (Luau has no __gc), so writing after errors
local function kill_releases_stdin()
	local child = spawn(SLEEPER)
	child.stdin:write("still open\n")
	child:kill()
	local ok = pcall(function()
		child.stdin:write("after kill\n")
		return nil
	end)
	assert(not ok, "stdin kill release: writing after kill should error (fd released)")
end

-- a child that exits on its own should release stdin once alive() observes the exit, even if we
-- never call close() ourselves (ECHO_TWO exits after two lines, so no EOF is needed)
local function exit_releases_stdin()
	local child = spawn(ECHO_TWO)
	child.stdin:write("a\n")
	child.stdin:write("b\n")
	wait_for(child) -- alive() returns false here, which drops stdin
	local ok = pcall(function()
		child.stdin:write("c\n")
		return nil
	end)
	assert(not ok, "stdin exit release: writing after exit should error (fd released)")
end

cheese.retry(multiple_writes_are_delivered, 2)
cheese.retry(kill_releases_stdin, 2)
cheese.retry(exit_releases_stdin, 2)

for _, path in written_scripts do
	fs.file.try_remove(path)
end
