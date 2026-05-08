local luau = require("@std/luau")

local function returnstable()
    local src = [[
    return {
        seal_version = "0.0.6-rc.1"
    }
    ]]
    local content = luau.eval(src)
    assert(typeof(content) ~= "error", "seal_version table shouldn't be error, should've worked")
    assert(typeof(content) == "table", "seal_version table should be table")
    assert((content :: any).seal_version == "0.0.6-rc.1", "should be 0.0.6-rc.1")
end
returnstable()

local function errorsshouldntactuallyerror()
    local bad_code = [[
    local stringy = "hi"
    table.insert(stringy, "tfatienftoieifwtf")
    ]]
    local res = luau.eval(bad_code)
    assert(typeof(res) == "error", "bad_code should error")
    assert(string.match(tostring(res), "invalid argument #1 to 'insert'"), "should invalid arg")
end
errorsshouldntactuallyerror()

local function printshouldnotbeavailableinnone()
    local src = [[print("happiness")]]
    local res = luau.eval(src, { stdlib = "None" })
    assert(tostring(res):match("nil"), "print should be attempt to call nil")
end
printshouldnotbeavailableinnone()

local function seallibs()
    local src = [[
    return function(s: string)
        local str = require("@std/str")
        return str.startswith(s, "huh")
    end
    ]]
    local f = luau.eval(src, { stdlib = "Seal" }) :: (s: string) -> boolean
    assert(typeof(f) == "function", "f should be function in seallibs")
    assert(f("huhmm") == true, "huhmm starts with huh")
    assert(f("hmm") == false, "hmm does not start with huh")
end
seallibs()

local function bytecodes()
    local src = [[local s = 2; local s = pcall(function() return string.meow() end); return s]]
    local byc = luau.bytecode(src)
    assert(typeof(byc) == "buffer", "byc should be buffer")
    local s, f = pcall(function(): nil
        luau.eval(buffer.tostring(byc))
        return nil
    end)
    assert(s == false and tostring(f):match("UNSAFE"), "should display big warning")
    local res = luau.eval_unsafe(byc)
    assert(res == false, "res should be false because string.meow() will error")
end
bytecodes()

local function customenv()
    local src = [[return meow() .. debug.info(1, "s")]]
    local function meow()
        return "hi i am a cat my name is nanuk"
    end
    local result = luau.eval(src, {
        name = "meower",
        stdlib = "Safe",
        globals = {
            meow = meow,
        }
    })
    assert(result == `hi i am a cat my name is nanuk[string "meower"]`, "should be able to load custom function")
end

customenv()

--- this test ensures that standard libary globals (like table.insert) are correctly
--- merged with custom globals (first() and second())
local function globals_are_merged()
    local src = [[
        local cats = {}
        table.insert(cats, first())
        table.insert(cats, second())
        return cats
    ]]
    local result = luau.eval(src, {
        name = "globals are merged",
        stdlib = "safe" :: "Safe", -- intentional backwards compat test
        globals = {
            first = function()
                return "this is the first cat"
            end,
            second = function()
                return "this is the second cat"
            end
        }
    })
    assert(typeof(result) == "table", "merged result should be a table")
    result = result :: { string }
    assert(result[1] == "this is the first cat", "first element in result table should be 'this is the first cat'")
    assert(result[2] == "this is the second cat", "second element in result table should be 'this is the second cat'")
end

globals_are_merged()

local function cant_pass_wacky_globals()
    local s, f = pcall(function()
        local now = require("@std/time/datetime").now()
        return luau.eval([[print("meow")]], {
            globals = {
                [now :: any] = now
            }
        })
    end)
    assert(s == false, "should not succeed when key in globals is DateTime userdata instead of string")
    assert(tostring(f):match("globals environment table should only have string keys"), "error mentions passed globals table including non-string keys")
end

cant_pass_wacky_globals()