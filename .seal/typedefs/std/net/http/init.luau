--[=[
    Send HTTP Requests.
]=]
export type http = {
    --[=[
        Send a `GET` request to `options.url`.

        Set `options.timeout` to prevent this function from blocking indefinitely
        when the internet is disconnected or the destination doesn't respond.

        Set `options.max_body_size` to get responses of over 10MB.
        
        Required fields:

        - `url`: A URL or URI string; for localhost use `"127.0.0.1:<PORT>"`.

        Optional fields:

        - `params`: query parameters map that gets serialized and appended to `url`.
        - `headers`: headers map, which can include API keys or similar.
        - `timeout`: `Duration` or `RequestTimeout` table containing `Durations`.
        - `max_body_size`: the max number of bytes to receive in the response body; defaults to 10MB.
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
        - `max_body_size`: the max number of bytes to receive in the response body; defaults to 10MB.
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
    ]=]
    post: (options: HttpRequestWithBody) -> HttpResponse,
    --[=[
        Send an HTTP 1.1 request to the url or uri at `options.url` and explicitly handle errors.

        Set `options.timeout` to prevent this function from blocking indefinitely 
        when the internet is disconnected or the destination doesn't respond.

        Required fields:

        - `url`: A URL or URI string; for localhost use `"127.0.0.1:<PORT>"`.

        Optional fields:

        - `params`: query parameters map that gets serialized and appended to `url`.
        - `headers`: headers map, which can include API keys or similar.
        - `timeout`: `Duration` or `RequestTimeout` table containing `Durations`.
        - `max_body_size`: the max number of bytes to receive in the response body; defaults to 10MB.
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
    ]=]
    request: (method: HttpMethod, options: HttpRequestWithoutBody | HttpRequestWithBody) -> HttpResponseResult
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

--[=[
    Configure how quickly a request times out.
]=]
export type RequestTimeout = {
    --- Max duration to send the request but not the request body.
    send_request: Duration?,
    --- Max duration to send the request body.
    send_body: Duration?,
    --- Max duration to wait for receiving the response headers but not the response body.
    receive_response: Duration?,
    --- Max duration to wait for receiving the response body.
    receive_body: Duration?,
}

type JsonSerializable = { [string]: any } | { any }

export type HttpRequestWithoutBody = {
    --- The URI or URL you want to send the request to. For localhost, use `127.0.0.1:80` (example uses port 80)
    url: string,
    --[=[
        [HTTP query parameters](https://developer.mozilla.org/en-US/docs/Learn_web_development/Howto/Web_mechanics/What_is_a_URL#parameters) to append to `url`.

        Key-value pairs are serialized and appended as `?key=value&key2=value2`.
    ]=]
    params: {
        [string]: string,
    }?,
    --[=[
        [HTTP headers](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers) to send with the request.
        
        Common headers include `Authorization`, `Accept`, and `User-Agent`.

        Header keys are case-insensitive as per the HTTP spec; values must be valid ASCII.
    ]=]
    headers: {
        [string]: string,
    }?,
    --- Prevent your request from taking forever. Pass a `Duration` from `@std/time` to set a global timeout or a `RequestTimeout` for more granular control.
    timeout: Duration | RequestTimeout | nil,
    max_body_size: number?,
    --- Max number of redirects to redirect through before erroring out; defaults to 10. Pass 0 to not redirect anywhere and return the original response.
    max_redirects: number?,
}

export type HttpRequestWithBody = {
    --- The URI or URL you want to send the request to. For localhost, use `127.0.0.1:80` (example uses port 80)
    url: string,
    --[=[
        The main content of your request. You can pass in a json-serializable table to send json, a string to send unchanged,
        or a buffer to send as arbitary/invalid utf-8.

        Note that strings have to be valid utf-8; if you need to pass invalid utf-8 here you must pass a buffer.
    ]=]
    body: string | JsonSerializable | buffer,
    --[=[
        [HTTP query parameters](https://developer.mozilla.org/en-US/docs/Learn_web_development/Howto/Web_mechanics/What_is_a_URL#parameters) to append to `url`.

        Key-value pairs are serialized and appended as `?key=value&key2=value2`.
    ]=]
    params: {
        [string]: string,
    }?,
    --[=[
        [HTTP headers](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers) to send with the request.

        Some common headers you'll want to send (or override) include `Authorization`, `Accept`, and `User-Agent`.

        Header keys are case-insensitive as per the HTTP spec; values must be valid ASCII.        

        *seal* automatically sets `Content-Type` for you when you send a table (application/json) or buffer (application/octet-stream) as body.
    ]=]
    headers: {
        [string]: string,
    }?,
    --- Prevent your request from taking forever. Pass a `Duration` from `@std/time` to set a global timeout or a `RequestTimeout` for more granular control.
    timeout: Duration | RequestTimeout | nil,
    max_body_size: number?,
    --- Max number of redirects to redirect through before erroring out; defaults to 10. Pass 0 to not redirect anywhere and return the original response.
    max_redirects: number?,
}

--[=[
    [MIME types](https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/MIME_types) identify the format/filetype of the content and usually come in the
    Content-Type header of a request/response.

    *seal* provides the following common MIME types for autocomplete convenience purposes only—this is not an exhaustive list.
]=]
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


export type HttpResponseResult =
    | HttpResponse
    | HttpTimeoutError
    | HttpIoError

export type HttpResponse = {
    --- `true` if the response was successful (status code 200 -> 299), `false` if the response represents an unsuccessful status code.
    ok: boolean,
    --- `"Ok"` if the response represents a successful status code (200 -> 299), `"Error"` if the response represents an unsuccessful status code.
    kind: "Ok" | "Error",
    headers: {
        [string]: string,
    },
    status: {
        code: number,
        --- The canonical HTTP reason phase for the status code; such as "Not Found" for status code 404
        reason: string,
    },
    --- The response's main contents; this can be any encoding, including UTF-8, arbitrary unreadable bytes, json, etc.
    body: string,
    --[=[
        The parsed [Content-Type](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Type) response header, if present.

        `content-type` describes the format of the response body so you know how to interpret it.
        It consists of a `mime_type` (what kind of data) and an optional `charset` (how it's encoded).

        For example, `content-type: application/json; charset=utf-8` parses to:
        - `mime_type = "application/json"`
        - `charset = "utf-8"`

        This field is `nil` if the server didn't send a `content-type` header at all.
    ]=]
    content_type: {
        --- The MIME type of the response body — describes *what kind* of data it is,
        --- e.g. `"application/json"`, `"text/html"`, `"image/png"`.
        --- `nil` if the `content-type` header was present but had no MIME type.
        mime_type: MimeType?,
        --- The character encoding of the response body, e.g. `"utf-8"` or `"iso-8859-1"`.
        --- Only present for text-based MIME types; typically absent for binary types like `"image/png"`.
        charset: string?,
    }?,
    --[=[
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
    ]=]
    try_json: <T>(self: HttpResponse) -> T?,
    --[=[
        Decode the response body as JSON into a table. Errors if the body wasn't json or decoding was otherwise unsuccessful.
        
        Pass an explicit type parameter with `<<T>>` syntax to *promise* that the returned type is of type `T`.

        This makes it easier to typecheck on the return type of this function, but doesn't provide any runtime assurance
        that the table returned is actually of type `T`.

        ## Usage

        ```luau
        type ApiResponse = { id: string, downloadUri: string }
        local data = http.get(options):expect_json<<ApiResponse>>()
        ```
    ]=]
    expect_json: <T>(self: HttpResponse) -> T,
}

--[=[
    The request timed out because you set `HttpRequestOptions.timeout`.
]=]
export type HttpTimeoutError = {
    ok: false,
    kind: "Timeout",
    reason: string,
    phase: "Global" | "SendRequest" | "ReceiveResponse" | "SendBody" | "ReceiveBody" | "Connect" | "Resolve" | "Other"
}

--[=[
    We encountered some network, io, or connection error that prevented *seal* from sending the request or
    receiving a proper response.
]=]
export type HttpIoError = {
    ok: false,
    kind: "BodyExceedsLimit" | "NetworkError" | "ConnectionFailed" | "TooManyRedirects" | "Other",
    reason: string,
}
return {} :: http