local zip = require("@std/archive/zip")
local fs = require("@std/fs")
local str = require("@std/str")

local path = "./tests/data/archives/zips/normal.zip"

-- local archive = zip.readfile(path)
-- local content = archive:read("hello.txt")
-- print(content)
local child = fs.path.child(path) :: string
local destination = str.trimback(
    fs.path.join(script:parent(), child),
    ".zip"
)
    
-- print(extract_path)
-- zip.extract(path, destination)

const archive = zip.readfile(path)

archive:add({
    type = "File",
    path = "meow.txt",
    content = "hi i am a cat definitely"
})

-- print(archive:paths())
local content = archive:serialize("Zip")
local outpath = fs.path.join(script:parent(), "out.zip")
-- fs.writefile(outpath, content)

-- zip.extract(outpath, fs.path.join(script:parent(), "out"))
fs.dir.try_remove(fs.path.join(script:parent(), "out"))
-- local badpath = "./tests/data/archives/zips/path_traversal.zip"



const badpath2 = "./tests/data/archives/zips/symlink_traversal.zip"
local contents = zip.readfile(badpath2, {
    allow_unsafe_path_traversals = true,
})
local paths = contents:paths()