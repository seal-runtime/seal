<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# net.http.server

`local server = require("@std/net/http/server")`

---

StatusCode: `| "200 OK"`

---

StatusCode: `| "201 Created"`

---

StatusCode: `| "204 No Content"`

---

StatusCode: `| "301 Moved Permanently"`

---

StatusCode: `| "302 Found"`

---

StatusCode: `| "304 Not Modified"`

---

StatusCode: `| "307 Temporary Redirect"`

---

StatusCode: `| "308 Permanent Redirect"`

---

StatusCode: `| "400 Bad Request"`

---

StatusCode: `| "401 Unauthorized"`

---

StatusCode: `| "403 Forbidden"`

---

StatusCode: `| "404 Not Found"`

---

StatusCode: `| "405 Method Not Allowed"`

---

StatusCode: `| "409 Conflict"`

---

StatusCode: `| "410 Gone"`

---

StatusCode: `| "412 Precondition Failed"`

---

StatusCode: `| "415 Unsupported Media Type"`

---

StatusCode: `| "429 Too Many Requests"`

---

StatusCode: `| "500 Internal Server Error"`

---

StatusCode: `| "501 Not Implemented"`

---

StatusCode: `| "502 Bad Gateway"`

---

StatusCode: `| "503 Service Unavailable"`

---

StatusCode: `| "504 Gateway Timeout"`

---

StatusCode: `| "505 HTTP Version Not Supported"`

---

`export type` ContentType

---

ContentType: `| "Text"`

---

ContentType: `| "HTML"`

---

ContentType: `| "JSON"`

---

ContentType: `| "XML"`

---

ContentType: `| "CSS"`

---

ContentType: `| "JavaScript"`

---

ContentType: `| "Binary"`

---

ContentType: `| string`

---

`export type` ServeRequest

---

ServeRequest.peer_address: `string`

---

ServeRequest.method: `"GET" | "POST" | "PUT" | "PATCH" | "DELETE"`

---

ServeRequest.path: `string`

---

ServeRequest.raw_text: `string`

---

ServeRequest.body: `string`

---

`export type` ServeResponse

---

ServeResponse.status_code: `StatusCode`

---

ServeResponse.content_type: `ContentType`

---

ServeResponse.body: `string`

---

ServeResponse.headers.cookies.http_version: `string?`

---

ServeResponse.headers.cookies.reason_phrase: `string?`

---

ServeResponse.headers.cookies.redirect_url: `string?`

---

`export type` ServeConfig

---

ServeConfig.address: `string`

---

ServeConfig.port: `string | number`

---

ServeConfig.handler: `(ServeRequest) -> ServeResponse`

---

server.serve: `(config: ServeConfig)`

<details>

<summary> See the docs </summary

Create a webserver that listens for incoming requests.

⚠️ Expect breaking changes. This API will be heavily modified in the future.

## Usage

```luau
local server = require("@std/net/http/server")
local fs = require("@std/fs")
local json = require("@std/json")

print("starting seal server")

server.serve {
    address = "localhost",
    port = 4242,
    handler = function(info: server.ServeRequest)
        local response = {}
        if info.path == "/meow.json" then
            response.status_code = "200 OK"
            response.content_type = "json"
            response.body = json.encode {
                ok = true,
                says = "meow"
            }
        elseif info.path == "/" then
            local meow_page = fs.readfile("./tests/data/server-views/index.html")
            response.status_code = "200 OK"
            response.content_type = "html"
            response.body = meow_page
        elseif info.path == "/info" then
            local body = fs.readfile("./tests/data/server-views/info.html")
            response = {
                status_code = "200 OK",
                content_type = "html",
                body = body
            }
        elseif info.path == "/some-post" then
            response = {
                status_code = "200 OK",
                content_type = "application/json",
                body = json.encode {
                    ok = true,
                    recvbody = info.body,
                }
            }
        else
            response.status_code = "404 Not Found"
            response.response_type = "json"
            response.body = json.encode {
                ok = false,
            }
        end
        return response :: server.ServeResponse
    end
}
```

</details>

---
