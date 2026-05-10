-- these require() calls are inside regular quoted strings and must not be bundled
local double_quoted = "local db = require('./database')"
local single_quoted = 'local fs = require("@std/fs")'

return { double_quoted = double_quoted, single_quoted = single_quoted }
