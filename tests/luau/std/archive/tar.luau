local tar = require("@std/archive/tar")
local archive = require("@std/archive")
local fs = require("@std/fs")

type ArchiveFormat = archive.ArchiveFormat

const temp_path = fs.path.join(script:parent(), "temp_tar")
fs.dir.ensure(temp_path, true)

-- every tar variant, paired with the ArchiveFormat that Archive:serialize uses for it
type Variant = {
    name: string,
    lib: typeof(tar.gz),
    format: ArchiveFormat,
    extension: string,
}

const variants: { Variant } = {
    { name = "uncompressed", lib = tar.uncompressed, format = "Tar", extension = ".tar" },
    { name = "gz", lib = tar.gz, format = "TarGz", extension = ".tar.gz" },
    { name = "xz", lib = tar.xz, format = "TarXz", extension = ".tar.xz" },
    { name = "bz2", lib = tar.bz2, format = "TarBz2", extension = ".tar.bz2" },
    { name = "lz4", lib = tar.lz4, format = "TarLz4", extension = ".tar.lz4" },
    { name = "zst", lib = tar.zst, format = "TarZst", extension = ".tar.zst" },
}

const binary_content = string.char(0, 1, 2, 253, 254, 255) .. "\0mid-null\0"

local function sample(): Archive
    const a = archive.create()
    a:append({ type = "File", path = "hello.txt", content = "hello world\n" })
    a:append({ type = "Directory", path = "dir" })
    a:append({ type = "File", path = "dir/nested.txt", content = "i am nested\n" })
    a:append({ type = "File", path = "bin.dat", content = binary_content })
    return a
end

local function assert_contents(loaded: Archive, variant: string)
    const hello = loaded:get("hello.txt")
    assert(hello and hello.type == "File", `tar.{variant}: hello.txt missing or not a File`)
    assert(hello.content == "hello world\n", `tar.{variant}: unexpected hello.txt content: {hello.content}`)

    const nested = loaded:get("dir/nested.txt")
    assert(nested and nested.type == "File", `tar.{variant}: dir/nested.txt missing`)
    assert(nested.content == "i am nested\n", `tar.{variant}: unexpected nested.txt content: {nested.content}`)

    const bin = loaded:get("bin.dat")
    assert(bin and bin.type == "File", `tar.{variant}: bin.dat missing`)
    assert(bin.content == binary_content, `tar.{variant}: binary content was corrupted`)
end

for _, variant in variants do
    const original = sample()

    -- create() gives back a usable, empty Archive
    const created = variant.lib.create()
    assert(#created == 0, `tar.{variant.name}.create() should return an empty Archive`)

    -- serialize -> load roundtrip
    const bytes = original:serialize(variant.format)
    assert(buffer.len(bytes) > 0, `tar.{variant.name}: serialize produced an empty buffer`)
    const loaded = variant.lib.load(bytes)
    assert_contents(loaded, variant.name)

    -- writefile -> readfile roundtrip
    const written_path = fs.path.join(temp_path, `written{variant.extension}`)
    variant.lib.writefile(written_path, original)
    assert(fs.path.exists(written_path), `tar.{variant.name}.writefile didn't write anything`)
    assert_contents(variant.lib.readfile(written_path), variant.name)

    -- load(readbytes(...)) agrees with readfile
    assert_contents(variant.lib.load(fs.readbytes(written_path)), variant.name)

    -- extract to disk
    const extract_path = fs.path.join(temp_path, `extracted_{variant.name}`)
    variant.lib.extract(written_path, extract_path)
    assert(
        fs.readfile(fs.path.join(extract_path, "hello.txt")) == "hello world\n",
        `tar.{variant.name}: extracted hello.txt has wrong content`
    )
    assert(
        fs.readfile(fs.path.join(extract_path, "dir", "nested.txt")) == "i am nested\n",
        `tar.{variant.name}: extracted dir/nested.txt has wrong content`
    )
end

-- the compressed variants should actually be smaller than the plain tar
const big = archive.create()
big:append({ type = "File", path = "big.txt", content = string.rep("seal seal seal ", 20000) })
const plain_size = buffer.len(big:serialize("Tar"))
for _, variant in variants do
    if variant.name == "uncompressed" then
        continue
    end
    const compressed_size = buffer.len(big:serialize(variant.format))
    assert(
        compressed_size < plain_size,
        `tar.{variant.name} didn't compress: {compressed_size} bytes vs {plain_size} for a plain tar`
    )
end

-- reading a tar as the wrong variant should error rather than silently misbehave
assert(not pcall(function()
    return tar.gz.load(sample():serialize("TarBz2"))
end), "loading a tar.bz2 as tar.gz should error")

--[=[
    Symlinks are always preserved when loading into memory (inspecting a symlink entry is
    harmless); `allow_symlinks` gates *materializing* them on disk, which is the dangerous part.
]=]
const linked = archive.create()
linked:append({ type = "File", path = "real.txt", content = "real" })
linked:append({ type = "Symlink", path = "link.txt", target = "real.txt" })
const linked_bytes = linked:serialize("Tar")

const link = tar.uncompressed.load(linked_bytes):get("link.txt")
assert(link and link.type == "Symlink", "the symlink should survive a tar roundtrip")
assert(link.target == "real.txt", `unexpected symlink target: {link.target}`)

-- ...but extracting one to disk requires opting in
const linked_path = fs.path.join(temp_path, "linked.tar")
tar.uncompressed.writefile(linked_path, linked)
assert(not pcall(function()
    tar.uncompressed.extract(linked_path, fs.path.join(temp_path, "linked_out"))
    return true
end), "extracting a tar containing a symlink should error unless allow_symlinks is set")

fs.dir.try_remove(temp_path)
