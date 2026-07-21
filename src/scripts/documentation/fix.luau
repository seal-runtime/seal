local fs = require("@std/fs")
local str = require("@std/str")
const path = fs.path.join(script:parent(), "components.luau")

const lines = {}
for _, line in fs.readlines(path) do
    if str.startswith(line, "class") then
        print(line)
        line = "export " .. line
    end
    table.insert(lines, line)
end
fs.writefile(path, table.concat(lines, "\n"))