--[=[
    lz4 bindings
]=]
export type lz4 = {
    compress: (input: string | buffer) -> buffer,
    decompress: (compressed: string | buffer, expected_size: number) -> buffer,
}

return {} :: lz4