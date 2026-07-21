--[=[
    *seal*'s string extension library. This adds back functions that usually are
    methods or just builtins in other languages, including:
    - ergonomic methods such as `str.startswith`, `str.trimfront/trimback`, etc., 
    - utf-8 aware string handling for visible string length/width/padding,
    - splitting and iterating over unicode graphemes, 
    - encoding and changing the encoding of strings from utf-16 to utf-8 and back,
    - string escaping and unescaping,
    - and more.

    Unlike many seal standard libraries, inputs to `str` library functions don't
    necessarily have to be valid utf-8 encoded strings; many operate just fine on
    arbitrary bytes or even buffers.

    Some graphemes-aware functions, such as `str.split` in graphemes mode and `str.graphemes`, 
    will only operate properly on valid utf-8 without BOM (use `str.convert` to normalize your input), 
    but should not error out when given arbitrary bytes.
]=]
local str = {}

--- check if a string starts with `prefix`
function str.startswith(s: string, prefix: string): boolean
    return nil :: any
end

--- check if a string ends with `suffix`
function str.endswith(s: string, suffix: string): boolean
    -- return string.sub(s, -#suffix) == suffix
    return nil :: any
end

--- like str.startswith, but accepts multiple prefixes
function str.starts(s: string, ...: string): boolean
    for _, prefix in {...} do
        if str.startswith(s, prefix) then
            return true
        end
    end
    return false
end

--- like str.endswith, but accepts multiple suffixes
function str.ends(s: string, ...: string): boolean
    for _, suffix in {...} do
        if str.endswith(s, suffix) then
            return true
        end
    end
    return false
end

--- trims any of the provided strings/characters/patterns from the front of the string `s` 
--- or trims whitespace if no patterns are specified.
---
--- Patterns are Luau string patterns passed to `string.gsub`, make sure to escape with
--- `%` if needed.
function str.trimfront(s: string, ...: string): string
    -- local patterns = {...}
    -- if #patterns == 0 then
    -- 	s = string.gsub(s, "^%s*", "")
    -- else
    -- 	for _, pattern in patterns do
    -- 		while string.find(s, "^" .. pattern) do
    -- 			s = string.gsub(s, "^" .. pattern, "")
    -- 		end
    -- 	end
    -- end

    -- return s
    return nil :: any
end


--- trims any of the provided strings/characters/patterns from the back of the string `s` 
--- or trims whitespace if no patterns are specified.
---
--- Patterns are Luau string patterns passed to `string.gsub`, make sure to escape with
--- `%` if needed.
function str.trimback(s: string, ...: string): string
    -- local patterns = {...}
    -- if #patterns == 0 then
    -- 	s = string.gsub(s, "%s*$", "")
    -- else
    -- 	for _, pattern in patterns do
    -- 		while string.find(s, pattern .. "$") do
    -- 			s = string.gsub(s, pattern .. "$", "")
    -- 		end
    -- 	end
    -- end

    -- return s
    return nil :: any
end

--- trims any of the provided strings/characters/patterns from the front and back
--- of the string `s` or trims whitespace if no patterns are specified.
---
--- Patterns are Luau string patterns passed to `string.gsub`, make sure to escape with
--- `%` if needed.
function str.trim(s: string, ...: string): string
    -- return str.trimback(str.trimfront(s, ...), ...)
    return nil :: any
end

--[=[
    Splits string `s` into an array of lines, correctly handling LF and CRLF (`\n` and `\r\n`),
    optionally trimming any whitespace left at the end of the string after splitting.

    By default, `trim_trailing_whitespace` defaults to `true` and `include_terminator` defaults to `false`.

    Empty lines are preserved; a leading empty line or empty lines between others show up as 
    empty strings in the result. When `include_terminator` is enabled, the result will include 
    a final empty string after the last line separator to allow for easy concatting of a file's lines
    while preserving the last `\n` or `\r\n`.

    This function is highly optimized, using a single pass over all lines with SIMD
    accelerated byte scanning with `memchr` and 0 intermediate allocations. Should be
    at least 2x faster than Luau's `string.split` for splitting by newlines.
]=]
function str.splitlines(s: string, trim_trailing_whitespace: boolean?, include_terminator: boolean?): { string }
    return nil :: any
end

--- returns the utf-8 length if `s` is utf-8 or the regular string length # otherwise
function str.len(s: string): number
    -- local utf8_len = utf8.len(s)
    -- if utf8_len then
    -- 	return utf8_len
    -- else
    -- 	return #s
    -- end
    return nil :: any
end

--[=[
    `str.width` estimates the number of monospace space characters required to correctly format/pad a utf8-encoded string
    in terminal output.

    ## Handles (or attempts to):
    - **ASCII** characters and strings.
    - Adjusts for **CJK (Chinese, Japanese, and Korean) characters**, which often take up double width.
    - Accounts for **emoji width**, ensuring proper alignment in terminal/monospace output.
    - Strips **ANSI colors** and formatting codes (such as those generated by the std/io/colors library)
    
    ## Simple usage:
    ```luau
    print(str.width("Hello")) -- 5
    print(str.width("田中良")) -- 6 (each character takes 2 spaces)
    print(str.width("🔥🎉")) -- 4 (each emoji takes 2 spaces)
    ```

    ## Actual example:
    ```luau
    local students = {
        { name = "Alex Johnson", score = 95 },
        { name = "田中良", score = 88 },
        { name = "🔥🎉 Emily Carter", score = 92 },
        { name = "Nadiya Kovalenko", score = 85 },
    }

    -- Calculate column widths dynamically using `str.width`
    local max_name_width = 0
    for _, student in students do
        max_name_width = math.max(max_name_width, str.width(student.name))
    end

    -- Print formatted table
    print("Name" .. string.rep(" ", max_name_width - str.width("Name")) .. " | Score")
    print(string.rep("-", max_name_width) .. "-|------")

    for _, student in students do
        print(
            student.name
            .. string.rep(" ", max_name_width - str.width(student.name)) 
            .. " | " .. student.score
        )
    end
    ```
]=]
function str.width(s: string): number
    return nil :: any
end

--- left pads `s` to make it at least `width` characters long, using `pad` as the padding character.
function str.leftpad(s: string, width: number, pad: string?): string
    -- pad = pad or " " -- default to space if no padding character is provided
    -- local padding_needed = width - #s
    -- if padding_needed > 0 then
    --     return string.rep(pad, padding_needed) .. s
    -- end
    -- return s
    return nil :: any
end

--- right pads `s` to make it at least `width` characters long, using `pad` as the padding character.
function str.rightpad(s: string, width: number, pad: string?): string
    return nil :: any
end

--[=[
    Replaces non-overlapping, literal (not pattern) occurrences of `old` in `s` with `new`.

    `start_index`/`end_index` follow `string.sub`'s indexing (1-indexed, negative counts from the
    end) and restrict which portion of `s` is searched - content outside that range is left
    untouched even if `old` occurs there. `max_replacements` caps how many occurrences get replaced
    (unlimited if omitted).

    ## Usage
    ```luau
    str.replace("the cat sat on the mat", "at", "og") --> "the cog sog on the mog"
    str.replace("aaaa", "a", "b", 2) --> "bbaa"
    ```
]=]
function str.replace(s: string, old: string, new: string, max_replacements: number?, start_index: number?, end_index: number?): string
    return nil :: any
end

--- Reverses `s` by unicode grapheme, unlike luau's own `string.reverse` (which operates byte by
--- byte and corrupts multi-byte utf-8 content).
function str.reverse(s: string): string
    return nil :: any
end

--- escapes special characters like `\n`, `\t`, `\\` for easier debugging
function str.escape(s: string): string
    -- return (string.gsub(s, "[\\\n\t\r]", {
    --     ["\\"] = "\\\\",
    --     ["\n"] = "\\n",
    --     ["\t"] = "\\t",
    --     ["\r"] = "\\r"
    -- }))
    return nil :: any
end

--- reverts `str.escape`
function str.unescape(s: string): string
    -- local unescape_map = {
    --     ["\\n"] = "\n",
    --     ["\\t"] = "\t",
    --     ["\\r"] = "\r",
    --     ["\\\\"] = "\\"
    -- }
    -- return (string.gsub(s, "\\(.)", unescape_map))
    return nil :: any
end

--- Like `string.sub` but indexes by utf-8 unicode graphemes instead of bytes.
function str.slice(s: string, first: number, final: number)
    return nil :: any
end

--- indents multiline string `count` characters; lines separated by `sep` (default "\n")
function str.indent(s: string, whitespace_type: "Tabs" | "Spaces", count: number, sep: ("\n" | "\r\n")?): string
    -- local indent = if whitespace_type == "Tabs" then string.rep("\t", count) else string.rep(" ", count)
    -- local sep = sep or "\n" -- allows users to pass in \r\n or w/e if needed

    -- local result = {}
    -- local lines = str.splitlines(s)
    -- for _, line in lines do
    --     table.insert(result, indent .. line)
    -- end

    -- return table.concat(result, sep)
    return nil :: any
end

--- unindents multiline string by `count` characters; lines separated by `sep` (default "\n")
function str.unindent(s: string, whitespace_type: "Tabs" | "Spaces", count: number, sep: ("\n" | "\r\n")?): string
    -- local indent = if whitespace_type == "Tabs" then string.rep("\t", count) else string.rep(" ", count)
    -- local sep = sep or "\n"

    -- local result = {}
    -- for _, line in str.splitlines(s) do
    --     if str.startswith(line, indent) then
    --         local trimmed_line = string.gsub(line, "^" .. indent, "")
    --         table.insert(result, trimmed_line)
    --     else
    --         table.insert(result, line)
    --     end
    -- end

    -- return table.concat(result, sep)
    return nil :: any
end

-- local internal = (require)("@std/str_internal")

--[=[
    `str.split` is an improvement on luau's `string.split` in that it can split by multiple different strings (not just one single character)
    at the same time and that the splitting is fully unicode grapheme aware.

    If no separators are passed, `str.split` splits the string by graphemes (human-readable unicode characters);
    otherwise, splitting is usually performed by the Aho-Corasick algorithm, which allows for efficient string splitting
    with multiple separator strings. 
    
    As a further optimization, if only one, single-byte separator is passed, this function will use a vectorized byte scan instead of the Aho-Corasick algorithm.

    ## Usage
    ```luau
    local chars = str.split("seals 🦭 ") 
    --> { "s", "e", "a", "l", "s", " ", "🦭", " "  }
    local words = str.split("seals 🦭 say hi", " ") 
    --> { "seals", "🦭", "say", "hi" }
    local omit_hi = str.split("seals 🦭 say hi", " ", "hi")
    --> { "seals", "🦭", "say" }
    ```

    ### Notes
    - Like with Luau's `string.split`, passing an empty separator string (`""`) to `str.split` will split the string by bytes instead of graphemes.
    - splits that result in an empty string are not included in the returned array.
    - `str.split` does not allow for overlapping splits when split with multiple separators.
    - Separators are evaluated in left-to-right order, meaning that separators in front have higher priority than those in the back.

    ### Edge cases:
    - Sometimes simple characters directly to the right of emojis don't render when printed (example `print[["🦭"]]`)
    - Some Hindi graphemes (like हा) don't render properly in terminals :(
]=]
function str.split(s: string, ...: string): { string }
    return nil :: any
end

--[=[
    Splits string `s` *around* one or more separator strings, keeping the separators in the final result.
    This is especially useful for parsing and tokenizing text!

    `str.splitaround` otherwise follows the same semantics as `str.split`.

    Separators are evaluated in left-to-right order, meaning that separators in front have higher priority than those in the back.

    Like `str.split`, `str.splitaround` is fully unicode grapheme-aware and can operate on full strings (instead of just characters).

    ## Usage
    
    ```luau
    local line = `function Cat.meow(name: string, age: number)`
    local tokens = str.splitaround(line, " ", ".", "(", ":", ",", ")")
    --> { "function", " ", "Cat", ".", "meow", "(", "name", ":", " ", "string", ",", " ", "age", ":", " ", "number", ")"}
    ```
]=]
function str.splitaround(s: string, seps: string, ...: string): { string }
    return nil :: any
end

--[=[
    Splits `s` in front of any passed separator strings, keeping the separator in the subsequent element of the returned array.

    Otherwise has the same semantics as `str.split`.

    ## Usage

    ```luau
    local messages = "[INFO] message\nnext line of message\n[WARN] bad warning\n[ERROR] message\n stack traceback"
    local splitted = str.splitbefore(messages, "[INFO]", "[WARN]", "[ERROR]")
    print(splitted) --> 
    {
        "[INFO] message\nnext line of message\n",
        "[WARN] bad warning\n",
        "[ERROR] message\n stack traceback",
    }
    ```
]=]
function str.splitbefore(s: string, seps: string, ...: string): { string }
    return nil :: any
end

--[=[
    Splits `s` after every occurrence of a separator string, keeping the separator in the current element of the returned array.

    Otherwise has the same semantics as `str.split`.

    ## Usage

    ```luau
    local httpheaders = "Content-Type: text/html\r\nContent-Length: 123\r\nConnection: keep-alive\r\n"
    local splitted = str.splitafter(httpheaders, "\r\n") --> 
    { 
        "Content-Type: text/html\r\n", 
        "Content-Length: 123\r\n", 
        "Connection: keep-alive\r\n"
    }
    ```
]=]
function str.splitafter(s: string, seps: string, ...: string): { string }
    return nil :: any
end

--[=[
    Splits `s` on `sep` starting from the front, stopping after `max` splits (default `1`) - the
    remainder (everything after the last split) becomes the final element as-is, even if `sep`
    occurs again within it. Like python's `str.split(sep, maxsplit)`.

    ## Usage
    ```luau
    str.splitfront("a,b,c,d", ",") --> {"a", "b,c,d"}
    str.splitfront("a,b,c,d", ",", 2) --> {"a", "b", "c,d"}
    ```
]=]
function str.splitfront(s: string, sep: string, max: number?): { string }
    return nil :: any
end

--[=[
    Splits `s` on `sep` starting from the back, stopping after `max` splits (default `1`) - the
    remainder (everything before the first split) becomes the first element as-is, even if `sep`
    occurs again within it. Like python's `str.rsplit(sep, maxsplit)`.

    ## Usage
    ```luau
    str.splitback("a,b,c,d", ",") --> {"a,b,c", "d"}
    str.splitback("a,b,c,d", ",", 2) --> {"a,b", "c", "d"}
    ```
]=]
function str.splitback(s: string, sep: string, max: number?): { string }
    return nil :: any
end

--[=[
    Iterate over the human-readable characters (graphemes) of a string

    This function counts by character position (1st grapheme, 2nd grapheme, etc.), which is what
    `str.slice` expects - not a byte offset. Use `str.graphemes` instead if you need byte offsets
    for `string.sub`.
]=]
function str.chars(s: string): (...any) -> (number, string)
    -- local current_index = 0
    -- local next_grapheme = internal.graphemes(s)
    -- return function()
    -- 	local bytes, grapheme = next_grapheme()
    -- 	if bytes ~= nil then
    -- 		current_index += 1
    -- 		return current_index, grapheme
    -- 	else
    -- 		return nil, nil :: any
    -- 	end
    -- end
    return nil :: any
end

--[=[
    Iterate over the utf-8 graphemes of `s`, yielding byte offsets useful for `string.sub` (not
    `str.slice`, which expects character positions - see `str.chars` for that instead).

    ## Usage
    ```luau
    local str = require("@std/str")

    local utf8_string = "सील hi i am a seal 🦭"
    for offset, grapheme in str.graphemes(utf8_string) do
        print(`found '{grapheme}' starting at {offset} and ending at {offset + #grapheme}`)
    end
    ```

    ### Edge cases:
    - Sometimes simple characters directly to the right of emojis don't render when printed (example "🦭")
    - Some Hindi graphemes (like हा) don't render properly in terminals :(
]=]
function str.graphemes(s: string): () -> (number, string)
    return nil :: any
end

export type Encoding = "Utf8" | "Utf8Bom" | "Utf16LE" | "Utf16LEBom" | "Utf16BE" | "Utf16BEBom" | "Binary"

--[=[
    Detects the encoding of arbitrary bytes `s` (string or buffer), returning one of:
    - `"Utf8"` / `"Utf8Bom"`
    - `"Utf16LE"` / `"Utf16LEBom"`
    - `"Utf16BE"` / `"Utf16BEBom"`
    - `"Binary"`, if `s` doesn't look like valid text in any of the above encodings

    See `str.convert` to convert `s` into a different encoding.
]=]
function str.encoding(s: string | buffer): Encoding
    return nil :: any
end

--[=[
    Converts `s` (string or buffer) from its `from` encoding (auto-detected via `str.encoding` if not given)
    into `to`, returning a string of the converted bytes.

    ## Usage
    ```luau
    local utf16 = str.convert("hi seals 🦭", "Utf16LEBom")
    local back = str.convert(utf16, "Utf8")
    ```
]=]
function str.convert(s: string | buffer, to: Encoding, from: Encoding?): string
    return nil :: any
end

return str
