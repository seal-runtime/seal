local _types = require("@self/_types")
export type ArchiveOptions = _types.ArchiveOptions
export type ArchiveFormat = _types.ArchiveFormat

--[=[
    Read, write, and extract Debian packages.
    
    Internally, debs are just an `ar` archive wrapping `control` and `data` tarballs.

    Like `ar`, the outer deb layer is flat with no directory structure; the `control` and `data`
    tarballs nested inside it are where actual directory trees (e.g. `/usr/bin/...`) live.
]=]
export type deb = {
    --[=[
        Directly extract the deb package into a directory at `destination`, creating
        a new directory if needed. This function has the same error throwing semantics as
        fs.writefile.

        This overwrites files in `destination` if those files existed already, but should not
        cause directory issues like overwriting a directory because `ar is a flat archive format that
        doesn't allow directories.
    ]=]
    extract: (path: string, destination: string, options: ArchiveOptions?) -> (),
    --[=[
        Reads the deb package at `path` into memory as an `Archive`, with the same erroring semantics as `fs.readfile`.

        This may throw an error if one file or the total archive size exceeds the archive lib defaults, pass an
        `ArchiveOptions` to adjust these settings.
    ]=]
    readfile: (path: string, options: ArchiveOptions?) -> Archive,
    --[=[
        Write an `Archive` to `path` as a deb package.
        
        Keep in mind the archive's top-level entries must be flat (no directories).
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