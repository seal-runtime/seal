local fs = require("@std/fs")
local extern = require("@interop/extern")
local json = require("@std/serde/json")

local webview = extern.load(fs.path.join(".", "target", "debug", "libseal_webview.so")) :: { 
    create: (config: {
        title: string,
        content: string,
        size: {
            x: number,
            y: number,
        },
        on_message: (message: string) -> { content: string }?,
    }) -> nil
}

webview.create({
    title = "meow",
    content = fs.readfile(fs.path.join(script:parent(), "view.html")),
    size = {
        x = 1024,
        y = 600,
    },
    on_message = function(message)
        local response = json.decode(message)
        if response.type == "mouse-position" then
            return nil
        elseif response.type == "text-update" then
            local text_content = response.content :: string
            if string.match(text_content, "meow") then
                return {
                    content = "<h1> haha you said meow </h1>"
                }
            end
        end
        return nil
    end
})