local lz4 = require("@std/serde/lz4")

local result = lz4.compress("we are the dead short days ago we lived felt dawn and saw the sunset glow")
local dec = lz4.decompress(result, 100)

assert(buffer.tostring(dec) == "we are the dead short days ago we lived felt dawn and saw the sunset glow")