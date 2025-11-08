--[=[
    Library for sending HTTP Requests.
]=]
export type http = {
    --[=[
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
    ]=]
    get: (config: GetConfig | string) -> HttpResponse,
    --[=[
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
    ]=]
    post: (config: PostConfig) -> HttpResponse,
    --[=[
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
    ]=]
    request: (config: RequestConfig) -> HttpResponse,
    --[=[
        Create a webserver that listens for incoming requests.

        ⚠️ Expect breaking changes. This API will be heavily modified in the future.
    ]=]
    server: typeof(require("@self/server")),
}

export type HttpResponse = ({
    ok: true,
    status_code: StatusCode,
    body: string,
    --- decodes body to table, errors if body is invalid json or otherwise cannot be converted to table
    decode: (self: HttpResponse) -> { [any]: any }
} | {
    ok: false,
    err: string,
}) & {
    --- decodes body as json or returns default value; errors if ok = false and default value not provided
    unwrap_json: (self: HttpResponse, default: { [any]: any }?) -> { [any]: any }
}

export type RequestConfig = {
    method: "GET" | "POST" | "PUT" | "PATCH" | "DELETE",
    url: string,
    headers: { [string]: string }?,
    params: { [string]: string }?,
}

export type GetConfig = {
    url: string,
    headers: { [string]: string }?,
    --- Query parameters to append to the url string
    params: {
        [string]: string,
    }?
}

export type PostConfig = {
    url: string,
    headers: {
        [string]: string,
    }?,
    body: string | {
        [any]: any,
    },
}

export type StatusCode =
    | "200 OK"
    | "201 Created"
    | "204 No Content"
    | "301 Moved Permanently"
    | "302 Found"
    | "304 Not Modified"
    | "307 Temporary Redirect"
    | "308 Permanent Redirect"
    | "400 Bad Request"
    | "401 Unauthorized"
    | "403 Forbidden"
    | "404 Not Found"
    | "405 Method Not Allowed"
    | "409 Conflict"
    | "410 Gone"
    | "412 Precondition Failed"
    | "415 Unsupported Media Type"
    | "429 Too Many Requests"
    | "500 Internal Server Error"
    | "501 Not Implemented"
    | "502 Bad Gateway"
    | "503 Service Unavailable"
    | "504 Gateway Timeout"
    | "505 HTTP Version Not Supported"

return {} :: http