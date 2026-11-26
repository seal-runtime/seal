-- Credit: Claude Opus 5.5; code not written by me

-- Checks that seal's plugin API table and sealbindings' copy of it haven't drifted apart. A plugin calls functions out
-- of the table by position, so a field added, removed, moved or retyped on one side only makes it call the wrong
-- function. Compares `LuauApi`'s fields in order (names and types, ignoring parameter names and whitespace) and the
-- SEAL_API_MAGIC and SEAL_API_VERSION constants. Exits 1 on any difference.
--
--   seal src/scripts/check_plugin_api.luau path/to/sealbindings

const env = require("@std/env")
const fs = require("@std/fs")
const process = require("@std/process")

const SEAL_TABLE = "src/interop/externs/ffi_api.rs"
const SEALBINDINGS_TABLE = "src/ffi/api.rs"

type Field = {
    name: string,
    type: string,
}

--- The text between `pub struct LuauApi {` and the `}` that closes it at the start of a line
const function struct_body(source: string, path: string): string
    const _, open = string.find(source, "pub struct LuauApi {", 1, true)
    if not open then
        error(`{path} has no 'pub struct LuauApi'`)
    end
    const close = string.find(source, "\n}", open, true) or error(`{path}: LuauApi never closes`)
    return string.sub(source, open + 1, close)
end

--- A type with its whitespace and parameter names removed, so `fn (state: *mut lua_State, n: c_int)` and
--- `fn(state: *mut lua_State, idx: c_int)` compare equal
const function normalize_type(text: string): string
    local result = string.gsub(text, "%s+", " ")
    result = string.gsub(result, "([%(,]) ?[%w_]+ ?: ?", "%1")
    result = string.gsub(result, " ?([%(%)%[%],<>]) ?", "%1")
    result = string.gsub(result, ",%)", ")")
    result = string.gsub(result, "fn%(", "fn(")
    result = string.gsub(result, "^ ", "")
    result = string.gsub(result, "[ ,]+$", "")
    return result
end

--- `LuauApi`'s fields in declaration order
const function fields_of(path: string): { Field }
    const body = string.gsub(struct_body(fs.readfile(path), path), "//[^\n]*", "")
    const fields: { Field } = {}
    -- every field starts with `pub name:` and its type runs to the next one: mark each start, then split on the marks
    const marked = string.gsub(body, "%f[%w]pub ([%w_]+)%s*:", "\0%1\1")
    for _, piece in string.split(marked, "\0") do
        const name, type_text = string.match(piece, "^([%w_]+)\1(.*)$")
        if name and type_text then
            table.insert(fields, {
                name = name,
                type = normalize_type(type_text),
            })
        end
    end
    return fields
end

--- The value of `pub const name: T = value;` without whitespace, or nil when the file doesn't declare it
const function constant(path: string, name: string): string?
    const value = string.match(fs.readfile(path), `pub const {name}%s*:[^=]*=%s*([^;]+);`)
    return if value then (string.gsub(value, "%s+", "")) else nil
end

const sealbindings = env.args[1] or "../sealbindings"
const seal_path = fs.path.join(fs.dir.project().path, SEAL_TABLE)
const bindings_path = fs.path.join(sealbindings, SEALBINDINGS_TABLE)

const problems: { string } = {}

for _, name in { "SEAL_API_MAGIC", "SEAL_API_VERSION" } do
    const ours, theirs = constant(seal_path, name), constant(bindings_path, name)
    if ours ~= theirs then
        table.insert(problems, `{name} is {ours or "missing"} in seal but {theirs or "missing"} in sealbindings`)
    end
end

const seal_fields = fields_of(seal_path)
const binding_fields = fields_of(bindings_path)
if #seal_fields ~= #binding_fields then
    table.insert(problems, `seal's LuauApi has {#seal_fields} fields but sealbindings' has {#binding_fields}`)
end

for index = 1, math.max(#seal_fields, #binding_fields) do
    const ours = seal_fields[index]
    const theirs = binding_fields[index]
    if not ours or not theirs then
        const extra = ours or theirs
        table.insert(problems, `field {index} '{extra.name}' is only in {if ours then "seal" else "sealbindings"}`)
    elseif ours.name ~= theirs.name then
        table.insert(problems, `field {index} is '{ours.name}' in seal but '{theirs.name}' in sealbindings`)
    elseif ours.type ~= theirs.type then
        table.insert(problems, `field {index} '{ours.name}' has type\n    {ours.type}\n  in seal but\n    {theirs.type}\n  in sealbindings`)
    end
end

if #problems > 0 then
    print(`seal's plugin API and sealbindings ({sealbindings}) have drifted:`)
    for _, problem in problems do
        print(`- {problem}`)
    end
    process.exit(1)
end

print(`seal's plugin API matches sealbindings: {#seal_fields} fields, version {constant(seal_path, "SEAL_API_VERSION") or "?"}`)
