local _types = require("@self/_types")

--[=[
    Read, write, and extract the ubiquitous DEFLATE-based zip format.

    Uses DEFLATE's default compression level (6). If you need a different level, open an issue, make a PR, or contact me.
]=]
export type zip = {
    --[=[
        Extract the zip file at `path` into a new or existing directory at `destination`.

        This protects against path traversal attacks (unexpectedly writing outside destination directory),
        symlink traversal attacks, and caps archive and individual file sizes to prevent zip bombs.

        To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.
    ]=]
    extract: (path: string, destination: string, options: ArchiveOptions?) -> (),
    --[=[
        Reads the zip file at `path` into memory as an `Archive`.

        To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.
    ]=]
    readfile: (path: string, options: ArchiveOptions?) -> Archive,
    --[=[
        Writes an archive to `path`.

        To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.
    ]=]
    writefile: (path: string, archive: Archive, options: ArchiveOptions?) -> (),
    --[=[
        Load a zip file into memory as an `Archive` from an existing buffer of bytes.
    ]=]
    load: (bytes: buffer, options: ArchiveOptions?) -> Archive,
    --[=[
        Create a new empty `Archive`.
    ]=]
    create: () -> Archive,
}

export type ArchiveOptions = _types.ArchiveOptions
export type ArchiveFormat = _types.ArchiveFormat

return {} :: zip