--!optimize 2

--[[
    As of 2026-08-21 I'm still quite happy with our custom require resolver implementation.
    It still has advantages over the Luau Require library and as is easier to maintain. Also allows
    us to be a bit divergent from official semantics as we choose.
]]

type RequireResolveResult = {
    path: string,
} | {
    err: string,
}

const fs = require("@std/fs")
const str = require("@std/str")
const err = require("@std/err")
const env = require("@std/env")
const luau = require("@std/luau")
const json = require("@std/serde/json")
const colors = require("@std/io/colors")

type Result<T> = err.Result<T>

const function find_module_file(path_without_extension: string): string?
    const luau_path = path_without_extension .. ".luau"
    if fs.is(luau_path) == "File" then
        return luau_path
    end
    const luwu_path = path_without_extension .. ".luwu"
    if fs.is(luwu_path) == "File" then
        return luwu_path
    end
    return nil
end

const function find_init_file(directory: string): string?
    return find_module_file(fs.path.join(directory, "init"))
end

type LuaurcConfig = {
    languageMode: LanguageMode?,
    lint: {
        [string]: boolean,
    }?,
    aliases: {
        [string]: string,
    }?,
}

-- start partial copy paste from https://rfcs.luau.org/config-luauconfig.html
type LanguageMode = "strict" | "nonstrict" | "nocheck"

type LintWarning =
    "*"
    | "UnknownGlobal"
    | "DeprecatedGlobal"
    | "GlobalUsedAsLocal"
    | "LocalShadow"
    | "SameLineStatement"
    | "MultiLineStatement"
    | "LocalUnused"
    | "FunctionUnused"
    | "ImportUnused"
    | "BuiltinGlobalWrite"
    | "PlaceholderRead"
    | "UnreachableCode"
    | "UnknownType"
    | "ForRange"
    | "UnbalancedAssignment"
    | "ImplicitReturn"
    | "DuplicateLocal"
    | "FormatString"
    | "TableLiteral"
    | "UninitializedLocal"
    | "DuplicateFunction"
    | "DeprecatedApi"
    | "TableOperations"
    | "DuplicateCondition"
    | "MisleadingAndOr"
    | "CommentDirective"
    | "IntegerParsing"
    | "ComparisonPrecedence"
    | "RedundantNativeAttribute"

type LanguageConfigSchema = {
    languagemode: LanguageMode?,
    lint: { [LintWarning]: boolean }?,
    linterrors: boolean?,
    typeerrors: boolean?,
    globals: { string }?,
    aliases: { [string]: string }?,
}

type DotConfig = {
    luau: LanguageConfigSchema?
}
-- end partial copy paste from https://rfcs.luau.org/config-luauconfig.html

export type ConfigFileAliases = {
    path: string,
    aliases: {
        [string]: string
    }
}

const function maptostring<K, V>(t: { [K]: V }, f: (key: K, val: V) -> string?): string
    local result = ""
    for k, v in t do
        local s = f(k, v)
        if s then
            result ..= "\n" .. s
        end
    end
    return result
end

--[=[
    Checks whether a config file (.luaurc or .config.luau) exists in the current_path directory,
    returning nil if neither do, a `ConfigFileAliases` table if we could successfully extract
    aliases, or return an error if we had trouble with the config file for whatever reason.
]=]
const function try_config_file_for_current_path(current_path: string): Result<ConfigFileAliases?>
    -- we need to check for both .luaurc and .config.luau in the current dir and reject
    -- if both are found we need to throw error by spec (which I agree with)

    const possible_luaurc_path = fs.path.join(current_path, ".luaurc")
    const possible_dotconfig_path = fs.path.join(current_path, ".config.luau")

    const use_luaurc = fs.is(possible_luaurc_path) == "File"
    const use_dotconfig = fs.is(possible_dotconfig_path) == "File"

    if use_luaurc and use_dotconfig then
        return err.message(`found both flavors of require alias configuration files in the same directory ('{current_path}'); \z
        this is ambiguous: please pick either a .luaurc or .config.luau file`)
    elseif use_luaurc == false and use_dotconfig == false then
        return nil
    end

    -- config file must be a .luaurc or .config.luau here; if we get here only one of use_luaurc or use_dotconfigdotluau must be true
    const config_file_path = if use_luaurc then possible_luaurc_path else possible_dotconfig_path

    const contents, result = fs.file.try_read(config_file_path)
    if not contents then
        if result == "PermissionDenied" then
            return err.message(`unable to read language configuration file at '{config_file_path}' because permission denied 🔐`)
        else
            return err.message(`unable to read language configuration file at '{config_file_path}' due to err: {result}`)
        end
    end

    local aliases: { [string]: string }?
    if use_luaurc then
        local success, config = pcall(json.decode, contents)
        if not success or typeof(config) == "error" then
            return err.message(`unable to decode .luaurc language configuration file at '{config_file_path}' as json/jsonc due to err {config}`)
        end
        config = config :: LuaurcConfig
        aliases = config.aliases
    else
        -- I wanted to allow "Seal" stdlib here so we can start using the language config for 
        -- our own config and allow users to specify seal test/seal setup/etc. functions here
        -- but problem is the LSP itself needs to load the .config.luau file and lsp obviously
        -- doesn't have seal; so we have to default to "Safe" and thereby follow spec
        local success, config = pcall(luau.eval, contents, {
            stdlib = "Safe",
            name = config_file_path
        })
        -- luau.eval can throw a runtime error if the contents are invalid; we want to expose that 
        -- failure mode differently than if luau.eval legitimatiely returned an "error" (runtime error in config file)
        if not success then
            return err.message(`cannot load/evaluate language configuration file at '{config_file_path}' due to err: {result}`)
        end
        if success and typeof(config) == "error" then
            return err.message(`configuration file at '{config_file_path}' threw a runtime error: {config}`)
        end
        config = config :: DotConfig
        if config.luau then
            aliases = config.luau.aliases
        end
    end

    if not aliases then
        -- we still want to return an empty aliases table here so error paths know we found
        -- the config file but couldn't find any aliases within it
        return {
            path = config_file_path,
            aliases = {}
        }
    end

    -- LUAU FIXME: without cast infers a trivial union of { [string]: string } | { [string]: string } | { [string]: string }
    aliases = aliases :: { [string]: string }

    -- mutate aliases in place to replace require aliases with with their substituted paths
    for key, replacement in aliases do
        if key == "self" then
            return err.message(`reserved alias 'self' used in language config file at '{config_file_path}'; \z
            @self refers to the current module as a directory when it contains an init.luau file`)
        end
        const parent_path = fs.path.parent(current_path) or current_path
        const absolutized_replacement =
            if str.startswith(replacement, "./") then
                fs.path.join(current_path, string.sub(replacement, 3))
            elseif str.startswith(replacement, "~/") then
                fs.path.join(fs.path.home(), string.sub(replacement, 3))
            elseif str.startswith(replacement, "../") then
                fs.path.join(parent_path, string.sub(replacement, 4))
            elseif replacement == "." then
                fs.path.join(current_path, string.sub(replacement, 2))
            elseif replacement == ".." then
                fs.path.join(current_path, string.sub(replacement, 3))
            else
                replacement
        aliases[key] = absolutized_replacement
    end

    return {
        path = config_file_path,
        aliases = aliases
    }
end

const function get_aliases_by_config_file(requiring_file_path: string): Result<{ ConfigFileAliases }>
    const aliases_by_config_file: { ConfigFileAliases } = {}

    local current_path = fs.path.parent(
        requiring_file_path, 
        -- init.luaus should not be able to resolve aliases in sibling config files
        if str.ends(requiring_file_path, "init.luau", "init.luwu") then 2 else 1
    )

    while current_path ~= nil do
        const aliases_result = try_config_file_for_current_path(current_path)
        if typeof(aliases_result) == "error" then
            return aliases_result
        end
        if aliases_result ~= nil and type(aliases_by_config_file) == "table" then
            -- we want to 'push_front' our array so the closest aliases to the requiring_file
            -- get assigned last (so overwriting those in farther luaurcs)
            table.insert(aliases_by_config_file, 1, aliases_result)
        end
        current_path = fs.path.parent(current_path)
    end

    if str.ends(requiring_file_path, "init.luau", "init.luwu") then
        -- @self should refer to the direct parent dir of an init.luau
        -- regardless of whichever meaning of ./relative we use in seal 
        const parent = fs.path.parent(requiring_file_path)
        if not parent then
            return err.message("your requiring file path from an init.luau doesn't even have a parent??? why is an init.luau sitting in root? what's going on")
        end
        table.insert(aliases_by_config_file, 1, {
            path = "self alias for init.luau (https://github.com/luau-lang/rfcs/pull/109)",
            aliases = {
                self = parent
            }
        } :: ConfigFileAliases)
    end
    
    return aliases_by_config_file
end

--[=[
    Resolves the requested_path into a path on the filesystem by replacing it with a require alias.

    Returns the actual path on the filesystem if successful, or returns an error describing why we can't.
]=]
const function resolve_alias(requested_path: string, aliases_by_config_files: { ConfigFileAliases }): Result<string>
    const extracted_alias = string.match(requested_path, "^@([%w_%-%.]+)/?")
    if not extracted_alias then
        return err.message(`couldn't extract require alias from requested path '{requested_path}'; is it a valid alias (like "@alias/path" or "@alias")?`)
    end

    local aliases_found, luaurcs_found = 0, 0
    local alias_replacement: string? = nil
    for priority, luaurc in aliases_by_config_files do
        luaurcs_found += 1
        for key, replacement in luaurc.aliases do
            aliases_found += 1
            if key == extracted_alias then
                alias_replacement = replacement
            end
        end
    end
    
    if alias_replacement then
        local rest_of_path = requested_path
        -- strip @alias prefix
        rest_of_path = string.sub(rest_of_path, #extracted_alias + 2, #rest_of_path)
        if str.startswith(rest_of_path, "/") then
            rest_of_path = string.sub(rest_of_path, 2, #rest_of_path)
        end
        
        -- check for file because specs allow directory to have the extension
        if str.ends(alias_replacement, ".luau", ".luwu") and fs.is(alias_replacement) == "File" then
            return err.message(`seal no longer allows require alias replacement suffixes to contain \`.luau\`/\`.luwu\` file extensions (got {alias_replacement}); please remove the file extension to stay in spec :3`)
        end
        
        const alias_with_extension = find_module_file(alias_replacement)
        if alias_with_extension then
            return alias_with_extension
        end
        
        if not str.endswith(alias_replacement, "/") then
            alias_replacement ..= "/"
        end
        return alias_replacement .. rest_of_path
    else
        const function list_aliases(aliases: { [string]: string }, indent: string)
            return maptostring(aliases, function(key, replacement)
                return ` {indent}- '{key}' => {colors.codes.GREEN}"{replacement}"{colors.codes.RESET}`
            end)
        end

        const function display_aliases_by_config_file(aliases_by_config_file: { ConfigFileAliases }): string
            local result = ""
            local current_config = 1
            for i = #aliases_by_config_file, 1, -1 do -- go backwards
                local config: ConfigFileAliases = aliases_by_config_file[i]
                result ..= `\n  {current_config}: {config.path}: {list_aliases(config.aliases, "    ")}`
                current_config += 1
            end
            return result
        end

        const err_message = `unable to resolve requested path '{requested_path}'\n  ` .. colors.codes.RESET ..
            if luaurcs_found == 0 then
                `unable to expand alias '{extracted_alias}', no language configuration files found in requiring file's parent/ancestor directories`
            elseif luaurcs_found == 1 and aliases_found == 0 then
                `no aliases defined in the .luaurc at '{aliases_by_config_files[1].path}' (looking for '{extracted_alias}')`
            elseif luaurcs_found == 1 and aliases_found > 0 then
                `alias '{colors.bold.red(extracted_alias)}' not found in the language configuration file at '{aliases_by_config_files[1].path}'; found aliases: {list_aliases(aliases_by_config_files[1].aliases, "  ")}`
            else
                `alias '{colors.bold.red(extracted_alias)}' not found in the following configuration files:` .. display_aliases_by_config_file(aliases_by_config_files)
        return err.message(err_message)
    end
end

--[=[
    Tries to resolve `requested_path` from the requiring file's debug name (optionally overridden by `override_debug_name`),
    using the new `init.luau` semantics if `try_new_init_luau_semantics` is enabled.

    seal uses the new semantics by default, and falls back to the old semantics if a module wasn't found.
    In either case, the alias @self should always be available and consistently refer to init.luau's direct parent directory

    This implementation doesn't support advanced features like aliases that refer to other aliases; this is because adding support
    for that would lead to extra complexity and require a large rewrite of this file. I might do it after Luwu classes release and
    we have a nice builtin Path data structure in seal.
]=]
const function resolve(
    requested_path: string,
    override_debug_name: string?,
    try_new_init_luau_semantics: boolean? --[[
        seal defaults to new init.luau semantics since that's what the newest luau-lsp currently supports
        - old semantics = ./meow is in the same directory as the requiring init.luau
        - new semantics = ./meow is in the parent directory of the requiring init.luau
        but if the new semantics don't work (file not found) we try the old one by calling resolve again recursively
        note that seal supports @self in either case
    ]]
): RequireResolveResult
    --[=[
        debug names come in the form `[string "./src/somewhere.luau"]` so we we strip them if needed to
        just ./src/somewhere.luau
    ]=]
    const debug_name: string = override_debug_name or debug.info(3, "s") --[[
        this should give us the debug name (set by luau.load().set_name) for the chunk that called require()
    ]]
    const requiring_file_path = 
        if str.startswith(debug_name, "[") then 
            string.sub(debug_name, 10, -3) -- grabs the part between `[string "` and `"]`
        else
            debug_name

    -- luau lsp has updated, so let's use new semantics by default
    try_new_init_luau_semantics = 
        try_new_init_luau_semantics == nil or try_new_init_luau_semantics == true
    const use_new_init_luau_semantics =
        try_new_init_luau_semantics and str.ends(requiring_file_path, "init.luau", "init.luwu")

    local current_path = requiring_file_path
    local trim_after_index = 0
    if string.sub(requested_path, 1, 1) == "@" then
        const aliases_by_config_files = get_aliases_by_config_file(requiring_file_path)
        if typeof(aliases_by_config_files) == "error" then
            return {
                err = tostring(aliases_by_config_files)
            }
        end

        const expanded_path = resolve_alias(requested_path, aliases_by_config_files)
        if typeof(expanded_path) == "error" then
            return {
                err = tostring(expanded_path)
            }
        end
        
        current_path = expanded_path
    elseif string.sub(requested_path, 1, 2) == "./" then
        current_path = fs.path.parent(current_path, if use_new_init_luau_semantics then 2 else 1)
        trim_after_index = 3
    elseif string.sub(requested_path, 1, 3) == "../" then
        current_path = fs.path.parent(current_path, if use_new_init_luau_semantics then 3 else 2)
        trim_after_index = 4
    else
        return {
            err = `requested path '{requested_path}' does not start with './', '../', or '@'; it cannot be required.\n`
                .. '  note: implicitly relative requires (like require("sibling")) are no longer supported by seal'
        }
    end

    if current_path == nil then
        return {
            err = `one or more parents could not be extracted from '{requested_path}'; is the requested path formatted correctly?`
        }
    end

    -- trim ./ and ../ from requested_path
    if trim_after_index > 0 then
        current_path = fs.path.join(current_path, string.sub(requested_path, trim_after_index))
    end

    assert(current_path ~= nil, "current_path should not be nil at this point 1")

    if env.os == "Windows" then
        -- it's possible we could've accidentally mixed \ and /s when combining paths
        -- (especially when using alias replacements, which always use /)
        current_path = fs.path.normalize(current_path)
    end

    assert(current_path ~= nil, "current_path should not be nil at this point 2")

    local trying_to_require_directory = false
    local path_type = fs.is(current_path)
    if path_type == "Directory" then
        const possible_init_path = find_init_file(current_path)
        const possible_concat_path = find_module_file(current_path)

        if possible_init_path and possible_concat_path then
            -- both foo.luau and foo/init.luau exist (which one do we want??)
            return {
                err = `the require path you requested ('{requested_path}') is ambiguous!! and can't be resolved.\nBoth '{possible_init_path}' and '{possible_concat_path}' exist; \z
                rename either the directory '{fs.path.child(current_path)}/' or file '{fs.path.child(possible_concat_path)}' so we can figure out which one you meant`
            }
        elseif possible_init_path then
            -- foo/init.luau exists
            current_path = possible_init_path
        elseif possible_concat_path then
            -- foo/ exists and foo/init.luau doesn't exist but foo.luau exists (we should resolve to foo.luau)
            warn(`require: both folder '{current_path}' and file '{possible_concat_path}' exist; consider renaming one or both. Require resolving to the file.`)
            current_path = possible_concat_path
        else
            trying_to_require_directory = true
        end
    elseif path_type == "NotFound" then
        -- can't find path/to/foo so let's put a file extension on it and try again
        current_path = find_module_file(current_path) or (current_path .. ".luau")
    elseif path_type == "PermissionDenied" then
        return {
            err = `requested path '{requested_path}' resolved to '{current_path}', \z
            which cannot be required because permission denied :(`
        }
    elseif path_type == "File" then
        -- it's probably a file alias @foo that resolved to foo.luau with an extension on it already
    else
        return {
            err = `requested path '{requested_path}' resolved to '{current_path}', which is unexpectedly not a file nor a directory (got: {path_type})`
        }
    end

    assert(current_path ~= nil, "current_path should not be nil at this point 2")

    if fs.path.exists(current_path) then
        return {
            path = current_path
        }
    elseif trying_to_require_directory then
        return {
            err = `'{requested_path}' can't be required: requested path resolved to '{fs.path.parent(current_path)}', \z
            which is a directory missing its init.luau (needed to treat filesystem directories as 'modules' in the Luau Require System)`
        }
    elseif try_new_init_luau_semantics and str.ends(requiring_file_path, "init.luau", "init.luwu") then
        return resolve(requested_path, debug_name, false)
    elseif use_new_init_luau_semantics == false and string.match(requested_path, "%.%.?/") then
        return {
            err = `module '{requested_path}' not found: relative require from the file at '{requiring_file_path}' resolved to '{current_path}', which wasn't found on the filesystem.\n`
                .. "  note on require resolution semantics:\n"
                .. "    - old: './meow' means meow.luau is in the same directory as the requiring init.luau\n"
                .. "    - new: './meow' means meow.luau is in the parent directory of the requiring init.luau\n"
                .. "    + for more information, check out: https://github.com/luau-lang/rfcs/pull/109\n"
                .. "  seal tried both old and new require resolution semantics but was unable to find your module :("
        }
    else
        return {
            err = `module '{requested_path}' not found: requested path resolved to '{current_path}', which wasn't found on the filesystem`
        }
    end
end

return {
    resolve = resolve,
    get_aliases = get_aliases_by_config_file,
    --- TODO: rename this to resolve_alias in seal 0.9.0; changing it rn would be breaking change to public api
    expand_aliases = resolve_alias,
}