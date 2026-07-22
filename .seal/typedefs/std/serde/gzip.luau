export type gzip = {
    compress: (content: string | buffer) -> buffer,
    --[=[
        Decompress gzip-encoded (.gz) data up to `max_size` (default 1 GB).
    ]=]
    decompress: (content: buffer, max_size: FileSize?) -> buffer,
}
return {} :: gzip
