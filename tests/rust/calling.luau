local fs = require("@std/fs")
local extern = require("@interop/extern")

local relative_path = fs.path.join(".", "target", "debug", "libsimple.so")

local library = extern.load(fs.path.absolutize(relative_path)) :: {
    hello: (name: string) -> string,
}

local result = library.hello("deviaze")
print(result)