local _types = require("@self/_types")
export type ArchiveOptions = _types.ArchiveOptions
export type ArchiveFormat = _types.ArchiveFormat

--[=[
    Read, write, and extract Debian packages, an `ar` archive wrapping `control` and `data` tarballs.

    Like `ar`, the outer deb layer is flat with no directory structure; the `control` and `data`
    tarballs nested inside it are where actual directory trees (e.g. `/usr/bin/...`) live.
]=]
export type deb = {
    --[=[
        Extract the deb package at `path` into a new or existing directory at `destination`.
    ]=]
    extract: (path: string, destination: string, options: ArchiveOptions?) -> (),
    --[=[
        Read the deb package at `path` into memory as an `Archive`.
    ]=]
    readfile: (path: string, options: ArchiveOptions?) -> Archive,
    --[=[
        Write an `Archive` to `path` as a deb package; the archive's top-level entries must be flat (no directories).
    ]=]
    writefile: (path: string, archive: Archive, options: ArchiveOptions?) -> (),
    --[=[
        Load a deb package into memory as an `Archive` from an existing buffer of bytes.
    ]=]
    load: (bytes: buffer, options: ArchiveOptions?) -> Archive,
    --[=[
        Create a new empty `Archive`.
    ]=]
    create: () -> Archive,
}

return {} :: deb