local fs = require("@std/fs")

local typedefs = fs.dir.project():join(".seal", "typedefs")

local files_with_tabs: { string } = {}

local path_sub_out = fs.path.join(fs.path.cwd(), ".seal", "typedefs")

for _, file in fs.listdir(typedefs, true) do
    for _, line in fs.readlines(file) do
        if string.match(line, "^\t") then
            table.insert(files_with_tabs, (file:gsub(path_sub_out, "./..")))
            break
        end
    end
end

print(files_with_tabs)