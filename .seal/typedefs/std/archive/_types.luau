--[=[
    Overrides for path safety and archive bomb limits.

    The default limits are kinda low, be sure to increase them if you know
    you're going to be extracting large files.

    - **Max total size:** 4 GB
    - **Max individual file size:** 500 MB
    - **allow_unsafe_path_traversals:** false, obviously
    - **allow_symlinks:** also false by default
]=]
export type ArchiveOptions = {
    max_file_size: FileSize?,
    max_total_size: FileSize?,
    allow_symlinks: boolean?,
    allow_unsafe_path_traversals: boolean?,
}

--[=[
    These hold multiple files, but only some (the ones that aren't Ar and Deb)
    can also hold directories. Can also hold symlinks, but symlinks are weird.
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
    Compressed file formats that hold a single file.

    We treat these as `Archive`s internally because they're handled by the same crate,
    so you can read these as an archive of just 1 file.
]=]
export type SingleFileFormat =
    | "Gz"
    | "Bz2"
    | "Xz"
    | "Lz4"
    | "Zst"

export type ArchiveFormat = ContainerFormat | SingleFileFormat

return {}
