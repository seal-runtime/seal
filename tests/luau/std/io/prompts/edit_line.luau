local input = require("@std/io/input")
local prompt = require("@std/io/prompt")

local we_are = input.editline("", "We are the ")
assert(we_are == "We are the dead", "we are the dead?")

local middle = input.editline("put lived:", "short days ago we ", ", felt dawn")
assert(middle == "short days ago we lived, felt dawn", "should type 'lived'")

local resp = prompt.edit("What's your name?", "My name is", ".")
assert(resp == "My name is deviaze.", "put deviaze lol")