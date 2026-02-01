local fs = require("@std/fs")
local extern = require("@interop/extern")
local env = require("@std/env")
local process = require("@std/process")
local time = require("@std/time")

if env.os == "Linux" then
    print("Building extern_example.so")
    process.run {
        program = "cargo",
        args = { "b", "-p", "extern-example" },
        stdio = "Inherit",
    }

    print(fs.listdir("./target/debug"))
    local debugdir = fs.dir("./target/debug")
    assert(debugdir ~= nil, "where ./target/debug??")

    -- plugins shouldn't need lib prefix (those are for compile time linked libs)
    local old_path = debugdir:join("libextern_example")
    fs.move(old_path .. ".so", debugdir:join("extern_example.so"))
    fs.move(old_path .. ".d", debugdir:join("extern_example.d"))

    local relative_path = debugdir:join("extern_example.so")
    
    local simple = extern.load(fs.path.absolutize(relative_path)) :: {
        hi: (string) -> (),
        uuid: {
            new_v4: (mode: "urn" | "simple" | "braced") -> string
        },
        see_duration: (d: Duration) -> (),
    }

    local urn = simple.uuid.new_v4("urn")
    print(urn)
    local function huh()
        local simple_uuid = simple.uuid.new_v4("simple")
        print(simple_uuid)
        local braced = simple.uuid.new_v4("braced")
        print(braced)
        local succ, err = pcall(function()
            return simple.uuid.new_v4("meow" :: any)
        end)
        assert(succ == false, "this should obviously fail")
        -- make sure we get an actual error userdata
        assert(typeof(err) == "error", `error should be made out of error, got: {typeof(err)}`)
    end
    huh()

    simple.see_duration(time.seconds(420))
end
