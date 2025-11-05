--[=[
Lib for serializing/deserializing/encoding/decoding different things.
]=]
local serde = {}

serde.json = require("@self/json")

serde.base64 = require("@self/base64")

serde.toml = require("@self/toml")

serde.yaml = require("@self/yaml")

return serde