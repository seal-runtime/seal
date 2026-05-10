-- simulates what a file looks like after being bundled once with thread.spawn path inlining:
-- the require inside the long string is inlined source code and must not be replaced when bundled again
local inlined_src = [=====[
local some_module = require("./some_module")
return some_module.value
]=====]
return inlined_src
