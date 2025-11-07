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
