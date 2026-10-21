-- tests the Archive extern type itself: create, append, insert, remove, get, paths, #, serialize

local archive = require("@std/archive")
local entry = require("@std/archive/entry")

const file = entry.create.file
const directory = entry.create.directory
local fs = require("@std/fs")
local format = require("@std/io/format")

type ArchiveFormat = archive.ArchiveFormat

const a = archive.create()
assert(#a == 0, "a freshly created Archive should be empty")
assert(#a:paths() == 0, "a freshly created Archive should have no paths")
assert(a:get("nope.txt") == nil, "get should return nil for a missing path")
assert(a:get(1) == nil, "get should return nil for an out of range index")

-- append, single and variadic
a:append(file("hello.txt", "hello world"))
a:append(
    directory("dir"),
    file("dir/nested.txt", "nested!")
)
assert(#a == 3, `expected 3 entries after appending, got {#a}`)

-- paths come back in insertion order
const paths = a:paths()
assert(paths[1] == "hello.txt", `expected paths[1] == "hello.txt", got {paths[1]}`)
assert(paths[2] == "dir", `expected paths[2] == "dir", got {paths[2]}`)
assert(paths[3] == "dir/nested.txt", `expected paths[3] == "dir/nested.txt", got {paths[3]}`)

-- get by path
const hello = a:get("hello.txt")
assert(hello, "couldn't get hello.txt by path")
assert(hello.type == "File", `expected hello.txt to be a File, got {hello.type}`)
const hello_contents = hello:expect_file():contents()
assert(hello_contents == "hello world", `unexpected contents: {hello_contents}`)

-- get by index, which should agree with get by path
const first = a:get(1)
assert(first, "couldn't get entry at index 1")
assert(first:path() == "hello.txt", `expected index 1 to be hello.txt, got {first.path}`)

const dir = a:get("dir")
assert(dir and dir.type == "Directory", "expected dir to be a Directory entry")

-- expect agrees with get on hits, but throws instead of returning nil on misses
assert(a:expect("hello.txt"):expect_file():contents() == "hello world", "expect(path) should return the same entry as get(path)")
assert(a:expect(1):path() == "hello.txt", "expect(index) should return the same entry as get(index)")
assert(not pcall(function()
    return a:expect("nope.txt")
end), "expect should throw for a missing path, not return nil")
assert(not pcall(function()
    return a:expect(0)
end), "expect(0) should throw, not return nil")
assert(not pcall(function()
    return a:expect(#a + 1)
end), "expect should throw for an out of range index, not return nil")

-- ArchiveEntryFile:extract writes straight to disk without going through :contents()
const extract_temp = fs.path.join(script:parent(), "temp_entry_extract")
fs.dir.ensure(extract_temp, true)

const hello_file = hello:expect_file()
const hello_dest = fs.path.join(extract_temp, "hello_extracted.txt")
hello_file:extract(hello_dest)
assert(fs.readfile(hello_dest) == "hello world", "ArchiveEntryFile:extract wrote wrong content")

-- creates parent directories as needed
const nested_dest = fs.path.join(extract_temp, "nested", "deeper", "hello.txt")
hello_file:extract(nested_dest)
assert(fs.readfile(nested_dest) == "hello world", "ArchiveEntryFile:extract didn't create missing parent directories")

-- mode and mtime survive extraction, same as Archive:extract/archive.extract
const datetime = require("@std/time/datetime")
const set_mtime = datetime.now()
const meta_file = file("meta.txt", "meta")
meta_file:set_unix_mode(0x1a4 --[[ 0o644 ]])
meta_file:set_modified(set_mtime)
const meta_dest = fs.path.join(extract_temp, "meta_extracted.txt")
meta_file:extract(meta_dest)
assert(fs.readfile(meta_dest) == "meta", "ArchiveEntryFile:extract lost content when metadata was set")

const env = require("@std/env")
const extracted_meta = fs.file(meta_dest)
assert(extracted_meta ~= nil, "ArchiveEntryFile:extract didn't write meta_extracted.txt")
const extracted_metadata = extracted_meta:metadata()
if env.os ~= "Windows" then
    const extracted_unix_mode = extracted_metadata.permissions.unix_mode
    assert(extracted_unix_mode ~= nil, "extracted meta.txt's metadata should include unix_mode on this platform")
    const extracted_mode = bit32.band(extracted_unix_mode, 0x1ff)
    assert(extracted_mode == 0x1a4 --[[ 0o644 ]], `expected extracted file to have mode 0o644, got: {string.format("%o", extracted_mode)}`)
end
if extracted_metadata.modified_at then
    assert(
        math.abs(extracted_metadata.modified_at.unix_timestamp - set_mtime.unix_timestamp) <= 2,
        "extracted meta_extracted.txt should have the mtime set on the entry"
    )
end

-- calling :extract on a non-File entry should error, not panic
assert(not pcall(function()
    (directory("meow") :: any):extract(fs.path.join(extract_temp, "shouldnt_exist"))
    return nil
end), "ArchiveEntryFile:extract on a Directory entry should error, not panic")

-- RUSTSEC-2026-0367: extraction must strip setuid/setgid bits from archive-supplied modes
if env.os ~= "Windows" then
    const uncompressed_tar = require("@std/archive/tar").uncompressed
    const setuid_file = file("setuid.sh", "#!/bin/sh\n")
    setuid_file:set_unix_mode(0x9ed --[[ 0o4755 ]])
    const setuid_archive = archive.create()
    setuid_archive:append(setuid_file)

    -- tar keeps the full mode (zip's writer masks to 0o777), so the archive on disk really carries setuid
    const setuid_tar_path = fs.path.join(extract_temp, "setuid.tar")
    uncompressed_tar.writefile(setuid_tar_path, setuid_archive)
    const reloaded_mode = uncompressed_tar.readfile(setuid_tar_path):expect("setuid.sh"):unix_mode()
    assert(reloaded_mode == 0x9ed, `setuid.tar should store mode 0o4755, got: {string.format("%o", reloaded_mode or 0)}`)

    const function assert_setuid_stripped(path: string, via: string)
        const extracted = fs.file(path)
        assert(extracted ~= nil, `{via}: didn't write {path}`)
        const mode = extracted:metadata().permissions.unix_mode
        assert(mode ~= nil, `{via}: extracted file's metadata should include unix_mode on this platform`)
        const mode_bits = bit32.band(mode, 0xfff)
        assert(mode_bits == 0x1ed --[[ 0o755 ]], `{via}: expected mode 0o755 with setuid stripped, got: {string.format("%o", mode_bits)}`)
    end

    const entry_dest = fs.path.join(extract_temp, "setuid_entry.sh")
    setuid_file:extract(entry_dest)
    assert_setuid_stripped(entry_dest, "ArchiveEntryFile:extract")

    const from_archive_dest = fs.path.join(extract_temp, "setuid_from_archive")
    uncompressed_tar.extract(setuid_archive, from_archive_dest)
    assert_setuid_stripped(fs.path.join(from_archive_dest, "setuid.sh"), "tar.extract(Archive)")

    const from_path_dest = fs.path.join(extract_temp, "setuid_from_path")
    uncompressed_tar.extract(setuid_tar_path, from_path_dest)
    assert_setuid_stripped(fs.path.join(from_path_dest, "setuid.sh"), "tar.extract(path)")
end

fs.dir.try_remove(extract_temp)

-- Archive supports generalized iteration, yielding (index, entry) in the same order as :paths()
const iter_paths = a:paths()
const seen = {}
for index, entry in a do
    seen[index] = entry:path()
end
assert(#seen == #a, `expected to iterate over all {#a} entries, got {#seen}`)
for index, path in iter_paths do
    assert(seen[index] == path, `expected entry {index} from iteration to be {path}, got {seen[index]}`)
end

-- insert shifts everything after it down
a:insert(1, file("first.txt", "i am first"))
assert(#a == 4, `expected 4 entries after insert, got {#a}`)
const after_insert = a:paths()
assert(after_insert[1] == "first.txt", `insert didn't put first.txt at index 1, got {after_insert[1]}`)
assert(after_insert[2] == "hello.txt", `insert didn't shift hello.txt to index 2, got {after_insert[2]}`)

-- remove by index
a:remove(1)
assert(#a == 3, `expected 3 entries after remove, got {#a}`)
assert(a:get("first.txt") == nil, "first.txt should be gone after being removed")
assert(a:paths()[1] == "hello.txt", "hello.txt should be back at index 1 after the remove")

-- out of range / zero indices should error cleanly, not panic
assert(not pcall(function()
    a:remove(0)
    return nil
end), "remove(0) should error, not panic")

assert(not pcall(function()
    a:remove(#a + 1)
    return nil
end), "remove(#a + 1) should error, not panic")

assert(not pcall(function()
    a:insert(0, file("zero.txt", "zero"))
    return nil
end), "insert(0, ...) should error, not panic")

assert(a:get(0) == nil, "get(0) should return nil like any other out of range index, not panic")

-- symlink entries keep their target
const linked = archive.create()
-- linked:append({ type = "File", path = "real.txt", content = "real" })
linked:append(entry.create.symlink("link.txt", "real.txt"))
const link_bytes = linked:serialize("Tar")
const link_loaded = require("@std/archive/tar").uncompressed.load(link_bytes, { allow_symlinks = true })
const link = link_loaded:get("link.txt")
assert(link, "couldn't get the symlink back out of the archive")
assert(link.type == "Symlink", `expected a Symlink, got {link.type}`)
assert(link:expect_symlink():target() == "real.txt", `expected symlink target "real.txt", got {format(link)}`)

-- serialize works for every format in the ArchiveFormat union, and each is non-empty
const formats: { ArchiveFormat } = { "Zip", "Tar", "TarGz", "TarBz2", "TarXz", "TarZst", "TarLz4", "7z" }
for _, format in formats do
    const bytes = a:serialize(format)
    assert(buffer.len(bytes) > 0, `serializing to {format} produced an empty buffer`)
end

-- serialize honors ArchiveOptions.compression_level, same as writefile
const tar = require("@std/archive/tar")
const big = archive.create()
big:append(file("big.txt", string.rep("seal seal seal ", 20000)))
const low_bytes = big:serialize("TarGz", { compression_level = tar.gz.compression(1) })
const high_bytes = big:serialize("TarGz", { compression_level = tar.gz.compression(9) })
assert(
    buffer.len(high_bytes) <= buffer.len(low_bytes),
    `expected serialize with level 9 to compress at least as well as level 1 ({buffer.len(high_bytes)} vs {buffer.len(low_bytes)} bytes)`
)
const high_back = tar.gz.load(high_bytes):get("big.txt")
assert(
    high_back and high_back.type == "File" and high_back:expect_file():contents() == string.rep("seal seal seal ", 20000),
    "content changed when serializing with a custom compression_level"
)

-- ...and rejects a compression_level built for the wrong format, same as writefile
assert(not pcall(function()
    return big:serialize("TarGz", { compression_level = tar.xz.compression(6) })
end), "serializing TarGz with a tar.xz compression_level should error")

-- duplicate paths: get returns the *first* match, per the typedef
const dupes = archive.create()
-- dupes:append({ type = "File", path = "dupe.txt", content = "first" })
dupes:append(file("dupe.txt", "first"))
dupes:append(file("dupe.txt", "second"))
const dupe = dupes:get("dupe.txt")
assert(dupe and dupe.type == "File", "couldn't get the duplicated path")
assert(dupe:expect_file():contents() == "first", `get should return the first match, got {dupe:expect_file():contents()}`)

const data = fs.dir.project():join("tests", "data")
const from_disk = archive.create()

const server_views = fs.path.join(data, "server-views")
const server_view_entries = entry.read(server_views)
-- adding mix of { ArchiveEntry } and ArchiveEntry should work
from_disk:append(server_view_entries, entry.read("./src/main.rs", "mainrs.rs"))
const mainrsrs = from_disk:get("mainrs.rs")
assert(mainrsrs and mainrsrs:expect_file().type == "File", "should be file")

-- calling a File-only method on a non-File entry should error cleanly, not panic
-- (the type system normally prevents this; the :: any cast simulates untyped/unsafe access)
assert(not pcall(function()
    (entry.create.directory("meow") :: any):replace_contents("hehe")
    return nil
end), "replace_contents on a Directory entry should error, not panic")

-- Archive:extract writes an in-memory Archive straight to disk, no format needed
const method_extract = archive.create()
method_extract:append(file("hello.txt", "hello world\n"))
method_extract:append(directory("dir"))
method_extract:append(file("dir/nested.txt", "i am nested\n"))

const method_extract_path = fs.path.join(script:parent(), "temp_archive_method_extract")
method_extract:extract(method_extract_path)
assert(
    fs.readfile(fs.path.join(method_extract_path, "hello.txt")) == "hello world\n",
    "Archive:extract wrote wrong hello.txt content"
)
assert(
    fs.readfile(fs.path.join(method_extract_path, "dir", "nested.txt")) == "i am nested\n",
    "Archive:extract wrote wrong dir/nested.txt content"
)
fs.dir.try_remove(method_extract_path)

-- archive.extract: format-generic extraction, both from a path on disk and from an in-memory Archive
const temp_path = fs.path.join(script:parent(), "temp_archive_extract")
fs.dir.ensure(temp_path, true)

-- a regular (container) archive
const container = archive.create()
container:append(file("hello.txt", "hello world\n"))
container:append(directory("dir"))
container:append(file("dir/nested.txt", "i am nested\n"))

const container_path = fs.path.join(temp_path, "container.tar.gz")
fs.writefile(container_path, buffer.tostring(container:serialize("TarGz")))

const container_from_path = fs.path.join(temp_path, "container_from_path")
archive.extract(container_path, container_from_path, "TarGz")
assert(
    fs.readfile(fs.path.join(container_from_path, "hello.txt")) == "hello world\n",
    "archive.extract from a path wrote wrong hello.txt content"
)
assert(
    fs.readfile(fs.path.join(container_from_path, "dir", "nested.txt")) == "i am nested\n",
    "archive.extract from a path wrote wrong dir/nested.txt content"
)

const container_from_archive = fs.path.join(temp_path, "container_from_archive")
archive.extract(container, container_from_archive, "TarGz")
assert(
    fs.readfile(fs.path.join(container_from_archive, "hello.txt")) == "hello world\n",
    "archive.extract from an Archive instance wrote wrong hello.txt content"
)
assert(
    fs.readfile(fs.path.join(container_from_archive, "dir", "nested.txt")) == "i am nested\n",
    "archive.extract from an Archive instance wrote wrong dir/nested.txt content"
)

-- a single compressed file (SingleFileFormat), which extracts straight to a file, not a directory
const single = archive.create()
single:append(file("only.txt", "just the one file\n"))

const single_path = fs.path.join(temp_path, "single.gz")
fs.writefile(single_path, buffer.tostring(single:serialize("Gz")))

const single_from_path = fs.path.join(temp_path, "single_from_path.txt")
archive.extract(single_path, single_from_path, "Gz")
assert(
    fs.readfile(single_from_path) == "just the one file\n",
    "archive.extract of a single-file format from a path wrote wrong content"
)

const single_from_archive = fs.path.join(temp_path, "single_from_archive.txt")
archive.extract(single, single_from_archive, "Gz")
assert(
    fs.readfile(single_from_archive) == "just the one file\n",
    "archive.extract of a single-file format from an Archive instance wrote wrong content"
)

fs.dir.try_remove(temp_path)