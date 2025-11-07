<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# net.http

`local http = require("@std/net/http")`

$\hspace{5pt}$ Library for sending HTTP Requests.

http.get: `(config: GetConfig | string) -> HttpResponse`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ Makes an HTTP `GET` request.
$\hspace{5pt}$
$\hspace{5pt}$ ## Usage
$\hspace{5pt}$ ```luau
$\hspace{5pt}$ local response = http.get({
$\hspace{5pt}$     url = "https://catfact.ninja/fact",
$\hspace{5pt}$ })
$\hspace{5pt}$ if response.ok then
$\hspace{5pt}$     local raw_body = response.body
$\hspace{5pt}$     local decoded_json_body = response:decode()
$\hspace{5pt}$ end
$\hspace{5pt}$
$\hspace{5pt}$ -- or with more features:
$\hspace{5pt}$
$\hspace{5pt}$ local cats = http.get {
$\hspace{5pt}$     url = "my.cats.net/get",
$\hspace{5pt}$     headers = {
$\hspace{5pt}$         Authorization = someauth
$\hspace{5pt}$     },
$\hspace{5pt}$     params = {
$\hspace{5pt}$         name = "Nanuk",
$\hspace{5pt}$     },
$\hspace{5pt}$ }:unwrap_json()
$\hspace{5pt}$```

</details>

http.post: `(config: PostConfig) -> HttpResponse`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ Makes an HTTP `POST` request.
$\hspace{5pt}$
$\hspace{5pt}$ ## Usage
$\hspace{5pt}$ ```luau
$\hspace{5pt}$ local response = http.post {
$\hspace{5pt}$     url = "https://somejson.com/post",
$\hspace{5pt}$     headers = {
$\hspace{5pt}$         ["API-KEY"] = api_key,
$\hspace{5pt}$         -- note: Content-Type: application/json automatically handled when you pass a table as body!
$\hspace{5pt}$     },
$\hspace{5pt}$     body = {
$\hspace{5pt}$         username = "hiItsMe",
$\hspace{5pt}$     }
$\hspace{5pt}$ }
$\hspace{5pt}$```

</details>

http.request: `(config: RequestConfig) -> HttpResponse`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ Sends an HTTP request:
$\hspace{5pt}$
$\hspace{5pt}$ ## Usage
$\hspace{5pt}$ ```luau
$\hspace{5pt}$ local response = http.request({
$\hspace{5pt}$     method = "PUT",
$\hspace{5pt}$     url = "https://somewhere.net/api/put",
$\hspace{5pt}$     body = somebody,
$\hspace{5pt}$ })
$\hspace{5pt}$
$\hspace{5pt}$ if response.ok then
$\hspace{5pt}$     print(response:decode())
$\hspace{5pt}$ end
$\hspace{5pt}$```

</details>

http.server: `HttpServerLib`

$\hspace{5pt}$ Create a webserver that listens for incoming requests.
$\hspace{5pt}$
$\hspace{5pt}$ ⚠️ Expect breaking changes. This API will be heavily modified in the future.

`export type` HttpServerLib

`export type` HttpResponse

HttpResponse.ok: `true`

HttpResponse.status_code: `StatusCode`

HttpResponse.body: `string`

HttpResponse.decode: `(self: HttpResponse) -> { [any]: any }`

$\hspace{5pt}$  decodes body to table, errors if body is invalid json or otherwise cannot be converted to table

HttpResponse.ok: `false`

HttpResponse.err: `string`

HttpResponse.unwrap_json: `(self: HttpResponse, default: { [any]: any }?) -> { [any]: any }`

$\hspace{5pt}$  decodes body as json or returns default value; errors if ok = false and default value not provided

`export type` RequestConfig

RequestConfig.method: `"GET" | "POST" | "PUT" | "PATCH" | "DELETE"`

RequestConfig.url: `string`

RequestConfig.headers: `{ [string]: string }?`

RequestConfig.params: `{ [string]: string }?`

`export type` GetConfig

GetConfig.url: `string`

GetConfig.headers: `{ [string]: string }?`

`export type` PostConfig

$\hspace{5pt}$  Query parameters to append to the url string

PostConfig.url: `string`

PostConfig.headers.body: `string | {`

`export type` StatusCode
