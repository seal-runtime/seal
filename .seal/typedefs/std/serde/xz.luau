export type xz = {
    compress: (content: string | buffer) -> buffer,
    --[=[
        Decompress xz-encoded (.xz) data up to `max_size` (default 1 GB).
    ]=]
    decompress: (content: buffer, max_size: FileSize?) -> buffer,
}

return {} :: xz
