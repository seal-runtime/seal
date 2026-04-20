local http = require("@std/net/http")

local result = http.post {
    url = "https://jsonplaceholder.typicode.com/posts",
    body = {
        title = "meow",
        body = "somebodytexthmm",
        userId = 2,
    }
}

assert(result.ok, "unless internet down this should work")

assert(result.status.code == 201, "status code should be 201 created")
assert(result.status.reason == "Created", "status reason for 201 should be 'Created'")

assert(
    (
        result.content_type and
        result.content_type.charset == "utf-8"
        and result.headers ~= nil
        and result.content_type.mime_type == "application/json"
    ),
    "HttpResponse sanity checks"
)