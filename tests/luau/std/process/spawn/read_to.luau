local process = require("@std/process")
local env = require("@std/env")
local path = require("@std/fs/path")
local str = require("@std/str")
local time = require("@std/time")

local cheese = require("@tests/cheese")

-- Timing assertions need slack: debug builds spawn and initialize child processes noticeably
-- slower, and the measured elapsed includes that variable startup overhead. Assert on a tolerance
-- window rather than an exact string prefix (which used to flake in debug mode).
local function assert_elapsed(elapsed: number, lo: number, hi: number, what: string)
	assert(elapsed >= lo and elapsed <= hi, `{what}: expected elapsed within [{lo}, {hi}]s, got {elapsed}s`)
end

local function spawn_longrunning(): process.PipedChild
	local child = process.spawn {
		program = env.executable_path,
		args = { path.join(script:parent(), "longrunning.luau"), "60", "50" }
	} :: process.PipedChild
	return child
end

local function onstart()
	local child = spawn_longrunning()
	local res = child.stdout:read_to("..", false)
	assert(typeof(res) == "string" and res == "Starting", ":read_to should be Starting here")
	local dot = child.stdout:read_to("\n")
	assert(dot == ".", "Should get the . after Starting.. and before \\n")
end

onstart()

local function readtoend()
	local child = spawn_longrunning()
	-- passing "" search term is equivalent to reading til `ChildProcess` exits
	local result = child.stdout:read_to("")
	assert(
		typeof(result) == "string"
		and str.endswith(result, "Stopping\n"),
		"Stopping\\n isn't the last thing in result after read to end??"
	)
end

readtoend()

local function interruptreadtowithtimeout()
	local child = process.spawn {
		program = env.executable_path,
		args = { path.join(script:parent(), "long_waiter.luau") }
	} :: process.PipedChild
	local start_time = os.clock()
	-- long_waiter prints "meow" but only after ~2 seconds, so if we read with a timeout of 0.15
	-- we shouldn't expect to see "meow", we should see `nil`
	local meowres = child.stdout:read_to("meow", true, 0.15)
	assert(meowres == nil, "meowres should be nil")
	-- should give up near the 0.15s timeout, nowhere near the 2s it takes "meow" to arrive
	assert_elapsed(os.clock() - start_time, 0.1, 0.75, "read_to with 0.15s timeout")
end

cheese.retry(interruptreadtowithtimeout, 3)

local function timeout_accepts_duration()
	local child = process.spawn {
		program = env.executable_path,
		args = { path.join(script:parent(), "long_waiter.luau") }
	} :: process.PipedChild
	local start_time = os.clock()
	-- timeout may be a Duration (from @std/time) in addition to a plain number of seconds
	local meowres = child.stdout:read_to("meow", true, time.milliseconds(150))
	assert(meowres == nil, "meowres should be nil (meow arrives ~2s later)")
	assert_elapsed(os.clock() - start_time, 0.1, 0.75, "read_to with a 150ms Duration timeout")
	child:kill()
end

cheese.retry(timeout_accepts_duration, 3)

local function interruptedreadallowpartial()
	local child = process.spawn {
		program = env.executable_path,
		args = { path.join(script:parent(), "long_waiter.luau") }
	} :: process.PipedChild

	local start_time = os.clock()
	-- long_waiter prints "meow" after ~2 seconds, so if we interrupt with a 0.15 timeout
	-- and allow partial reads, we should get hehe\n instead
	local heheres = child.stdout:read_to("meow", true, 0.15, true)
	assert(heheres == "hehe\n", "heheres should be hehe\\n")
	-- partial read should return at the 0.15s timeout, not wait out the full 2s for "meow"
	assert_elapsed(os.clock() - start_time, 0.1, 0.75, "read_to partial with 0.15s timeout")
	-- now that's been read, we should be able to easily get meowres
	local start_2_time = os.clock()
	local meowres = child.stdout:read_to("meow", true)
	assert(typeof(meowres) == "string" and (meowres:match("meow")), "meowres should be meow now and we shouldn't have the trailing \\n included")
	-- "meow" arrives ~2s after the child starts; ~0.15s was already consumed above, so the resumed
	-- read should land in the neighborhood of 1.85s (with generous slack for child startup jitter).
	assert_elapsed(os.clock() - start_2_time, 1.4, 2.4, "read_to resuming to find \"meow\"")
end

cheese.retry(interruptedreadallowpartial, 3)