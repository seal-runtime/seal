local _types = require("@self/_types")
export type ArchiveOptions = _types.ArchiveOptions
export type ArchiveFormat = _types.ArchiveFormat

--[=[
    Read, write, and extract the Unix `ar` format used for static libraries and as the outer container for `.deb` packages.

    ar is a single flat layer of entries with no directory structure, so an `Archive` written to ar can't contain subfolders.
]=]
export type ar = {
    --[=[
        Directly extract the ar archive at `path` into directory at `destination`, creating
        a new directory if needed.

        This overwrites files in `destination` if those files existed already, but should not
        cause directory issues like overwriting a directory because `ar is a flat archive format that
        doesn't allow directories.
    ]=]
    extract: (path: string, destination: string, options: ArchiveOptions?) -> (),
    --[=[
        Reads the ar archive at `path` into memory as an `Archive`, with the same erroring semantics as `fs.readfile`.

        This may throw an error if one file or the total archive size exceeds the archive lib defaults, pass an
        `ArchiveOptions` to adjust these settings.
    ]=]
    readfile: (path: string, options: ArchiveOptions?) -> Archive,
    --[=[
        Write an `Archive` to `path` as an ar archive, following the same erroring semantics as `fs.writefile`.

        The archive must be completely flat (no directories) because directories aren't supported by
        the ar format.
    ]=]
    writefile: (path: string, archive: Archive, options: ArchiveOptions?) -> (),
    --[=[
        Load an ar archive into memory as an `Archive` from an existing buffer of bytes.

        This may throw an error if one file or the total archive size exceeds the archive lib defaults, pass an
        `ArchiveOptions` to adjust these settings.
    ]=]
    load: (bytes: buffer, options: ArchiveOptions?) -> Archive,
    --[=[
        Create a new empty `Archive`.
    ]=]
    create: () -> Archive,
}

return {} :: ar