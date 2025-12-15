-- TODO: remove this and everything it did when mluau updates to support <<>> syntax
local fs = require("@std/fs")
local str = require("@std/str")

local rust_files = fs.listdir("./src", true, function(p)
    return str.endswith(p, ".rs")
end)

for _, file in rust_files do
    local lines: { string } = {}
    for _, line in fs.readlines(file) do
        local loading_identifier = string.match(line, "luau%.load%(([%w_]+)%)")
        if loading_identifier then
            line = line:gsub("luau%.load%(([%w_]+)%)", `luau.load(temp_transform_luau_src({loading_identifier}))`) .. " // <<>> HACK"
        end
        table.insert(lines, line)
    end
    fs.writefile(file, table.concat(lines, "\n"))
end
