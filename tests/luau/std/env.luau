local env = require("@std/env")

local function iterate_args()
    for _, arg in env.args do
        print(arg)
    end
end

iterate_args()

local function vars_get()
    local PATH = env.vars.get("PATH")
    print(PATH)
    assert(PATH ~= nil, "unable to get path")
    assert(#PATH > 4, "why path so small?")
end

vars_get()

print(env.vars.all())

-- local function get_env_var()
-- 	local PATH = env.getvar("PATH")
-- assert(PATH ~= nil, "unable to get path")
-- assert(#PATH > 4, "why path so small?")
-- end

-- get_env_var()

-- local function set_env_var()
-- 	env.setvar("PROD", "false")
-- 	assert(env.getvar("PROD") == "false", "issue setting env variable PROD")
-- end

-- set_env_var()

-- local function env_remove_var()
-- 	set_env_var()
-- 	env.removevar("PROD")
-- 	assert(env.getvar("PROD") == nil, "error removing env variable PROD")
-- end

-- env_remove_var()