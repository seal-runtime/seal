local time = require("@std/time")

local function timewaits()
    local t0 = os.clock()
    time.wait(1)
    assert(1 - (os.clock() - t0) < 0.001, "we should have 1 ms precision")
end

local function timewaitsduration()
    local t0 = os.clock()
    time.wait(time.milliseconds(15))
    local since = time.seconds(os.clock() - t0)
    assert(since.milliseconds >= 15, "should wait for at least 15 milliseconds")
end

timewaits()

timewaitsduration()