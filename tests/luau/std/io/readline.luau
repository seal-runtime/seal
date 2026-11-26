local fs = require("@std/fs")
local env = require("@std/env")
local process = require("@std/process")
local time = require("@std/time")
local cheese = require("@tests/cheese")

local function runsealwithprompt(prompt: string)
    local src = string.gsub([[
    local input = require("@std/io/input")
    local result = input.readline("PROMPT")
    if typeof(result) == "string" then
        print(result)
    else
        print(`{typeof(result)}: {tostring(result)}`)
    end
    ]], "PROMPT", prompt)
    local dir = fs.dir.ensure(fs.path.join(fs.path.home(), "Downloads"))
    local path = dir:join("promptsrc.luau")
    fs.writefile(path, src)
    local child = process.spawn {
        program = env.executable_path,
        args = { path }
    }
    return child, path
end

local function rawlinefallback()
    -- remove the script only after the child exits: on a slow machine the child may not have opened it yet
    local child, path = runsealwithprompt("how are you?: ")
    assert(child.stdout ~= nil and child.stderr ~= nil and child.stdin ~= nil, "everything is piped")

    local poll = time.milliseconds(120)
    while time.wait(poll) and child:alive() do
        local res = child.stdout:read(nil, 0)
        if res and res == "how are you?: " then
            child.stdin:write("good!!\n")
        elseif res and res == "good!!" then
            fs.file.try_remove(path)
            return
        end
        local err = child.stderr:read(nil, 0)
        if err then
            warn(`stderr: {err}`)
        end
    end

    fs.file.try_remove(path)
    print("child died")
    local good = child.stdout:read()
    assert(good and good:match("good!!"), `should be good here? got: {good}`)
end

cheese.retry(rawlinefallback, 4)