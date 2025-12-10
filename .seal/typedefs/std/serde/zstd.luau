export type zstd = {
    compress: (data: buffer | string, level: number?) -> buffer,
    decompress: (compressed: buffer) -> buffer,
}

return {} :: zstd