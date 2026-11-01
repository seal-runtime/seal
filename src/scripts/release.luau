-- packages and creates a draft release of the current seal version
-- this runs in CI, not locally

const archive = require("@std/archive")
const entry = require("@std/archive/entry")
const fs = require("@std/fs")
const env = require("@std/env")
const process = require("@std/process")
const str = require("@std/str")

-- gets inherited by process.shell automatically
const _ = env.vars.get("GITHUB_TOKEN") or error("GITHUB_TOKEN not set")
--[=[
    Directory structure expected inside artifacts_dir:

    ```luau
    artifacts_dir/
    ├── seal-v{version}-linux-x64/
    │   └── seal
    ├── seal-v{version}-linux-arm64/
    │   └── seal
    ├── seal-v{version}-macos-darwin-arm64/
    │   └── seal
    ├── seal-v{version}-windows-x64/
    │   ├── seal.exe
    │   └── seal.pdb
    ├── seal-v{version}-windows-arm64/
    │   ├── seal.exe
    │   └── seal.pdb
    ├── seal-v{version}-android-arm64/
    │   └── seal
    └── seal-v{version}-android-armv7/  -- may be missing entirely; that build is best-effort/continue-on-error
        └── seal
    ```
]=]
const artifacts_dir = fs.dir.from(env.vars.get("SEAL_ARTIFACTS_DIR") or error("SEAL_ARTIFACTS_DIR not set"))
const seal_dir = fs.dir.cwd() -- cwd is known to be seal/ here in CI
const version = _SEAL_VERSION

const readme_entry = entry.read(seal_dir:join("README.md"))
assert(typeof(readme_entry) == "ArchiveEntry", "solely README.md is ArchiveEntry not a directory")

const license_entry = entry.read(seal_dir:join("LICENSE.txt"))
assert(typeof(license_entry) == "ArchiveEntry", "solely LICENSE.txt is a single ArchiveEntry, not a directory")

const docs_entries = entry.read(seal_dir:join("docs"))
assert(typeof(docs_entries) == "table", "docs is obviously a directory")

const function contains(path: string, substring: string)
    return string.find(path, substring, 1, true) ~= nil
end

-- 0o755 (rwxr-xr-x)
const EXECUTABLE_MODE = 0x1ED
const output_dir = fs.dir.ensure(seal_dir:join("release"))

const release_artifact_directories = artifacts_dir:list(false)
for _, directory in release_artifact_directories do
    local directory_name = fs.path.child(directory) or error(`can't get child (directory name) of {directory}`)
    const arc = archive.create()

    const is_linux = contains(directory, "linux")
    const is_android = contains(directory, "android")
    const is_mac = contains(directory, "macos")
    const is_unix = is_linux or is_android or is_mac

    if is_android and contains(directory_name, "arm64") then
        directory_name ..= "-aarch64-linux-android"
    elseif is_android and contains(directory_name, "armv7") then
        directory_name = str.trimback(directory_name, "armv7") .. "armv7-linux-androideabi"
    end

    const contents = fs.listdir(directory, true)
    for _, file in contents do
        const executable_or_pdb = entry.read(file)
        if 
            typeof(executable_or_pdb) == "ArchiveEntry" 
            and executable_or_pdb:try_file()
            and is_unix
        then
            if executable_or_pdb:path() == "seal" then
                executable_or_pdb:expect_file():set_unix_mode(EXECUTABLE_MODE)
            end
        end
        arc:append(executable_or_pdb)
    end

    arc:append(readme_entry, license_entry, docs_entries)

    if is_unix then
        const archive_name = directory_name .. ".tar.gz"
        archive.tar.gz.writefile(output_dir:join(archive_name), arc)
    else
        const archive_name = directory_name.. ".zip"
        archive.zip.writefile(output_dir:join(archive_name), arc)
    end
end

const tag = `v{version}`
const view_result = process.shell(`gh release view {tag}`)
if view_result.ok then
    const upload_result = process.shell(`gh release upload {tag} {output_dir.path}/* --clobber`)
    assert(upload_result.ok, `updating existing release {tag} failed with error {upload_result.stderr}`)
else
    const create_result = process.shell(`gh release create {tag} --draft --generate-notes {output_dir.path}/*`)
    assert(create_result.ok, `creating a new release failed with error {create_result.stderr}`)
end
