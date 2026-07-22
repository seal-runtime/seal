local tar = require("@std/archive/tar")
local env = require("@std/env")
local fs = require("@std/fs")
local format = require("@std/io/format")
local json = require("@std/serde/json")
local http = require("@std/net/http")
local process = require("@std/process")
local str = require("@std/str")

if env.os ~= "Linux" then
    return
end

format.defaults {
    max_depth = 2,
    indent_spaces = 2,
}

const endpoint = "https://api.github.com/repos/seal-runtime/seal/releases"

const temp_path = fs.path.join(script:parent(), "temp")
fs.dir.ensure(temp_path, true)

const cache_path = fs.path.join(temp_path, "cache.json")
const tar_path = fs.path.join(temp_path, "seal_release.tar.gz")
const temp_seal_path = fs.path.join(temp_path, "seal")

type ReleaseInfo = {
    {
        assets: { Asset },
    }
}

type Asset = {
    browser_download_url: string,
    content_type: string,
    name: string
}

local data: ReleaseInfo
if fs.path.exists(cache_path) then
    data = json.readfile(cache_path) :: ReleaseInfo
else
    const result = http.get {
        url = endpoint,
        headers = {
            accept = "application/vnd.github+json",
            ["X-GitHub-Api-Version"] = "2026-03-10"
        }
    }
    
    data = result:expect_json<<ReleaseInfo>>()
    json.writefile(cache_path, data :: any)
end

local bytes: buffer
if fs.path.exists(tar_path) then
    bytes = fs.readbytes(tar_path)
else
    -- print(data[1].assets[1].browser_download_url)
    const latest_release = data[1]
    assert(latest_release, "can't find latest release")
    
    local url_to_download: string?
    for _, asset in latest_release.assets do
        if
            str.endswith(asset.name, ".tar.gz")
            and string.find(asset.name, "linux")
        then
            url_to_download = asset.browser_download_url
            break
        end
    end
    assert(url_to_download, "can't find url to download")
    
    const response = http.get {
        url = url_to_download,
    }
    assert(response.ok, "cannot download bc error response, got " .. format(response.status))
    
    bytes = buffer.fromstring(response.body)
    fs.writefile(tar_path, bytes)
end
assert(bytes, "can't get buffer")

const archive = tar.gz.load(bytes)
const seal_data = archive:get("seal")
if seal_data then
    assert(seal_data.type == "File")
    const exe_bytes = seal_data.content
    fs.writefile(temp_seal_path, exe_bytes)
    process.shell(`chmod +x {temp_seal_path}`)
    const result = process.run {
        program = temp_seal_path,
        args = { "--help" },
    }
    assert(result.ok, "download reponse not successful")
    assert(string.match(result.out, "cutest"), "seal --help doesn't have 'cutest' in it anymore?")
end

fs.dir.try_remove(temp_path)
