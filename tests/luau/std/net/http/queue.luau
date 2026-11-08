--!strict
local http = require("@std/net/http")

local BASE = "https://jsonplaceholder.typicode.com"

-- every request comes back once with the key it was added with
local queue = http.queue(4)
local type_name: string = typeof(queue)
assert(type_name == "HttpQueue", "http.queue should return an HttpQueue")
assert(#queue == 0, "a new queue should be empty")

for id = 1, 6 do
    assert(queue:get(`{BASE}/todos/{id}`, `todo {id}`) == `todo {id}`, "get should return the key it was given")
end
assert(#queue == 6, "#queue should count unfinished requests")

local seen: { [unknown]: boolean } = {}
for key, result in queue do
    assert(not seen[key], `key {key} came back twice`)
    seen[key] = true
    assert(result.ok, `request {key} should succeed`)
end
for id = 1, 6 do
    assert(seen[`todo {id}`], `todo {id} never came back`)
end
assert(#queue == 0, "iterating should empty the queue")

-- keys default to numbers counting up from 1 per queue
local numbered = http.queue(2)
assert(numbered:get(`{BASE}/todos/1`) == 1, "first default key should be 1")
assert(numbered:request("GET", { url = `{BASE}/todos/2` } :: http.HttpRequestWithoutBody) == 2, "second default key should be 2")
for _ in numbered do end

-- breaking out leaves the rest in the queue for the next loop
for id = 1, 4 do
    queue:get(`{BASE}/posts/{id}`)
end
for _ in queue do
    break
end
assert(#queue == 3, "breaking after one result should leave three")
local rest = 0
for _ in queue do
    rest += 1
end
assert(rest == 3, "the second loop should get the other three results")

-- requests can be added while iterating
local added = false
local total = 0
queue:get(`{BASE}/todos/1`)
for _ in queue do
    total += 1
    if not added then
        added = true
        queue:get(`{BASE}/todos/2`)
    end
end
assert(total == 2, "a request added while iterating should be iterated in the same loop")

-- POST bodies are sent like http.post
queue:post({
    url = `{BASE}/posts`,
    body = {
        title = "queued",
    },
}, "post")
for key, result in queue do
    assert(key == "post", "post key should come back")
    assert(result.ok and result.status.code == 201, "POST /posts should be 201 Created")
end

-- network errors come back as results instead of throwing
queue:get("http://127.0.0.1:1/", "refused")
for key, result in queue do
    assert(key == "refused", "refused key should come back")
    assert(not result.ok and result.kind ~= "Ok" and result.kind ~= "Error", "a refused connection should be an HttpIoError")
end

-- cancel drops everything that hasn't been iterated
for id = 1, 5 do
    queue:get(`{BASE}/todos/{id}`)
end
queue:cancel()
assert(#queue == 0, "cancel should empty the queue")
for key in queue do
    error(`cancelled request {key} came back`)
end
queue:get(`{BASE}/todos/3`, "after cancel")
for key in queue do
    assert(key == "after cancel", "only the request added after cancel should come back")
end

-- cancel with a key drops every request added with that key and leaves the rest
-- one worker leaves most requests waiting, which used to hang the loop
local single = http.queue(1)
single:get(`{BASE}/todos/1`, "a")
single:get(`{BASE}/todos/2`, "b")
single:get(`{BASE}/todos/3`, "c")
single:get(`{BASE}/todos/4`, "b")
single:cancel("b")
assert(#single == 2, "cancelling 'b' should leave 'a' and 'c'")
local kept: { [unknown]: boolean } = {}
for key, result in single do
    assert(key ~= "b", "a cancelled request came back")
    assert(result.ok, `request {key} should succeed`)
    kept[key] = true
end
assert(kept.a and kept.c, "'a' and 'c' should both come back")

-- invalid options and workers throw when the request is added
assert(not pcall(function()
    return http.queue(0)
end), "0 workers should throw")
assert(not pcall(function()
    return http.queue(2.5)
end), "fractional workers should throw")
assert(not pcall(function()
    return queue:get({ url = "not a url" } :: http.HttpRequestWithoutBody)
end), "an invalid url should throw when added")
assert(#queue == 0, "a request that threw shouldn't be in the queue")
