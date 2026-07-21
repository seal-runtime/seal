local _types = require("./_types")

--[=[
    Read, write, and extract tar ("Tape Archive") archives, affectionally known as
    tarballs, split by the compression codec applied to the tarball 
    ("uncompressed" for none).
]=]
export type tar = {
    --[=[
        gzip-compressed tarballs (`.tar.gz`/`.tgz`) are the most common type of tar 
        on Linux/Unix-like platforms.

        Fast to compress and decompress with the most universal support of any codec here, but its
        compression ratio trails xz/zstd/7z.

        The safe default when you don't know what'll be reading the file back.

        Uses gzip's default compression level (6). If you need a different level, open an issue, make a PR, or contact me.
    ]=]
    gz: {
        --[=[
            Extract the tar.gz archive at `path` into a new or existing directory at `destination`.

            This protects against path traversal attacks (unexpectedly writing outside destination directory),
            symlink traversal attacks, and caps archive and individual file sizes to prevent extraction bombs.

            To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.
        ]=]
        extract: (path: string, destination: string, options: ArchiveOptions?) -> (),
        --[=[
            Read the tar.gz archive at `path` into memory as an `Archive`.

            To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.
        ]=]
        readfile: (path: string, options: ArchiveOptions?) -> Archive,
        --[=[
            Write an `Archive` to `path` as a tar.gz archive.

            To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.
        ]=]
        writefile: (path: string, archive: Archive, options: ArchiveOptions?) -> (),
        --[=[
            Load a tar.gz archive into memory as an `Archive` from an existing buffer of bytes.
        ]=]
        load: (bytes: buffer, options: ArchiveOptions?) -> Archive,
        --[=[
            Create a new empty `Archive`.
        ]=]
        create: () -> Archive,
    },
    --[=[
        Plain tar archives with no compression.

        No compression overhead means the fastest reads/writes and no CPU cost, at the expense of the
        largest output size. Useful when bundling data that's already compressed (e.g. media, other archives)
        where recompressing wouldn't help anyway.
    ]=]
    uncompressed: {
        --[=[
            Extract the tar archive at `path` into a new or existing directory at `destination`.

            This protects against path traversal attacks (unexpectedly writing outside destination directory),
            symlink traversal attacks, and caps archive and individual file sizes to prevent extraction bombs.

            To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.
        ]=]
        extract: (path: string, destination: string, options: ArchiveOptions?) -> (),
        --[=[
            Read the tar archive at `path` into memory as an `Archive`.

            To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.
        ]=]
        readfile: (path: string, options: ArchiveOptions?) -> Archive,
        --[=[
            Write an `Archive` to `path` as a tar archive.

            To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.
        ]=]
        writefile: (path: string, archive: Archive, options: ArchiveOptions?) -> (),
        --[=[
            Load a tar archive into memory as an `Archive` from an existing buffer of bytes.
        ]=]
        load: (bytes: buffer, options: ArchiveOptions?) -> Archive,
        --[=[
            Create a new empty `Archive`.
        ]=]
        create: () -> Archive,
    },
    --[=[
        xz-compressed tarballs (`.tar.xz`), using LZMA2.

        Best-in-class compression ratio, on par with 7z, but noticeably slower to compress than
        gzip or zstd (decompression stays cheap). Good for release tarballs you compress once and
        many people decompress.

        Uses xz preset 6. If you need a different preset, open an issue, make a PR, or contact me.
    ]=]
    xz: {
        --[=[
            Extract the tar.xz archive at `path` into a new or existing directory at `destination`.

            This protects against path traversal attacks (unexpectedly writing outside destination directory),
            symlink traversal attacks, and caps archive and individual file sizes to prevent extraction bombs.

            To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.
        ]=]
        extract: (path: string, destination: string, options: ArchiveOptions?) -> (),
        --[=[
            Read the tar.xz archive at `path` into memory as an `Archive`.

            To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.
        ]=]
        readfile: (path: string, options: ArchiveOptions?) -> Archive,
        --[=[
            Write an `Archive` to `path` as a tar.xz archive.

            To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.
        ]=]
        writefile: (path: string, archive: Archive, options: ArchiveOptions?) -> (),
        --[=[
            Load a tar.xz archive into memory as an `Archive` from an existing buffer of bytes.
        ]=]
        load: (bytes: buffer, options: ArchiveOptions?) -> Archive,
        --[=[
            Create a new empty `Archive`.
        ]=]
        create: () -> Archive,
    },
    --[=[
        lz4-compressed tarballs (`.tar.lz4`).

        The fastest codec here to compress and decompress by a wide margin, at the cost of the
        worst compression ratio. Best when speed (e.g. repeated packing/unpacking, hot paths) matters
        more than size on disk.

        Uses lz4's fast mode (level 0), not its high-compression mode. If you need high-compression
        mode or a specific level, open an issue, make a PR, or contact me.
    ]=]
    lz4: {
        --[=[
            Extract the tar.lz4 archive at `path` into a new or existing directory at `destination`.

            This protects against path traversal attacks (unexpectedly writing outside destination directory),
            symlink traversal attacks, and caps archive and individual file sizes to prevent extraction bombs.

            To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.
        ]=]
        extract: (path: string, destination: string, options: ArchiveOptions?) -> (),
        --[=[
            Read the tar.lz4 archive at `path` into memory as an `Archive`.

            To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.
        ]=]
        readfile: (path: string, options: ArchiveOptions?) -> Archive,
        --[=[
            Write an `Archive` to `path` as a tar.lz4 archive.

            To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.
        ]=]
        writefile: (path: string, archive: Archive, options: ArchiveOptions?) -> (),
        --[=[
            Load a tar.lz4 archive into memory as an `Archive` from an existing buffer of bytes.
        ]=]
        load: (bytes: buffer, options: ArchiveOptions?) -> Archive,
        --[=[
            Create a new empty `Archive`.
        ]=]
        create: () -> Archive,
    },
    --[=[
        bzip2-compressed tarballs (`.tar.bz2`).

        Can beat gzip's ratio on text-heavy data, but is slower to compress and decompress than
        gzip, lz4, or zstd, and has been largely superseded by xz/zstd. Mainly useful for
        compatibility with older tooling that expects bz2.

        Uses bzip2's default compression level (6). If you need a different level, open an issue, make a PR, or contact me.
    ]=]
    bz2: {
        --[=[
            Extract the tar.bz2 archive at `path` into a new or existing directory at `destination`.

            This protects against path traversal attacks (unexpectedly writing outside destination directory),
            symlink traversal attacks, and caps archive and individual file sizes to prevent extraction bombs.

            To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.
        ]=]
        extract: (path: string, destination: string, options: ArchiveOptions?) -> (),
        --[=[
            Read the tar.bz2 archive at `path` into memory as an `Archive`.

            To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.
        ]=]
        readfile: (path: string, options: ArchiveOptions?) -> Archive,
        --[=[
            Write an `Archive` to `path` as a tar.bz2 archive.

            To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.
        ]=]
        writefile: (path: string, archive: Archive, options: ArchiveOptions?) -> (),
        --[=[
            Load a tar.bz2 archive into memory as an `Archive` from an existing buffer of bytes.
        ]=]
        load: (bytes: buffer, options: ArchiveOptions?) -> Archive,
        --[=[
            Create a new empty `Archive`.
        ]=]
        create: () -> Archive,
    },
    --[=[
        Zstandard-compressed tarballs (`.tar.zst`).

        We compress at zstd's default level (3), which is fast like gzip/lz4 while landing closer to
        xz's ratio than gzip does. The best general-purpose choice unless you need gzip's ubiquity or
        xz/7z's maximum ratio.

        If you want to write `tar.zst` archives with a different compression level or other zstd
        options (checksums, window log, etc.), create the tarball as tar.uncompressed and then
        compress it with`@std/serde/zstd` before writing it to disk.
    ]=]
    zst: {
        --[=[
            Extract the tar.zst archive at `path` into a new or existing directory at `destination`.

            This protects against path traversal attacks (unexpectedly writing outside destination directory),
            symlink traversal attacks, and caps archive and individual file sizes to prevent extraction bombs.

            To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.
        ]=]
        extract: (path: string, destination: string, options: ArchiveOptions?) -> (),
        --[=[
            Read the tar.zst archive at `path` into memory as an `Archive`.

            To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.
        ]=]
        readfile: (path: string, options: ArchiveOptions?) -> Archive,
        --[=[
            Write an `Archive` to `path` as a tar.zst archive.

            To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.
        ]=]
        writefile: (path: string, archive: Archive, options: ArchiveOptions?) -> (),
        --[=[
            Load a tar.zst archive into memory as an `Archive` from an existing buffer of bytes.
        ]=]
        load: (bytes: buffer, options: ArchiveOptions?) -> Archive,
        --[=[
            Create a new empty `Archive`.
        ]=]
        create: () -> Archive,
    },
}

export type ArchiveOptions = _types.ArchiveOptions
export type ArchiveFormat = _types.ArchiveFormat

return {} :: tar
