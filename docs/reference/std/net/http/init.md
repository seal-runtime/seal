<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# net.http

`local http = require("@std/net/http")`

Library for sending HTTP Requests.

---

<h3>

```luau
function http.get(config: GetConfig | string) -> HttpResponse,
```

</h3>

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

<h3>

```luau
function http.post(config: PostConfig) -> HttpResponse,
```

</h3>

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

<h3>

```luau
function http.request(config: RequestConfig) -> HttpResponse,
```

</h3>

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

<h4>

```luau
server: HttpServerLib,
```

</h4>

Create a webserver that listens for incoming requests.

⚠️ Expect breaking changes. This API will be heavily modified in the future.

---

## `export type` HttpServerLib

---

## `export type` HttpResponse

---

### HttpResponse.ok

<h4>

```luau
ok: true,
```

</h4>

---

### HttpResponse.status_code

<h4>

```luau
status_code: StatusCode,
```

</h4>

---

### HttpResponse.body

<h4>

```luau
body: string,
```

</h4>

---

<h3>

```luau
function HttpResponse.decode(self: HttpResponse) -> { [any]: any }
```

</h3>

 decodes body to table, errors if body is invalid json or otherwise cannot be converted to table

---

### HttpResponse.ok

<h4>

```luau
ok: false,
```

</h4>

---

### HttpResponse.err

<h4>

```luau
err: string,
```

</h4>

---

<h3>

```luau
function HttpResponse.unwrap_json(self: HttpResponse, default: { [any]: any }?) -> { [any]: any }
```

</h3>

 decodes body as json or returns default value; errors if ok = false and default value not provided

---

## `export type` RequestConfig

---

### RequestConfig.method

<h4>

```luau
method: "GET" | "POST" | "PUT" | "PATCH" | "DELETE",
```

</h4>

---

### RequestConfig.url

<h4>

```luau
url: string,
```

</h4>

---

### RequestConfig.headers

<h4>

```luau
headers: { [string]: string }?,
```

</h4>

---

### RequestConfig.params

<h4>

```luau
params: { [string]: string }?,
```

</h4>

---

## `export type` GetConfig

---

### GetConfig.url

<h4>

```luau
url: string,
```

</h4>

---

### GetConfig.headers

<h4>

```luau
headers: { [string]: string }?,
```

</h4>

---

## `export type` PostConfig

 Query parameters to append to the url string

---

### PostConfig.url

<h4>

```luau
url: string,
```

</h4>

---

### PostConfig.headers.body

<h4>

```luau
body: string | {
```

</h4>

---

## `export type` StatusCode

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
