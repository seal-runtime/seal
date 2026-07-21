local _types = require("@self/_types")
export type ArchiveOptions = _types.ArchiveOptions
export type ArchiveFormat = _types.ArchiveFormat

export type deb = {
    extract: (path: string, destination: string, options: ArchiveOptions?) -> (),
    readfile: (path: string, options: ArchiveOptions?) -> Archive,
    writefile: (path: string, archive: Archive, options: ArchiveOptions?) -> (),
    load: (bytes: buffer, options: ArchiveOptions?) -> Archive,
    create: () -> Archive,
}

return {} :: deb