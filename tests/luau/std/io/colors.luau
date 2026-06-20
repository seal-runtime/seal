local colors = require("@std/io/colors")
local format = require("@std/io/format")

local function assert_contains(s: string, substr: string, label: string)
    if not string.find(s, substr, 1, true) then
        error(`FAIL [{label}]: expected {format.debug(s)} to contain {format.debug(substr)}`)
    end
    print(colors.bold.green(`PASS [{label}]`))
end

local function assert_eq(a: any, b: any, label: string)
    if a ~= b then
        error(`FAIL [{label}]: {format.debug(a)} ~= {format.debug(b)}`)
    end
    print(colors.bold.green(`PASS [{label}]`))
end

local function assert_no_ansi(s: string, label: string)
    local stripped = format.uncolor(s)
    if s ~= stripped then
        error(`FAIL [{label}]: string still contains ANSI codes: {format.debug(s)}`)
    end
    print(colors.bold.green(`PASS [{label}]`))
end

-- basic color functions wrap text with escape codes and RESET
assert_contains(colors.red("hello"), "hello",       "red() preserves text")
assert_contains(colors.red("hello"), "\27[",        "red() adds escape code")
assert_contains(colors.blue("x"),    "\27[0m",      "blue() appends RESET")
assert_contains(colors.green("y"),   "\27[32m",     "green() uses correct code")

-- bold variants
assert_contains(colors.bold.red("hi"),  "\27[1;31m", "bold.red() uses bold+red code")
assert_contains(colors.bold.blue("hi"), "hi",        "bold.blue() preserves text")

-- style functions always apply regardless of NO_COLOR
assert_contains(colors.style.dim("dim text"),       "\27[2m",  "style.dim() applies DIM")
assert_contains(colors.style.bold("bold text"),     "\27[1m",  "style.bold() applies BOLD")
assert_contains(colors.style.underline("underline"),"\27[4m",  "style.underline() applies UNDERLINE")

-- rgb: with text wraps and resets; without text returns just the code
local rgb_with = colors.rgb(vector.create(100, 150, 200), "hello")
assert_contains(rgb_with, "hello",   "rgb() with text preserves text")
assert_contains(rgb_with, "\27[38;2;100;150;200m", "rgb() with text uses truecolor code")
assert_contains(rgb_with, "\27[0m",  "rgb() with text appends RESET")

local rgb_code_only = colors.rgb(vector.create(100, 150, 200))
assert_contains(rgb_code_only, "\27[38;2;100;150;200m", "rgb() without text returns code")
assert_eq(string.find(rgb_code_only, "\27[0m", 1, true), nil, "rgb() without text has no RESET")

local rgb_empty = colors.rgb(vector.create(100, 150, 200), "")
assert_contains(rgb_empty, "\27[38;2;", "rgb() with empty string returns code only")
assert_eq(string.find(rgb_empty, "\27[0m", 1, true), nil, "rgb() with empty string has no RESET")

-- rgb() validates range
local ok, err = pcall(function() return colors.rgb(vector.create(300, 0, 0)) end)
assert_eq(ok, false, "rgb() errors on out-of-range channel")

-- colors.override / colors.enabled toggle
assert_eq(colors.enabled(), true, "enabled() returns true by default")

colors.override(false)
assert_eq(colors.enabled(), false, "enabled() returns false after override(false)")
assert_no_ansi(colors.red("hello"),  "red() strips color when disabled")
assert_no_ansi(colors.blue("world"), "blue() strips color when disabled")
assert_eq(colors.rgb(vector.create(0, 128, 255), "hi"), "hi", "rgb() returns plain text when disabled")
assert_eq(colors.rgb(vector.create(0, 128, 255)), "",          "rgb() returns empty code when disabled")

-- style functions are NOT affected by override(false) per NO_COLOR spec
assert_contains(colors.style.dim("still dim"),       "\27[2m", "style.dim() ignores override(false)")
assert_contains(colors.style.bold("still bold"),     "\27[1m", "style.bold() ignores override(false)")

colors.override(true)
assert_eq(colors.enabled(), true, "enabled() returns true after override(true)")
assert_contains(colors.red("restored"), "\27[31m", "red() works again after override(true)")

print("\nAll colors tests passed.")
