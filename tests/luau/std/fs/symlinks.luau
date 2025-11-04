local env = require("@std/env")
local fs = require("@std/fs")
local try = require("@tests/try")

local symlink_path = fs.path.join(".", "tests", "data", "some_symlink")
local target = fs.path.canonicalize("./.vscode/settings.json")

local function creatingsymlinks()
    if env.os == "Linux" or env.os == "MacOS" then
        fs.symlink(target, symlink_path)
        assert(fs.is(symlink_path) == "Symlink", "symlink to .vscode/settings.json should be symlink")
    end
end

creatingsymlinks()

local function seeingwheresymlinkgoes()
    if env.os == "Linux" or env.os == "MacOS" then
        local path = fs.readlink(symlink_path)
        assert(path == target, "symlink path should == target")
    end
end

seeingwheresymlinkgoes()

local function removingsymlinks()
    if env.os == "Linux" or env.os == "MacOS" then
        if fs.is(symlink_path) == "Symlink" then
            fs.unsymlink(symlink_path)
        end
        assert(fs.is(symlink_path) == "NotFound", "after removing symlink_path should be NotFound")
    end
end

removingsymlinks()

local function readlinknotasymlink()
    try(function()
        return fs.readlink(fs.path.join(".", "tests", "data", "cats.json"))
    end):throws("leads to a real file, not a symlink")
end

readlinknotasymlink()