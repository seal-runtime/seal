local colors = require("@std/io/colors")
local str = require("@std/str")

const function enclose_component(name: string, text: string, color: vector): string
    const code = colors.rgb(color)
    return colors.codes.BOLD .. code .. name .. "<" .. colors.codes.RESET .. text .. colors.codes.BOLD .. code .. ">" .. colors.codes.RESET
end

type Component = HeadingComponent | TextComponent | CodeblockComponent | ListComponent

export class HeadingComponent
    public level: number
    public text: string
    function markdown(self)
        return string.rep("#", self.level) .. " " .. self.text
    end
    function __tostring(self)
        return enclose_component("Heading", self:markdown(), vector.create(225, 100, 100))
    end
end
export class TextComponent
    public text: string
    function __tostring(self)
        return enclose_component("Text", self.text, vector.create(180, 180, 255))
    end
end
export class CodeblockComponent
    public text: string
    function markdown(self)
        return "```luau\n" .. self.text .. "\n```"
    end
    function __tostring(self)
        return enclose_component("Code", "\n" .. self.text, vector.create(144, 144, 244))
    end
end
export class ListItem
    public indent: number
    public text: number
    function __tostring(self)
        return enclose_component("ListItem", `i: {self.indent}: {self.text}`, vector.create(80, 80, 80))
    end
end
export class ListComponent
    public type: "Ordered" | "Unordered"
    public items: { ListItem }

    function from(lines: { string }, type: "Ordered" | "Unordered")
        const items: { ListItem } = {}
        for _, line in lines do
            const trimmed = str.trimfront(line)
            const diff = #line - #trimmed
            table.insert(items, ListItem { indent = diff, text = trimmed })
        end
        return ListComponent { items = items, type = type }
    end
    function markdown(self)
        const lines: { string } = {}
        for prefix, item in self.items do
            const bullet = if self.type == "Ordered" then tostring(prefix) else "-"
            table.insert(lines, string.rep(" ", item.indent) .. bullet .. " " .. item.text)
        end
        return table.concat(lines, "\n")
    end
    function __tostring(self)
        const lines = {}
        for _, item in self.items do
            table.insert(lines, "  " .. tostring(item))
        end
        return enclose_component(
            "List",
            `{colors.blue(`({self.type})`)}\n` 
            .. table.concat(lines, "\n") .. "\n",
            vector.create(140, 70, 80)
        )
    end
end

export class Documentation
    public components: { Component }
    public location: number

    function from(text: string, location: number)
        const lines = str.splitlines(text)
        const components: { Component } = {}

        const last_lines: { string } = {}
        const function push_text()
            if #last_lines == 0 then
                return
            end
            const new_lines = str.trimback(table.concat(last_lines, "\n"))
            table.insert(components, TextComponent { text = new_lines })
            table.clear(last_lines)
        end

        local in_codeblock = false
        local codeblock_indentation: number?

        local in_list = false
        local list_indentation: number?
        local list_type: "Ordered" | "Unordered" | nil

        const function push_list(t: "Unordered" | "Ordered")
            table.insert(components, ListComponent.from(last_lines, t))
            table.clear(last_lines)
            in_list = false
            list_indentation = nil
            list_type = nil
        end

        const function push_codeblock()
            const codeblock_lines = table.concat(last_lines, "\n")
            table.insert(components, CodeblockComponent { text = codeblock_lines })
            table.clear(last_lines)
            in_codeblock = false
            codeblock_indentation = nil
        end

        for l, original_line in lines do
            local line = original_line

            if not in_codeblock then
                -- trimming first element of list is safe because should be farthest left
                line = str.trimfront(line)
                if str.starts(line, "--[=[", "]=]") then
                    continue
                end
            elseif in_codeblock and codeblock_indentation then
                line = string.sub(line, codeblock_indentation)
            end

            if not in_codeblock and str.startswith(line, "#") then
                const hashes, heading = string.match(line, "^([#]*)%s?([%w%s%p]+)")
                if hashes and heading then
                    push_text()
                    table.insert(components, HeadingComponent {
                        level = #hashes,
                        text = heading
                    })
                    continue
                end
            end

            if not in_codeblock and str.startswith(line, "```luau") then
                push_text()
                in_codeblock = true
                continue
            end

            if in_codeblock and codeblock_indentation == nil then
                const preceding_whitespace = string.match(line, "^([%s]+)[^%s+]")
                if preceding_whitespace then
                    codeblock_indentation = #preceding_whitespace + 1
                    line = string.sub(line, codeblock_indentation)
                end
            end

            if in_codeblock then
                const trimmed_line = str.trimfront(line)
                if str.startswith(trimmed_line, "```") then
                    push_codeblock()
                    continue
                end
            end

            if in_list and list_indentation and list_type then
                const still_starts_with_valid_prefix = str.startswith(line, "-") or string.find(line, "^[%d]+%. ")
                if still_starts_with_valid_prefix then
                    const digit_size = if list_type == "Ordered" then string.match(line, "^([%d]+)%. ") else nil
                    if digit_size then
                        line = string.sub(original_line, list_indentation + #digit_size)
                    else
                        line = string.sub(original_line, list_indentation + 1)
                    end
                else
                    push_list(list_type)
                end
            end

            if not in_list and not in_codeblock then
                const start_unordered_list = str.startswith(line, "-") and not str.startswith(line, "--[")
                const start_ordered_list = str.startswith(line, "1.")
                if start_ordered_list or start_unordered_list then
                    push_text()
                    in_list = true
                    list_type =
                        if start_unordered_list then
                            "Unordered"
                        elseif start_ordered_list then
                            "Ordered"
                        else
                            error("unreachable")
                    const preceding_whitespace = string.match(original_line, "^([%s]+)[^%s+]")
                    if preceding_whitespace then
                        list_indentation = #preceding_whitespace + 1
                        line = string.sub(original_line, list_indentation + if start_ordered_list then 2 else 1)
                    else
                        list_indentation = 0
                    end
                end
            end

            if in_list and in_codeblock then
                error("we should not be allowed to be in list and codeblock at same time")
            end

            table.insert(last_lines, str.trimback(line))
        end
        if in_list and list_type then
            push_list(list_type :: any)
        elseif in_codeblock then
            push_codeblock()
        else
            push_text()
        end

        return Documentation {
            components = components,
            location = location
        }
    end
    function __tostring(self)
        const count = #self.components
        const serialized = {}
        for _, component in self.components do
            table.insert(serialized, tostring(component))
        end
        const content = str.indent(table.concat(serialized, "\n"), "Spaces", 2)
        return enclose_component(
            "Documentation",
            colors.blue(`({count} component{if count == 1 then "" else "s"})\n`) .. content .. "\n",
            vector.create(190, 220, 190)
        )
    end
end