local core = require("./core")

local dependencies = core.dependencies()
print(`dependencies: {table.concat(dependencies, ", ")}`)

return {
    got_dependencies = dependencies
}