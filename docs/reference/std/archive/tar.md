<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# archive.tar

`local tar = require("@std/archive/tar")`

Read, write, and extract tar ("Tape Archive") archives, affectionally known as
tarballs, split by the compression codec applied to the tarball
("uncompressed" for none).

---

### tar.gz

<h4>

```luau
gz: {
```

</h4>

<details>

<summary> See the docs </summary

gzip-compressed tarballs (`.tar.gz`/`.tgz`) are the most common type of tar
on Linux/Unix-like platforms.

Fast to compress and decompress with the most universal support of any codec here, but its
compression ratio trails xz/zstd/7z.

The safe default when you don't know what'll be reading the file back.

Uses gzip's default compression level (6). If you need a different level, open an issue, make a PR, or contact me.

</details>

---

### tar.gz.extract

<h4>

```luau
function tar.gz.extract(path: string, destination: string, options: ArchiveOptions?) -> (),
```

</h4>

<details>

<summary> See the docs </summary

Extract the tar.gz archive at `path` into a new or existing directory at `destination`.

This protects against path traversal attacks (unexpectedly writing outside destination directory),
symlink traversal attacks, and caps archive and individual file sizes to prevent extraction bombs.

To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.

</details>

---

### tar.gz.readfile

<h4>

```luau
function tar.gz.readfile(path: string, options: ArchiveOptions?) -> Archive,
```

</h4>

Read the tar.gz archive at `path` into memory as an `Archive`.

To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.

---

### tar.gz.writefile

<h4>

```luau
function tar.gz.writefile(path: string, archive: Archive, options: ArchiveOptions?) -> (),
```

</h4>

Write an `Archive` to `path` as a tar.gz archive.

To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.

---

### tar.gz.load

<h4>

```luau
function tar.gz.load(bytes: buffer, options: ArchiveOptions?) -> Archive,
```

</h4>

Load a tar.gz archive into memory as an `Archive` from an existing buffer of bytes.

---

### tar.gz.create

<h4>

```luau
function tar.gz.create() -> Archive,
```

</h4>

Create a new empty `Archive`.

---

```luau
  }, -- closes gz
```

---

### tar.uncompressed

<h4>

```luau
uncompressed: {
```

</h4>

Plain tar archives with no compression.

No compression overhead means the fastest reads/writes and no CPU cost, at the expense of the
largest output size. Useful when bundling data that's already compressed (e.g. media, other archives)
where recompressing wouldn't help anyway.

---

### tar.uncompressed.extract

<h4>

```luau
function tar.uncompressed.extract(path: string, destination: string, options: ArchiveOptions?) -> (),
```

</h4>

<details>

<summary> See the docs </summary

Extract the tar archive at `path` into a new or existing directory at `destination`.

This protects against path traversal attacks (unexpectedly writing outside destination directory),
symlink traversal attacks, and caps archive and individual file sizes to prevent extraction bombs.

To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.

</details>

---

### tar.uncompressed.readfile

<h4>

```luau
function tar.uncompressed.readfile(path: string, options: ArchiveOptions?) -> Archive,
```

</h4>

Read the tar archive at `path` into memory as an `Archive`.

To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.

---

### tar.uncompressed.writefile

<h4>

```luau
function tar.uncompressed.writefile(path: string, archive: Archive, options: ArchiveOptions?) -> (),
```

</h4>

Write an `Archive` to `path` as a tar archive.

To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.

---

### tar.uncompressed.load

<h4>

```luau
function tar.uncompressed.load(bytes: buffer, options: ArchiveOptions?) -> Archive,
```

</h4>

Load a tar archive into memory as an `Archive` from an existing buffer of bytes.

---

### tar.uncompressed.create

<h4>

```luau
function tar.uncompressed.create() -> Archive,
```

</h4>

Create a new empty `Archive`.

---

```luau
  }, -- closes uncompressed
```

---

### tar.xz

<h4>

```luau
xz: {
```

</h4>

<details>

<summary> See the docs </summary

xz-compressed tarballs (`.tar.xz`), using LZMA2.

Best-in-class compression ratio, on par with 7z, but noticeably slower to compress than
gzip or zstd (decompression stays cheap). Good for release tarballs you compress once and
many people decompress.

Uses xz preset 6. If you need a different preset, open an issue, make a PR, or contact me.

</details>

---

### tar.xz.extract

<h4>

```luau
function tar.xz.extract(path: string, destination: string, options: ArchiveOptions?) -> (),
```

</h4>

<details>

<summary> See the docs </summary

Extract the tar.xz archive at `path` into a new or existing directory at `destination`.

This protects against path traversal attacks (unexpectedly writing outside destination directory),
symlink traversal attacks, and caps archive and individual file sizes to prevent extraction bombs.

To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.

</details>

---

### tar.xz.readfile

<h4>

```luau
function tar.xz.readfile(path: string, options: ArchiveOptions?) -> Archive,
```

</h4>

Read the tar.xz archive at `path` into memory as an `Archive`.

To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.

---

### tar.xz.writefile

<h4>

```luau
function tar.xz.writefile(path: string, archive: Archive, options: ArchiveOptions?) -> (),
```

</h4>

Write an `Archive` to `path` as a tar.xz archive.

To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.

---

### tar.xz.load

<h4>

```luau
function tar.xz.load(bytes: buffer, options: ArchiveOptions?) -> Archive,
```

</h4>

Load a tar.xz archive into memory as an `Archive` from an existing buffer of bytes.

---

### tar.xz.create

<h4>

```luau
function tar.xz.create() -> Archive,
```

</h4>

Create a new empty `Archive`.

---

```luau
  }, -- closes xz
```

---

### tar.lz4

<h4>

```luau
lz4: {
```

</h4>

<details>

<summary> See the docs </summary

lz4-compressed tarballs (`.tar.lz4`).

The fastest codec here to compress and decompress by a wide margin, at the cost of the
worst compression ratio. Best when speed (e.g. repeated packing/unpacking, hot paths) matters
more than size on disk.

Uses lz4's fast mode (level 0), not its high-compression mode. If you need high-compression
mode or a specific level, open an issue, make a PR, or contact me.

</details>

---

### tar.lz4.extract

<h4>

```luau
function tar.lz4.extract(path: string, destination: string, options: ArchiveOptions?) -> (),
```

</h4>

<details>

<summary> See the docs </summary

Extract the tar.lz4 archive at `path` into a new or existing directory at `destination`.

This protects against path traversal attacks (unexpectedly writing outside destination directory),
symlink traversal attacks, and caps archive and individual file sizes to prevent extraction bombs.

To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.

</details>

---

### tar.lz4.readfile

<h4>

```luau
function tar.lz4.readfile(path: string, options: ArchiveOptions?) -> Archive,
```

</h4>

Read the tar.lz4 archive at `path` into memory as an `Archive`.

To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.

---

### tar.lz4.writefile

<h4>

```luau
function tar.lz4.writefile(path: string, archive: Archive, options: ArchiveOptions?) -> (),
```

</h4>

Write an `Archive` to `path` as a tar.lz4 archive.

To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.

---

### tar.lz4.load

<h4>

```luau
function tar.lz4.load(bytes: buffer, options: ArchiveOptions?) -> Archive,
```

</h4>

Load a tar.lz4 archive into memory as an `Archive` from an existing buffer of bytes.

---

### tar.lz4.create

<h4>

```luau
function tar.lz4.create() -> Archive,
```

</h4>

Create a new empty `Archive`.

---

```luau
  }, -- closes lz4
```

---

### tar.bz2

<h4>

```luau
bz2: {
```

</h4>

<details>

<summary> See the docs </summary

bzip2-compressed tarballs (`.tar.bz2`).

Can beat gzip's ratio on text-heavy data, but is slower to compress and decompress than
gzip, lz4, or zstd, and has been largely superseded by xz/zstd. Mainly useful for
compatibility with older tooling that expects bz2.

Uses bzip2's default compression level (6). If you need a different level, open an issue, make a PR, or contact me.

</details>

---

### tar.bz2.extract

<h4>

```luau
function tar.bz2.extract(path: string, destination: string, options: ArchiveOptions?) -> (),
```

</h4>

<details>

<summary> See the docs </summary

Extract the tar.bz2 archive at `path` into a new or existing directory at `destination`.

This protects against path traversal attacks (unexpectedly writing outside destination directory),
symlink traversal attacks, and caps archive and individual file sizes to prevent extraction bombs.

To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.

</details>

---

### tar.bz2.readfile

<h4>

```luau
function tar.bz2.readfile(path: string, options: ArchiveOptions?) -> Archive,
```

</h4>

Read the tar.bz2 archive at `path` into memory as an `Archive`.

To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.

---

### tar.bz2.writefile

<h4>

```luau
function tar.bz2.writefile(path: string, archive: Archive, options: ArchiveOptions?) -> (),
```

</h4>

Write an `Archive` to `path` as a tar.bz2 archive.

To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.

---

### tar.bz2.load

<h4>

```luau
function tar.bz2.load(bytes: buffer, options: ArchiveOptions?) -> Archive,
```

</h4>

Load a tar.bz2 archive into memory as an `Archive` from an existing buffer of bytes.

---

### tar.bz2.create

<h4>

```luau
function tar.bz2.create() -> Archive,
```

</h4>

Create a new empty `Archive`.

---

```luau
  }, -- closes bz2
```

---

### tar.zst

<h4>

```luau
zst: {
```

</h4>

<details>

<summary> See the docs </summary

Zstandard-compressed tarballs (`.tar.zst`).

We compress at zstd's default level (3), which is fast like gzip/lz4 while landing closer to
xz's ratio than gzip does. The best general-purpose choice unless you need gzip's ubiquity or
xz/7z's maximum ratio.

If you want to write `tar.zst` archives with a different compression level or other zstd
options (checksums, window log, etc.), create the tarball as tar.uncompressed and then
compress it with`@std/serde/zstd` before writing it to disk.

</details>

---

### tar.zst.extract

<h4>

```luau
function tar.zst.extract(path: string, destination: string, options: ArchiveOptions?) -> (),
```

</h4>

<details>

<summary> See the docs </summary

Extract the tar.zst archive at `path` into a new or existing directory at `destination`.

This protects against path traversal attacks (unexpectedly writing outside destination directory),
symlink traversal attacks, and caps archive and individual file sizes to prevent extraction bombs.

To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.

</details>

---

### tar.zst.readfile

<h4>

```luau
function tar.zst.readfile(path: string, options: ArchiveOptions?) -> Archive,
```

</h4>

Read the tar.zst archive at `path` into memory as an `Archive`.

To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.

---

### tar.zst.writefile

<h4>

```luau
function tar.zst.writefile(path: string, archive: Archive, options: ArchiveOptions?) -> (),
```

</h4>

Write an `Archive` to `path` as a tar.zst archive.

To increase size limits, allow unsafe path traversals or allow symlinks, see `ArchiveOptions`.

---

### tar.zst.load

<h4>

```luau
function tar.zst.load(bytes: buffer, options: ArchiveOptions?) -> Archive,
```

</h4>

Load a tar.zst archive into memory as an `Archive` from an existing buffer of bytes.

---

### tar.zst.create

<h4>

```luau
function tar.zst.create() -> Archive,
```

</h4>

Create a new empty `Archive`.

---

```luau
  }, -- closes zst
```

---

## `export type` ArchiveOptions

See [ArchiveOptions in @std/archive/_types](/docs/reference/std/archive/_types.md#export-type-archiveoptions)

---

## `export type` ArchiveFormat

See [ArchiveFormat in @std/archive/_types](/docs/reference/std/archive/_types.md#export-type-archiveformat)

---

Autogenerated from [std/archive/tar.luau](/.seal/typedefs/std/archive/tar.luau).

*seal* is best experienced with inline, in-editor documentation. Please see the linked typedefs file if this documentation is confusing, too verbose, or inaccurate.
