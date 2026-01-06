local env = require("@std/env")

print(env.vars.get("PROD"))
print(env.vars.get("SHELL"))