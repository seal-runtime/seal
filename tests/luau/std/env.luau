local env = require("@std/env")
local _vars = require("@std/env/vars")

local function iterate_args()
    for _, arg in env.args do
        print(arg)
    end
end

iterate_args()

local function getter()
    local PATH = env.vars.get("PATH")
    assert(PATH ~= nil, "unable to get path")
    assert(#PATH > 4, "why path so small?")
end

getter()

local function setting()
    env.vars.set("CATS", "2")
    assert(env.vars.get("CATS") == "2", "cats should be \"2\"")
end

setting()

local function vally()
    local cats = env.vars.validate("CATS", function(s)
        return tonumber(s)
    end)
    assert(cats and cats == 2, "cats should be a number and 2")
end

vally()

local function loading()
    local fs = require("@std/fs")
    local loaded_path = fs.path.join(script:parent(), "loaded.env")
    fs.writefile(loaded_path, [[
# temporary env file
THIS_SHOULD_WORK=TRUE
SEAL_IS_COOL=YESITIS
]])
    env.vars.load(loaded_path)
    assert(env.vars.get("SEAL_IS_COOL") == "YESITIS", "SEAL_IS_COOL should've loaded from loaded.env")
    fs.file.try_remove(loaded_path)
end

loading()

local function all()
    local current = env.vars.all()
    assert(current["THIS_SHOULD_WORK"] == "TRUE", "env.vars.all should get from loaded.env")
end

all()

local function rem()
    env.vars.unset("CATS")
    assert(env.vars.get("CATS") == nil, "CATS should be unset")
end

rem()
