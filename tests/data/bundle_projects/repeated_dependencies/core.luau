local core = {}

core.version = _SEAL_VERSION

core.environment = require("@std")

function core.dependencies()
    return {
        "luau",
        "seal",
        "python",
        "uv"
    }
end

return core