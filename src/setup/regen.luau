local colors = require("@std/colors")
local env = require("@std/env")
local fs = require("@std/fs")
local prompt = require("@std/io/prompt")
local luau = require("@std/luau")
local process = require("@std/process")
local semver = require("@std/semver")
local seal_setup = (require :: any)("@internal/setup") :: {
    extract_typedefs: (path: string) -> (),
    extract_extra: (path: string) -> (),
    GUIDED_TOUR_SRC: string,
    DEFAULT_CONFIG_SRC: string,
}

local function dircolor(path: string, no_red: boolean?): string
    if no_red == nil then
        return colors.blue(path) .. colors.codes.RED -- error messages are red so let's not make them white halfway thru
    else
        return colors.blue(path)
    end
end

local function get_dotseal_path(): string
    local function from_project_dir(): string
        local project_path = fs.path.project(nil, env.cwd())
        if not project_path then
            error(`can't find project path to replace type definitions of,\n  make sure your cwd is in a folder that contains a {dircolor(".seal")} directory or you explicitly pass the project path like 'seal regen <path>'`)
        end
        local dotseal_path = fs.path.join(project_path, ".seal")
        if not fs.path.exists(dotseal_path) then
            error(`implicit project path at '{dircolor(project_path)}' does not contain a '{dircolor(".seal")}' folder`)
        end
        return dotseal_path
    end

    local path = env.args[1]
    if not path then
        return from_project_dir()
    end

    -- if user explicitly passed a project directory in we should check it instead
    local project_path = fs.path.project(nil, path) or path

    if fs.is(path) ~= "Directory" then
        error(`expected explicit <path> to point to a directory with a '{dircolor(".seal")}' folder, but '{project_path}' is not a directory, it's a {fs.is(project_path)}`)
    end

    local dotseal_path = fs.path.join(project_path, ".seal")

    if fs.is(dotseal_path) ~= "Directory" then
        error(`cannot find '{dircolor(dotseal_path)}'; it doesn't exist or isn't a directory`)
    end

    return dotseal_path    
end

local old_seal_version: string?

local function version_mismatch(seal_dir: string): boolean
    local self_version = semver.from(_SEAL_VERSION)
    local config_path = fs.path.join(seal_dir, "config.luau")
    local contents = fs.file.try_read(config_path)
    if not contents then
        -- if config.luau doesn't exist let's just assume user does not care if there's a version mismatch
        return false
    end

    local result = luau.eval(contents)
    if typeof(result) == "error" then
        error(`cannot read config.luau at {config_path} due to err: {tostring(result)}`)
    else
        result = result :: {
            seal_version: string?
        }
    end

    if not result.seal_version then
        -- similarly, assume user doesn't care about version mismatches
        return false
    end

    local project_version = semver.from(result.seal_version)

    if self_version > project_version then
        old_seal_version = tostring(project_version)
        return true
    end

    return false
end

local function update_config_luau_version(config_path: string)
    local lines: { string } = {}
    for _, line in fs.readlines(config_path) do
        if string.match(line, "seal_version = \"") then
            line = `    seal_version = "{_SEAL_VERSION}"`
        end
        table.insert(lines, line)
    end
    fs.writefile(config_path, table.concat(lines, "\n"))
end

local path = get_dotseal_path()

-- check if version mismatch
local should_force = false -- TODO: not hardcode this
local mismatch = version_mismatch(path)
if mismatch and not should_force then
    if not prompt.confirm(`seal regen: seal version > project version; update project typedefs to {colors.green(_SEAL_VERSION)} [Y/n]?`) then
        print("seal regen: exiting without replacing typedefs.")
        process.exit(1)
    end
end

local typedefs_path = fs.path.join(path, "typedefs")
if fs.is(typedefs_path) ~= "Directory" then
    error(`typedefs path at '{dircolor(typedefs_path)}' is not a directory--you're probably in a script codebase. if so, go to your ~/.seal or SEAL_DIR folder and regen its typedefs instead`)
end

fs.removetree(typedefs_path)

fs.makedir(typedefs_path)
seal_setup.extract_typedefs(typedefs_path)

if mismatch and old_seal_version then
    local config_path = fs.path.join(path, "config.luau")
    update_config_luau_version(config_path)
    print(`\nUpdated project's typedefs from {colors.red(old_seal_version)} -> {colors.green(_SEAL_VERSION)}`)
    print(`Updated {colors.yellow((config_path:gsub(env.cwd(), ".")))}`)
else
    print(`Regenerated project typedefs at '{dircolor(typedefs_path, true)}`)
end

