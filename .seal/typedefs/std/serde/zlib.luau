--[=[
    Encode/decode with the zlib compression algorithm provided by the flake2 crate.
]=]
export type zlib = {
    --[=[
        Encodes (or compresses) the input data with zlib; "Fast" optimizes for compression speed
        whereas "Best" optimizes for compressed size.
    ]=]
    compress: (data: string | buffer, level: "Default" | "Fast" | "Best"?) -> buffer,
    decompress: (compressed: buffer) -> buffer,
}

return {} :: zlib