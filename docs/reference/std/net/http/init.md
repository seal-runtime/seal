<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# net.http

`local http = require("@std/net/http")`

Library for sending HTTP Requests.

http.get: `(config: GetConfig | string) -> HttpResponse`

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

http.post: `(config: PostConfig) -> HttpResponse`

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

http.request: `(config: RequestConfig) -> HttpResponse`

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

http.server: `HttpServerLib`

Create a webserver that listens for incoming requests.

⚠️ Expect breaking changes. This API will be heavily modified in the future.

---

`export type` HttpServerLib

---

`export type` HttpResponse

---

HttpResponse.ok: `true`

---

HttpResponse.status_code: `StatusCode`

---

HttpResponse.body: `string`

---

HttpResponse.decode: `(self: HttpResponse) -> { [any]: any }`

 decodes body to table, errors if body is invalid json or otherwise cannot be converted to table

---

HttpResponse.ok: `false`

---

HttpResponse.err: `string`

---

HttpResponse.unwrap_json: `(self: HttpResponse, default: { [any]: any }?) -> { [any]: any }`

 decodes body as json or returns default value; errors if ok = false and default value not provided

---

`export type` RequestConfig

---

RequestConfig.method: `"GET" | "POST" | "PUT" | "PATCH" | "DELETE"`

---

RequestConfig.url: `string`

---

RequestConfig.headers: `{ [string]: string }?`

---

RequestConfig.params: `{ [string]: string }?`

---

`export type` GetConfig

---

GetConfig.url: `string`

---

GetConfig.headers: `{ [string]: string }?`

---

`export type` PostConfig

 Query parameters to append to the url string

---

PostConfig.url: `string`

---

PostConfig.headers.body: `string | {`

---

`export type` StatusCode

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
