<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# net.http.server

`local server = require("@std/net/http/server")`

---

```luau
| "200 OK"
```

---

```luau
| "201 Created"
```

---

```luau
| "204 No Content"
```

---

```luau
| "301 Moved Permanently"
```

---

```luau
| "302 Found"
```

---

```luau
| "304 Not Modified"
```

---

```luau
| "307 Temporary Redirect"
```

---

```luau
| "308 Permanent Redirect"
```

---

```luau
| "400 Bad Request"
```

---

```luau
| "401 Unauthorized"
```

---

```luau
| "403 Forbidden"
```

---

```luau
| "404 Not Found"
```

---

```luau
| "405 Method Not Allowed"
```

---

```luau
| "409 Conflict"
```

---

```luau
| "410 Gone"
```

---

```luau
| "412 Precondition Failed"
```

---

```luau
| "415 Unsupported Media Type"
```

---

```luau
| "429 Too Many Requests"
```

---

```luau
| "500 Internal Server Error"
```

---

```luau
| "501 Not Implemented"
```

---

```luau
| "502 Bad Gateway"
```

---

```luau
| "503 Service Unavailable"
```

---

```luau
| "504 Gateway Timeout"
```

---

```luau
| "505 HTTP Version Not Supported"
```

---

### `export type` ContentType

---

```luau
| "Text"
```

---

```luau
| "HTML"
```

---

```luau
| "JSON"
```

---

```luau
| "XML"
```

---

```luau
| "CSS"
```

---

```luau
| "JavaScript"
```

---

```luau
| "Binary"
```

---

```luau
| string
```

---

### `export type` ServeRequest

---

### ServeRequest.peer_address

```luau
ServeRequest.peer_address: string,
```

---

### ServeRequest.method

```luau
ServeRequest.method: "GET" | "POST" | "PUT" | "PATCH" | "DELETE",
```

---

### ServeRequest.path

```luau
ServeRequest.path: string,
```

---

### ServeRequest.raw_text

```luau
ServeRequest.raw_text: string,
```

---

### ServeRequest.body

```luau
ServeRequest.body: string,
```

---

### `export type` ServeResponse

---

### ServeResponse.status_code

```luau
ServeResponse.status_code: StatusCode,
```

---

### ServeResponse.content_type

```luau
ServeResponse.content_type: ContentType,
```

---

### ServeResponse.body

```luau
ServeResponse.body: string,
```

---

### ServeResponse.headers.cookies.http_version

```luau
ServeResponse.headers.cookies.http_version: string?,
```

---

### ServeResponse.headers.cookies.reason_phrase

```luau
ServeResponse.headers.cookies.reason_phrase: string?,
```

---

### ServeResponse.headers.cookies.redirect_url

```luau
ServeResponse.headers.cookies.redirect_url: string?
```

---

### `export type` ServeConfig

---

### ServeConfig.address

```luau
ServeConfig.address: string,
```

---

### ServeConfig.port

```luau
ServeConfig.port: string | number,
```

---

### ServeConfig.handler

```luau
ServeConfig.handler: (ServeRequest) -> ServeResponse,
```

---

### net.http.server.serve

```luau
net.http.server.serve: (config: ServeConfig)
```

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
