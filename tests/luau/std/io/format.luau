local env = require("@std/env")
local colors = require("@std/io/colors")
local format = require("@std/io/format")

local function meow() end
print({
    meow,
    cats = 2,
})

local t = setmetatable({
    x = 1,
    y = 2,
}, {
    __call = function(self: any, hmm: string): string
        return hmm .. "call"
    end
})

format.defaults({
    indent_spaces = 2,
})

local options: format.FormatOptions = {
    indent_spaces = 2,
    show_metatables = false,
}
local formatted = format.pretty(t)
print(formatted)
local animals = {"cats", "dogs", "seals", "walruses", { hmm = true }}
print(animals)
print(format.pretty(animals, {
    indent_spaces = 6,
    show_array_indices = false,
}))
-- print(t)
-- print(format(t, {

local depth_check = {
    {
        {
            hi = table.freeze(animals)
        }, 
        cats = 4,
        meow,
    }
}
colors.override(true)
print("setting indent_spaces = 2 enables guidelines")
print(depth_check)
print(env)
print("colors disabled at runtime")
colors.override(false)
print({ hi = "math" })

print(meow)


local pretty = format(t)
dp(pretty)
local debuggable = format.debug(t)
print(debuggable)