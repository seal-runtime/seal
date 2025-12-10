local fs = require("@std/fs")
local zstd = require("@std/serde/zstd")

local mainrs = fs.readbytes("./src/main.rs")

local compressed = zstd.compress(mainrs)
assert(
    buffer.tostring(zstd.decompress(compressed)) == buffer.tostring(mainrs),
    "buffers don't match"
)
