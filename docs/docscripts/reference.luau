local fs = require("@std/fs")
local tt = require("@extra/tt")
local str = require("@std/str")
local time = require("@std/time")
local process = require("@std/process")

local fix = require("./fix_stragglers")

local defs_path = fs.dir.project():join(".seal", "typedefs", "std")
local refs_path = fs.dir.project():join("docs", "reference")

type Api = {
    is: "Function" | "Property" | "Type" | "Enum",
    name: string,
    defs: string,
    docs: string,
}

type StandardLibrary = {
    is: "Library",
    name: string,
    path: string,
    apis: { Api },
    sublibs: { [string]: string },
    api_closings: { [string]: string },
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
        if (fs.path.child(existing) :: string) == "README.md" then
            continue
        end
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
    end
    local library_name = str.trimfront(str.trimback(library_path, ".init"), "std/"):gsub("/", ".")

    local in_docs = false
    local reset_in_docs_next_iteration = false

    local in_a_codeblock = false
    local last_line_started_codeblock = false
    local codeblock_preceding_whitespace: string? = nil
    
    local current_docs: { string } = {}
    local sublibs: { [string]: string } = {}
    local api_closings: { [string]: string } = {}

    --- represents the 'path' to an api, such as 'SpawnOptions' or 'SpawnOptions.stream.stdout_capacity'
    local current_names: { string } = {}
    local current_indents: { number } = {}
    local function push_name(name: string, indent: number)
        assert(#current_names == #current_indents, "names/indents misaligned")
        table.insert(current_names, name)
        table.insert(current_indents, indent)
    end
    local function pop_name()
        assert(#current_names == #current_indents, "names/indents misaligned")
        local n = #current_names
        local last_name = table.remove(current_names, n)
        local last_indent = table.remove(current_indents, n)
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
            codeblock_preceding_whitespace = string.match(line, "^([%s]+)[%w%p]+")
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
                defs = line,
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
                local front_name = table.concat(current_names, ".")
                if #front_name == 0 then front_name = library_name end
                local new_api: Api = {
                    name = front_name .. "." .. found_property_name,
                    defs = found_property_name .. ": {",
                    docs = table.concat(current_docs, "\n"),
                    is = "Property",
                }
                table.insert(apis_for_path, new_api)
                table.clear(current_docs)
                push_name(found_property_name, #indent)
                continue
            end
        end

        local found_indent, found_close = string.match(line, "^(%s*)(}%??[,]?)%s*$")
        if #current_names > 0 and found_indent ~= nil and #found_indent == last_indent() then
            local full_name = table.concat(current_names, ".")
            api_closings[full_name] = found_close
            pop_name()
        end

        local trimmed_line = str.trimfront(line)

        if str.startswith(trimmed_line, "--") then
            continue -- is comment
        end

        local local_id, local_req = string.match(trimmed_line, "^local ([%w_]+) = require%(\"([^\"]+)\"%)%s*$")
        if local_id and local_req then
            sublibs[local_id] = local_req
            continue
        end

        if str.startswith(trimmed_line, "function") then
            trimmed_line = str.trimfront(trimmed_line, "function"):gsub("%(", ": (", 1)
        end

        if string.match(line, "^%s*|") then -- enum-like types
            local found_name = current_names[1]
            local new_api: Api = {
                name = found_name,
                defs = trimmed_line,
                docs = table.concat(current_docs, "\n"),
                is = "Enum",
            }
            table.insert(apis_for_path, new_api)
            table.clear(current_docs)
        elseif string.match(line, "[%w_]+: ") then
            local front_name = table.concat(current_names, ".")
            if #front_name == 0 then front_name = library_name end
            if str.startswith(trimmed_line, "local ") then
                continue
            end

            local property_name, rest = string.match(trimmed_line, "([%w_%.]+): (.+)")
            assert(property_name and rest, "unreachable")
            property_name = str.trimfront(property_name, front_name .. ".") -- fixes input.input.capture.focus

            local name = front_name .. "." .. property_name

            local is = "Property"
            local defs = rest
            if (str.startswith(rest, "(") and (string.match(rest, " %-> ") or string.match(rest, "%):"))) or str.startswith(rest, "<T") then
                defs = `function {name}{rest}`
                is = "Function"
            else
                defs = `{property_name}: {rest}`
            end

            local new_api: Api = {
                name = name,
                defs = defs,
                docs = table.concat(current_docs, "\n"),
                is = is,
            }

            table.insert(apis_for_path, new_api)
            table.clear(current_docs)
        elseif string.match(trimmed_line, "^%[") then -- map/index types like [string]: string
            local key_type, value_type = string.match(trimmed_line, "^%[([%w_%s%|%.]+)%]:%s*(.+)")
            if key_type and value_type then
                local front_name = table.concat(current_names, ".")
                if #front_name == 0 then front_name = library_name end
                local new_api: Api = {
                    name = front_name .. ".[" .. key_type .. "]",
                    defs = "[" .. key_type .. "]: " .. value_type,
                    docs = table.concat(current_docs, "\n"),
                    is = "Property",
                }
                table.insert(apis_for_path, new_api)
                table.clear(current_docs)
            end
        elseif #apis_for_path == 0 and string.match(trimmed_line, "return require") then
            local new_api: Api = {
                name = library_name,
                defs = trimmed_line,
                docs = table.concat(current_docs, "\n"),
                is = "Property",
            }
            table.insert(apis_for_path, new_api)
            table.clear(current_docs)
        end
    end

    local library: StandardLibrary = {
        is = "Library",
        name = library_name,
        path = library_path,
        apis = apis_for_path,
        sublibs = sublibs,
        api_closings = api_closings,
    }

    table.insert(libraries, library)
end

print(`finished processing in {os.clock() - processing_start_time} seconds!`)

local init_lib_paths: { [string]: boolean } = {}
for _, lib in libraries do
    if str.endswith(lib.path, "/init") then
        init_lib_paths[lib.path:sub(1, #lib.path - #"/init")] = true
    end
end

local child_handles: { [string]: process.ChildProcess } = {}

for _, library in libraries do
    local lines: { string } = {}
    local function push(line: string, newlines: number?)
        newlines = newlines or 1
        table.insert(lines, line .. string.rep("\n", newlines))
    end

    local function resolve_require_path(raw_path: string): string?
        if str.startswith(raw_path, "@self/") then
            local sublib = str.trimfront(raw_path, "@self/")
            local lib_dir = if str.endswith(library.path, "/init")
                then library.path:sub(1, #library.path - #"/init")
                else library.path
            return lib_dir .. "/" .. sublib
        elseif str.startswith(raw_path, "@std/") then
            return "std/" .. str.trimfront(raw_path, "@std/")
        elseif str.startswith(raw_path, "./") then
            local rel = str.trimfront(raw_path, "./")
            local lib_dir = library.path:match("^(.+)/[^/]+$") or library.path
            return lib_dir .. "/" .. rel
        end
        return nil
    end

    push("<!-- markdownlint-disable MD033 -->\n<!-- markdownlint-disable MD024 -->")
    push(`# {library.name}`)
    local require_path = str.split(library.path, "/init")[1]
    local lib_child = fs.path.child(require_path) :: string
    if str.startswith(lib_child, "_") then
        local parent_path = library.path:match("^(.+)/[^/]+$") or library.path
        push(`*This file contains common types and is not an importable library. See [this library's top level docs](/docs/reference/{parent_path}/README.md) for usage information.*`)
    else
        push("`" .. `local {lib_child} = require("@{require_path}")` .. "`")
    end

    local main_type_name: string? = nil
    if not str.startswith(lib_child, "_") then
        for _, a in library.apis do
            if a.is == "Type" then
                main_type_name = a.name
                break
            end
        end
    end

    local function get_ancestors(name: string): { string }
        local ancestors: { string } = {}
        local prefix = ""
        for segment in name:gmatch("([^%.]+)%.") do
            if #prefix > 0 then prefix ..= "." end
            prefix ..= segment
            table.insert(ancestors, prefix)
        end
        return ancestors
    end

    for index, api in library.apis do
        if main_type_name ~= nil and #library.apis > 1 and index == 1 then
            push(api.docs, 2)
            push("---")
            continue
        end

        local name =
            if api.is == "Type" then
                `\`export type\` {api.name}`
            elseif api.is == "Enum" then
                `\`enum\` {api.name}`
            else api.name
        local defs = api.defs

        local first_name_segment = api.name:match("^([^%.]+)")
        local dot_count = 0
        for _ in api.name:gmatch("%.") do dot_count += 1 end
        local type_depth = if main_type_name ~= nil and first_name_segment == main_type_name
            then math.max(0, dot_count - 1)
            else dot_count
        local is_nested_in_type = (api.is == "Property" or api.is == "Function") and type_depth > 0
        local indent = if api.is == "Enum" then "  "
            elseif api.is == "Function" then ""
            else string.rep("  ", type_depth)

        if api.is == "Type" then
            push(`## {name}`)
        end

        if api.is == "Property" or api.is == "Function" then
            if is_nested_in_type then
                local h = if type_depth >= 2 then "####" else "###"
                push(`{h} {name}`)
            else
                push(`### {name}`)
            end
        end

        if #api.defs > 0 then
            local require_path =
                string.match(defs, `^[%w_]+: typeof%(require%("([^"]+)"%)%),?$`) or
                string.match(defs, `^return require%("([^"]+)"%)$`)
            local target_path: string? = nil
            local target_anchor: string? = nil
            local target_display: string? = nil
            if require_path then
                target_path = resolve_require_path(require_path)
            end
            if not target_path then
                local lib_id = string.match(defs, `^[%w_]+:%s+([%w_]+)%.[%w_]+,?$`)
                local raw_path = if lib_id then library.sublibs[lib_id] else nil
                if raw_path then
                    target_path = resolve_require_path(raw_path)
                end
            end
            if not target_path then
                local lib_id, type_name = string.match(defs, `^%s*export type [%w_]+ = ([%w_]+)%.([%w_]+)%s*$`)
                local raw_path = if lib_id then library.sublibs[lib_id] else nil
                if raw_path then
                    target_path = resolve_require_path(raw_path)
                    target_anchor = "#export-type-" .. string.lower(type_name)
                    target_display = `{type_name} in @{target_path}`
                end
            end
            if target_path then
                local suffix = if init_lib_paths[target_path] then "/README.md" else ".md"
                local anchor = target_anchor or ""
                local display = target_display or ("@" .. target_path)
                push(`See [{display}](/docs/reference/{target_path}{suffix}{anchor})`, 2)
            else
                if api.is ~= "Enum" then
                    push("<h4>", 2)
                end
                push("```luau", 0)
                push(indent .. defs, 0)
                push("```", 2)
                if api.is ~= "Enum" then
                    push("</h4>")
                end
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

        local next_api = library.apis[index + 1]
        local current_ancestors = get_ancestors(api.name)
        local next_ancestor_set: { [string]: boolean } = {}
        if next_api then
            for _, na in get_ancestors(next_api.name) do
                next_ancestor_set[na] = true
            end
        end
        for i = #current_ancestors, 1, -1 do
            local ancestor = current_ancestors[i]
            local closing = library.api_closings[ancestor]
            if not next_ancestor_set[ancestor] and closing and ancestor ~= main_type_name then
                local ancestor_depth = 0
                for _ in ancestor:gmatch("%.") do ancestor_depth += 1 end
                local close_indent = string.rep("  ", ancestor_depth)
                local short_name = ancestor:match("([^%.]+)$") or ancestor
                push("```luau", 0)
                push(close_indent .. closing .. " -- closes " .. short_name, 0)
                push("```", 2)
            end
        end
        -- also emit closing for the current api itself if it opened a sub-table
        -- but has no child apis (e.g. add: { [string]: string }? where [string] can't be parsed)
        local own_closing = library.api_closings[api.name]
        if own_closing and api.name ~= main_type_name and not next_ancestor_set[api.name] then
            local own_depth = 0
            for _ in api.name:gmatch("%.") do own_depth += 1 end
            local close_indent = string.rep("  ", own_depth)
            local short_name = api.name:match("([^%.]+)$") or api.name
            push("```luau", 0)
            push(close_indent .. own_closing .. " -- closes " .. short_name, 0)
            push("```", 2)
        end

        push("---")
    end

    local library_path_dot_luau = library.path .. ".luau"

    push(`Autogenerated from ({library_path_dot_luau})[/.seal/typedefs/{library_path_dot_luau}].`, 2)
    push("*seal* is best experienced with inline, in-editor documentation. Please see the linked typedefs file if this documentation is confusing, too verbose, or inaccurate.")

    local new_path = if str.endswith(library.path, "/init")
        then fs.path.join(refs_path, fs.path.parent(library.path) :: string, "README.md")
        else fs.path.join(refs_path, library.path .. ".md")
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

fix.semver()
print("semver.md fixed")

fix.thread()
print("thread.md fixed")