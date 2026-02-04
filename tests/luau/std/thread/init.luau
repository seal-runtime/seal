--!nonstrict
local thread = require("@std/thread")
local time = require("@std/time")

assert(thread.parallelism() > 0, "where is thread.parallelism?")

local handle = thread.spawn({
	path = "./conc_1.luau"
})

print("first")

local t = 0
repeat
	time.wait(0.25)
	t += 1
	print("hm")
until t == 20

handle:join()
print("two")