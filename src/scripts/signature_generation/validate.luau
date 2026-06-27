-- Validates that Rust with_function_and_signature calls match the typedef files.
-- Run with: seal src/scripts/signature_generation/validate.luau

local discovery = require("./discovery")
local fs = require("@std/fs")
local colors = require("@std/io/colors")

local SRC_DIR = discovery.REPO_ROOT .. "/src"

-- ── Build typedef index ───────────────────────────────────────────────────────

local typedef_index: { [string]: { [string]: string } } = {}

for _, entry in discovery.discover_entries() do
    local module_var_hint = string.match(entry.prefix, "[^%.]+$") or entry.prefix
    local fns = discovery.extract_fn_sigs(entry.file_path, entry.is_method, entry.type_name, module_var_hint, entry.prop_path)
    -- Merge into index: later entries for the same prefix extend it (multiple aliases per file)
    if not typedef_index[entry.prefix] then
        typedef_index[entry.prefix] = fns
    else
        for k, v in fns do
            typedef_index[entry.prefix][k] = v
        end
    end
end

-- ── Rust signature extraction ─────────────────────────────────────────────────

type RustSig = {
    fn_reg_name: string,
    c_string: string,
}

local function scan_rust_file(filepath: string): { RustSig }
    local content = fs.file.try_read(filepath)
    if not content then return {} end

    local results: { RustSig } = {}
    local pos = 1
    local len = #content

    while pos <= len do
        local call_pos = string.find(content, "with_function_and_signature", pos, true)
        if not call_pos then break end

        local _, name_end, fn_reg_name = string.find(content, '"([^"]+)"', call_pos)
        if not name_end or not fn_reg_name then
            pos = call_pos + 1
            continue
        end

        local next_call = string.find(content, "with_function_and_signature", name_end + 1, true)
        local search_end = if next_call then next_call - 1 else len

        local csig_s, csig_e, c_string = string.find(content, 'c"([^"]*)"', name_end + 1)
        if csig_s and csig_e and c_string and csig_s <= search_end then
            table.insert(results, { fn_reg_name = fn_reg_name, c_string = c_string })
            pos = csig_e + 1
        else
            pos = name_end + 1
        end
    end

    return results
end

-- ── Sig parsing ───────────────────────────────────────────────────────────────

type ParsedSig = {
    prefix: string,
    sep: string,
    fn_name: string,
    type_sig: string,
}

local function parse_c_string(sig: string): ParsedSig?
    if string.find(sig, "is deprecated", 1, true) then return nil end

    local dot_prefix, fn_name = string.match(sig, "^([%w%.]+)%.([%w_]+)%(")
    if dot_prefix and fn_name then
        return { prefix = dot_prefix, sep = ".", fn_name = fn_name, type_sig = string.sub(sig, #dot_prefix + 1 + #fn_name + 1) }
    end

    local colon_prefix, colon_fn = string.match(sig, "^([%w]+):([%w_]+)%(")
    if colon_prefix and colon_fn then
        return { prefix = colon_prefix, sep = ":", fn_name = colon_fn, type_sig = string.sub(sig, #colon_prefix + 1 + #colon_fn + 1) }
    end

    return nil
end

local function normalize(s: string): string
    s = string.gsub(s, "%s+", " ")
    s = string.match(s, "^%s*(.-)%s*$") or s
    -- { [number]: T } and { T } are equivalent; prefer the shorter form
    s = string.gsub(s, "{ %[number%]: (.+) }", "{ %1 }")
    return s
end

-- ── Compare ───────────────────────────────────────────────────────────────────

type Mismatch = { rust_sig: string, expected: string }
type Missing  = { rust_sig: string, prefix: string, fn_name: string }

local mismatches: { Mismatch } = {}
local missing_from_typedef: { Missing } = {}
local unknown_prefixes: { string } = {}
local seen_prefixes: { [string]: boolean } = {}

local rs_files = fs.listdir(SRC_DIR, true, function(path)
    return string.sub(path, -3) == ".rs"
end)

for _, rs_path in rs_files do
    for _, entry in scan_rust_file(rs_path) do
        local parsed = parse_c_string(entry.c_string)
        if not parsed then continue end

        local prefix  = parsed.prefix
        local fn_name = parsed.fn_name

        if string.sub(fn_name, 1, 2) == "__" then continue end  -- internal metamethods

        local typedef_fns = typedef_index[prefix]
        if not typedef_fns then
            if not seen_prefixes[prefix] then
                seen_prefixes[prefix] = true
                table.insert(unknown_prefixes, prefix)
            end
            continue
        end

        local expected_type_sig = typedef_fns[fn_name]
        if not expected_type_sig then
            table.insert(missing_from_typedef, { rust_sig = entry.c_string, prefix = prefix, fn_name = fn_name })
            continue
        end

        if expected_type_sig == "<complex>" then continue end  -- present but unverifiable

        if normalize(parsed.type_sig) ~= normalize(expected_type_sig) then
            table.insert(mismatches, {
                rust_sig = entry.c_string,
                expected = prefix .. parsed.sep .. fn_name .. expected_type_sig,
            })
        end
    end
end

-- ── Report ────────────────────────────────────────────────────────────────────

local errors = 0

if #mismatches > 0 then
    print(colors.bold.red(`\n{#mismatches} signature mismatch(es):`))
    for _, m in mismatches do
        errors += 1
        print(`  {colors.bold.red("✗")} {m.rust_sig}`)
        print(`    {colors.style.dim("expected:")} {m.expected}`)
    end
end

if #missing_from_typedef > 0 then
    print(colors.bold.yellow(`\n{#missing_from_typedef} registered function(s) not found in typedef:`))
    for _, m in missing_from_typedef do
        errors += 1
        print(`  {colors.bold.yellow("?")} {m.rust_sig}`)
        print(`    {colors.style.dim(`(no '{m.fn_name}' in typedef for library '{m.prefix}')`)}`)
    end
end

if #unknown_prefixes > 0 then
    print(colors.style.dim(`\n{#unknown_prefixes} prefix(es) not mapped to a typedef (skipped):`))
    for _, p in unknown_prefixes do
        print(`  {colors.style.dim(`· {p}`)}`)
    end
end

if errors == 0 then
    print(colors.bold.green("\n✓ all signatures match"))
else
    error(`signature validation failed: {errors} error(s)`)
end
