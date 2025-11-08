<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# net.http.server

`local server = require("@std/net/http/server")`

---

### StatusCode

```luau
| "200 OK"
```

---

### StatusCode

```luau
| "201 Created"
```

---

### StatusCode

```luau
| "204 No Content"
```

---

### StatusCode

```luau
| "301 Moved Permanently"
```

---

### StatusCode

```luau
| "302 Found"
```

---

### StatusCode

```luau
| "304 Not Modified"
```

---

### StatusCode

```luau
| "307 Temporary Redirect"
```

---

### StatusCode

```luau
| "308 Permanent Redirect"
```

---

### StatusCode

```luau
| "400 Bad Request"
```

---

### StatusCode

```luau
| "401 Unauthorized"
```

---

### StatusCode

```luau
| "403 Forbidden"
```

---

### StatusCode

```luau
| "404 Not Found"
```

---

### StatusCode

```luau
| "405 Method Not Allowed"
```

---

### StatusCode

```luau
| "409 Conflict"
```

---

### StatusCode

```luau
| "410 Gone"
```

---

### StatusCode

```luau
| "412 Precondition Failed"
```

---

### StatusCode

```luau
| "415 Unsupported Media Type"
```

---

### StatusCode

```luau
| "429 Too Many Requests"
```

---

### StatusCode

```luau
| "500 Internal Server Error"
```

---

### StatusCode

```luau
| "501 Not Implemented"
```

---

### StatusCode

```luau
| "502 Bad Gateway"
```

---

### StatusCode

```luau
| "503 Service Unavailable"
```

---

### StatusCode

```luau
| "504 Gateway Timeout"
```

---

### StatusCode

```luau
| "505 HTTP Version Not Supported"
```

---

### `export type` ContentType

---

### ContentType

```luau
| "Text"
```

---

### ContentType

```luau
| "HTML"
```

---

### ContentType

```luau
| "JSON"
```

---

### ContentType

```luau
| "XML"
```

---

### ContentType

```luau
| "CSS"
```

---

### ContentType

```luau
| "JavaScript"
```

---

### ContentType

```luau
| "Binary"
```

---

### ContentType

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

### .config

```luau
`function` .configserver.serve: (config: ServeConfig)
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
