local args = require("@std/args")
local vars = require("@std/env/vars")
local http = require("@std/net/http")
local json = require("@std/serde/json")
local colors = require("@std/io/colors")

-- automatically loads from .env
local API_KEY = vars.get("SOME_API_KEY") or error("can't find api key")
local OUTPUT_FILE = "endpoint_results.json"

local parsed = args.parse("endpointer", "get the api"):simple(
    args.positional("name", "what param name to search for"),
    args.positional("value", "parameter value")
)

local name = parsed:expect<<string>>("name", "missing parameter name")
local value = parsed:expect<<string>>("value", "missing parameter value")

local response = http.get({
    url = "https://some_endpoint.dev/api",
    params = {
        [name] = value
    },
    headers = {
        Authorization = `Bearer {API_KEY}`
    }
})
type EndpointResult = {
    detections: { string },
    updated: string,
}

if response.ok then
    local contents = response:expect_json<<EndpointResult>>()
    json.writefile(OUTPUT_FILE, contents :: json.JsonData, {
        pretty = false,
        sorted = false,
    })
    print(`Saved {colors.blue(tostring(#contents.detections))} detections to {colors.green(OUTPUT_FILE)}`)
elseif response.status.code >= 400 and response.status.code < 500 then
    error("invalid parameter name or value?")
end
