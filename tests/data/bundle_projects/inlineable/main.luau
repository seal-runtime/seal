local thread = require("@std/thread")
local time = require("@std/time")
local check = require("./check")

local urls = {
    "www.meow.com",
    "www.polytoria.com",
    "www.deviaze.com",
}

local pool: {
    {
        url: string,
        handle: thread.ThreadHandle,
    }
} = {}

for index, url in urls do
    if check.compat(url) then
        local abs_url = check.url(url)
        local handle = thread.spawn {
            path = "./threads/call_api.luau",
            data = {
                url = url,
            }
        }
        table.insert(pool, {
            url = abs_url,
            handle = handle
        })
    end
end

while time.wait(time.milliseconds(50)) do
    for index, data in pool do
        local resp = data.handle:read()
        if resp then
            pool[index] = nil
            break
        end
    end
end
