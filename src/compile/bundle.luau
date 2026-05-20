--!optimize 2

-- yes, this is in luau.
local fs = require("@std/fs")
local str = require("@std/str")
local err = require("@std/err")
local env = require("@std/env")
local luau = require("@std/luau")

local reserved_aliases = (require :: any)("@internal/reserved_aliases") :: { string }

local function get_entry_path(path: string): err.Result<string>
    if fs.file(path) then
        return path -- seal compile ./direct/path/to/file.luau -o myexec
    end

    local project_dir = fs.dir(path)
    if not project_dir then
        return err.message(`path at '{path} doesn't exist (so it can't be bundled)`)
    end

    local seal_dir = project_dir:find(".seal"):try_dir()
    if not seal_dir then
        return err.message(`directory at '{path}' isn't a seal project (it doesn't contain a .seal dir), so it cannot be automatically bundled; consider specifying an entry path (seal compile ./path -o exe)`)
    end

    local config_luau = seal_dir:find("config.luau"):try_file()
    if not config_luau then
        return err.message(`seal project at '{path}' missing its .seal/config.luau, it cannot be bundled because we don't know its entry path`)
    end

    local config = luau.eval(config_luau:read())
    if typeof(config) == "error" then
        return err.message(`config.luau at '{config_luau.path}' couldn't be evaluated due to err: {tostring(config)}`)
    elseif typeof(config) ~= "table" then
        return err.message(`config.luau at '{config_luau.path}' returned unexpected datatype; expected table, got: {typeof(config)}`)
    else
        config = config :: {
            entry_path: string?
        }
    end

    local found_entry_path = config.entry_path
    if not found_entry_path then
        return err.message(`config.luau at '{config_luau.path}' missing 'entry_path' field; we need a valid entry_path to bundle the project`)
    end

    found_entry_path = fs.path.normalize(project_dir:join(found_entry_path))

    local find_entry = fs.find(found_entry_path)
    if find_entry.type == "Directory" then
        local find_init_luau = find_entry:unwrap_dir():find("init.luau")
        find_entry = find_init_luau    
    end

    if find_entry.type == "File" then
        return find_entry.path
    elseif find_entry.type == "NotFound" then
        return err.message(`entry path at '{find_entry.path}' doesn't exist on the filesystem`)
    else
        return err.message(`entry_path at '{find_entry.path}' unexpectedly is a {find_entry.type}`)
    end
end

local resolver = luau.require_resolver()

-- Strip single/double quoted and interpolated string literals that contain a require() call
-- so that extract_identifier_and_path doesn't match require() inside string values.
-- Uses require%s*%( (requires an opening paren) to avoid stripping paths that merely
-- contain the word "require" (e.g. "./require-helper.luau" paths stay intact).
local function strip_strings_containing_require(line: string): string
    line = string.gsub(line, '"[^"]*require%s*%([^"]*"', '""')
    line = string.gsub(line, "'[^']*require%s*%([^']*'", "''")
    line = string.gsub(line, "`[^`]*require%s*%([^`]*`", "``")
    return line
end

--- Extract a call to `require` from the line, returning the identifier bound to and the requested path,
--- correctly ensuring that it doesn't catch a require call within a single line string
local function extract_identifier_and_path(line: string, debug_path: string): (err.Result<string?>, string?)
    -- strip quoted string literals containing require() so we don't match require() inside string values
    local stripped = strip_strings_containing_require(line)

    -- identifier = require("...") or require '...' or require "..."
    local identifier, requested_path = string.match(stripped, '([%w%.]+)%s*=%s*require%s*[%(%s]*["\'](.-)["\'][%)]?')
    if identifier and requested_path then
        return identifier, requested_path
    end

    -- require("...") or require '...' or require "@alias/path"
    requested_path = string.match(stripped, 'require%s*[%(%s]*["\'](.-)["\'][%)]?')
    if requested_path then
        return nil, requested_path
    end

    -- bad dynamic require: require(var)
    local dynamic_arg = string.match(stripped, 'require%s*%(%s*([%w_%.]+)%s*%)')
    if dynamic_arg then
        return err.message(`detected dynamic require at {debug_path}; only static requires may be bundled. offending line: {line}`), nil
    end

    return nil, nil
end

--- this gets inlined by the luau compiler
local function find_plain(s: string, n: string): boolean
    return if string.find(s, n, 1, true) then true else false
end

--[=[
    Check if the line is a comment.

    - If the line starts a new long comment, returns the new long comment prefix `(true, string)`.
    - If the line is within a long comment, returns the passed comment prefix `(true, string)`.
    - If the line is a single line comment, returns `(true, nil)`

    Usage

    ```luau
    local comment_prefix: string? = nil
    for l, line in readlines(f) do
        local is_comment, prefix = comment(line)
        comment_prefix = prefix
        if is_comment then
            continue
        end
    end
    ```
]=]
local function comment(line: string, current_comment_prefix: string?, context: TraverseContext): (
    --- whether or not the current line is a comment
    boolean,
    --- the new or existing prefix if this line starts or is within a long comment; 
    --- nil if it ends a long comment, is a single line comment, isn't a comment, etc.
    string?
)
    if current_comment_prefix then
        local equal_signs = string.match(current_comment_prefix, "%-%-%[(=*)%[") :: string
        local closing = "]" .. equal_signs .. "]"
        if find_plain(line, closing) then
            -- ends long comment
            return true, nil
        else
            -- line within long comment
            return true, current_comment_prefix
        end
    end

    local new_comment_prefix = string.match(line, "(%-%-%[=*%[)")
    if new_comment_prefix then
        -- check if comment closed on same line as it opened
        local equal_signs = string.match(new_comment_prefix, "%-%-%[(=*)%[") :: string
        local closing = "]" .. equal_signs .. "]"

        if find_plain(line, closing) then
            -- starts and ends on the same line
            return true, nil
        else
            -- starts long comment
            return true, new_comment_prefix
        end
    end

    local trimmed_line = str.trimfront(line)

    if str.startswith(trimmed_line, "--!") and not context.hot_comments[trimmed_line] then
        context.hot_comments[trimmed_line] = true
    end

    if str.startswith(trimmed_line, "--") then
        -- single line comment
        return true, nil
    end

    return false, nil
end

--[=[
    Check if the line is inside or opens a multi-line long string ([[ or [=[ syntax).

    - If the line is within a long string, returns `(true, current_level)`.
    - If the line closes a long string, returns `(true, nil)`.
    - If the line opens a new long string that doesn't close on the same line, returns `(true, new_level)`.
    - Otherwise returns `(false, nil)`.

    Usage

    ```luau
    local long_string_level: string? = nil
    for l, line in readlines(f) do
        local is_long_string, level = long_string(line, long_string_level)
        long_string_level = level
        if is_long_string then
            append(line)
            continue
        end
    end
    ```
]=]
local function long_string(line: string, current_level: string?): (
    --- whether this line should be appended as-is without processing (inside or opening a long string)
    boolean,
    --- the active level after processing this line; nil if no long string is open
    string?
)
    if current_level ~= nil then
        -- we're inside a long string: check if it closes on this line
        local closing = "]" .. current_level .. "]"
        if find_plain(line, closing) then
            -- ends long string
            return true, nil
        else
            -- line within long string
            return true, current_level
        end
    end

    -- check if this line opens a new long string ([[ or [=[ or [===[ etc.)
    local open_start, _, open_eqs = string.find(line, "%[(=*)%[")
    if not open_start or open_eqs == nil then
        return false, nil
    end

    -- ignore [=*[ that appears after -- on the same line (trailing comment, not a string)
    local before_open = string.sub(line, 1, open_start - 1)
    if find_plain(before_open, "--") then
        return false, nil
    end

    -- check whether the matching closing bracket also appears on this line
    local closing = "]" .. open_eqs .. "]"
    local after_open = open_start + #open_eqs + 2 -- skip past the opening [=*[ delimiter
    if find_plain(string.sub(line, after_open), closing) then
        -- starts and ends on the same line
        return false, nil
    else
        -- opens a long string that continues past this line; return true so the opener
        -- is appended without processing (same as lines inside the string)
        return true, open_eqs
    end
end

--- Handle to the traverse function for recursion
type TraverseFn = (path: string, depth: number?, context: TraverseContext?) -> err.Result<{string}>

--- We transform every usage of thread.spawn with a path into inlining the path's src
local function transform_thread_spawn_path(
    line: string,
    line_number: number,
    path: string,
    depth: number,
    --- recursive handle to function calling this function cause uh yeah
    traverse: TraverseFn
): err.Result<{string}>
    local relative_path = string.match(line, [=[path%s*=%s*["']([^"']+)["']]=])
    if not relative_path then
        return err.message(`{path}:{line_number}: unable to extract relative path in thread.spawn to transform from path to src...`)
    end

    local resolved = resolver.resolve(relative_path, path)
    if not resolved.path then
        return err.message(`{path}:{line_number}: unable to resolve relative path in thread.spawn to transform from path to src...`)
    end

    -- depth is intentionally 0 so that traverse inlines and hoists dependencies in the inlined src
    -- we do not pass in context to traverse here so that traverse creates a new context for the inlined src
    local resolved_lines = traverse(resolved.path, 0)
    if typeof(resolved_lines) == "error" then
        return resolved_lines
    end

    local line_indent = 
        if str.starts(line, " ", "\t") then
            string.match(line, "^(%s*)") :: string
        else 
            ""

    -- dynamic # of equal signs in case a file being thread.spawned also spawns another thread
    -- if it does so recursively all bets are off however
    local n_equal_signs = math.min(depth + 1, 12)
    local src_line = line_indent .. "src = " .. "[===" .. string.rep("=", n_equal_signs) .. "["

    local result: { string } = { src_line }
    for _, l in resolved_lines do
        table.insert(result, l)
    end

    table.insert(result, "]" .. string.rep("=", n_equal_signs) .. "===],")

    return result
end

--- Traverse into the dependency at `path` to convert it into a list of dependency lines.
--- Goes through the lines in the dependency's src, finds the closest one to the back to transform
--- from a `return` statement into a binding to the local `identifier` for that dependency
--- The `identifier` should be hoisted by the caller.
local function traverse_into_dependency(
    path: string,
    depth: number,
    context: TraverseContext,
    --- Recursive handle to `traverse` function
    traverse: TraverseFn
): err.Result<{string}>
    -- we want to preserve context when transforming dependency lines;
    local result = traverse(path, depth, context)

    local lines: { string }
    if typeof(result) == "error" then
        return result
    else
        lines = result
    end

    local identifier = context.seen[path]

    -- replace the most suitable `return` closest to the end of the required file w/ a local binding
    -- and then put those lines from the dependency in front of this line of code (and transform this loc)
    local return_replaced = false
    for i = #lines, 1, -1 do -- go backwards
        local maybe_return_line = lines[i]
        -- we want to pick the return with NO whitespace to the left, otherwise we may grab the wrong return
        if string.match(maybe_return_line, "^return%s+") then
            -- modify dependency_lines in place to fix last return
            -- this identifier should already be initialized from outside the do/end block,
            -- so this is just assigning to an existing identifier
            lines[i] = string.gsub(maybe_return_line, "^return%s+", `{identifier} = `)
            return_replaced = true
            break
        end
    end

    if not return_replaced then
        return err.message(`Can't find a top-level return statement in {path}, \z
        please make sure the module returns something and the top-level return statement is completely unindented; aligned all the way to the left`)
    end

    return lines
end

local function get_reserved_identifier(path: string): string
    -- @std/fs -> seal_reserved_library_std_fs
    -- @std/io/input -> seal_reserved_library_std_io_input
    -- @interop/mlua -> seal_reserved_library_interop_mlua
    local stripped = string.sub(path, 2) -- strip leading "@"
    return "seal_reserved_library_" .. string.gsub(stripped, "/", "_")
end

local function identifier_from_path(resolved_path: string): string
    -- derive a unique identifier from the resolved absolute path rather than the captured
    -- variable name; variable names aren't unique across files but resolved paths are,
    -- so this prevents two files that both do `local utils = require(...)` to different
    -- targets from producing the same resolved_require_utils and clobbering each other
    local without_ext = string.match(resolved_path, "(.-)%.[^%.]+$") or resolved_path
    local normalized = string.gsub(without_ext, "[^%w]", "_")
    -- strip leading underscores left over from the leading / on absolute paths
    normalized = string.match(normalized, "^_*(.+)") or normalized
    return "resolved_require_" .. normalized
end

--- Replace a require call in a `line` containing a require to `requested_path` with `identifier`,
--- returning the line with the require call replaced.
--- Handles both `require "some/path"` and `require("some/path")` syntax with ' and " style quotes
local function replace_identifier(line: string, requested_path: string, identifier: string)
    local replace_parens_pattern = 'require%s*%(%s*["\']' .. requested_path .. '["\']%s*%)'
    local replace_string_call_pattern = 'require%s*["\']' .. requested_path .. '["\']'
    line = string.gsub(line, replace_parens_pattern, identifier)
    line = string.gsub(line, replace_string_call_pattern, identifier)
    return line
end

--- Creates a list of lines to `hoist` to the top of a context's bundled lines.
--- Should be called at the end of `traverse` only when depth == 0
local function hoist(context: TraverseContext): { string }
    local top: { string } = {}

    local function push(s: string | { string })
        if typeof(s) == "string" then
            table.insert(top, s)
        else
            for _, line in s do
                table.insert(top, line)
            end
        end
    end

    -- hot comments need to be on top of the output file so we handle them first
    for hot_comment, _ in context.hot_comments do
        push(hot_comment)
    end

    push("")

    -- we should be able to identify seal-bundled applications
    push(`-- bundled by seal {_SEAL_VERSION}; the bundler isn't perfect, report bundling issues to at https://github.com/seal-runtime/seal/issues`)

    push("")

    -- we sort all reserved/seen identifiers in alphabetical order so their order
    -- remains deterministic

    -- sort and hoist all reserved libraries
    local reserved_library_requires: { string } = {}
    for library_identifier, library_path in context.reserved do
        table.insert(reserved_library_requires, `local {library_identifier} = require("{library_path}")`)
    end
    table.sort(reserved_library_requires)
    push(reserved_library_requires)

    push("")

    -- now, sort and hoist all seen dependencies; we only hoist the bindings themselves
    -- not their values, because their values are assigned inside the do/end blocks already handled in traverse
    -- before this function gets called
    local seen_dependency_identifiers: { string } = {}
    for _, dependency_identifier in context.seen do
        table.insert(seen_dependency_identifiers, `local {dependency_identifier}`)
    end
    table.sort(seen_dependency_identifiers)
    push(seen_dependency_identifiers)

    push("")

    return top
end

type TraverseContext = {
    --- Luau comment directives that must appear at the top of the output file
    hot_comments: {
        [string]: boolean?
    },
    --- Map of canonical paths on the filesystem to canonical identifiers
    --- for that dependency in this context
    seen: {
        [string]: string?,
    },
    --- Map of reserved seal identifiers to require paths
    reserved: {
        [string]: string?,
    },
    --- Is this the top level context (not thread.spawn inlined src)?
    top: boolean,
    --- On Unixlike: a #! directive that tells the shell how to execute the script
    shebang: string?,
}
local function new_context(top: boolean?): TraverseContext
    return {
        hot_comments = {
            ["--!optimize 2"] = true,
            ["--!nolint LocalShadow"] = true,
        },
        seen = {},
        reserved = {},
        top = top == true
    }
end

local function traverse(
    path: string,
    depth: number?,
    context: TraverseContext?
): err.Result<{string}>
    context = context or new_context()
    depth = depth or 0

    if depth > 333 then
        return err.message(`ahhh!!! detected circular require of '{path}' !! not allowed!`)
    end

    if not fs.path.exists(path) then
        return err.message(`cannot transform path '{path}' because it doesn't exist on the filesystem`)
    end

    local transformed: { string } = {}

    local is_comment = false
    local comment_prefix: string? = nil

    local is_long_string = false
    local long_string_level: string? = nil
    
    local previous_line = ""

    local function append(line: string)
        previous_line = line
        table.insert(transformed, line)
    end

    local function on_line_or_previous(line: string, first: string, second: string): boolean
        return (
            find_plain(previous_line, first) and find_plain(line, second)
        ) or (
            find_plain(line, first) and find_plain(line, second)
        )
    end

    local file = fs.file.from(path) 
    if not file:is_valid_utf8() then
        return err.message(`file at '{path}' is not valid utf-8 ☠️`)
    end
    
    for line_number, line in file:readlines() do
        is_comment, comment_prefix = comment(line, comment_prefix, context)
        if is_comment then
            continue
        end

        -- If we're inside a multi-line long string ([[ or [=[ syntax), we may be in a string
        -- that represents Luau source code; we shouldn't process require() calls
        -- on these lines — they belong to the string content, not the surrounding scope.
        is_long_string, long_string_level = long_string(line, long_string_level)
        if is_long_string then
            append(line)
            continue
        end

        -- it's a bash shebang used for running luau scripts as executables directly,
        -- hoist and move it to the top but don't include it here (or it syntax errors)
        -- if a non-main file contains a bash shebang we also strip it to prevent syntax errors
        if line_number == 1 and str.startswith(line, "#!") then
            if depth == 0 and context.top then
                context.shebang = line
            end 
            continue
        end

        -- note that extract_identifier_and_path correctly handles possible requires within
        -- single line strings (ignoring them)
        local identifier, requested_require = extract_identifier_and_path(line, `{path}:{line_number}`)
        if typeof(identifier) == "error" then
            return identifier
        end

        -- line doesn't contain any imports, so we don't need to handle requires
        -- but we need to check for cases we need to transform 
        -- and cases where we should spit a warning
        if requested_require == nil then
            if on_line_or_previous(line, "thread.spawn", "path =") then
                -- we need to transform the path = into src = ... and transform the file
                local inlined_src_lines = transform_thread_spawn_path(line, line_number, path, depth, traverse)
                if typeof(inlined_src_lines) == "error" then
                    return inlined_src_lines
                end
                for _, l in inlined_src_lines do
                    append(l)
                end
            elseif on_line_or_previous(line, "thread.spawn", "data =") then
                warn(`{path}:{line_number}: thread.spawn will not be inlined here; put path field above data field to inline.`)
                append(line)
            elseif
                (
                    find_plain(line, "script:parent") or 
                    find_plain(line, "script:path")
                ) and
                depth > 0
            then
                warn(`{path}:{line_number}: behavior of script:parent()/script:path() may change when compiled`)
                append(line)
            else
                append(line)
            end
            -- it's a normal line of code w/out any imports, add it and keep going
            continue
        end

        if str.starts(requested_require, table.unpack(reserved_aliases)) then
            -- replace the require call with the deterministic stdlib identifier
            local library_identifier = get_reserved_identifier(requested_require)
            -- note that we've seen this reserved library for later hoisting
            context.reserved[library_identifier] = requested_require
            -- replace the require call with the library identifier
            line = replace_identifier(line, requested_require, library_identifier)
            append(line)
            
            continue
        end

        -- find out where this require resolves to
        local resolved = resolver.resolve(requested_require, path)
        if resolved.err then
            return err.message(`unable to resolve require '{requested_require}' at {path}:{line_number}: {resolved.err}`)
        end

        -- if we've already seen this require (and therefore assigned an identifier to it)
        -- then just replace the require with the identifier name, add it to transformed,
        -- and continue to the next line

        local seen_identifier = context.seen[resolved.path]
        if seen_identifier then
            -- replace the require call with the identifier name
            line = replace_identifier(line, requested_require, seen_identifier)
            append(line)

            continue
        end

        -- if we get here, we haven't seen this require path yet
        -- we need to inline the require in front of this line and transform it

        -- mark this dependency as seen and give it an identifier so we can later hoist it
        identifier = identifier_from_path(resolved.path)
        context.seen[resolved.path] = identifier

        -- now we need to recurse to get the lines of the required file,
        -- replace the most suitable `return` closest to the end of the required file w/ a local binding
        -- and then put those lines from the dependency in front of this line of code (and transform this loc)
        local dependency_lines = traverse_into_dependency(resolved.path, depth + 1, context, traverse) -- context should be preserved when inlining a dependency
        if typeof(dependency_lines) == "error" then
            return dependency_lines
        end

        -- and simply add all dependency lines to transformed
        table.insert(transformed, `-- START {resolved.path}`)

        -- wrap entire thing in a do/end block so we don't hit the 200 locals limit
        table.insert(transformed, "do")
        for _, dependency_line in dependency_lines do
            append("    " .. dependency_line)
        end
        table.insert(transformed, "end")

        table.insert(transformed, `-- END {resolved.path}`)

        -- replace the require call with the identifier name
        line = replace_identifier(line, requested_require, identifier)
        -- add finally, put the original (transformed) line behind its dependency lines!
        append(line)
    end

    -- if depth == 0, we are in the top-level of a context; if so, we need to hoist
    -- all the standard libraries and seen requires in this context to the top

    if depth == 0 then
        local top = hoist(context)
        for i = #top, 1, -1 do
            local line = top[i]
            table.insert(transformed, 1, line)
        end
        if context.top and context.shebang then
            table.insert(transformed, 1, context.shebang)
        end
    end

    return transformed
end

local function bundle(project_path: string): err.Result<string>
    local entry_path = get_entry_path(project_path)
    if typeof(entry_path) == "error" then
        return entry_path
    end

    local bundled_lines = traverse(entry_path, 0, new_context(true))
    if typeof(bundled_lines) == "error" then
        return bundled_lines
    end

    local bundled = table.concat(bundled_lines, if env.os == "Windows" then "\r\n" else "\n")

    return bundled
end

return bundle
