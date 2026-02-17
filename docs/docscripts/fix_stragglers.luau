local fs = require("@std/fs")

--- semver is very cursed so we reparse the outputted md 
local function fix_semver()
    local path = "./docs/reference/std/semver.md"
    
    local lines: { string } = {}
    
    local skipping = false
    
    for _, line in fs.readlines(path) do
        if line == "## `export type` SemverImpl" then
            skipping = true
        end
        if skipping and string.match(line, "from") then
            skipping = false
        end
        if skipping then
            continue
        end
    
        if line:match("SemverImpl%.") then
            line = line:gsub("SemverImpl%.", "")
        elseif line:match("self: Semver") or line:match("###") then
            line = line:gsub("semver%.", "Semver.")
        end
    
        table.insert(lines, line)
    end
    
    fs.writefile(path, table.concat(lines, "\n"))
end

--- std/thread lib's export type has to be 'threadlib' now to avoid redefinition of primitive type typeerror
local function fix_thread()
    local path = "./docs/reference/std/thread.md"
    local contents = fs.readfile(path)
    contents = string.gsub(contents, "threadlib", "thread")
    fs.writefile(path, contents)
end

return {
    semver = fix_semver,
    thread = fix_thread,
}