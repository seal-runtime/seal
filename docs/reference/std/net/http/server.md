<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# net.http.server

`local server = require("@std/net/http/server")`

---

<h3>

```luau
| "200 OK"
```

</h3>

---

<h3>

```luau
| "201 Created"
```

</h3>

---

<h3>

```luau
| "204 No Content"
```

</h3>

---

<h3>

```luau
| "301 Moved Permanently"
```

</h3>

---

<h3>

```luau
| "302 Found"
```

</h3>

---

<h3>

```luau
| "304 Not Modified"
```

</h3>

---

<h3>

```luau
| "307 Temporary Redirect"
```

</h3>

---

<h3>

```luau
| "308 Permanent Redirect"
```

</h3>

---

<h3>

```luau
| "400 Bad Request"
```

</h3>

---

<h3>

```luau
| "401 Unauthorized"
```

</h3>

---

<h3>

```luau
| "403 Forbidden"
```

</h3>

---

<h3>

```luau
| "404 Not Found"
```

</h3>

---

<h3>

```luau
| "405 Method Not Allowed"
```

</h3>

---

<h3>

```luau
| "409 Conflict"
```

</h3>

---

<h3>

```luau
| "410 Gone"
```

</h3>

---

<h3>

```luau
| "412 Precondition Failed"
```

</h3>

---

<h3>

```luau
| "415 Unsupported Media Type"
```

</h3>

---

<h3>

```luau
| "429 Too Many Requests"
```

</h3>

---

<h3>

```luau
| "500 Internal Server Error"
```

</h3>

---

<h3>

```luau
| "501 Not Implemented"
```

</h3>

---

<h3>

```luau
| "502 Bad Gateway"
```

</h3>

---

<h3>

```luau
| "503 Service Unavailable"
```

</h3>

---

<h3>

```luau
| "504 Gateway Timeout"
```

</h3>

---

<h3>

```luau
| "505 HTTP Version Not Supported"
```

</h3>

---

---

<h3>

```luau
| "Text"
```

</h3>

---

<h3>

```luau
| "HTML"
```

</h3>

---

<h3>

```luau
| "JSON"
```

</h3>

---

<h3>

```luau
| "XML"
```

</h3>

---

<h3>

```luau
| "CSS"
```

</h3>

---

<h3>

```luau
| "JavaScript"
```

</h3>

---

<h3>

```luau
| "Binary"
```

</h3>

---

<h3>

```luau
| string
```

</h3>

---

---

<h3>

```luau
ServeRequest.peer_address: string,
```

</h3>

---

<h3>

```luau
ServeRequest.method: "GET" | "POST" | "PUT" | "PATCH" | "DELETE",
```

</h3>

---

<h3>

```luau
ServeRequest.path: string,
```

</h3>

---

<h3>

```luau
ServeRequest.raw_text: string,
```

</h3>

---

<h3>

```luau
ServeRequest.body: string,
```

</h3>

---

---

<h3>

```luau
ServeResponse.status_code: StatusCode,
```

</h3>

---

<h3>

```luau
ServeResponse.content_type: ContentType,
```

</h3>

---

<h3>

```luau
ServeResponse.body: string,
```

</h3>

---

<h3>

```luau
ServeResponse.headers.cookies.http_version: string?,
```

</h3>

---

<h3>

```luau
ServeResponse.headers.cookies.reason_phrase: string?,
```

</h3>

---

<h3>

```luau
ServeResponse.headers.cookies.redirect_url: string?
```

</h3>

---

---

<h3>

```luau
ServeConfig.address: string,
```

</h3>

---

<h3>

```luau
ServeConfig.port: string | number,
```

</h3>

---

<h3>

```luau
ServeConfig.handler: (ServeRequest) -> ServeResponse,
```

</h3>

---

<h3>

```luau
net.http.server.serve: (config: ServeConfig)
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
