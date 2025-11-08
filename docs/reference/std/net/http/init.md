<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# net.http

`local http = require("@std/net/http")`

Library for sending HTTP Requests.

---

<h3>

```luau
http.get: (config: GetConfig | string) -> HttpResponse,
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
http.post: (config: PostConfig) -> HttpResponse,
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
http.request: (config: RequestConfig) -> HttpResponse,
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

<h3>

```luau
http.server: HttpServerLib,
```

</h3>

Create a webserver that listens for incoming requests.

⚠️ Expect breaking changes. This API will be heavily modified in the future.

---

---

---

<h3>

```luau
HttpResponse.ok: true,
```

</h3>

---

<h3>

```luau
HttpResponse.status_code: StatusCode,
```

</h3>

---

<h3>

```luau
HttpResponse.body: string,
```

</h3>

---

<h3>

```luau
HttpResponse.decode: (self: HttpResponse) -> { [any]: any }
```

</h3>

 decodes body to table, errors if body is invalid json or otherwise cannot be converted to table

---

<h3>

```luau
HttpResponse.ok: false,
```

</h3>

---

<h3>

```luau
HttpResponse.err: string,
```

</h3>

---

<h3>

```luau
HttpResponse.unwrap_json: (self: HttpResponse, default: { [any]: any }?) -> { [any]: any }
```

</h3>

 decodes body as json or returns default value; errors if ok = false and default value not provided

---

---

<h3>

```luau
RequestConfig.method: "GET" | "POST" | "PUT" | "PATCH" | "DELETE",
```

</h3>

---

<h3>

```luau
RequestConfig.url: string,
```

</h3>

---

<h3>

```luau
RequestConfig.headers: { [string]: string }?,
```

</h3>

---

<h3>

```luau
RequestConfig.params: { [string]: string }?,
```

</h3>

---

---

<h3>

```luau
GetConfig.url: string,
```

</h3>

---

<h3>

```luau
GetConfig.headers: { [string]: string }?,
```

</h3>

---

 Query parameters to append to the url string

---

<h3>

```luau
PostConfig.url: string,
```

</h3>

---

<h3>

```luau
PostConfig.headers.body: string | {
```

</h3>

---

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
