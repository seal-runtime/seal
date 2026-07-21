local xz = require("@std/serde/xz")
local filesize = require("@std/fs/filesize")
local fs = require("@std/fs")

-- roundtrip from a string
const text = "hello there, seal!"
const compressed = xz.compress(text)
assert(buffer.tostring(xz.decompress(compressed)) == text, "xz string roundtrip failed")

-- roundtrip from a buffer
const bytes = buffer.fromstring(text)
assert(buffer.tostring(xz.decompress(xz.compress(bytes))) == text, "xz buffer roundtrip failed")

-- xz magic bytes (fd '7' 'z' 'X' 'Z' 00)
assert(buffer.readstring(compressed, 0, 6) == "\xfd7zXZ\0", "xz output missing xz magic bytes")

-- repetitive data should actually get smaller
const repetitive = string.rep("seal seal seal ", 2000)
const squished = xz.compress(repetitive)
assert(buffer.len(squished) < #repetitive, "xz didn't compress repetitive data")
assert(buffer.tostring(xz.decompress(squished)) == repetitive, "xz roundtrip of repetitive data failed")

-- binary-safe: all byte values, including embedded nulls
const binary = buffer.create(256)
for i = 0, 255 do
    buffer.writeu8(binary, i, i)
end
const binary_out = xz.decompress(xz.compress(binary))
assert(buffer.len(binary_out) == 256, "xz binary roundtrip changed length")
for i = 0, 255 do
    assert(buffer.readu8(binary_out, i) == i, `xz binary roundtrip corrupted byte {i}`)
end

-- empty input
assert(buffer.len(xz.decompress(xz.compress(""))) == 0, "xz empty roundtrip failed")

-- max_size is respected
assert(
    buffer.tostring(xz.decompress(compressed, filesize.megabytes(1))) == text,
    "xz decompress failed with a generous max_size"
)
const too_small = pcall(function()
    return xz.decompress(squished, filesize.bytes(16))
end)
assert(not too_small, "xz decompress should error when output exceeds max_size")

-- we can decompress a real, actually-compressed .xz file made by the xz CLI
const fixture = fs.readfile("./tests/data/archives/compressed/hello.txt")
const fixture_xz = fs.readbytes("./tests/data/archives/compressed/hello.txt.xz")
assert(buffer.len(fixture_xz) < #fixture, "the xz fixture isn't actually compressed")
assert(
    buffer.tostring(xz.decompress(fixture_xz)) == fixture,
    "xz couldn't decompress a real CLI-compressed .xz file"
)

-- bad input should error, not panic
assert(not pcall(function()
    return xz.decompress(buffer.fromstring("definitely not xz compressed"))
end), "xz decompress should error on non-xz data")

assert(not pcall(function()
    return (xz :: any).compress(nil)
end), "xz compress should error without content")
