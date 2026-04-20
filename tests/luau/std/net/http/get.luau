--!strict
--!nolint LocalShadow
local http = require("@std/net/http")
local time = require("@std/time")

local todo1 = http.get {
    url = "https://jsonplaceholder.typicode.com/todos/1",
    timeout = {
        send_request = time.seconds(2),
    }
}

local postsuserid1 = http.get({
    url = "https://jsonplaceholder.typicode.com/posts",
    params = {
        userId = "1",
    },
})

-- bad uri
local s, err = pcall(function()
    return http.get {
        url = "meow is cool",
    }
end)

assert(s == false, "should be not successful with spaces in url")
assert(typeof(err) == "error" and tostring(err):match("invalid uri character"), "we shouldn't be able to send a request with bad uri character")

local notfound = http.get {
    url = "https://jsonplaceholder.typicode.com/posts/200",
}

assert(notfound.ok == false, "404 should be not ok")
assert(notfound.status.code == 404, "status code should be 404")
assert(notfound.status.reason == "Not Found", "notfound status reason should be 'Not Found'")
