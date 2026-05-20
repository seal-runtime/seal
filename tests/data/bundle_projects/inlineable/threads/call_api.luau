local http = require("@std/net/http")
-- we need to test that relative require is properly inlined here in child thread
local check = require("../check")

if channel then
    local url = check.url(channel.data.url :: string)
    local response = http.get({
        url = url,
    })
    if response.ok then
        channel:send(response.body)
    end
end
