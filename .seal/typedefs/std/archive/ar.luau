local _types = require("@self/_types")
export type ArchiveOptions = _types.ArchiveOptions
export type ArchiveFormat = _types.ArchiveFormat

--[=[
    Read, write, and extract the Unix `ar` format used for static libraries and as the outer container for `.deb` packages.

    ar is a single flat layer of entries with no directory structure, so an `Archive` written to ar can't contain subfolders.
]=]
export type ar = {
    --[=[
        Extract the ar archive at `path` into a new or existing directory at `destination`.
    ]=]
    extract: (path: string, destination: string, options: ArchiveOptions?) -> (),
    --[=[
        Read the ar archive at `path` into memory as an `Archive`.
    ]=]
    readfile: (path: string, options: ArchiveOptions?) -> Archive,
    --[=[
        Write an `Archive` to `path` as an ar archive; the archive's entries must be flat (no directories).
    ]=]
    writefile: (path: string, archive: Archive, options: ArchiveOptions?) -> (),
    --[=[
        Load an ar archive into memory as an `Archive` from an existing buffer of bytes.
    ]=]
    load: (bytes: buffer, options: ArchiveOptions?) -> Archive,
    --[=[
        Create a new empty `Archive`.
    ]=]
    create: () -> Archive,
}

return {} :: ar