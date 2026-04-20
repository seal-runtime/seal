--!strict
--!nolint LocalShadow

local http = require("@std/net/http")
local time = require("@std/time")

-- GET: basic success, response fields
local get_response = http.request("GET", {
    url = "https://jsonplaceholder.typicode.com/todos/1",
} :: http.HttpRequestWithoutBody)

assert(typeof(get_response) == "table", "GET should return an HttpResponse table")
assert(get_response.ok == true, "GET /todos/1 should be ok")
assert(get_response.status.code == 200, "GET /todos/1 should be 200")
assert(get_response.status.reason == "OK", "GET /todos/1 reason should be OK")
assert(get_response.body ~= nil and #get_response.body > 0, "GET response should have a body")

-- GET: 404 is returned as ok=false HttpResponse, not an HttpError
local notfound = http.request("GET", {
    url = "https://jsonplaceholder.typicode.com/posts/99999",
} :: http.HttpRequestWithoutBody)

assert(typeof(notfound) == "table", "404 should still be an HttpResponse table")
assert(notfound.ok == false, "404 should have ok=false")
assert(notfound.status.code == 404, "404 status code should be 404")

-- POST: 201 Created
local post_response = http.request("POST", {
    url = "https://jsonplaceholder.typicode.com/posts",
    body = {
        title = "seal test",
        body = "testing http.request POST",
        userId = 1,
    },
} :: http.HttpRequestWithBody)

assert(typeof(post_response) == "table", "POST should return an HttpResponse table")
assert(post_response.ok == true, "POST /posts should be ok")
assert(post_response.status.code == 201, "POST /posts should be 201 Created")

-- PUT: update a resource
local put_response = http.request("PUT", {
    url = "https://jsonplaceholder.typicode.com/posts/1",
    body = {
        id = 1,
        title = "updated title",
        body = "updated body",
        userId = 1,
    },
} :: http.HttpRequestWithBody)

assert(typeof(put_response) == "table", "PUT should return an HttpResponse table")
assert(put_response.ok == true, "PUT /posts/1 should be ok")
assert(put_response.status.code == 200, "PUT /posts/1 should be 200")

-- PATCH: partial update
local patch_response = http.request("PATCH", {
    url = "https://jsonplaceholder.typicode.com/posts/1",
    body = { title = "patched title" },
} :: http.HttpRequestWithBody)

assert(typeof(patch_response) == "table", "PATCH should return an HttpResponse table")
assert(patch_response.ok == true, "PATCH /posts/1 should be ok")
assert(patch_response.status.code == 200, "PATCH /posts/1 should be 200")

-- DELETE: returns 200 on jsonplaceholder
local delete_response = http.request("DELETE", {
    url = "https://jsonplaceholder.typicode.com/posts/1",
} :: http.HttpRequestWithoutBody)

assert(typeof(delete_response) == "table", "DELETE should return an HttpResponse table")
assert(delete_response.ok == true, "DELETE /posts/1 should be ok")
assert(delete_response.status.code == 200, "DELETE /posts/1 should be 200")

-- Timeout: tiny timeout returns HttpError, does not throw
local timeout_response = http.request("GET", {
    url = "https://meow.com",
    timeout = time.microseconds(14),
} :: http.HttpRequestWithoutBody)

assert(typeof(timeout_response) == "HttpError", "tiny timeout should return HttpError, not throw")
assert(timeout_response.kind == "Timeout", "kind should be Timeout")
assert(timeout_response.timeout_phase == "Global", "global timeout should set timeout_phase to Global")
assert(typeof(timeout_response.message) == "string" and #timeout_response.message > 0, "timeout message should be non-empty")

-- BodyExceedsLimit: small max_body_size on a large response returns HttpError
local limit_response = http.request("GET", {
    url = "https://jsonplaceholder.typicode.com/posts",
    max_body_size = 100,
} :: http.HttpRequestWithoutBody)

assert(typeof(limit_response) == "HttpError", "exceeding max_body_size should return HttpError")
assert(limit_response.kind == "BodyExceedsLimit", "kind should be BodyExceedsLimit")

-- Bad URI: static error, should throw not return HttpError
local ok, err = pcall(function()
    return http.request("GET", { url = "not a valid url !!" } :: http.HttpRequestWithoutBody)
end)

assert(ok == false, "bad URI should throw")
assert(typeof(err) == "error" and tostring(err):match("invalid url"), "error message should mention invalid url")

-- try_json: returns decoded table for a json response
type Todo = { id: number, title: string, completed: boolean, userId: number }
local json_response = http.request("GET", {
    url = "https://jsonplaceholder.typicode.com/todos/1",
} :: http.HttpRequestWithoutBody)

assert(typeof(json_response) == "table", "json_response should be an HttpResponse")
local decoded = json_response:try_json() :: Todo?
assert(decoded ~= nil, "try_json should return a table for a json response")
assert(decoded.id == 1, "try_json decoded id should be 1")
assert(typeof(decoded.title) == "string", "try_json decoded title should be a string")
assert(typeof(decoded.completed) == "boolean", "try_json decoded completed should be a boolean")

-- try_json: returns nil for a non-json body (plain text endpoint)
local text_response = http.request("GET", {
    url = "https://www.rfc-editor.org/rfc/rfc2549.txt",
} :: http.HttpRequestWithoutBody)

assert(typeof(text_response) == "table", "text_response should be an HttpResponse")
local text_decoded = text_response:try_json()
assert(text_decoded == nil, "try_json should return nil for a non-json response")

-- expect_json: returns decoded table for a json response
local expected = json_response:expect_json() :: Todo
assert(expected.id == 1, "expect_json decoded id should be 1")

-- expect_json: throws for a non-json mime type
local ok2, err2 = pcall(function()
    return text_response:expect_json<<Todo>>()
end)

assert(ok2 == false, "expect_json should throw on non-json response")
assert(typeof(err2) == "error", "expect_json error should be an error type")

