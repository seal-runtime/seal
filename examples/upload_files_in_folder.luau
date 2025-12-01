-- every time the user adds a file to the folder, we upload it to an endpoint
local fs = require("@std/fs")
local str = require("@std/str")
local time = require("@std/time")
local http = require("@std/net/http")

local API_KEY = str.trim(str.splitafter(fs.readfile("./.env"), "API_KEY=")[2])

--- a map of paths to times they were last seen (to prevent repeated uploads)
local debounce: { [string]: number? } = {}

local function should_skip(path: string): boolean
    local last_modified = debounce[path]
    if last_modified and (os.clock() - last_modified) < 1 then
        -- been less than 1 second since we've seen the path
        return true
    elseif not str.endswith(path, ".json") then
        -- not a json file
        return true
    end
    return false
end

local function name_and_content(path: string): (string?, string?)
    local file_name = fs.path.child(path)
    if not file_name then
        warn(`Couldn't find filename of '{path}'`)
        return nil, nil
    end
    time.wait(0.05) -- fix race condition
    local content, result = fs.file.try_read(path)
    if not content then
        warn(`Couldn't read '{path}' due to err: {result}`)
        return nil, nil
    end
    return file_name, content
end

fs.makedir(fs.path.join(".", "data", "input"), {
    create_missing = true,
    error_if_exists = false,
})

for category, event in fs.watch(fs.path.join(".", "data", "input")) do
    if event.is_write then
        for _, path in event.paths do
            if should_skip(path) then
                continue
            end
            local file_name, content = name_and_content(path)
            if not file_name or not content then
                continue
            end
            debounce[path] = os.clock()
            local response = http.post {
                url = "http://127.0.0.1:8080/post",
                headers = {
                    Authorization = `Bearer {API_KEY}`
                },
                body = {
                    name = file_name,
                    content = p(content)
                }
            }
            if not response.ok then
                warn(`Couldn't upload '{path}' due to reason: {response.err}`)
            end
        end
    end
end
