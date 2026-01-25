local url = require("@std/serde/url")

local encoded = url.encode("seal is cool lol")
assert(encoded == "seal%20is%20cool%20lol", `encoded should be seal%20is%20cool%20lol, got {encoded}`)

local decoded = url.decode(encoded)
assert(decoded == "seal is cool lol", `decoded should be seal is good lol got: {decoded}`)

local buffy = buffer.create(1)
buffer.writebits(buffy, 0, 8, 122)
buffer.writebits(buffy, 2, 3, 3)

local bincoded = url.binary.encode(buffy)
local bindecoded = url.binary.decode(bincoded)

local roundtrip = buffer.readbits(bindecoded, 0, 8)

assert(roundtrip == 110, `roundtrip should be 110, got {roundtrip}`)