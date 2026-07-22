export type bz2 = {
    compress: (content: string | buffer) -> buffer,
    --[=[
        Decompress bzip2-encoded (.bz2) data up to `max_size` (default 1 GB).
    ]=]
    decompress: (content: buffer, max_size: FileSize?) -> buffer,
}

return {} :: bz2
