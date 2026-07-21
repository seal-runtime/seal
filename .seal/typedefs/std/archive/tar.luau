local _types = require("./_types")

export type tar = {
    gz: {
        extract: (path: string, destination: string, options: ArchiveOptions?) -> (),
        readfile: (path: string, options: ArchiveOptions?) -> Archive,
        writefile: (path: string, archive: Archive, options: ArchiveOptions?) -> (),
        load: (bytes: buffer, options: ArchiveOptions?) -> Archive,
        create: () -> Archive,
    },
    uncompressed: {
        extract: (path: string, destination: string, options: ArchiveOptions?) -> (),
        readfile: (path: string, options: ArchiveOptions?) -> Archive,
        writefile: (path: string, archive: Archive, options: ArchiveOptions?) -> (),
        load: (bytes: buffer, options: ArchiveOptions?) -> Archive,
        create: () -> Archive,
    },
    xz: {
        extract: (path: string, destination: string, options: ArchiveOptions?) -> (),
        readfile: (path: string, options: ArchiveOptions?) -> Archive,
        writefile: (path: string, archive: Archive, options: ArchiveOptions?) -> (),
        load: (bytes: buffer, options: ArchiveOptions?) -> Archive,
        create: () -> Archive,
    },
    lz4: {
        extract: (path: string, destination: string, options: ArchiveOptions?) -> (),
        readfile: (path: string, options: ArchiveOptions?) -> Archive,
        writefile: (path: string, archive: Archive, options: ArchiveOptions?) -> (),
        load: (bytes: buffer, options: ArchiveOptions?) -> Archive,
        create: () -> Archive,
    },
    bz2: {
        extract: (path: string, destination: string, options: ArchiveOptions?) -> (),
        readfile: (path: string, options: ArchiveOptions?) -> Archive,
        writefile: (path: string, archive: Archive, options: ArchiveOptions?) -> (),
        load: (bytes: buffer, options: ArchiveOptions?) -> Archive,
        create: () -> Archive,
    },
    zst: {
        extract: (path: string, destination: string, options: ArchiveOptions?) -> (),
        readfile: (path: string, options: ArchiveOptions?) -> Archive,
        writefile: (path: string, archive: Archive, options: ArchiveOptions?) -> (),
        load: (bytes: buffer, options: ArchiveOptions?) -> Archive,
        create: () -> Archive,
    },
}

export type ArchiveOptions = _types.ArchiveOptions
export type ArchiveFormat = _types.ArchiveFormat

return {} :: tar