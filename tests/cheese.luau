local err = require("@std/err")
local cheese = {}

function cheese.retry(f: () -> (any), tries: number?)
    tries = tries or 3
    local current = 0
    local last_err: error
    while current < tries do
        local success, err = pcall(f)
        if success then
            return
        else
            last_err = err :: error
            current += 1
        end
    end
    if typeof(last_err) == "error" then
        err.throw(last_err)
    else
        error(last_err)
    end
end

return cheese