<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# net.http

`local http = require("@std/net/http")`

Library for sending HTTP Requests.

---

### http.get

```luau
http.get: (config: GetConfig | string) -> HttpResponse,
```

<details>

<summary> See the docs </summary

Makes an HTTP `GET` request.

## Usage

```luau
local response = http.get({
    url = "https://catfact.ninja/fact",
})
if response.ok then
    local raw_body = response.body
    local decoded_json_body = response:decode()
end

-- or with more features:

local cats = http.get {
    url = "my.cats.net/get",
    headers = {
        Authorization = someauth
    },
    params = {
        name = "Nanuk",
    },
}:unwrap_json()
```

</details>

---

### http.post

```luau
http.post: (config: PostConfig) -> HttpResponse,
```

Makes an HTTP `POST` request.

## Usage

```luau
local response = http.post {
    url = "https://somejson.com/post",
    headers = {
        ["API-KEY"] = api_key,
        -- note: Content-Type: application/json automatically handled when you pass a table as body!
    },
    body = {
        username = "hiItsMe",
    }
}
```

---

### http.request

```luau
http.request: (config: RequestConfig) -> HttpResponse,
```

Sends an HTTP request:

## Usage

```luau
local response = http.request({
    method = "PUT",
    url = "https://somewhere.net/api/put",
    body = somebody,
})

if response.ok then
    print(response:decode())
end
```

---

### http.server

```luau
http.server: HttpServerLib,
```

Create a webserver that listens for incoming requests.

⚠️ Expect breaking changes. This API will be heavily modified in the future.

---

### `export type` HttpServerLib

---

### `export type` HttpResponse

---

### HttpResponse.ok

```luau
HttpResponse.ok: true,
```

---

### HttpResponse.status_code

```luau
HttpResponse.status_code: StatusCode,
```

---

### HttpResponse.body

```luau
HttpResponse.body: string,
```

---

### HttpResponse.decode

```luau
HttpResponse.decode: (self: HttpResponse) -> { [any]: any }
```

 decodes body to table, errors if body is invalid json or otherwise cannot be converted to table

---

### HttpResponse.ok

```luau
HttpResponse.ok: false,
```

---

### HttpResponse.err

```luau
HttpResponse.err: string,
```

---

### HttpResponse.unwrap_json

```luau
HttpResponse.unwrap_json: (self: HttpResponse, default: { [any]: any }?) -> { [any]: any }
```

 decodes body as json or returns default value; errors if ok = false and default value not provided

---

### `export type` RequestConfig

---

### RequestConfig.method

```luau
RequestConfig.method: "GET" | "POST" | "PUT" | "PATCH" | "DELETE",
```

---

### RequestConfig.url

```luau
RequestConfig.url: string,
```

---

### RequestConfig.headers

```luau
RequestConfig.headers: { [string]: string }?,
```

---

### RequestConfig.params

```luau
RequestConfig.params: { [string]: string }?,
```

---

### `export type` GetConfig

---

### GetConfig.url

```luau
GetConfig.url: string,
```

---

### GetConfig.headers

```luau
GetConfig.headers: { [string]: string }?,
```

---

### `export type` PostConfig

 Query parameters to append to the url string

---

### PostConfig.url

```luau
PostConfig.url: string,
```

---

### PostConfig.headers.body

```luau
PostConfig.headers.body: string | {
```

---

### `export type` StatusCode

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
