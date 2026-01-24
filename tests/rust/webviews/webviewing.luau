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
    title = "hypothetical seal install ui",
    content = fs.readfile(fs.path.join(script:parent(), "view.html")),
    size = {
        x = 720,
        y = 600,
    },
    on_message = function(message)
        local response = json.decode(message)
        print(response)
        return nil
    end
})
