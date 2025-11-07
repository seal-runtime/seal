<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# net.http.server

`local server = require("@std/net/http/server")`

`export type` ContentType

`export type` ServeRequest

ServeRequest.peer_address: `string`

ServeRequest.method: `"GET" | "POST" | "PUT" | "PATCH" | "DELETE"`

ServeRequest.path: `string`

ServeRequest.raw_text: `string`

ServeRequest.body: `string`

`export type` ServeResponse

ServeResponse.status_code: `StatusCode`

ServeResponse.content_type: `ContentType`

ServeResponse.body: `string`

ServeResponse.headers.cookies.http_version: `string?`

ServeResponse.headers.cookies.reason_phrase: `string?`

ServeResponse.headers.cookies.redirect_url: `string?`

`export type` ServeConfig

ServeConfig.address: `string`

ServeConfig.port: `string | number`

ServeConfig.handler: `(ServeRequest) -> ServeResponse`

.function server.serve(config: `ServeConfig)`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ Create a webserver that listens for incoming requests.
$\hspace{5pt}$
$\hspace{5pt}$ ⚠️ Expect breaking changes. This API will be heavily modified in the future.
$\hspace{5pt}$
$\hspace{5pt}$ ## Usage
$\hspace{5pt}$
$\hspace{5pt}$ ```luau
$\hspace{5pt}$ local server = require("@std/net/http/server")
$\hspace{5pt}$ local fs = require("@std/fs")
$\hspace{5pt}$ local json = require("@std/json")
$\hspace{5pt}$
$\hspace{5pt}$ print("starting seal server")
$\hspace{5pt}$
$\hspace{5pt}$ server.serve {
$\hspace{5pt}$     address = "localhost",
$\hspace{5pt}$     port = 4242,
$\hspace{5pt}$     handler = function(info: server.ServeRequest)
$\hspace{5pt}$         local response = {}
$\hspace{5pt}$         if info.path == "/meow.json" then
$\hspace{5pt}$             response.status_code = "200 OK"
$\hspace{5pt}$             response.content_type = "json"
$\hspace{5pt}$             response.body = json.encode {
$\hspace{5pt}$                 ok = true,
$\hspace{5pt}$                 says = "meow"
$\hspace{5pt}$             }
$\hspace{5pt}$         elseif info.path == "/" then
$\hspace{5pt}$             local meow_page = fs.readfile("./tests/data/server-views/index.html")
$\hspace{5pt}$             response.status_code = "200 OK"
$\hspace{5pt}$             response.content_type = "html"
$\hspace{5pt}$             response.body = meow_page
$\hspace{5pt}$         elseif info.path == "/info" then
$\hspace{5pt}$             local body = fs.readfile("./tests/data/server-views/info.html")
$\hspace{5pt}$             response = {
$\hspace{5pt}$                 status_code = "200 OK",
$\hspace{5pt}$                 content_type = "html",
$\hspace{5pt}$                 body = body
$\hspace{5pt}$             }
$\hspace{5pt}$         elseif info.path == "/some-post" then
$\hspace{5pt}$             response = {
$\hspace{5pt}$                 status_code = "200 OK",
$\hspace{5pt}$                 content_type = "application/json",
$\hspace{5pt}$                 body = json.encode {
$\hspace{5pt}$                     ok = true,
$\hspace{5pt}$                     recvbody = info.body,
$\hspace{5pt}$                 }
$\hspace{5pt}$             }
$\hspace{5pt}$         else
$\hspace{5pt}$             response.status_code = "404 Not Found"
$\hspace{5pt}$             response.response_type = "json"
$\hspace{5pt}$             response.body = json.encode {
$\hspace{5pt}$                 ok = false,
$\hspace{5pt}$             }
$\hspace{5pt}$         end
$\hspace{5pt}$         return response :: server.ServeResponse
$\hspace{5pt}$     end
$\hspace{5pt}$ }
$\hspace{5pt}$```

</details>
