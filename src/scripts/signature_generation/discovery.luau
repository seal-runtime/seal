-- Shared typedef auto-discovery for generate.luau and validate.luau.
-- Scans .seal/typedefs/std/** and produces typed entries describing each
-- extractable prefix (module functions and class methods).

local parser = require("@libraries/LuauParser/Parser")
local fs = require("@std/fs")

local REPO_ROOT = fs.path.project()
assert(REPO_ROOT, "project root doesn't exist?")
local TYPEDEFS_DIR = fs.path.join(REPO_ROOT, ".seal/typedefs/std")

-- ── Path → prefix ─────────────────────────────────────────────────────────────
-- fs/file.luau       → fs.file
-- net/http/init.luau → net.http
-- str.luau           → str

local function path_to_prefix(rel_path: string): string
    local p = string.gsub(rel_path, "%.luau$", "")
    local parts: { string } = {}
    for part in string.gmatch(p, "[^/]+") do
        table.insert(parts, part)
    end
    local last = parts[#parts]
    if last == "init" or last == "_types" then table.remove(parts) end
    return table.concat(parts, ".")
end

-- ── AST type → string ─────────────────────────────────────────────────────────

local type_to_string: (node: any) -> string
local format_return: (tp: any) -> string

local function typepack_to_string(tp: any): string
    if not tp then return "()" end
    local kind = tp.kind :: string
    if kind == "TypePackExplicit" then
        local types = tp.types :: { any }
        local tail = tp.tailType
        if #types == 0 and not tail then return "()" end
        local parts = {}
        for _, t in types do table.insert(parts, type_to_string(t)) end
        if tail then table.insert(parts, typepack_to_string(tail)) end
        return "(" .. table.concat(parts, ", ") .. ")"
    elseif kind == "TypePackVariadic" then
        return "..." .. type_to_string(tp.variadicType)
    elseif kind == "TypePackGeneric" then
        return (tp.genericName :: string) .. "..."
    end
    return "?"
end

type_to_string = function(node: any): string
    if not node then return "?" end
    local kind = node.kind :: string

    if kind == "TypeReference" then
        local prefix = node.prefix :: string?
        local name = node.name :: string
        local full = if prefix then prefix .. "." .. name else name
        local params = node.parameters :: { any }
        if params and #params > 0 then
            local ps = {}
            for _, p in params do
                if p.type then
                    table.insert(ps, type_to_string(p.type))
                elseif p.typePack then
                    table.insert(ps, typepack_to_string(p.typePack))
                end
            end
            return full .. "<" .. table.concat(ps, ", ") .. ">"
        end
        return full

    elseif kind == "TypeFunction" then
        local generics = node.generics :: { any }
        local gen_str = ""
        if generics and #generics > 0 then
            local gs = {}
            for _, g in generics do table.insert(gs, g.name :: string) end
            gen_str = "<" .. table.concat(gs, ", ") .. ">"
        end
        local arg_types = (node.argTypes :: any).types :: { any }
        local arg_names = node.argNames :: { any }
        local args = {}
        for i, arg_t in arg_types do
            local name_info = arg_names and arg_names[i]
            local arg_prefix = ""
            if type(name_info) == "table" and (name_info :: any).name then
                arg_prefix = (name_info :: any).name .. ": "
            end
            table.insert(args, arg_prefix .. type_to_string(arg_t))
        end
        return gen_str .. "(" .. table.concat(args, ", ") .. ")" .. format_return(node.returnTypes)

    elseif kind == "TypeTable" then
        local props = node.props :: { any }
        local indexer = node.indexer :: any?
        if (#props == 0) and not indexer then return "{}" end
        local parts = {}
        for _, prop in props do
            local pname = (prop :: any).name.value :: string
            table.insert(parts, pname .. ": " .. type_to_string((prop :: any).type))
        end
        if indexer then
            table.insert(parts, "[" .. type_to_string(indexer.indexType) .. "]: " .. type_to_string(indexer.resultType))
        end
        return "{ " .. table.concat(parts, ", ") .. " }"

    elseif kind == "TypeUnion" then
        local types = node.types :: { any }
        if #types == 2 and types[2].kind == "TypeOptional" then
            return type_to_string(types[1]) .. "?"
        end
        local parts = {}
        for _, t in types do
            table.insert(parts, if t.kind == "TypeOptional" then "nil" else type_to_string(t))
        end
        return table.concat(parts, " | ")

    elseif kind == "TypeIntersection" then
        local parts = {}
        for _, t in (node.types :: { any }) do table.insert(parts, type_to_string(t)) end
        return table.concat(parts, " & ")

    elseif kind == "TypeOptional" then
        return "nil"

    elseif kind == "TypeGroup" then
        return type_to_string(node.type)

    elseif kind == "TypeSingletonBool" then
        return tostring(node.value :: boolean)

    elseif kind == "TypeSingletonString" then
        local v = node.value :: string
        v = string.gsub(v, "\\", "\\\\")
        v = string.gsub(v, "\r", "\\r")
        v = string.gsub(v, "\n", "\\n")
        v = string.gsub(v, "\t", "\\t")
        return '"' .. v .. '"'

    elseif kind == "TypeTypeof" then
        return "typeof(...)"
    end

    return "?" .. kind
end

format_return = function(tp: any): string
    if not tp then return "" end
    local kind = tp.kind :: string
    if kind == "TypePackExplicit" then
        local types = tp.types :: { any }
        local tail = tp.tailType
        if #types == 0 and not tail then return "" end
        if #types == 1 and not tail then return " -> " .. type_to_string(types[1]) end
        local parts = {}
        for _, t in types do table.insert(parts, type_to_string(t)) end
        if tail then table.insert(parts, typepack_to_string(tail)) end
        return " -> (" .. table.concat(parts, ", ") .. ")"
    elseif kind == "TypePackVariadic" then
        return " -> ..." .. type_to_string(tp.variadicType)
    elseif kind == "TypePackGeneric" then
        return " -> " .. (tp.genericName :: string) .. "..."
    end
    return ""
end

-- ── Table type helpers ────────────────────────────────────────────────────────

local function extract_table_from_type(t: any): any?
    if not t then return nil end
    local kind = t.kind :: string
    if kind == "TypeTable" then return t
    elseif kind == "TypeReference" then
        if t.name == "setmetatable" or t.name == "typeof" then
            local params = t.parameters :: { any }
            if params and #params >= 1 and params[1].type then
                return extract_table_from_type(params[1].type)
            end
        end
    elseif kind == "TypeGroup" then
        return extract_table_from_type(t.type)
    elseif kind == "TypeIntersection" then
        -- e.g. `{ methods } & (Ok | Err)` — take the first table-typed part
        for _, sub in (t.types :: { any }) do
            local tbl = extract_table_from_type(sub)
            if tbl then return tbl end
        end
    end
    return nil
end

-- True if a TypeFunction node has 'self' as its first argument.
local function fn_has_self(fn_node: any): boolean
    local arg_names = fn_node.argNames :: { any }
    if not arg_names or #arg_names == 0 then return false end
    local first = arg_names[1]
    return type(first) == "table" and (first :: any).name == "self"
end

-- True if a TypeTable has any non-metamethod function prop with 'self' —
-- meaning the type is a class (methods bound to an instance) not a module.
local function table_is_class(table_type: any): boolean
    for _, prop in (table_type :: any).props do
        local prop_name = (prop :: any).name.value :: string
        if string.sub(prop_name, 1, 2) == "__" then continue end
        local prop_type = (prop :: any).type :: any
        if prop_type.kind == "TypeFunction" and fn_has_self(prop_type) then
            return true
        end
    end
    return false
end

-- ── Extraction ────────────────────────────────────────────────────────────────

-- Returns { fn_name → type_sig } extracted from one typedef file.
--
-- type_name:       process only this specific alias (for class types like ChildProcess);
--                  nil = process all non-class aliases (module-level functions).
-- is_method:       skip the first 'self' arg from extracted signatures.
-- module_var_hint: local var name to match StatFunction declarations (e.g. "str", "input").
--                  Used as fallback when no type alias produces functions.
--
-- Non-function props are stored as "<complex>" so callers can distinguish
-- "present in typedef but unverifiable" from "missing from typedef".
local function extract_fn_sigs(typedef_path: string, is_method: boolean, type_name: string?, module_var_hint: string?, prop_path: { string }?): { [string]: string }
    local content = fs.file.try_read(typedef_path)
    if not content then return {} end

    -- local success, result = Parser.parse(content)
    local result = parser.parse(content)
    if typeof(result) == "error" or not result.root then
        return {}
    end

    local fns: { [string]: string } = {}

    for _, stat in result.root.body do
        if stat.kind ~= "StatTypeAlias" then continue end
        local stat_name = (stat :: any).name :: string

        if type_name and stat_name ~= type_name then continue end

        local table_type = extract_table_from_type((stat :: any).type)
        if not table_type then continue end

        if prop_path then
            -- Navigate into nested table prop (e.g. { "bold" } → colors.bold sub-table)
            for _, prop_name in prop_path do
                local found: any = nil
                for _, prop in (table_type :: any).props do
                    if ((prop :: any).name :: any).value == prop_name then
                        found = extract_table_from_type((prop :: any).type)
                        break
                    end
                end
                if not found then table_type = nil :: any; break end
                table_type = found
            end
            if not table_type then continue end
        elseif not type_name then
            -- Without a specific type_name, skip class-type aliases —
            -- they have their own entries and are extracted separately.
            if table_is_class(table_type) then continue end
        end

        for _, prop in (table_type :: any).props do
            local prop_name = (prop :: any).name.value :: string
            if string.sub(prop_name, 1, 2) == "__" then continue end  -- skip metamethods
            local prop_type = (prop :: any).type :: any

            if prop_type.kind ~= "TypeFunction" then
                fns[prop_name] = "<complex>"
                continue
            end

            local fn_node = prop_type
            local arg_types = (fn_node.argTypes :: any).types :: { any }
            local arg_names = fn_node.argNames :: { any }

            local skip_first = false
            if is_method and #arg_types >= 1 then
                local first_name = arg_names and arg_names[1]
                if type(first_name) == "table" and (first_name :: any).name == "self" then
                    skip_first = true
                end
            end

            local gen_str = ""
            local generics = fn_node.generics :: { any }
            if generics and #generics > 0 then
                local gs: { string } = {}
                for _, g in generics do table.insert(gs, g.name :: string) end
                gen_str = "<" .. table.concat(gs, ", ") .. ">"
            end

            local args: { string } = {}
            local start = if skip_first then 2 else 1
            for i = start, #arg_types do
                local arg_t = arg_types[i]
                local name_info = arg_names and arg_names[i]
                local arg_prefix = ""
                if type(name_info) == "table" and (name_info :: any).name then
                    arg_prefix = ((name_info :: any).name :: string) .. ": "
                end
                table.insert(args, arg_prefix .. (type_to_string(arg_t) :: string))
            end

            fns[prop_name] = gen_str .. "(" .. table.concat(args, ", ") .. ")" .. format_return(fn_node.returnTypes)
        end
    end

    -- Fallback: typedef files using `function module_var.fn_name(...)` style
    -- (e.g. str.luau, io/input.luau) where type aliases produce no real function signatures.
    -- Non-function type aliases (like an `InputReadOptions` options table) leave only "<complex>"
    -- entries behind, so check for actual extracted functions rather than an empty table.
    local has_extracted_fn = false
    for _, sig in fns do
        if sig ~= "<complex>" then
            has_extracted_fn = true
            break
        end
    end
    if not has_extracted_fn and module_var_hint then
        for _, stat in result.root.body do
            if stat.kind ~= "StatFunction" then continue end
            local stat_name_expr = (stat :: any).name :: any
            if not stat_name_expr or stat_name_expr.kind ~= "ExprIndexName" then continue end
            local fn_name = stat_name_expr.index :: string
            if string.sub(fn_name, 1, 2) == "__" then continue end  -- skip metamethods, like the alias path
            local base_expr = stat_name_expr.expr :: any
            local base_name: string
            if base_expr.kind == "ExprGlobal" then
                base_name = base_expr.name :: string
            elseif base_expr.kind == "ExprLocal" then
                base_name = (base_expr :: any)["local"].name :: string
            else
                continue
            end
            if base_name ~= module_var_hint then continue end

            local func = (stat :: any).func :: any

            -- Skip `function module.foo(self, ...)` methods — those belong to the module's instance
            -- type (e.g. `Semver:satisfies`), not the module library itself.
            local first_arg = (func.args :: { any })[1]
            if first_arg and (first_arg :: any).name == "self" then continue end

            local gen_str = ""
            local generics = func.generics :: { any }
            if generics and #generics > 0 then
                local gs: { string } = {}
                for _, g in generics do table.insert(gs, g.name :: string) end
                gen_str = "<" .. table.concat(gs, ", ") .. ">"
            end

            local args: { string } = {}
            for _, arg_local in (func.args :: { any }) do
                local a = arg_local :: any
                local arg_name = a.name :: string
                local annotation = a.annotation :: any?
                if annotation then
                    table.insert(args, arg_name .. ": " .. (type_to_string(annotation) :: string))
                else
                    table.insert(args, arg_name)
                end
            end
            if func.vararg then
                local vararg_ann = func.varargAnnotation :: any?
                if vararg_ann and vararg_ann.kind == "TypePackVariadic" then
                    table.insert(args, "...: " .. (type_to_string(vararg_ann.variadicType) :: string))
                elseif vararg_ann then
                    table.insert(args, typepack_to_string(vararg_ann))
                else
                    table.insert(args, "...")
                end
            end

            fns[fn_name] = gen_str .. "(" .. table.concat(args, ", ") .. ")" .. format_return(func.returnAnnotation)
        end
    end

    return fns
end

-- ── Auto-discovery ────────────────────────────────────────────────────────────

export type Entry = {
    prefix: string,         -- Luau registration prefix used in the c-string, e.g. "ChildProcess"
    naming_prefix: string,  -- prefix used for constant naming, e.g. "process.ChildProcess"
    file_path: string,      -- absolute path to the typedef file
    is_method: boolean,     -- true → colon separator, self skipped
    type_name: string?,     -- specific alias to extract (nil = all non-class aliases + StatFn fallback)
    prop_path: { string }?, -- navigate into nested table props, e.g. {"bold"} for io.colors.bold
}

-- Scans all typedef files under TYPEDEFS_DIR and returns entries describing
-- what to extract. Rules:
--   - A type alias whose non-metamethod functions all lack 'self' → module type
--     (prefix = path-derived, e.g. fs.file, not the alias name FileLib)
--   - A type alias whose non-metamethod functions include 'self' → class type
--     (prefix = alias name, e.g. ChildProcess; only exported aliases)
--   - Files with no usable type aliases but with `function var.fn()` declarations
--     → StatFunction entry (type_name = nil triggers fallback in extract_fn_sigs)
local function discover_entries(): { Entry }
    local entries: { Entry } = {}

    local all_files = fs.listdir(TYPEDEFS_DIR, true, function(p)
        return string.sub(p, -5) == ".luau"
    end)
    table.sort(all_files)

    for _, abs_path in all_files do
        local rel_path = string.sub(abs_path, #TYPEDEFS_DIR + 2)
        local base_prefix = path_to_prefix(rel_path)
        local last_part = string.match(base_prefix, "[^%.]+$") or base_prefix

        local content = fs.file.try_read(abs_path)
        if not content then continue end

        local result = parser.parse(content)
        if typeof(result) == "error" or not result.root then
            continue
        end

        local has_module_entry = false

        for _, stat in result.root.body do
            if stat.kind ~= "StatTypeAlias" then continue end
            local alias_name = (stat :: any).name :: string
            local exported = (stat :: any).exported :: boolean

            local table_type = extract_table_from_type((stat :: any).type)
            if not table_type then continue end

            local has_fns = false
            local is_class = false
            for _, prop in (table_type :: any).props do
                local prop_name = (prop :: any).name.value :: string
                if string.sub(prop_name, 1, 2) == "__" then continue end
                local prop_type = (prop :: any).type :: any
                if prop_type.kind ~= "TypeFunction" then continue end
                has_fns = true
                if fn_has_self(prop_type) then
                    is_class = true
                    break
                end
            end

            if is_class then
                if not has_fns then continue end
                if not exported then continue end
                -- Class prefix in c-strings is just the alias name (e.g. "ChildProcess:alive()").
                -- Constant naming includes the library so "process.ChildProcess" → STD_PROCESS_CHILD_PROCESS_*.
                table.insert(entries, {
                    prefix = alias_name,
                    naming_prefix = base_prefix .. "." .. alias_name,
                    file_path = abs_path,
                    is_method = true,
                    type_name = alias_name,
                })
            else
                -- Module alias with its own callable functions → module entry.
                -- Aliases with no direct functions but nested function tables (e.g. archive/tar's
                -- `tar` alias with `gz`, `xz`, ... sub-tables) get no module entry here but still
                -- have their sub-modules discovered below.
                if has_fns then
                    table.insert(entries, {
                        prefix = base_prefix,
                        naming_prefix = base_prefix,
                        file_path = abs_path,
                        is_method = false,
                        type_name = alias_name,
                    })
                    has_module_entry = true
                end

                -- Also discover nested table sub-modules (e.g. io.colors.bold, terminal.rawmode,
                -- archive.tar.gz). Runs whether or not the alias itself has direct functions.
                for _, prop in (table_type :: any).props do
                    local pname = (prop :: any).name.value :: string
                    if string.sub(pname, 1, 2) == "__" then continue end
                    local ptype = (prop :: any).type :: any
                    if ptype.kind == "TypeFunction" then continue end  -- already handled above
                    local nested_tbl = extract_table_from_type(ptype)
                    if not nested_tbl then continue end
                    local nested_has_fns = false
                    for _, nprop in (nested_tbl :: any).props do
                        if ((nprop :: any).type :: any).kind == "TypeFunction" then
                            nested_has_fns = true
                            break
                        end
                    end
                    if nested_has_fns then
                        local sub_prefix = base_prefix .. "." .. pname
                        table.insert(entries, {
                            prefix = sub_prefix,
                            naming_prefix = sub_prefix,
                            file_path = abs_path,
                            is_method = false,
                            type_name = alias_name,
                            prop_path = { pname },
                        })
                    end
                end
            end
        end

        -- Fallback: no module-type aliases found, check for StatFunction declarations
        if not has_module_entry then
            local has_stat_fns = false
            for _, stat in result.root.body do
                if stat.kind ~= "StatFunction" then continue end
                local name_expr = (stat :: any).name :: any
                if name_expr.kind ~= "ExprIndexName" then continue end
                local base = name_expr.expr :: any
                local var_name: string? = nil
                if base.kind == "ExprGlobal" then
                    var_name = base.name :: string
                elseif base.kind == "ExprLocal" then
                    var_name = (base :: any)["local"].name :: string
                end
                if var_name and var_name:lower() == last_part:lower() then
                    has_stat_fns = true
                    break
                end
            end
            if has_stat_fns then
                table.insert(entries, {
                    prefix = base_prefix,
                    naming_prefix = base_prefix,
                    file_path = abs_path,
                    is_method = false,
                    type_name = nil,
                })
            end
        end
    end

    return entries
end

return {
    REPO_ROOT = REPO_ROOT,
    TYPEDEFS_DIR = TYPEDEFS_DIR,
    discover_entries = discover_entries,
    extract_fn_sigs = extract_fn_sigs,
}
