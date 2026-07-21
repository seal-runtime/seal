export type ArchiveOptions = {
    max_file_size: FileSize?,
    max_total_size: FileSize?,
    allow_symlinks: boolean?,
    allow_unsafe_path_traversals: boolean?,
}

--[=[
    Multi-file archive formats: containers that hold many entries (files, directories, symlinks).
]=]
export type ContainerFormat =
    | "Zip"
    | "Tar"
    | "Ar"
    | "Deb"
    | "TarGz"
    | "TarBz2"
    | "TarXz"
    | "TarZst"
    | "TarLz4"
    | "7z"

--[=[
    Single-file compression formats: these wrap exactly one file's contents rather than a
    directory tree, so serializing to one of these requires an `Archive` holding a single entry.
]=]
export type SingleFileFormat =
    | "Gz"
    | "Bz2"
    | "Xz"
    | "Lz4"
    | "Zst"

--- Any format an `Archive` can be serialized to; see `ContainerFormat` and `SingleFileFormat`.
export type ArchiveFormat = ContainerFormat | SingleFileFormat

return {}
