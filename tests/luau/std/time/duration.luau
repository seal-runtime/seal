local time = require("@std/time")

local function daymath()
    local days = time.days(10) - time.days(40)
    assert(days.days == -30, "we should be -30 days now")
end

daymath()

local function minusing()
    local d = time.days(10) - time.seconds(200)
    assert(d.seconds == 863800, "should be 863800 seconds here")
end

minusing()

local function durationmadeofduration()
    local five_seconds = time.seconds(5)
    assert(typeof(five_seconds) == "Duration", "should be Duration")
end

durationmadeofduration()

local function mul()
    local ten_seconds = time.seconds(10)
    assert((ten_seconds * time.seconds(2)).seconds == 20, "Duration * Duration incorrect")
    assert((ten_seconds * 3).seconds == 30, "Duration * integer incorrect")
    assert((ten_seconds * 1.5).seconds == 15, "Duration * float incorrect")
end

mul()

local function div()
    local ten_seconds = time.seconds(10)
    assert((ten_seconds / time.seconds(2)).seconds == 5, "Duration / Duration incorrect")
    assert((ten_seconds / 2).seconds == 5, "Duration / integer incorrect")
    assert((ten_seconds / 2.5).seconds == 4, "Duration / float incorrect")
end

div()

local function mul_div_errors()
    local ten_seconds = time.seconds(10)
    local s1, res1 = pcall(function()
        return ten_seconds / 0
    end)
    assert(s1 == false, "dividing by zero should error")
    assert(tostring(res1):match("cannot divide by zero"), "divide by zero error message incorrect")

    local s2, res2 = pcall(function()
        return ten_seconds / time.seconds(0)
    end)
    assert(s2 == false, "dividing by a zero Duration should error")
    assert(tostring(res2):match("cannot divide by a zero Duration"), "divide by zero Duration error message incorrect")
end

mul_div_errors()