local fs = require("@std/fs")
local extern = require("@interop/extern")
local env = require("@std/env")
local process = require("@std/process")

if env.os == "Linux" then
    process.run {
        program = "cargo",
        args = { "b", "-p", "simple" },
        stdio = "Inherit",
    }

    local relative_path = fs.path.join(".", "target", "debug", "libsimple.so")
    
    local library = extern.load(fs.path.absolutize(relative_path)) :: {
        hi: (string) -> (),
        uuid: {
            new_v4: (mode: "urn" | "simple" | "braced") -> string
        }
    }

    local urn = library.uuid.new_v4("urn")
    print(urn)
    local function huh()
        local simple = library.uuid.new_v4("simple")
        print(simple)
        local braced = library.uuid.new_v4("braced")
        print(braced)
    end
    huh()
end
