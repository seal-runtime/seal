local _types = require("@self/_types")
local ar = require("@self/ar")
local deb = require("@self/deb")
local sevenz = require("@self/sevenz")
local tar = require("@self/tar")
local zip = require("@self/zip")

export type ArchiveOptions = _types.ArchiveOptions
export type ArchiveFormat = _types.ArchiveFormat
export type ContainerFormat = _types.ContainerFormat
export type SingleFileFormat = _types.SingleFileFormat

--[=[
    Extract and create compressed archives (and compressed files) of most common formats.
    This library protects against path/symlink traversal attacks and extraction bombs by default,
    meaning you may need to increase some defaults to extract large movies or high-def FLAC albums.

    We cap *max total archive size* and *max file size* by default:
    - **Max total size:** 4 GB
    - **Max individual file size:** 500 MB

    To adjust these settings, see `ArchiveOptions`.

    This library currently supports read and write for:
    - zip
    - tar (including tar.gz, tar.bz2, tar.xz, tar.zst, tar.lz4)
    - ar
    - deb
    - 7z

    As well as compressed files of: gz, bz2, xz, lz4, zst

    ## Usage

    To extract an archive directly to disk, use the respective library's extract function:

    ```luau
    const tar = require("@std/archive/tar")

    const tar_path = fs.path.join(script:parent(), "credentials.tar.gz")
    tar.gz.extract(tar_path, fs.path.join(script:parent(), "credentials"))
    fs.removefile(tar_path)
    ```

    To read an `Archive` into memory, use the respective library's `load` function or `archive.load`.

    Note that `Archives` are completely format-generic, you can unzip a zip file and resave it as a tar.gz without
    writing to disk first.
]=]
export type archive = {
    zip: zip.zip,
    tar: tar.tar,
    ar: ar.ar,
    deb: deb.deb,
    sevenz: sevenz.sevenz,
    --[=[
        Create a new empty `Archive`.
    ]=]
    create: () -> Archive,
    --[=[
        Load an archive or single compressed file from bytes into memory as an `Archive`.

        Use this when the format isn't known up front, or you want to read compressed files (single files)
        that don't have a dedicated archive library as an archive.

        If you know what format you're going to use up front, prefer using its dedicated library's own `load`
        (`zip.load`, `tar.gz.load`), which doesn't need `format` passed in.

        To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.
    ]=]
    load: (bytes: buffer, format: ArchiveFormat, options: ArchiveOptions?) -> Archive,
}

return {} :: archive