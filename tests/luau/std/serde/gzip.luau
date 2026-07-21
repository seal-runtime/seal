local gzip = require("@std/serde/gzip")
local filesize = require("@std/fs/filesize")
local fs = require("@std/fs")

-- roundtrip from a string
const text = "hello there, seal!"
const compressed = gzip.compress(text)
assert(buffer.tostring(gzip.decompress(compressed)) == text, "gzip string roundtrip failed")

-- roundtrip from a buffer
const bytes = buffer.fromstring(text)
assert(buffer.tostring(gzip.decompress(gzip.compress(bytes))) == text, "gzip buffer roundtrip failed")

-- gzip magic bytes (1f 8b)
assert(buffer.readu8(compressed, 0) == 0x1f, "gzip output missing 0x1f magic byte")
assert(buffer.readu8(compressed, 1) == 0x8b, "gzip output missing 0x8b magic byte")

-- repetitive data should actually get smaller
const repetitive = string.rep("seal seal seal ", 500)
const squished = gzip.compress(repetitive)
assert(buffer.len(squished) < #repetitive, "gzip didn't compress repetitive data")
assert(buffer.tostring(gzip.decompress(squished)) == repetitive, "gzip roundtrip of repetitive data failed")

-- binary-safe: all byte values, including embedded nulls
const binary = buffer.create(256)
for i = 0, 255 do
    buffer.writeu8(binary, i, i)
end
const binary_out = gzip.decompress(gzip.compress(binary))
assert(buffer.len(binary_out) == 256, "gzip binary roundtrip changed length")
for i = 0, 255 do
    assert(buffer.readu8(binary_out, i) == i, `gzip binary roundtrip corrupted byte {i}`)
end

-- empty input
assert(buffer.len(gzip.decompress(gzip.compress(""))) == 0, "gzip empty roundtrip failed")

-- max_size is respected
assert(
    buffer.tostring(gzip.decompress(compressed, filesize.megabytes(1))) == text,
    "gzip decompress failed with a generous max_size"
)
const too_small = pcall(function()
    return gzip.decompress(squished, filesize.bytes(16))
end)
assert(not too_small, "gzip decompress should error when output exceeds max_size")

-- we can decompress a real .gz file made by the gzip CLI
const fixture = fs.readfile("./tests/data/archives/compressed/hello.txt")
assert(
    buffer.tostring(gzip.decompress(fs.readbytes("./tests/data/archives/compressed/hello.txt.gz"))) == fixture,
    "gzip couldn't decompress a real CLI-compressed .gz file"
)

-- bad input should error, not panic
assert(not pcall(function()
    return gzip.decompress(buffer.fromstring("definitely not gzipped"))
end), "gzip decompress should error on non-gzip data")

assert(not pcall(function()
    return (gzip :: any).compress(nil)
end), "gzip compress should error without content")
