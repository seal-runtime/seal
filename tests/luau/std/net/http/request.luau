--!strict
--!nolint LocalShadow

local filesize = require("@std/fs/filesize")
local http = require("@std/net/http")
local time = require("@std/time")

-- GET: basic success, response fields
local get_response = http.request("GET", {
    url = "https://jsonplaceholder.typicode.com/todos/1",
} :: http.HttpRequestWithoutBody)

assert(get_response.kind == "Ok", "should work")
assert(get_response.ok == true, "200 is an OK status code")
assert(get_response.status.code == 200, "GET /todos/1 should be 200")
assert(get_response.status.reason == "OK", "Get /todos/1 reason should be OK")
assert(get_response.body ~= nil and #get_response.body > 0, "GET response should have a body")

-- GET: 404 is returned as ok=false HttpResponse, not an HttpError
local notfound = http.request("GET", {
    url = "https://jsonplaceholder.typicode.com/posts/99999",
} :: http.HttpRequestWithoutBody)

assert(notfound.kind == "Error", "404 should still be a response not an error/timeout")
assert(notfound.ok == false, "404 is not an OK status code")
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

assert(post_response.kind == "Ok", "POST should return a response")
assert(post_response.ok == true, "201 is an OK status code")
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

assert(put_response.kind == "Ok", "PUT shuld return a Response")
assert(put_response.ok == true, "PUT 200 is an OK status code")
assert(put_response.status.code == 200, "PUT /posts/1 should be 200")

-- PATCH: partial update
local patch_response = http.request("PATCH", {
    url = "https://jsonplaceholder.typicode.com/posts/1",
    body = { title = "patched title" },
} :: http.HttpRequestWithBody)

assert(patch_response.kind == "Ok", "PATCH should return a Response")
assert(patch_response.ok == true, "PATCH 200 is an OK status code")
assert(patch_response.status.code == 200, "PATCH /posts/1 should be 200")

-- DELETE: returns 200 on jsonplaceholder
local delete_response = http.request("DELETE", {
    url = "https://jsonplaceholder.typicode.com/posts/1",
} :: http.HttpRequestWithoutBody)

assert(delete_response.kind == "Ok", "DELETE should return a response")
assert(delete_response.ok == true, "DELETE 200 is an OK status code")
assert(delete_response.status.code == 200, "DELETE /posts/1 should be 200")

-- Timeout: tiny timeout returns HttpTimeoutError, does not throw
local timeout_response = http.request("GET", {
    url = "https://meow.com",
    timeout = time.microseconds(14),
} :: http.HttpRequestWithoutBody)

assert(timeout_response.kind == "Timeout", "tiny timeout should not be a Response it should be Timeout")
assert(timeout_response.ok == false, "a TimeoutError is not OK")
assert(timeout_response.phase == "Global", "should be a global timeout")
assert(typeof(timeout_response.reason) == "string" and #timeout_response.reason > 0, "timeout message should be non-empty")

-- BodyExceedsLimit: small max_body_size on a large response returns HttpError
local limit_response = http.request("GET", {
    url = "https://jsonplaceholder.typicode.com/posts",
    max_body_size = filesize.bytes(100),
} :: http.HttpRequestWithoutBody)
assert(limit_response.kind == "BodyExceedsLimit", "exceeding max_body_size should result in BodyExceedsLimit")
assert(limit_response.ok == false, "BodyExceedsLimit is not OK")

-- Bad URI: static error, should throw not return any kind of error
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

assert(json_response.kind == "Ok")
assert(json_response.ok == true, "json response should be OK")
local decoded = json_response:try_json<<Todo>>()
assert(decoded ~= nil, "try_json should return table for json response")
assert(decoded.id == 1, "try_json id should be 1")
assert(typeof(decoded.title) == "string", "try_json decoded title sholud be a string")
assert(typeof(decoded.completed) == "boolean", "try_json decoded.completed should be a boolean")

-- try_json: returns nil for a non-json body (plain text endpoint)
local text_response = http.request("GET", {
    url = "https://www.rfc-editor.org/rfc/rfc2549.txt",
} :: http.HttpRequestWithoutBody)

assert(text_response.kind == "Ok", "text_response should be Ok HttpResponse")
assert(text_response.ok == true, "text_response should be OK status code")
local decoded = text_response:try_json()
assert(decoded == nil, "calling rfc editor endpoint shouldn't result in json decodeable response")

-- expect_json: returns decoded table for a json response
local expected = json_response:expect_json<<Todo>>()
assert(expected.id == 1, "expect_json decoded id should be 1")

-- expect_json: throws for a non-json mime type
local ok2, err2 = pcall(function()
    return text_response:expect_json<<Todo>>()
end)

assert(ok2 == false, "expect_json should throw on non-json response")
assert(typeof(err2) == "error", "expect_json error should be an error type")
