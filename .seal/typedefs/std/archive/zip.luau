local _types = require("@self/_types")

export type zip = {
    extract: (path: string, destination: string, options: ArchiveOptions?) -> (),
    readfile: (path: string, options: ArchiveOptions?) -> Archive,
    writefile: (path: string, archive: Archive, options: ArchiveOptions?) -> (),
    load: (bytes: buffer, options: ArchiveOptions?) -> Archive,
    create: () -> Archive,
}

export type ArchiveOptions = _types.ArchiveOptions
export type ArchiveFormat = _types.ArchiveFormat

return {} :: zip