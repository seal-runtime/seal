local _types = require("@self/_types")

export type zip = {
    extract: (path: string, destination: string, options: ArchiveOptions) -> (),
    read: (path: string, options: ArchiveOptions) -> Archive,
    load: (bytes: buffer, options: ArchiveOptions) -> Archive,
}

export type ArchiveOptions = _types.ArchiveOptions
