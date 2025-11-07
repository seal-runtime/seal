<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# net.http.server

`local server = require("@std/net/http/server")`

`export type` ContentType

---

`export type` ServeRequest

---

ServeRequest.peer_address: `string`

---

ServeRequest.method: `"GET" | "POST" | "PUT" | "PATCH" | "DELETE"`

---

ServeRequest.path: `string`

---

ServeRequest.headers.raw_text: `string`

---

ServeRequest.headers.body: `string`

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

function server.serve(config: `ServeConfig)`

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
