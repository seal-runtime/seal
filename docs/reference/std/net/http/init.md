<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# net.http

`local http = require("@std/net/http")`

Send HTTP Requests.

---

### http.get

<h4>

```luau
function http.get(options: HttpRequestWithoutBody) -> HttpResponse,
```

</h4>

<details>

<summary> See the docs </summary

Send a `GET` request to `options.url`.

Set `options.timeout` to prevent this function from blocking indefinitely
when the internet is disconnected or the destination doesn't respond.

Set `options.max_body_size` to receive responses larger than the default 10MB limit.

Required fields:

- `url`: A URL or URI string; for localhost use `"127.0.0.1:<PORT>"`.

Optional fields:

- `params`: query parameters map that gets serialized and appended to `url`.
- `headers`: headers map, which can include API keys or similar.
- `timeout`: `Duration` or `RequestTimeout` table containing `Durations`.
- `max_body_size`: a `FileSize` limiting how much of the response body to receive; defaults to 10MB.
- `max_redirects`: if 0, no redirects will be followed and response will be returned as-is; defaults to 10.
- `body`: `GET` requests are not *supposed* to have a body but some APIs require it.
See `http.post` for `body` options.

This function blocks the Luau VM - if you want to send multiple HTTP requests at the same time,
wrap it with `@std/thread`.

## Usage

```luau
local response = http.get({
    url = "https://mysecretapi.com/api/cats",
    params = {
        page = 2,
    },
    headers = {
        Authorization = `Bearer {API_KEY}`,
    },
    timeout = time.seconds(4), -- roundtrip (global) timeout
})

if response.ok then -- successful status code
    local data = response:expect_json<<CatResponse>>()
else
    print(
        "Calling the API failed due to reason " ..
        ` ({response.status.code}) {response.status.reason}`
    )
end
```

## Errors

This function **does not** throw an error on 400/500/etc. HTTP error status codes.

Throws an error when:

- A timeout was explicitly set and elapsed,
- An invalid URL or URI was passed to `options.url`,
- The response body is too large (`max_body_size` exceeded)
- A network error occurs.

</details>

---

### http.post

<h4>

```luau
function http.post(options: HttpRequestWithBody) -> HttpResponse,
```

</h4>

<details>

<summary> See the docs </summary

Send a `POST` request to `options.url`.

Set `options.timeout` to prevent this function from blocking indefinitely
when the internet is disconnected or the destination doesn't respond.

Required fields:

- `url`: A URL or URI string; for localhost use `"127.0.0.1:<PORT>"`.

Optional fields:

- `params`: query parameters map that gets serialized and appended to `url`.
- `headers`: headers map, which can include API keys or similar.
- `timeout`: `Duration` or `RequestTimeout` table containing `Durations`.
- `max_body_size`: a `FileSize` limiting how much of the response body to receive; defaults to 10MB.
- `max_redirects`: if 0, no redirects will be followed and response will be returned as-is; defaults to 10.
- `body`: the content to send with the request, can be a table, string, or buffer.

Depending on the type sent, *seal* performs additional serialization and applies the relevant headers.

- tables are serialized to JSON and sent with header `content-type: application/json`.
- buffers may contain invalid UTF-8; sent with header `content-type: application/octet-stream`.
- strings may not contain invalid UTF-8; sent with header `content-type: text/plain`.

Explicitly set `headers["content-type"]` to override these default `content-type` headers.

This function blocks the Luau VM - if you want to send multiple HTTP requests at the same time,
wrap it with `@std/thread`.

## Errors

This function **does not** throw an error on 400/500/etc. HTTP error status codes.

Throws an error when:

- A timeout was explicitly set and elapsed,
- An invalid URL or URI was passed to `options.url`,
- The response body is too large (`max_body_size` exceeded)
- A network error occurs.

</details>

---

### http.request

<h4>

```luau
function http.request(method: HttpMethod, options: HttpRequestWithoutBody | HttpRequestWithBody) -> HttpResponseResult
```

</h4>

<details>

<summary> See the docs </summary

Send an HTTP 1.1 request to the url or uri at `options.url` and explicitly handle errors.

Set `options.timeout` to prevent this function from blocking indefinitely
when the internet is disconnected or the destination doesn't respond.

Required fields:

- `url`: A URL or URI string; for localhost use `"127.0.0.1:<PORT>"`.

Optional fields:

- `params`: query parameters map that gets serialized and appended to `url`.
- `headers`: headers map, which can include API keys or similar.
- `timeout`: `Duration` or `RequestTimeout` table containing `Durations`.
- `max_body_size`: a `FileSize` limiting how much of the response body to receive; defaults to 10MB.
- `max_redirects`: if 0, no redirects will be followed and response will be returned as-is; defaults to 10.
- `body`: the content to send with the request, can be a table, string, or buffer.

Depending on the type sent, *seal* performs additional serialization and applies the relevant headers.

- tables are serialized to JSON and sent with header `content-type: application/json`.
- buffers may contain invalid UTF-8; sent with header `content-type: application/octet-stream`.
- strings may not contain invalid UTF-8; sent with header `content-type: text/plain`.

Explicitly set `headers["content-type"]` to override these default `content-type` headers.

This function blocks the Luau VM - if you want to send multiple HTTP requests at the same time,
wrap it with `@std/thread`.

## Usage

This function tries *not* to throw an error unless you've made a usage error, so you're expected to error handle its response accordingly:

```luau
local response = http.request("PUT", {
    url = "https://jsonplaceholder.typicode.com/posts/1",
    body = {
        id = 1,
        title = "updated title",
        body = "updated body",
        userId = 1,
    },
} :: http.HttpRequestWithBody)
if response.ok then -- successful 200-299 status code
    local body = response.body
    local json_decoded = response:expect_json<<YourType>>()
end
```

A more complete example where we try to handle and log error cases explicitly:

```luau
local URL = ""
local TOKEN = ""
type UserInfo = { id: string, name: string, data: any }
local function get_user_by_id(id: string, retries: number?): UserInfo?
    retries = retries or 0

    local options: http.HttpRequestWithoutBody = {
        url = URL,
        params = {
            id = id
        },
        headers = {
            Authorization = `Basic {TOKEN}`,
        },
    }

    local response = http.request("GET", options)
    if response.ok then -- response is success status code (200-299)
        local decoded = response:try_json<<UserInfo>>()
        if decoded and decoded.id and decoded.name and decoded.data then
            return decoded
        elseif decoded then
            warn(`Incorrect JSON response, wrong fields (got {format(decoded)})`)
        else
            warn(`Unable to decode OK response to json?`)
        end
        return nil
    end

    -- handle timeout error case for retry
    if response.kind == "Timeout" then
        if retries < 3 then
            time.wait(2)
            return get_user_by_id(id, retries + 1)
        else
            warn(`Max retries exceeded for user with id {id}`)
            return nil
        end
    elseif response.kind == "Error" then
        if response.status.code ~= 404 then
            warn(`Unexpected failure status encountered: {format(response)}`)
        end
        return nil
    else
        assert(
            response.kind ~= "Ok" and response.kind ~= "Error",
            "should be IoError table"
        )
        warn(`An IO error occured when calling for user with id {id}, got: {response.kind}: {response.reason}`)
    end

    warn(`fallthrough when calling for user with id {id}`)
    return nil
end
```

</details>

---

## `export type` HttpMethod

<h4>

```luau
export type HttpMethod =
```

</h4>

---

```luau
| "GET"
```

---

```luau
| "TRACE"
```

---

```luau
| "DELETE"
```

---

```luau
| "CONNECT"
```

---

```luau
| "HEAD"
```

---

```luau
| "OPTIONS"
```

---

```luau
| "POST"
```

---

```luau
| "PATCH"
```

---

```luau
| "PUT"
```

---

## `export type` RequestTimeout

<h4>

```luau
export type RequestTimeout = {
```

</h4>

Configure how quickly a request times out.

---

### RequestTimeout.send_request

<h4>

```luau
send_request: Duration?,
```

</h4>

 Max duration to send the request but not the request body.

---

### RequestTimeout.send_body

<h4>

```luau
send_body: Duration?,
```

</h4>

 Max duration to send the request body.

---

### RequestTimeout.receive_response

<h4>

```luau
receive_response: Duration?,
```

</h4>

 Max duration to wait for receiving the response headers but not the response body.

---

### RequestTimeout.receive_body

<h4>

```luau
receive_body: Duration?,
```

</h4>

 Max duration to wait for receiving the response body.

---

## `export type` JsonSerializable

<h4>

```luau
type JsonSerializable = { [string]: any } | { any }
```

</h4>

---

## `export type` HttpRequestWithoutBody

<h4>

```luau
export type HttpRequestWithoutBody = {
```

</h4>

---

### HttpRequestWithoutBody.url

<h4>

```luau
url: string,
```

</h4>

 The URI or URL you want to send the request to. For localhost, use `127.0.0.1:80` (example uses port 80)

---

### HttpRequestWithoutBody.params.headers.timeout

<h4>

```luau
timeout: Duration | RequestTimeout | nil,
```

</h4>

<details>

<summary> See the docs </summary

[HTTP query parameters](https://developer.mozilla.org/en-US/docs/Learn_web_development/Howto/Web_mechanics/What_is_a_URL#parameters) to append to `url`.

Key-value pairs are serialized and appended as `?key=value&key2=value2`.
[HTTP headers](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers) to send with the request.

Common headers include `Authorization`, `Accept`, and `User-Agent`.

Header keys are case-insensitive as per the HTTP spec; values must be valid ASCII.
 Prevent your request from taking forever. Pass a `Duration` from `@std/time` to set a global timeout or a `RequestTimeout` for more granular control.

</details>

---

### HttpRequestWithoutBody.params.headers.max_body_size

<h4>

```luau
max_body_size: FileSize?,
```

</h4>

 Limits the size of the response body; defaults to 10MB. Use `@std/fs/filesize` to construct (`filesize.megabytes(50)`).

---

### HttpRequestWithoutBody.params.headers.max_redirects

<h4>

```luau
max_redirects: number?,
```

</h4>

 Max number of redirects to redirect through before erroring out; defaults to 10. Pass 0 to not redirect anywhere and return the original response.

---

## `export type` HttpRequestWithBody

<h4>

```luau
export type HttpRequestWithBody = {
```

</h4>

---

### HttpRequestWithBody.url

<h4>

```luau
url: string,
```

</h4>

 The URI or URL you want to send the request to. For localhost, use `127.0.0.1:80` (example uses port 80)

---

### HttpRequestWithBody.body

<h4>

```luau
body: string | JsonSerializable | buffer,
```

</h4>

The main content of your request. You can pass in a json-serializable table to send json, a string to send unchanged,
or a buffer to send as arbitary/invalid utf-8.

Note that strings have to be valid utf-8; if you need to pass invalid utf-8 here you must pass a buffer.

---

### HttpRequestWithBody.params.headers.timeout

<h4>

```luau
timeout: Duration | RequestTimeout | nil,
```

</h4>

<details>

<summary> See the docs </summary

[HTTP query parameters](https://developer.mozilla.org/en-US/docs/Learn_web_development/Howto/Web_mechanics/What_is_a_URL#parameters) to append to `url`.

Key-value pairs are serialized and appended as `?key=value&key2=value2`.
[HTTP headers](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers) to send with the request.

Some common headers you'll want to send (or override) include `Authorization`, `Accept`, and `User-Agent`.

Header keys are case-insensitive as per the HTTP spec; values must be valid ASCII.

By default, *seal* automatically sets `Content-Type` for you when you send a string (text/plain), table (application/json), or buffer (application/octet-stream) as body.
To override this behavior, set a `content-type` header explicitly.
 Prevent your request from taking forever. Pass a `Duration` from `@std/time` to set a global timeout or a `RequestTimeout` for more granular control.

</details>

---

### HttpRequestWithBody.params.headers.max_body_size

<h4>

```luau
max_body_size: FileSize?,
```

</h4>

 Limits the size of the response body; defaults to 10MB. Use `@std/fs/filesize` to construct (`filesize.megabytes(50)`).

---

### HttpRequestWithBody.params.headers.max_redirects

<h4>

```luau
max_redirects: number?,
```

</h4>

 Max number of redirects to redirect through before erroring out; defaults to 10. Pass 0 to not redirect anywhere and return the original response.

---

## `export type` MimeType

<h4>

```luau
export type MimeType =
```

</h4>

[MIME types](https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/MIME_types) identify the format/filetype of the content and usually come in the
Content-Type header of a request/response.

*seal* provides the following common MIME types for autocomplete convenience purposes only—this is not an exhaustive list.

---

```luau
| "text/plain"
```

---

```luau
| "text/html"
```

---

```luau
| "text/css"
```

---

```luau
| "text/csv"
```

---

```luau
| "text/xml"
```

---

```luau
| "application/json"
```

---

```luau
| "application/xml"
```

---

```luau
| "application/octet-stream"
```

---

```luau
| "application/pdf"
```

---

```luau
| "application/zip"
```

---

```luau
| "application/gzip"
```

---

```luau
| "application/x-www-form-urlencoded"
```

---

```luau
| "application/x-ndjson"
```

---

```luau
| "multipart/form-data"
```

---

```luau
| "text/javascript"
```

---

```luau
| "application/javascript"
```

---

```luau
| "text/event-stream"
```

---

```luau
| "image/png"
```

---

```luau
| "image/jpeg"
```

---

```luau
| "image/gif"
```

---

```luau
| "image/svg+xml"
```

---

```luau
| "image/webp"
```

---

```luau
| "audio/mpeg"
```

---

```luau
| "audio/ogg"
```

---

```luau
| "video/mp4"
```

---

```luau
| "video/webm"
```

---

```luau
| string
```

---

## `export type` HttpResponseResult

<h4>

```luau
export type HttpResponseResult =
```

</h4>

---

```luau
| HttpResponse
```

---

```luau
| HttpTimeoutError
```

---

```luau
| HttpIoError
```

---

## `export type` HttpResponse

<h4>

```luau
export type HttpResponse = {
```

</h4>

---

### HttpResponse.ok

<h4>

```luau
ok: boolean,
```

</h4>

 `true` if the response was successful (status code 200 -> 299), `false` if the response represents an unsuccessful status code.

---

### HttpResponse.kind

<h4>

```luau
kind: "Ok" | "Error",
```

</h4>

 `"Ok"` if the response represents a successful status code (200 -> 299), `"Error"` if the response represents an unsuccessful status code.

---

### HttpResponse.status.code

<h4>

```luau
code: number,
```

</h4>

---

### HttpResponse.status.reason

<h4>

```luau
reason: string,
```

</h4>

 The canonical HTTP reason phase for the status code; such as "Not Found" for status code 404

---

### HttpResponse.body

<h4>

```luau
body: string,
```

</h4>

 The response's main contents; this can be any encoding, including UTF-8, arbitrary unreadable bytes, json, etc.

---

### HttpResponse.content_type.mime_type

<h4>

```luau
mime_type: MimeType?,
```

</h4>

<details>

<summary> See the docs </summary

The parsed [Content-Type](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Type) response header, if present.

`content-type` describes the format of the response body so you know how to interpret it.
It consists of a `mime_type` (what kind of data) and an optional `charset` (how it's encoded).

For example, `content-type: application/json; charset=utf-8` parses to:

- `mime_type = "application/json"`
- `charset = "utf-8"`

This field is `nil` if the server didn't send a `content-type` header at all.
 The MIME type of the response body — describes *what kind* of data it is,
 e.g. `"application/json"`, `"text/html"`, `"image/png"`.
 `nil` if the `content-type` header was present but had no MIME type.

</details>

---

### HttpResponse.content_type.charset

<h4>

```luau
charset: string?,
```

</h4>

 The character encoding of the response body, e.g. `"utf-8"` or `"iso-8859-1"`.
 Only present for text-based MIME types; typically absent for binary types like `"image/png"`.

---

### HttpResponse.content_type.try_json

<h4>

```luau
function HttpResponse.content_type.try_json<T>(self: HttpResponse) -> T?,
```

</h4>

<details>

<summary> See the docs </summary

Try to decode the response body as JSON into a table. If body is not json or decoding was unsuccessful, returns `nil`.

Pass an explicit type parameter with `<<T>>` syntax to *promise* that the decoded table is of type `T`.

This makes it easier to typecheck on the return type of this function, but doesn't provide any assurances
that the table returned is *actually* of type `T`.

## Usage

```luau
type ApiResponse = {
    id: string,
    info: {
        downloadUrl: string,
        contentSize: number,
    }
}

local response = http.get(options)
if response.ok then
    local data = response:try_json<<ApiResponse>>()
    if data then
        print(data.info.downloadUrl) -- should typecheck fine
    end
end
```

</details>

---

### HttpResponse.content_type.expect_json

<h4>

```luau
function HttpResponse.content_type.expect_json<T>(self: HttpResponse) -> T,
```

</h4>

<details>

<summary> See the docs </summary

Decode the response body as JSON into a table. Errors if the body wasn't json or decoding was otherwise unsuccessful.

Pass an explicit type parameter with `<<T>>` syntax to *promise* that the returned type is of type `T`.

This makes it easier to typecheck on the return type of this function, but doesn't provide any runtime assurance
that the table returned is actually of type `T`.

## Usage

```luau
type ApiResponse = { id: string, downloadUri: string }
local data = http.get(options):expect_json<<ApiResponse>>()
```

</details>

---

## `export type` HttpTimeoutError

<h4>

```luau
export type HttpTimeoutError = {
```

</h4>

The request timed out because you set `HttpRequestOptions.timeout`.

---

### HttpTimeoutError.ok

<h4>

```luau
ok: false,
```

</h4>

---

### HttpTimeoutError.kind

<h4>

```luau
kind: "Timeout",
```

</h4>

---

### HttpTimeoutError.reason

<h4>

```luau
reason: string,
```

</h4>

---

### HttpTimeoutError.phase

<h4>

```luau
phase: "Global" | "SendRequest" | "ReceiveResponse" | "SendBody" | "ReceiveBody" | "Connect" | "Resolve" | "Other"
```

</h4>

---

## `export type` HttpIoError

<h4>

```luau
export type HttpIoError = {
```

</h4>

We encountered some network, io, or connection error that prevented *seal* from sending the request or
receiving a proper response.

---

### HttpIoError.ok

<h4>

```luau
ok: false,
```

</h4>

---

### HttpIoError.kind

<h4>

```luau
kind: "BodyExceedsLimit" | "NetworkError" | "ConnectionFailed" | "TooManyRedirects" | "Other",
```

</h4>

---

### HttpIoError.reason

<h4>

```luau
reason: string,
```

</h4>

---

Autogenerated from [std/net/http/init.luau](/.seal/typedefs/std/net/http/init.luau).

*seal* is best experienced with inline, in-editor documentation. Please see the linked typedefs file if this documentation is confusing, too verbose, or inaccurate.
