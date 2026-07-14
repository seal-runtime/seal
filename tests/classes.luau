local http = require("@std/net/http")

local m = require("./exportable")

print(m)

class NotFound
    public code: number
    public message: string

    function new(): NotFound
        return NotFound {
            code = 404,
            message = "404 Not Found"
        }
    end
end
class Success
    public code: number
    public body: string

    function new(body: string)
        return Success {
            code = 200,
            body = body
        }
    end
end
class Other
    public code: number
    public message: string

    function new(code: number, message: string)
        return Other {
            code = code,
            message = message
        }
    end
end

local function get_response(url: string): Success | NotFound | Other
    local resp = http.get {
        url = url,
    }
    if resp.ok then
        return Success.new(resp.body)
    elseif resp.status.code == 404 then
        return NotFound.new()
    else
        return Other.new(resp.status.code, resp.body)
    end
end

local result = get_response("https://meow.com")
if class.isinstance(result, Success) then
    print(`yay, got {#result.body} chars in response`)
elseif class.isinstance(result, NotFound) then
    print("oh no, not found :(")
else
    print(tostring(result))
end
