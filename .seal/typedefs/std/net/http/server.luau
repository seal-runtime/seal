local server = {}

--[=[
    Create a webserver that listens for incoming requests.

    ⚠️ Expect breaking changes. This API will be heavily modified in the future.

    ## Usage
    
    ```luau
    local server = require("@std/net/http/server")
    local fs = require("@std/fs")
    local json = require("@std/json")

    print("starting seal server")

    server.serve {
        address = "localhost",
        port = 4242,
        handler = function(info: server.ServeRequest)
            local response = {}
            if info.path == "/meow.json" then
                response.status_code = "200 OK"
                response.content_type = "json"
                response.body = json.encode {
                    ok = true,
                    says = "meow"
                }
            elseif info.path == "/" then
                local meow_page = fs.readfile("./tests/data/server-views/index.html")
                response.status_code = "200 OK"
                response.content_type = "html"
                response.body = meow_page
            elseif info.path == "/info" then
                local body = fs.readfile("./tests/data/server-views/info.html")
                response = {
                    status_code = "200 OK",
                    content_type = "html",
                    body = body
                }
            elseif info.path == "/some-post" then
                response = {
                    status_code = "200 OK",
                    content_type = "application/json",
                    body = json.encode {
                        ok = true,
                        recvbody = info.body,
                    }
                }
            else
                response.status_code = "404 Not Found"
                response.response_type = "json"
                response.body = json.encode {
                    ok = false,
                }
            end
            return response :: server.ServeResponse
        end
    }
    ```
]=]
function server.serve(config: ServeConfig)
    
end

type StatusCode =
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

type ContentType = 
    | "Text"
    | "HTML"
    | "JSON"
    | "XML"
    | "CSS"
    | "JavaScript"
    | "Binary"
    | string

export type ServeRequest = {
    peer_address: string,
    method: "GET" | "POST" | "PUT" | "PATCH" | "DELETE",
    path: string,
    headers: {
        [string]: string,
    },
    raw_text: string,
    body: string,
}

export type ServeResponse = {
    status_code: StatusCode,
    content_type: ContentType,
    body: string,
    headers: {
        [string]: string,
    }?,
    cookies: {
        [string]: string,
    }?,
    http_version: string?,
    reason_phrase: string?,
    redirect_url: string?
}
    
export type ServeConfig = {
    address: string,
    port: string | number,
    handler: (ServeRequest) -> ServeResponse,
}

return server