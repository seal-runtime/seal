local cursor = require("@std/io/cursor")
local output = require("@std/io/output")
local terminal = require("@std/io/terminal")

-- local command = cursor.commands.to(vector.create(1, 2))
cursor.move(
    cursor.commands.to(vector.create(3, 2))
)