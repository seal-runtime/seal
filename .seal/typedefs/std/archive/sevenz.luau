local _types = require("@self/_types")
export type ArchiveOptions = _types.ArchiveOptions
export type ArchiveFormat = _types.ArchiveFormat

--[=[
    Read, write, and extract the 7z format.

    7z (LZMA2) usually beats zip and tar.xz on compression ratio, at the cost of being slower
    to compress and less universally supported than zip.

    Uses LZMA2 preset 6, the same default as tar.xz. If you need a different preset, open an issue, make a PR, or contact me.
]=]
export type sevenz = {
    --[=[
        Extract the 7z archive at `path` into a new or existing directory at `destination`.

        This protects against path traversal attacks (unexpectedly writing outside destination directory),
        symlink traversal attacks, and caps archive and individual file sizes to prevent zip bombs.

        To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.
    ]=]
    extract: (path: string, destination: string, options: ArchiveOptions?) -> (),
    --[=[
        Read the 7z archive at `path` into memory as an `Archive`.

        To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.
    ]=]
    readfile: (path: string, options: ArchiveOptions?) -> Archive,
    --[=[
        Write an `Archive` to `path` as a 7z archive.

        To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.
    ]=]
    writefile: (path: string, archive: Archive, options: ArchiveOptions?) -> (),
    --[=[
        Load a 7z archive into memory as an `Archive` from an existing buffer of bytes.
    ]=]
    load: (bytes: buffer, options: ArchiveOptions?) -> Archive,
    --[=[
        Create a new empty `Archive`.
    ]=]
    create: () -> Archive,
}

return {} :: sevenz