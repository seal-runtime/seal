local colors = require("@std/io/colors")
local output = require("@std/io/output")
local str = require("@std/str")

type HttpResponse = {
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
        mime_type: string?,
        charset: string?,
    }?,
    try_json: <T>(self: HttpResponse) -> T?,
    expect_json: <T>(self: HttpResponse) -> T,
}

local function format_bytes_for_display(n: number): string
    if n >= 1048576 then
        return string.format("%.2f MB", n / 1048576)
    elseif n >= 1024 then
        return string.format("%.2f KB", n / 1024)
    else
        return string.format("%d bytes", n)
    end
end

return function(self: HttpResponse): string
    local header_count = 0
    local output_horizontal_width = output.size().x
    local header_names: { string } = {}
    for name, _ in self.headers do
        header_count += 1

        local last = header_names[#header_names]
        if last and (2 + #last + #name) < output_horizontal_width then
            header_names[#header_names] = last .. ", " .. name
        else
            table.insert(header_names, `    {name}`)
        end
    end

    local lines = {
        colors.bold.white("HttpResponse") .. " " .. colors.red("<table>"),
        `  ok: {colors.bold.yellow(tostring(self.ok))}`,
        `  status: {colors.blue(tostring(self.status.code))} {self.status.reason}`,
        (
            if self.content_type and self.content_type.mime_type and self.content_type.charset then
                `  content_type: {self.content_type.mime_type}; charset={self.content_type.charset}`
            elseif self.content_type and self.content_type.mime_type then
                `  content_type: {self.content_type.mime_type};`
            else 
                "  content_type: unspecified"
        ),
        `  headers: {header_count} header pairs:`,
        table.concat(header_names, "\n"),
    }

    if self.content_type then
        local mime_type = self.content_type.mime_type
        if mime_type then
            if 
                mime_type == "application/json" or 
                mime_type == "text/plain" or 
                str.startswith(mime_type, "text/") 
            then
                local show_full_body = #self.body < 240
                local body_for_display = 
                    if show_full_body then 
                        self.body
                    else 
                        string.sub(self.body, 1, 240) .. colors.bold.white("...")
                local formatted_bytes = format_bytes_for_display(#self.body)
                table.insert(lines, `  body: {if show_full_body then formatted_bytes else `1..240 bytes of {formatted_bytes}`}`)
                table.insert(lines, colors.bold.white("──────── BODY ────────"))
                table.insert(lines, body_for_display)
                if show_full_body then
                    table.insert(lines, colors.bold.white("──────── BODY ────────"))
                end
            end
        end
    end
    
    return table.concat(lines, "\n")
end
