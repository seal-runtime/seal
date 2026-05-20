local str = require("@std/str")
local function check_url(location: string): string
    local url = location
    if not str.startswith(location, "https://") then
        url = "https://" .. url
    end
    return url
end

local function check_compat(url: string)
    return true
end

return {
    url = check_url,
    compat = check_compat,
}