local luau = require("@std/luau")
local fs = require("@std/fs")

local function count_occurrences(haystack: string, needle: string): number
    local count = 0
    local pos = 1
    while true do
        local found = string.find(haystack, needle, pos, true)
        if not found then break end
        count += 1
        pos = found + #needle
    end
    return count
end

local function basic_bundling()
    local bundled = luau.bundle("./tests/data/bundle_examples/basic/main.luau")
    assert(typeof(bundled) == "string", `bundle should return a string, got: {typeof(bundled)}`)
    local result = luau.eval(bundled)
    assert(result == "hello from dep", `bundled code should return 'hello from dep', got: {tostring(result)}`)
end
basic_bundling()

local function require_inside_long_string_is_not_replaced()
    -- if the fix is broken, bundle errors trying to resolve ./some_module from inside the string
    local bundled = luau.bundle("./tests/data/bundle_examples/long_string/main.luau")
    assert(typeof(bundled) == "string", `bundle should succeed, got error: {tostring(bundled)}`)
    -- the require inside the long string should be preserved literally in the bundled output
    assert(
        string.find(bundled, 'require("./some_module")', 1, true) ~= nil,
        "require inside long string should be preserved literally in bundled output"
    )
end
require_inside_long_string_is_not_replaced()

local function require_inside_quoted_string_is_not_resolved()
    -- if the fix is broken, the bundler tries to resolve './database' and '@std/fs'
    -- as real requires from inside the string values, which errors or incorrectly inlines them
    local bundled = luau.bundle("./tests/data/bundle_examples/quoted_string/main.luau")
    assert(typeof(bundled) == "string", `bundle should succeed, got error: {tostring(bundled)}`)
    -- the require calls should be preserved literally as string content
    assert(
        string.find(bundled :: string, "require('./database')", 1, true) ~= nil,
        "require inside double-quoted string should be preserved literally"
    )
    assert(
        string.find(bundled :: string, 'require("@std/fs")', 1, true) ~= nil,
        "require inside single-quoted string should be preserved literally"
    )
end
require_inside_quoted_string_is_not_resolved()

local function bundling_nonexistent_path_returns_error()
    local result = luau.bundle("./nonexistent/path/main.luau")
    assert(typeof(result) == "error", `bundling a nonexistent path should return an error, got: {typeof(result)}`)
end
bundling_nonexistent_path_returns_error()

-- lsl (https://github.com/deviaze/lsl) is a real-world seal project that uses thread.spawn
-- with path= to spin up worker threads. the bundler inlines those workers as long strings,
-- so it's a good end-to-end test for both correct bundling and the long-string require fix.
local LSL_ENTRY = "./tests/data/bundle_examples/lsl/src/main.luau"

local function lsl_bundles_successfully()
    local bundled = luau.bundle(LSL_ENTRY)
    assert(typeof(bundled) == "string", `lsl bundle should return a string, got: {tostring(bundled)}`)

    -- sizer.luau is spawned via thread.spawn { path = ... }, so the bundler should have
    -- inlined its source as a long string — verify by checking require("@std/fs") appears
    -- both in the hoisted stdlib block AND preserved inside the inlined long string
    local fs_require_count = count_occurrences(bundled, 'require("@std/fs")')
    assert(
        fs_require_count >= 2,
        `bundled lsl should contain require("@std/fs") at least twice (hoisted + inside inlined sizer), got {fs_require_count}`
    )

    -- bundled output should be syntactically valid luau
    local bytecode = luau.bytecode(bundled :: string)
    assert(typeof(bytecode) == "buffer", `bundled lsl should compile to bytecode without syntax errors, got: {tostring(bytecode)}`)
end
lsl_bundles_successfully()

local function lsl_double_bundle_preserves_inlined_requires()
    -- bundle lsl once to get output with sizer.luau inlined as a long string containing require()
    local first_bundle = luau.bundle(LSL_ENTRY)
    assert(typeof(first_bundle) == "string", `first bundle should succeed, got: {tostring(first_bundle)}`)

    -- write the first bundle to disk so we can bundle it again
    local tmp_path = "/tmp/seal_test_lsl_bundled.luau"
    fs.writefile(tmp_path, first_bundle :: string)

    -- bundle the already-bundled output: this is where the long-string require bug would trigger.
    -- without the fix, the bundler would incorrectly replace require("@std/fs") inside the
    -- [=====[...]=====] long string with the outer-scope identifier, which doesn't exist there.
    local second_bundle = luau.bundle(tmp_path)
    assert(
        typeof(second_bundle) == "string",
        `second bundle should succeed (long-string require fix), got error: {tostring(second_bundle)}`
    )

    -- require("@std/fs") should still appear inside the long string after the second pass
    local fs_require_count = count_occurrences(second_bundle :: string, 'require("@std/fs")')
    assert(
        fs_require_count >= 2,
        `double-bundled lsl should still have require("@std/fs") inside the inlined long string (not replaced), got {fs_require_count}`
    )

    -- second bundle should also be syntactically valid
    local bytecode = luau.bytecode(second_bundle :: string)
    assert(typeof(bytecode) == "buffer", `double-bundled lsl should be syntactically valid, got: {tostring(bytecode)}`)
end
lsl_double_bundle_preserves_inlined_requires()
