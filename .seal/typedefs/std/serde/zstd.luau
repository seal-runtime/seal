export type ZstdOptions = {
    checksum: boolean?,
    content_size: boolean?,
    dictid: boolean?,
    window_log: number?,
}

--[=[
    Use `Zstd` compression.
]=]
export type zstd = {
    --[=[
        By default, `zstd.compress` tries to match the Python implementation for wider compatibility, setting the options to:

        ```luau
        local level = 0 -- which defaults to Zstd encoding level 3.
        local zstd_options: zstd.ZstdOptions = {
            checksum = false,
            content_size = true, -- setting pledged_src size to input data's length
            dictid = false,
            window_log = nil,
        }
        zstd.compress(data, level, zstd_options)
        ```
    ]=]
    compress: (data: buffer | string, level: number?, options: ZstdOptions?) -> buffer,
    decompress: (compressed: buffer) -> buffer,
}

return {} :: zstd