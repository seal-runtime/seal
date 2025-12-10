local zlib = require("@std/serde/zlib")

local bytes = buffer.create(16)
buffer.writestring(bytes, 0, "hello there")

local compressed = zlib.compress(bytes, "Default")

local buffy = zlib.decompress(compressed)
assert(buffer.tostring(buffy) == "hello there\0\0\0\0\0")