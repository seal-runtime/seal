-- --[=[
--     Library for sending HTTP Requests.
-- ]=]
-- export type http = {
--     --[=[
--         Makes an HTTP `GET` request.

--         ## Usage
--         ```luau
--         local response = http.get({
--             url = "https://catfact.ninja/fact",
--         })
--         if response.ok then
--             local raw_body = response.body
--             local decoded_json_body = response:decode()
--         end

--         -- or with more features:

--         local cats = http.get {
--             url = "my.cats.net/get",
--             headers = {
--                 Authorization = someauth
--             },
--             params = {
--                 name = "Nanuk",
--             },
--         }:unwrap_json()
--         ```
--     ]=]
--     get: (config: GetConfig | string) -> HttpResponse,
--     --[=[
--         Makes an HTTP `POST` request.

--         ## Usage
--         ```luau
--         local response = http.post {
--             url = "https://somejson.com/post",
--             headers = {
--                 ["API-KEY"] = api_key,
--                 -- note: Content-Type: application/json automatically handled when you pass a table as body!
--             },
--             body = {
--                 username = "hiItsMe",
--             }
--         }
--         ```
--     ]=]
--     post: (config: PostConfig) -> HttpResponse,
--     --[=[
--         Sends an HTTP request:

--         ## Usage
--         ```luau
--         local response = http.request({
--             method = "PUT",
--             url = "https://somewhere.net/api/put",
--             body = somebody,
--         })

--         if response.ok then
--             print(response:decode())
--         end
--         ```
--     ]=]
--     request: (config: RequestConfig) -> HttpResponse,
--     --[=[
--         Create a webserver that listens for incoming requests.

--         ⚠️ Expect breaking changes. This API will be heavily modified in the future.
--     ]=]
--     server: typeof(require("@self/server")),
-- }

-- export type HttpResponse = ({
--     ok: true,
--     status_code: StatusCode,
--     body: string,
--     --- decodes body to table, errors if body is invalid json or otherwise cannot be converted to table
--     decode: (self: HttpResponse) -> { [any]: any }
-- } | {
--     ok: false,
--     status_code: StatusCode,
--     body: string,
--     err: string,
-- }) & {
--     --- decodes body as json or returns default value; errors if ok = false and default value not provided
--     unwrap_json: (self: HttpResponse, default: { [any]: any }?) -> { [any]: any }
-- }

-- export type RequestConfig = {
--     method: "GET" | "POST" | "PUT" | "PATCH" | "DELETE",
--     url: string,
--     headers: { [string]: string }?,
--     params: { [string]: string }?,
-- }

-- export type GetConfig = {
--     url: string,
--     headers: { [string]: string }?,
--     --- Query parameters to append to the url string
--     params: {
--         [string]: string,
--     }?
-- }

-- export type PostConfig = {
--     url: string,
--     headers: {
--         [string]: string,
--     }?,
--     body: string | {
--         [any]: any,
--     },
-- }

-- export type StatusCode =
--     | "200 OK"
--     | "201 Created"
--     | "204 No Content"
--     | "301 Moved Permanently"
--     | "302 Found"
--     | "304 Not Modified"
--     | "307 Temporary Redirect"
--     | "308 Permanent Redirect"
--     | "400 Bad Request"
--     | "401 Unauthorized"
--     | "403 Forbidden"
--     | "404 Not Found"
--     | "405 Method Not Allowed"
--     | "409 Conflict"
--     | "410 Gone"
--     | "412 Precondition Failed"
--     | "415 Unsupported Media Type"
--     | "429 Too Many Requests"
--     | "500 Internal Server Error"
--     | "501 Not Implemented"
--     | "502 Bad Gateway"
--     | "503 Service Unavailable"
--     | "504 Gateway Timeout"
--     | "505 HTTP Version Not Supported"

-- return {} :: http

--[=[
    Send HTTP Requests.
]=]
export type http = {
    --[=[
        Send a `GET` request to `options.url`.

        Set `options.timeout` to prevent this function from blocking indefinitely
        when the internet is disconnected or the destination doesn't respond.
        
        Required fields:

        - `url`: A URL or URI string; for localhost use `"127.0.0.1:<PORT>"`.

        Optional fields:

        - `params`: query parameters map that gets serialized and appended to `url`.
        - `headers`: headers map, which can include API keys or similar.
        - `timeout`: `Duration` or `RequestTimeout` table containing `Durations`.
        - `limit`: the max number of bytes to receive in the response payload.
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
        - A network error occurs.
    ]=]
    get: (options: HttpRequestWithoutBody) -> HttpResponse,
    --[=[
        Send a `POST` request to `options.url`.

        Set `options.timeout` to prevent this function from blocking indefinitely 
        when the internet is disconnected or the destination doesn't respond.

        Required fields:

        - `url`: A URL or URI string; for localhost use `"127.0.0.1:<PORT>"`.

        Optional fields:

        - `params`: query parameters map that gets serialized and appended to `url`.
        - `headers`: headers map, which can include API keys or similar.
        - `timeout`: `Duration` or `RequestTimeout` table containing `Durations`.
        - `limit`: the max number of bytes to receive in the response payload.
        - `body`: the content to send with the request, can be a table, string, or buffer.
        
            Depending on the type sent, *seal* performs additional serialization and applies the relevant headers.
            - tables are serialized to JSON and sent with header `content-type: application/json`.
            - buffers may contain invalid UTF-8; sent with header `content-type: application/octet-stream`.
            - strings may not contain invalid UTF-8; sent with header `content-type: text/plain`.
            
            Explicitly set `headers["content-type"]` to override these default `content-type` headers.
        
        This function blocks the Luau VM - if you want to send multiple HTTP requests at the same time,
        wrap it with `@std/thread`.

    ]=]
    post: (options: HttpRequestWithBody) -> HttpResponse,
    request: (method: HttpMethod, options: HttpRequestWithoutBody | HttpRequestWithBody) -> HttpResponse | HttpError
}

export type HttpMethod =
    | "GET"
    | "TRACE"
    | "DELETE"
    | "CONNECT"
    | "HEAD"
    | "OPTIONS"
    | "POST"
    | "PATCH"
    | "PUT"

export type RequestTimeout = {
    send_request: Duration?,
    send_response: Duration?,
    send_body: Duration?,
    receive_body: Duration?,
}

type JsonSerializable = { [string]: any } | { any }

export type HttpRequestWithoutBody = {
    url: string,
    params: {
        [string]: string,
    }?,
    headers: {
        [string]: string,
    }?,
    timeout: Duration | RequestTimeout | nil,
    max_body_size: number?,
}

export type HttpRequestWithBody = {
    url: string,
    body: string | JsonSerializable | buffer,
    params: {
        [string]: string,
    }?,
    headers: {
        [string]: string,
    }?,
    timeout: Duration | RequestTimeout | nil,
    max_body_size: number?,
}

export type MimeType =
    | "text/plain"
    | "text/html"
    | "text/css"
    | "text/csv"
    | "text/xml"
    | "application/json"
    | "application/xml"
    | "application/octet-stream"
    | "application/pdf"
    | "application/zip"
    | "application/gzip"
    | "application/x-www-form-urlencoded"
    | "application/x-ndjson"
    | "multipart/form-data"
    | "text/javascript"
    | "application/javascript"
    | "text/event-stream"
    | "image/png"
    | "image/jpeg"
    | "image/gif"
    | "image/svg+xml"
    | "image/webp"
    | "audio/mpeg"
    | "audio/ogg"
    | "video/mp4"
    | "video/webm"
    | string

export type HttpResponse = {
    ok: boolean,
    headers: {
        [string]: string,
    },
    status: {
        code: number,
        reason: string,
    },
    body: string,
    content_type: {
        mime_type: MimeType?,
        charset: string?,
    }?,
    try_json: <T>(self: HttpResponse) -> T?,
    expect_json: <T>(self: HttpResponse) -> T,
}

return {} :: http