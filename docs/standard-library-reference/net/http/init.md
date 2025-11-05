<!-- markdownlint-disable MD033 -->

# Http

` params: {`

`function HttpResponse.decode(self: HttpResponse): { [any]: any }`

`function HttpResponse.unwrap_json(self: HttpResponse, default: { [any]: any }?): { [any]: any }`

`function http.get(url: GetConfig): HttpResponse`

<details>

<summary> See the docs </summary

Makes an HTTP `GET` request.

## Usage

```lua
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

`function http.post(config: PostConfig): HttpResponse`

<details>

<summary> See the docs </summary

Makes an HTTP `POST` request.

## Usage

```lua
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

```#

</details>


`function http.request(config: RequestConfig): HttpResponse`

Sends an HTTP request:

## Usage
```lua
local response = http.request({
 method = "PUT",
 url = "https://somewhere.net/api/put",
 body = somebody,
})

if response.ok then
 print(response:decode())
end
```
