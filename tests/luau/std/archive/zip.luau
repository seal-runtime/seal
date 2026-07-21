local zip = require("@std/archive/zip")
local archive = require("@std/archive")
local env = require("@std/env")
local fs = require("@std/fs")

const normal_path = "./tests/data/archives/zips/normal.zip"
const temp_path = fs.path.join(script:parent(), "temp_zip")
fs.dir.ensure(temp_path, true)

-- readfile: read a real zip off disk
const read = zip.readfile(normal_path)
const hello = read:get("hello.txt")
assert(hello and hello.type == "File", "normal.zip should contain hello.txt as a File")
assert(hello.content == "hello world\n", `unexpected hello.txt content: {hello.content}`)

const deep = read:get("dir/subdir/deep.txt")
assert(deep and deep.type == "File", "normal.zip should contain dir/subdir/deep.txt")
assert(deep.content == "pretty deep\n", `unexpected deep.txt content: {deep.content}`)

-- load: same archive, but from bytes we read ourselves
const loaded = zip.load(fs.readbytes(normal_path))
assert(#loaded == #read, "zip.load and zip.readfile disagree on entry count")
const loaded_hello = loaded:get("hello.txt")
assert(loaded_hello and loaded_hello.type == "File" and loaded_hello.content == "hello world\n",
    "zip.load didn't produce the same hello.txt as zip.readfile")

-- create -> serialize -> load roundtrip
const built = zip.create()
built:append({ type = "File", path = "a.txt", content = "aaa" })
built:append({ type = "File", path = "nested/b.txt", content = "bbb" })
const built_back = zip.load(built:serialize("Zip"))
assert(#built_back == 2, `expected 2 entries after zip roundtrip, got {#built_back}`)
const a_entry = built_back:get("a.txt")
const b_entry = built_back:get("nested/b.txt")
assert(a_entry and a_entry.type == "File" and a_entry.content == "aaa", "a.txt didn't survive the zip roundtrip")
assert(b_entry and b_entry.type == "File" and b_entry.content == "bbb", "nested/b.txt didn't survive the zip roundtrip")

-- binary content survives a roundtrip intact
const binary_content = string.char(0, 1, 2, 253, 254, 255) .. "\0mid-null\0"
const binary_archive = archive.create()
binary_archive:append({ type = "File", path = "bin.dat", content = binary_content })
const binary_back = zip.load(binary_archive:serialize("Zip")):get("bin.dat")
assert(binary_back and binary_back.type == "File", "bin.dat missing after roundtrip")
assert(binary_back.content == binary_content, "binary content was corrupted by the zip roundtrip")

-- writefile then readfile
const written_path = fs.path.join(temp_path, "written.zip")
zip.writefile(written_path, built)
assert(fs.path.exists(written_path), "zip.writefile didn't write anything")
const written_back = zip.readfile(written_path)
assert(#written_back == 2, `expected 2 entries in the written zip, got {#written_back}`)
const written_a = written_back:get("a.txt")
assert(written_a and written_a.type == "File" and written_a.content == "aaa",
    "a.txt didn't survive writefile -> readfile")

-- extract to disk
const extract_path = fs.path.join(temp_path, "extracted")
zip.extract(normal_path, extract_path)
assert(fs.path.exists(fs.path.join(extract_path, "hello.txt")), "extract didn't write hello.txt")
assert(fs.readfile(fs.path.join(extract_path, "hello.txt")) == "hello world\n", "extracted hello.txt has wrong content")
assert(fs.readfile(fs.path.join(extract_path, "dir", "nested.txt")) == "i am nested\n", "extracted nested.txt has wrong content")
assert(fs.readfile(fs.path.join(extract_path, "dir", "subdir", "deep.txt")) == "pretty deep\n", "extracted deep.txt has wrong content")

-- path traversal is refused by default
const traversal_path = "./tests/data/archives/zips/path_traversal.zip"
assert(not pcall(function()
    return zip.readfile(traversal_path)
end), "reading a path traversal zip should error by default")

-- ...and allowed only when explicitly opted into
const traversal = zip.readfile(traversal_path, { allow_unsafe_path_traversals = true })
const good = traversal:get("good/file.txt")
assert(good and good.type == "File", "the benign entry in path_traversal.zip should still be readable")

-- symlink traversal is refused by default too
assert(not pcall(function()
    return zip.readfile("./tests/data/archives/zips/symlink_traversal.zip")
end), "reading a symlink traversal zip should error by default")

-- max_file_size is enforced
const filesize = require("@std/fs/filesize")
assert(not pcall(function()
    return zip.readfile(normal_path, { max_file_size = filesize.bytes(1) })
end), "readfile should error when an entry exceeds max_file_size")

-- mode and mtime survive a create -> serialize -> load roundtrip
const datetime = require("@std/time/datetime")
const set_mtime = datetime.now()
const meta_built = zip.create()
meta_built:append({ type = "File", path = "meta.txt", content = "meta", mode = 0x1a0 --[[ 0o640 ]], mtime = set_mtime })
meta_built:append({ type = "Directory", path = "meta_dir/", mode = 0x1e8 --[[ 0o750 ]] })
const meta_back = zip.load(meta_built:serialize("Zip"))

-- zip's MS-DOS timestamps only have 2-second resolution, so allow a small tolerance
const function mtime_close(a: DateTime, b: DateTime)
    return math.abs(a.unix_timestamp - b.unix_timestamp) <= 2
end

const meta_entry = meta_back:get("meta.txt")
assert(meta_entry and meta_entry.mode == 0x1a0 --[[ 0o640 ]], `expected meta.txt mode 0o640, got: {meta_entry and meta_entry.mode}`)
assert(meta_entry.mtime and mtime_close(meta_entry.mtime, set_mtime), "meta.txt mtime didn't survive the zip roundtrip")

const meta_dir_entry = meta_back:get("meta_dir/")
assert(meta_dir_entry and meta_dir_entry.mode == 0x1e8 --[[ 0o750 ]], `expected meta_dir/ mode 0o750, got: {meta_dir_entry and meta_dir_entry.mode}`)

-- Archive:info exposes the same metadata as :get, minus content
const meta_info = meta_back:info("meta.txt")
assert(meta_info ~= nil, "info() should find meta.txt")
assert(meta_info.type == "File", "info() should report meta.txt as a File")
assert(meta_info.path == "meta.txt", "info() should report meta.txt's path")
assert(meta_info.mode == 0x1a0 --[[ 0o640 ]], "info() should report meta.txt's mode")
assert(meta_info.mtime and mtime_close(meta_info.mtime, set_mtime), "info() should report meta.txt's mtime")
assert(meta_info.size == filesize.bytes(4), `expected info().size to be 4 bytes, got: {meta_info.size}`)

const missing_info = meta_back:info("does_not_exist.txt")
assert(missing_info == nil, "info() should return nil for a path that doesn't exist")

-- mode and mtime survive an extract-to-disk roundtrip too
const meta_extract_path = fs.path.join(temp_path, "meta_extracted")
zip.writefile(fs.path.join(temp_path, "meta.zip"), meta_built)
zip.extract(fs.path.join(temp_path, "meta.zip"), meta_extract_path)
const extracted_meta_file = fs.file(fs.path.join(meta_extract_path, "meta.txt"))
assert(extracted_meta_file ~= nil, "extract didn't write meta.txt")

const extracted_meta_metadata = extracted_meta_file:metadata()
const extracted_unix_mode = extracted_meta_metadata.permissions.unix_mode
if env.os ~= "Windows" then
    assert(extracted_unix_mode ~= nil, "extracted meta.txt's metadata should include unix_mode on this platform")
    const extracted_meta_mode = bit32.band(extracted_unix_mode, 0x1ff)
    assert(
        extracted_meta_mode == 0x1a0 --[[ 0o640 ]],
        `expected extracted meta.txt to have mode 0o640, got: {string.format("%o", extracted_meta_mode)}`
    )
end

const extracted_modified_at = extracted_meta_metadata.modified_at
assert(extracted_modified_at ~= nil, "extracted meta.txt's metadata should include modified_at")
assert(
    mtime_close(extracted_modified_at, set_mtime),
    "extracted meta.txt should have the mtime recorded in the archive"
)

fs.dir.try_remove(temp_path)
