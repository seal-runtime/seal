local fs = require("@std/fs")
-- semver is very cursed so we reparse the outputted md 

local function fix()
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

return fix