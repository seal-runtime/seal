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

## `export type` ContentType

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

## `export type` ServeRequest

---

### ServeRequest.peer_address

<h4>

```luau
peer_address: string,
```

</h4>

---

### ServeRequest.method

<h4>

```luau
method: "GET" | "POST" | "PUT" | "PATCH" | "DELETE",
```

</h4>

---

### ServeRequest.path

<h4>

```luau
path: string,
```

</h4>

---

### ServeRequest.raw_text

<h4>

```luau
raw_text: string,
```

</h4>

---

### ServeRequest.body

<h4>

```luau
body: string,
```

</h4>

---

## `export type` ServeResponse

---

### ServeResponse.status_code

<h4>

```luau
status_code: StatusCode,
```

</h4>

---

### ServeResponse.content_type

<h4>

```luau
content_type: ContentType,
```

</h4>

---

### ServeResponse.body

<h4>

```luau
body: string,
```

</h4>

---

### ServeResponse.headers.cookies.http_version

<h4>

```luau
http_version: string?,
```

</h4>

---

### ServeResponse.headers.cookies.reason_phrase

<h4>

```luau
reason_phrase: string?,
```

</h4>

---

### ServeResponse.headers.cookies.redirect_url

<h4>

```luau
redirect_url: string?
```

</h4>

---

## `export type` ServeConfig

---

### ServeConfig.address

<h4>

```luau
address: string,
```

</h4>

---

### ServeConfig.port

<h4>

```luau
port: string | number,
```

</h4>

---

<h3>

```luau
function ServeConfig.handler(ServeRequest) -> ServeResponse,
```

</h3>

---

<h3>

```luau
function net.http.server.serve(config: ServeConfig)
```

</h3>

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
