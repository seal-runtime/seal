-- tests the Archive extern type itself: create, append, insert, remove, get, paths, #, serialize

local archive = require("@std/archive")

type ArchiveFormat = archive.ArchiveFormat

const a = archive.create()
assert(#a == 0, "a freshly created Archive should be empty")
assert(#a:paths() == 0, "a freshly created Archive should have no paths")
assert(a:get("nope.txt") == nil, "get should return nil for a missing path")
assert(a:get(1) == nil, "get should return nil for an out of range index")

-- append, single and variadic
a:append({ type = "File", path = "hello.txt", content = "hello world" })
a:append(
    { type = "Directory", path = "dir" },
    { type = "File", path = "dir/nested.txt", content = "nested!" }
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
assert(hello.content == "hello world", `unexpected content: {hello.content}`)

-- get by index, which should agree with get by path
const first = a:get(1)
assert(first, "couldn't get entry at index 1")
assert(first.path == "hello.txt", `expected index 1 to be hello.txt, got {first.path}`)

const dir = a:get("dir")
assert(dir and dir.type == "Directory", "expected dir to be a Directory entry")

-- insert shifts everything after it down
a:insert(1, { type = "File", path = "first.txt", content = "i am first" })
assert(#a == 4, `expected 4 entries after insert, got {#a}`)
const after_insert = a:paths()
assert(after_insert[1] == "first.txt", `insert didn't put first.txt at index 1, got {after_insert[1]}`)
assert(after_insert[2] == "hello.txt", `insert didn't shift hello.txt to index 2, got {after_insert[2]}`)

-- remove by index
a:remove(1)
assert(#a == 3, `expected 3 entries after remove, got {#a}`)
assert(a:get("first.txt") == nil, "first.txt should be gone after being removed")
assert(a:paths()[1] == "hello.txt", "hello.txt should be back at index 1 after the remove")

-- symlink entries keep their target
const linked = archive.create()
linked:append({ type = "File", path = "real.txt", content = "real" })
linked:append({ type = "Symlink", path = "link.txt", target = "real.txt" })
const link_bytes = linked:serialize("Tar")
const link_loaded = require("@std/archive/tar").uncompressed.load(link_bytes, { allow_symlinks = true })
const link = link_loaded:get("link.txt")
assert(link, "couldn't get the symlink back out of the archive")
assert(link.type == "Symlink", `expected a Symlink, got {link.type}`)
assert(link.target == "real.txt", `expected symlink target "real.txt", got {link.target}`)

-- serialize works for every format in the ArchiveFormat union, and each is non-empty
const formats: { ArchiveFormat } = { "Zip", "Tar", "TarGz", "TarBz2", "TarXz", "TarZst", "TarLz4", "7z" }
for _, format in formats do
    const bytes = a:serialize(format)
    assert(buffer.len(bytes) > 0, `serializing to {format} produced an empty buffer`)
end

-- duplicate paths: get returns the *first* match, per the typedef
const dupes = archive.create()
dupes:append({ type = "File", path = "dupe.txt", content = "first" })
dupes:append({ type = "File", path = "dupe.txt", content = "second" })
const dupe = dupes:get("dupe.txt")
assert(dupe and dupe.type == "File", "couldn't get the duplicated path")
assert(dupe.content == "first", `get should return the first match, got {dupe.content}`)
