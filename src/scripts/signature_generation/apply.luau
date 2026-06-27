-- Replaces c"..." signature literals in Rust source with signatures::STD_* constants.
-- Run after generate.luau whenever typedefs change.

local fs = require("@std/fs")
local colors = require("@std/io/colors")

local REPO_ROOT = fs.path.project()
assert(REPO_ROOT)
local SRC_DIR = REPO_ROOT .. "/src"
local SIGNATURES_RS = REPO_ROOT .. "/src/signatures.rs"

-- Build the set of valid constant names from signatures.rs
local known: { [string]: boolean } = {}
local sigs_content = fs.readfile(SIGNATURES_RS)
for const_name in string.gmatch(sigs_content, "pub const (%S+):") do
    known[const_name] = true
end

-- ── Constant name derivation (mirrors generate.luau) ─────────────────────────

local function to_screaming(s: string): string
    s = string.gsub(s, "(%l)(%u)", "%1_%2")
    return string.upper(s)
end

local function make_const_name(prefix: string, fn_name: string): string
    local parts = {}
    for part in string.gmatch(prefix, "[^%.]+") do
        table.insert(parts, to_screaming(part))
    end
    table.insert(parts, to_screaming(fn_name))
    return "STD_" .. table.concat(parts, "_")
end

-- Returns { prefix, fn_name } if sig looks like a function signature, nil otherwise
local function parse_c_sig(sig: string): { prefix: string, fn_name: string }?
    if string.find(sig, "is deprecated", 1, true) then return nil end
    -- module.fn_name(
    local dot_prefix, fn_name = string.match(sig, "^([%w%.]+)%.([%w_]+)%(")
    if dot_prefix and fn_name then return { prefix = dot_prefix, fn_name = fn_name } end
    -- Type:fn_name(
    local colon_prefix, colon_fn = string.match(sig, "^([%w]+):([%w_]+)%(")
    if colon_prefix and colon_fn then return { prefix = colon_prefix, fn_name = colon_fn } end
    return nil
end

-- ── Scanner: skips \" escape sequences to find the real closing " ─────────────

-- Returns the index of the closing '"' for a c"..." that opens at `open_quote_pos`.
-- Advances past any \" pairs so they aren't treated as end-of-string.
local function find_cstr_close(content: string, open_quote_pos: number): number?
    local i = open_quote_pos + 1  -- first char after opening "
    local len = #content
    while i <= len do
        local bs = string.find(content, "\\", i, true)
        local q  = string.find(content, '"',  i, true)
        if not q then return nil end        -- unclosed — shouldn't happen in valid Rust
        if not bs or q < bs then return q end  -- closing quote comes first
        i = bs + 2                          -- skip the backslash and the escaped char
    end
    return nil
end

local function apply_replacements(content: string): (string, number)
    local out = {}
    local pos = 1
    local len = #content
    local replacements = 0

    while pos <= len do
        -- Find next c"
        local cstr_pos = string.find(content, 'c"', pos, true)
        if not cstr_pos then
            table.insert(out, string.sub(content, pos))
            break
        end

        table.insert(out, string.sub(content, pos, cstr_pos - 1))

        local close = find_cstr_close(content, cstr_pos + 1)  -- +1 = index of opening "
        if not close then
            table.insert(out, string.sub(content, cstr_pos))
            break
        end

        -- inner is the raw bytes between the quotes (may contain \" sequences)
        local inner = string.sub(content, cstr_pos + 2, close - 1)

        local parsed = parse_c_sig(inner)
        if parsed then
            local const_name = make_const_name(parsed.prefix, parsed.fn_name)
            if known[const_name] then
                table.insert(out, "signatures::" .. const_name)
                replacements += 1
                pos = close + 1
                continue
            end
        end

        table.insert(out, string.sub(content, cstr_pos, close))
        pos = close + 1
    end

    return table.concat(out), replacements
end

-- ── Apply ─────────────────────────────────────────────────────────────────────

local rs_files = fs.listdir(SRC_DIR, true, function(p)
    return string.sub(p, -3) == ".rs"
end)

local total_files = 0
local total_replacements = 0

for _, rs_path in rs_files do
    if rs_path == SIGNATURES_RS then continue end

    local content = fs.readfile(rs_path)
    local new_content, file_reps = apply_replacements(content)

    if file_reps > 0 then
        fs.writefile(rs_path, new_content)
        total_files += 1
        total_replacements += file_reps
        local rel = string.sub(rs_path, #REPO_ROOT + 2)
        print(`  {colors.green(tostring(file_reps))}  {colors.style.dim(rel)}`)
    end
end

print(colors.bold.green("✓") .. ` replaced {total_replacements} c"..." literals across {total_files} files`)
