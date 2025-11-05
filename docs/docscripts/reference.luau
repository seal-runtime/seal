local fs = require("@std/fs")
local process = require("@std/process")
local str = require("@std/str")

local standard_library_reference = "./docs/standard-library-reference"
for _, existing in fs.listdir(standard_library_reference) do
    if fs.is(existing) == "File" then
        fs.removefile(existing)
    elseif fs.is(existing) == "Directory" then
        fs.removetree(existing)
    end
end

local std_defs = "./.seal/typedefs/std"

type Api = {
    path: string,
    name: string,
    docs: string,
}

local luau_files = fs.listdir(std_defs, true, function(path: string): boolean
    return str.endswith(path, ".luau")
end)

local apis_by_path: {
    [string]: { Api },
} = {}

for _, filename in luau_files do
    local in_doc = false
    local last_line_was_doc_end = false
    local in_a_codeblock = false
    local current_api_parent: string? = nil

    local apis_for_file: { Api } = {}
    local current_doc: { string } = {}

    for line_number, line in fs.readlines(filename) do
        local type_name = string.match(line, "export type ([%w_]+) =")
        if type_name then
            current_api_parent = type_name
        elseif string.match(line, "^%s*%-%-%-") then
            in_doc = true
            last_line_was_doc_end = true
        elseif string.match(line, "^%s*%-%-%[=%[") then
            in_doc = true
            continue
        elseif string.match(line, "%]=%]") then
            in_doc = false
            last_line_was_doc_end = true
            continue
        end
        if in_doc then
            if string.match(line, "```luau?") then
                in_a_codeblock = true
                line = str.trimfront(line)
            elseif in_a_codeblock and string.match(line, "```") then
                in_a_codeblock = false
            end
            if not in_a_codeblock then
                line = str.trimfront(line)
            end
            table.insert(current_doc, line)
        elseif last_line_was_doc_end then
            if in_doc then in_doc = false end
            last_line_was_doc_end = false
            line = str.trimback(line, ",")
            line = str.trimback(line, "=")
            if str.starts(line, "  ", "\t") and string.match(line, "%->") and current_api_parent then
                line = `{current_api_parent}.{str.trimfront(line)}`
                if string.match(line, `{current_api_parent}%.[%w_]+: `) then
                    line = `function {line:gsub(": ", "", 1):gsub(" %->", ":")}`
                end
            end
            local new_api: Api = {
                name = line,
                docs = table.concat(current_doc, "\n"),
                path = filename
            }
            table.insert(apis_for_file, new_api)
            table.clear(current_doc)
        elseif str.startswith(line, "function") then
            local new_api: Api = {
                name = line,
                docs = "",
                path = filename
            }
            table.insert(apis_for_file, new_api)
            table.clear(current_doc)
        end
    end

    if #apis_for_file > 0 then
        apis_by_path[filename] = apis_for_file
    end

end

local markdownlint_help = process.shell("markdownlint-cli2 --help")
if #markdownlint_help.stderr > 5 then
    print("you need markdownlint-cli2 installed: https://github.com/DavidAnson/markdownlint-cli2")
    process.exit(1)
end

for path, apis in apis_by_path do
    local reference_file_path = fs.path.join(
        standard_library_reference, 
        str.split(path, "typedefs/std", ".luau")[2]
    ) .. ".md"
    if str.startswith(fs.path.child(reference_file_path) :: string, "_") then
        continue
    end
    print(reference_file_path)
    
    local lines: { string } = {}
    local function push(line: string, newlines: number?)
        newlines = newlines or 1
        table.insert(lines, line .. string.rep("\n", newlines))
    end

    local header = (fs.path.child(reference_file_path) :: string):gsub("%.md$", "")
    if header == "init" then
        header = fs.path.child(fs.path.parent(reference_file_path) :: string) :: string
    end
    header = string.upper(header:sub(1, 1)) .. header:sub(2, #header)
    push("<!-- markdownlint-disable MD033 -->")
    push("# " .. header)
    local first_api = apis[1]
    if str.starts(first_api.name, "local", "export type") then
        push(first_api.docs)
        table.remove(apis, 1) -- pop header name
    end
    for _, api in apis do
        push("`" .. api.name .. "`")
        if #api.docs > 1 and #api.docs < 240 then
            push(api.docs)
        elseif #api.docs > 240 then
            push("<details>")
            push("<summary> See the docs </summary")
            push(api.docs)
            push("</details>", 2)
        end
    end

    local parent_path = fs.path.parent(reference_file_path)
    assert(parent_path ~= nil, "where parent path")

    fs.makedir(parent_path, {
        error_if_exists = false,
        create_missing = true,
    })

    fs.writefile(reference_file_path, table.concat(lines, "\n"))

    local markdownlinted_result = process.run {
        program = "markdownlint-cli2",
        args = { "--fix", reference_file_path }
    }

end
