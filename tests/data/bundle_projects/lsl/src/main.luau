--!optimize 2

local fs = require("@std/fs")
local args = require("@std/args")
local colors = require("@std/io/colors")
local env = require("@std/env")
local base64 = require("@std/serde/base64")
local json = require("@std/json")
local str = require("@std/str")
local thread = require("@std/thread")
local datetime = require("@std/time/datetime")

local get_size_of_directory = require("./get_size_of_directory")

local PARALLEL = true
local MAX_PARALLEL = thread.parallelism()
local CACHE_PATH = fs.dir.home():join(".cache", "lsl", "sizes.json")
local PRETTY_ICONS = env.vars.flag("LSL_PRETTY_ICONS", true) -- default on; set LSL_PRETTY_ICONS=0 to disable

-- nerd fonts icons as base64 to avoid binary literals in source
-- options to pick from (run lsl to compare, then delete the ones you don't want):
local ICON_DIR = buffer.tostring(base64.decode("74G7"))  -- nf-fa-folder
-- buffer.tostring(base64.decode("74SV"))  -- nf-fa-folder-open
-- buffer.tostring(base64.decode("75OT"))  -- nf-oct-file-directory
-- buffer.tostring(base64.decode("87CJiw=="))  -- nf-md-folder (previous)
local ICON_FILE = buffer.tostring(base64.decode("74Wb"))  -- nf-fa-file
-- buffer.tostring(base64.decode("74O2"))  -- nf-fa-file-text
-- buffer.tostring(base64.decode("87CIlA=="))  -- nf-md-file (previous)

local parsed = args.parse("lsl", "list files/directories with recentness and size, sorted by date modified or size", {
    description = "Recentness is determined by date modified, and directory size is computed recursively in parallel."
}):simple(
    args.named("--match", "optional pattern to filter by"):aliases("-m"),
    args.named("--path", "search for an explicit path instead of cwd"):aliases("-p"),
    args.named("--limit", "max number of results to display after sorting by modified"):aliases("-l"),
    args.named("--timeout", "how long in seconds before we abort trying to recursively get size"):aliases("-t"),
    args.flag("--size", "sort by size instead of modified"):aliases("-s"),
    args.flag("--norec", "omit recursive directory size checks"):aliases("-nr"),
    args.flag("--date", "show date last modified instead of duration since last modified"):aliases("-d")
)

local t0 = os.clock()

-- cache dir sizes keyed by path; invalidated per-entry when directory mtime changes
-- last_modified_time stored as tostring(DateTime) which is stable and comparable across runs
type CacheEntry = {
    size: number,
    last_modified_time: string,
}
local cache: { [string]: CacheEntry } = {}
local cache_dirty = false

if fs.is(CACHE_PATH) == "File" then
    local ok, data = pcall(json.readfile, CACHE_PATH)
    if ok and data then
        cache = data :: { [string]: CacheEntry }
    end
end

local pattern = parsed:get("--match") :: string?
local chosen_path = (parsed:get("--path") :: string?) or fs.path.cwd()

local limit: number?
local limit_str = parsed:get("--limit") :: string?
if limit_str then
    limit = tonumber(limit_str)
end

local sort_by_size = parsed.flags["--size"] or false
local omit_recursive = parsed.flags["--norec"] or false

local timeout: number = 2.5
local timeout_str = parsed:get("--timeout")
if timeout_str then
    timeout = tonumber(timeout_str) or error(`bad timeout given (not a number): {timeout_str}`)
end

local all_entries = fs.entries(chosen_path)

local function get_modified_at(entry: fs.Entry): DateTime
    -- have to do this separately bc stupid cannot call union type
    local metadata = if entry.type == "Directory" then entry:metadata() else entry:metadata()
    local modified = metadata.modified_at
    assert(modified ~= nil, `no modified_at for {entry.path}?`)
    return modified
end

local function stringify_size(bytes: number): string
    if bytes < 1024 then
        return colors.style.dim(string.format("%d B", bytes))
    elseif bytes < 1024 * 1024 then
        return string.format("%.2f KB", bytes / 1024)
    elseif bytes < 1024 * 1024 * 1024 then
        local mb = bytes / (1024^2)
        if mb > 500 then
            return colors.yellow(string.format("%.2f MB", mb))
        else
            return string.format("%.2f MB", mb)
        end
    else
        return colors.bold.yellow(string.format("%.2f GB", bytes / (1024 * 1024 * 1024)))
    end
end

-- transform with metadata
type EntryWithMetadata = {
    type: "File" | "Directory",
    basename: string,
    path: string,
    modified: DateTime,
    ago: string,
    size: number?,
    size_partial: boolean?,
}

local entries: { EntryWithMetadata } = {}
-- overflow queue for when we hit MAX_PARALLEL; drained one-at-a-time as active threads finish
local pending_paths: { string } = {}
local pending_last_modified_times: { [string]: string } = {} -- mtimes for queued paths, looked up when dequeuing
local active_threads: { { path: string, handle: thread.ThreadHandle, last_modified_time: string? } } = {}

local function push_thread(path: string, last_modified_time: string?)
    if #active_threads < MAX_PARALLEL then
        local handle = thread.spawn {
            path = "./sizer.luau",
            data = {
                path = path,
            }
        }
        table.insert(active_threads, {
            path = path,
            handle = handle,
            last_modified_time = last_modified_time,
        })
    else
        table.insert(pending_paths, path)
        if last_modified_time then
            pending_last_modified_times[path] = last_modified_time
        end
    end
end

local now = datetime.now()

for path, entry in all_entries do
    local basename = fs.path.child(path)
    assert(basename ~= nil, `no basename for {path}?`)

    if pattern and not string.match(basename, pattern) then
        continue
    end

    local modified = get_modified_at(entry)
    local size: number? = nil
    local size_partial: boolean? = nil

    if entry.type == "File" then
        size = entry:size()
    else
        if fs.is(path) == "PermissionDenied" then
            continue
        end
        if not omit_recursive then
            if PARALLEL then
                local last_modified_time = tostring(entry:metadata().modified_at)
                local cached = cache[path]
                if cached and cached.last_modified_time == last_modified_time then
                    size = cached.size
                else
                    push_thread(path, last_modified_time)
                end
            else
                local result, result_err = get_size_of_directory(path)
                size = result
                if result_err then
                    size_partial = true
                end
            end
        end
    end
    
    local ago = modified:to(now):duration():display() .. " ago"

    table.insert(entries, {
        type = entry.type,
        basename = basename,
        modified = modified,
        path = path,
        ago = ago,
        size = size,
        size_partial = size_partial,
    })
end

local hit_timeout = false
while #active_threads > 0 or #pending_paths > 0 do
    if os.clock() - t0 > timeout then
        hit_timeout = true
        break
    end

    local sleep = true
    for index, info in active_threads do
        local msg = info.handle:read()
        if msg then
            local data = msg :: { size: number, partial: boolean }
            for _, entry in entries do
                if entry.path == info.path then
                    entry.size = data.size
                    if data.partial then
                        entry.size_partial = true
                    end
                end
            end

            -- only cache complete results; partial means some subdirs were inaccessible so size may grow if fixed
            if not data.partial and info.last_modified_time then
                cache[info.path] = {
                    size = data.size,
                    last_modified_time = info.last_modified_time,
                }
                cache_dirty = true
            end

            info.handle:join()

            table.remove(active_threads, index)

            if #pending_paths > 0 then
                local next_path = table.remove(pending_paths, 1) :: string
                local next_last_modified_time = pending_last_modified_times[next_path]
                pending_last_modified_times[next_path] = nil
                local handle = thread.spawn {
                    path = "./sizer.luau",
                    data = {
                        path = next_path,
                    }
                }
                table.insert(active_threads, {
                    path = next_path,
                    handle = handle,
                    last_modified_time = next_last_modified_time,
                })
            end

            sleep = false -- we dont want to wait before iterating again if we found smth
            break -- prevent from removing > 1 index during iteration
        end
    end
    
    if sleep then
        thread.sleep(5)
    end
end

if cache_dirty then
    fs.dir.ensure(fs.dir.home():join(".cache", "lsl"))
    json.writefile_raw(CACHE_PATH, cache)
end

-- sort by date modified
table.sort(entries, function(a: EntryWithMetadata, b: EntryWithMetadata)
    if sort_by_size and a.size and b.size then
        return a.size > b.size
    else
        return a.modified > b.modified
    end
end)

local max_name_len = 0
local ago_len = 0
for _, entry in entries do
    -- we have to use str.width to handle utf8 names where #bytes ~= visible length
    local utf8_len = str.width(entry.basename)
    if utf8_len > max_name_len then
        max_name_len = utf8_len
    end
    if #entry.ago > ago_len then
        ago_len = #entry.ago
    end
end

local total_size = 0
local lines: { string } = {}

local PIPE = colors.style.dim("│") -- box drawing thin vertical line
local DIR = colors.blue(if PRETTY_ICONS then ICON_DIR else "D")
local FILE = if PRETTY_ICONS then ICON_FILE else "F"

for index, entry in entries do
    local basename = entry.basename
    local needed_spaces = max_name_len - str.width(basename)
    if needed_spaces > 0 then
        basename ..= string.rep(" ", needed_spaces)
    end

    local ago = entry.ago
    local ago_needed_spaces = ago_len - #ago
    if ago_needed_spaces > 0 then
        ago ..= string.rep(" ", ago_needed_spaces)
    end

    local stringified_size = ""
    if entry.size then
        stringified_size =
            if entry.size_partial then
                colors.yellow(`~{stringify_size(entry.size)}`)
            else
                stringify_size(entry.size)
    end
    
    if entry.size then
        total_size += entry.size
    end

    table.insert(lines, `{if entry.type == "Directory" then DIR else FILE} {PIPE} {basename} {PIPE} {
        if parsed.flags["--date"] then
            entry.modified:format("MM/DD/YYYY HH:MM (AM/PM)")
        else 
            ago
    } {PIPE} {stringified_size}`)

    if limit and index >= limit then
        break
    end
end

print(`{chosen_path}: {stringify_size(total_size)} {if omit_recursive or hit_timeout then colors.red("(inaccurate - partially completed)") else ""}`)
print(table.concat(lines, "\n"))

if hit_timeout then
    print(colors.red(`hit timeout of {timeout} seconds, size may be missing or inaccurate for some directories`))
end

local dt = os.clock() - t0
if dt > 0.25 then
    print(colors.style.dim(`\n{if PARALLEL then "parallel" else "serial"}: {string.format("%.4f", os.clock() - t0)} seconds`))
end

