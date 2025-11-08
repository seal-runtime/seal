local tt = require("@extra/tt")
local fs = require("@std/fs")
local process = require("@std/process")
local str = require("@std/str")
local time = require("@std/time")

local defs_path = fs.dir.project():join(".seal", "typedefs", "std")
local refs_path = fs.dir.project():join("docs", "reference")

type Api = {
    is: "Function" | "Property" | "Type",
    name: string,
    docs: string,
}

type StandardLibrary = {
    is: "Library",
    name: string,
    path: string,
    defs: string,
    docs: string,
    apis: { Api },
}

local function setup()
    -- ensure user has markdownlint-cli2 installed before doing anything destructive
    local markdownlint_help = process.shell("markdownlint-cli2 --help")
    if #markdownlint_help.stderr > 5 then
        print("you need markdownlint-cli2 installed: https://github.com/DavidAnson/markdownlint-cli2")
        process.exit(1)
    end
    
    -- burn existing libraries
    for _, existing in fs.listdir(refs_path) do
        if fs.is(existing) == "File" then
            fs.removefile(existing)
        elseif fs.is(existing) == "Directory" then
            fs.removetree(existing)
        end
    end    
end

setup()

local libraries: { StandardLibrary } = {}

local typedefs_paths = fs.listdir(defs_path, true, function(path: string)
    return str.endswith(path, ".luau")
end)

local processing_start_time = os.clock()

for _, filepath in typedefs_paths do
    local apis_for_path: { Api } = {}

    local library_path: string = filepath do
        library_path = str.trimback(library_path:gsub(fs.path.cwd() .. "/.seal/typedefs/", ""), ".luau")
        library_path ..= ".md"
    end
    local library_name = str.trimfront(str.trimback(library_path, ".init.md", ".md"), "std/"):gsub("/", ".")

    local top_defs: { string } = {}
    local top_docs: { string } = {}

    local in_docs = false
    local reset_in_docs_next_iteration = false

    local in_a_codeblock = false
    local last_line_started_codeblock = false
    local codeblock_preceding_whitespace: string? = nil

    --- represents the 'path' to an api, such as 'SpawnOptions' or 'SpawnOptions.stream.stdout_capacity'
    local current_docs: { string } = {}

    local current_names: { string } = {}
    local current_indents: { number } = {}
    local function push_name(name: string, indent: number)
        assert(#current_names == #current_indents, "names/indents misaligned")
        table.insert(current_names, name)
        table.insert(current_indents, indent)
    end
    local function pop_name()
        assert(#current_names == #current_indents, "names/indents misaligned")
        local last_name = table.remove(current_names, #current_names)
        local last_indent = table.remove(current_indents, #current_names)
        return last_name, last_indent
    end
    local function clear_names()
        table.clear(current_names)
        table.clear(current_indents)    
    end
    local function last_indent()
        return current_indents[#current_indents] or 0
    end
    
    for _, line in fs.readlines(filepath) do
        if not in_docs and string.match(line, "^%s*%-%-%[=%[") then
            in_docs = true
            continue
        elseif not in_docs and string.match(line, "^%s*%-%-%-") then
            in_docs = true
            reset_in_docs_next_iteration = true
        elseif in_docs and string.match(line, "%]=%]") then
            in_docs = false
            continue
        elseif in_docs and string.match(line, "```luau") then
            line = str.trimfront(line)
            in_a_codeblock = true
            last_line_started_codeblock = true
        elseif in_docs and in_a_codeblock and string.match(line, "```") then
            in_a_codeblock = false
            codeblock_preceding_whitespace = nil
        elseif in_docs and in_a_codeblock and last_line_started_codeblock then
            codeblock_preceding_whitespace = string.match(line, "^([%s]+)[%w]+")
            last_line_started_codeblock = false
        end

        if in_docs then
            if not in_a_codeblock then
                line = str.trimfront(line)
            elseif in_a_codeblock and codeblock_preceding_whitespace then
                line = str.unindent(line, "Spaces", #codeblock_preceding_whitespace)
            end
            if reset_in_docs_next_iteration then
                line = str.trimfront(line, "%-%-%-")
                reset_in_docs_next_iteration = false
                in_docs = false
            end
            table.insert(current_docs, line)
            continue
        end

        local found_type_name = string.match(line, "type ([%w_]+) =")
        if found_type_name then
            -- found 'export type NAME' or 'type NAME', this means we serialize the current
            -- top level api as a type and clear names
            local new_api: Api = {
                -- name = table.concat(current_names, "."),
                name = found_type_name,
                docs = table.concat(current_docs, "\n"),
                is = "Type",
            }
            table.insert(apis_for_path, new_api)
            table.clear(current_docs)
            clear_names()
            push_name(found_type_name, 0)
            continue
        else
            local indent, found_property_name = string.match(line, "(%s*)([%w_]+): {%s*$")
            if indent and found_property_name then
                push_name(found_property_name, #indent)
                continue
            end
        end

        local found_indent = string.match(line, "^(%s*)}[,]?%s*$")
        if #current_names > 0 and found_indent and #found_indent == last_indent() then
            pop_name()
        end

        if string.match(line, "[%w_]+:") then
            if not str.startswith(str.trimfront(line), "--") then
                local trimmed_line = str.trimfront(line)
                local found_name = table.concat(current_names, ".")
                if str.startswith(trimmed_line, "local ") then
                    continue
                end
                if #found_name == 0 or str.startswith(trimmed_line, "function") then
                    found_name = str.trimfront(trimmed_line, "function ")
                        :gsub("%(", ": (", 1)
                        :gsub("%): ", ") -> ")
                else
                    found_name = table.concat(current_names, ".") .. `.{trimmed_line}`
                end
                local new_api: Api = {
                    name = found_name,
                    docs = table.concat(current_docs, "\n"),
                    is = "Property",
                }
                table.insert(apis_for_path, new_api)
                table.clear(current_docs)
            end
        end

    end

    local library: StandardLibrary = {
        is = "Library",
        name = library_name,
        path = library_path,
        docs = table.concat(top_docs, "\n"),
        defs = table.concat(top_defs, "\n"),
        apis = apis_for_path
    }

    table.insert(libraries, library)
end

print(`finished processing in {os.clock() - processing_start_time} seconds!`)

local child_handles: { [string]: process.ChildProcess } = {}

for _, library in libraries do
    local lines: { string } = {}
    local function push(line: string, newlines: number?)
        newlines = newlines or 1
        table.insert(lines, line .. string.rep("\n", newlines))
    end

    push("<!-- markdownlint-disable MD033 -->\n<!-- markdownlint-disable MD024 -->")
    push(`# {library.name}`)
    local require_path = str.split(library.path, ".md", "/init.md")[1]
    push("`" .. `local {fs.path.child(require_path)} = require("@{require_path}")` .. "`")

    for index, api in library.apis do
        if index == 1 then
            push(api.docs, 2)
            continue
        end
        local split_name = str.splitafter(api.name, ":")
        local name, defs = table.remove(split_name, 1), str.trim(table.concat(split_name, ""))
        if name and defs then
            defs = str.trimback(defs, ",")
            if api.is == "Type" then
                name = "`export type` " .. name
            end
            if #defs > 0 then
                push(`{name} \`{defs}\``)
            else
                push(name)
            end
        end

        if #api.docs > 1 and #api.docs < 340 then
            push(api.docs)
        elseif #api.docs > 340 then
            push("<details>")
            push("<summary> See the docs </summary")
            push(api.docs)
            push("</details>", 2)
        end
        push("---")
        -- push(api.docs, 2)
    end

    local new_path = fs.path.join(refs_path, library.path)
    local parent_path = fs.path.parent(new_path)
    assert(parent_path ~= nil, "where parent path")

    fs.makedir(parent_path, {
        error_if_exists = false,
        create_missing = true,
    })

    fs.writefile(new_path, table.concat(lines, "\n"))

    local handle = process.spawn {
        program = "markdownlint-cli2",
        args = { "--fix", new_path }
    }

    child_handles[library.path] = handle
end

while time.wait(0.05) do
    for path, handle in child_handles do
        if not handle:alive() then
            print(`finished {path}`)
            child_handles[path] = nil
        end
    end
    if #tt.keys(child_handles :: any) == 0 then
        break
    end
end
