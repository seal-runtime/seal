--!nolint LocalShadow
-- @std/str is partially implemented in rust as well as luau
-- turns out operations like startswith are faster in luau
-- but stuff like utf-8 aware string manipulation really prefer to just be rust bindings

--[=[
    Features ergonomic methods like `str.startwith`, `str.trimfront/trimback`, etc.

    This library features utf-8-aware string handling, including easy access to splitting utf-8 strings,
    iterating over the graphemes of a string, etc.

    Unlike many seal standard libraries, inputs to `str` library functions don't necessarily have
    to be valid utf-8 encoded strings; functions like `str.split`/`str.graphemes` won't error on invalid
    utf-8, but grapheme-aware results only make sense for utf-8 text.
    
    Other encodings (utf-16, a utf-8 bom, etc.) won't cause str.split or str.graphemes to
    to error out, but they won't produce meaningful results either — use
    `str.convert` to normalize to plain utf-8 first if you're not sure what encoding you have.
]=]
local str = {}

--- check if a string starts with `prefix`
function str.startswith(s: string, prefix: string): boolean
    return string.sub(s, 1, #prefix) == prefix
end

--- check if a string ends with `suffix`
function str.endswith(s: string, suffix: string): boolean
    return string.sub(s, -#suffix) == suffix
end

function str.starts(s: string, ...: string): boolean
    for _, prefix in {...} do
        if str.startswith(s, prefix) then
            return true
        end
    end
    return false
end

function str.ends(s: string, ...: string): boolean
    for _, suffix in {...} do
        if str.endswith(s, suffix) then
            return true
        end
    end
    return false
end

--- trims any of the provided strings/characters from the front of the string `s`
---
--- if no strings provided as ..., `str.trimfront` will trim whitespace (" ", "\n", etc.)
function str.trimfront(s: string, ...: string): string
    local patterns = {...}
    if #patterns == 0 then
        s = string.gsub(s, "^%s*", "")
    else
        for _, pattern in patterns do
            while string.find(s, "^" .. pattern) do
                s = string.gsub(s, "^" .. pattern, "")
            end
        end
    end
    
    return s
end

--- trims any of the provided strings/characters/patterns from the back of the string `s`
---
--- if no strings provided as ..., `str.trimback` will trim whitespace (" ", "\n", etc.)
function str.trimback(s: string, ...: string): string
    local patterns = {...}
    if #patterns == 0 then
        s = string.gsub(s, "%s*$", "")
    else
        for _, pattern in patterns do
            while string.find(s, pattern .. "$") do
                s = string.gsub(s, pattern .. "$", "")
            end
        end
    end
    
    return s
end

--- trims one or many strings/characters/patterns from both front and back of string `s`
--- 
--- if no strings provided to `...`, then default is whitespace
function str.trim(s: string, ...: string): string
    return str.trimback(str.trimfront(s, ...), ...)
end

--- indents multiline string `count` characters; lines separated by `sep` (default "\n")
function str.indent(s: string, whitespace_type: "Tabs" | "Spaces", count: number, sep: ("\n" | "\r\n")?): string
    local indent = if whitespace_type == "Tabs" then string.rep("\t", count) else string.rep(" ", count)
    local sep = sep or "\n" -- allows users to pass in \r\n or w/e if needed

    local result = {}
    local lines = str.splitlines(s)
    for _, line in lines do
        table.insert(result, indent .. line)
    end

    return table.concat(result, sep)
end

--- unindents multiline string by `count` characters; lines separated by `sep` (default "\n")
function str.unindent(s: string, whitespace_type: "Tabs" | "Spaces", count: number, sep: ("\n" | "\r\n")?): string
    local indent = if whitespace_type == "Tabs" then string.rep("\t", count) else string.rep(" ", count)
    local sep = sep or "\n"

    local result: { string } = {}
    -- LUAU FIXME: removing :: { string } cast causes blocked types + solver fails to complete
    for _, line in str.splitlines(s) :: { string } do
        if str.startswith(line :: string, indent) then
            local trimmed_line: string = string.gsub(line, "^" .. indent, "")
            table.insert(result, trimmed_line)
        else
            table.insert(result, line)
        end
    end

    return table.concat(result, sep)
end

--- returns the utf-8 length if `s` is utf-8 or the regular string length #
function str.len(s: string): number
    local utf8_len = utf8.len(s)
    if utf8_len then
        return utf8_len
    else
        return #s
    end
end

-- inlined equivalent of format.uncolor's ansi-stripping regex (\x1b\[[0-9;]*m). str.width is a hot
-- path for TUI code, so this avoids requiring @std/io/format (which would also risk a circular
-- import, since formatter.luau pulls in @std/terminal) just to make one cross-module namecall.
-- this local function itself gets inlined by the luau compiler, so there's no call overhead either
local function strip_ansi_colors(s: string): string
    return (string.gsub(s, "\27%[[%d;]*m", ""))
end

--[=[
    `str.width` estimates the number of monospace space characters required to correctly format/pad a utf8-encoded string.

    ## Handles (or attempts to):
    - **ASCII** characters and strings.
    - Adjusts for **CJK (Chinese, Japanese, and Korean) characters**, which often take up double width.
    - Accounts for **emoji width**, ensuring proper alignment in terminal/monospace output.
    
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
    s = strip_ansi_colors(s)
    local length = str.len(s)

    for _, grapheme in str.graphemes(s) do
        local codepoint = utf8.codepoint(grapheme)

        -- Detect **CJK wide characters**
        if (codepoint >= 0x4E00 and codepoint <= 0x9FFF) -- Common CJK
        or (codepoint >= 0x3400 and codepoint <= 0x4DBF) -- CJK Extension A
        or (codepoint >= 0x20000 and codepoint <= 0x2A6DF) -- CJK Extension B
        or (codepoint >= 0x2A700 and codepoint <= 0x2B73F) -- CJK Extension C
        or (codepoint >= 0x2B740 and codepoint <= 0x2B81F) -- CJK Extension D
        or (codepoint >= 0x2B820 and codepoint <= 0x2CEAF) -- CJK Extension E
        or (codepoint >= 0xF900 and codepoint <= 0xFAFF) -- CJK Compatibility Ideographs
        then
            length += 1 -- Treat CJK characters as wide
        end

        -- Detect **emoji width**
        if codepoint >= 0x1F300 and codepoint <= 0x1FAFF then
            length += 1 -- Emoji takes extra width
        end
    end

    return length
end

--- left pads `s` to make it at least `width` characters long, using `pad` as the padding character.
function str.leftpad(s: string, width: number, pad: string?): string
    pad = pad or " " -- default to space if no padding character is provided
    local padding_needed = width - #s
    if padding_needed > 0 then
        return string.rep(pad, padding_needed) .. s
    end
    return s
end

--- right pads `s` to make it at least `width` characters long, using `pad` as the padding character.
function str.rightpad(s: string, width: number, pad: string?): string
    pad = pad or " "
    local padding_needed = width - #s
    if padding_needed > 0 then
        return s .. string.rep(pad, padding_needed)
    end
    return s
end

--- escapes special characters like `\n`, `\t`, `\\` for easier debugging
function str.escape(s: string): string
    return (string.gsub(s, "[\\\n\t\r]", {
        ["\\"] = "\\\\",
        ["\n"] = "\\n",
        ["\t"] = "\\t",
        ["\r"] = "\\r"
    }))
end

--- reverts `str.escape`
function str.unescape(s: string): string
    local unescape_map = {
        ["\\n"] = "\n",
        ["\\t"] = "\t",
        ["\\r"] = "\r",
        ["\\\\"] = "\\"
    }
    return (string.gsub(s, "\\(.)", unescape_map))
end

--- Like string.sub, but unicode graphemes instead.
function str.slice(s: string, first: number, final: number?): string
    local chars = str.split(s)
    local b = ""
    for i, ch in chars do
        if i < first then
            continue
        elseif final and i == final then
            b ..= ch
            break
        elseif i >= first then
            b ..= ch
        end
    end
    return b
end

local str_internal = (require)("@internal/str")

--[=[
    str.split is an improvement on luau's string.split in that you can split on multiple different choices of characters/strings
    (not just a single string) and that the splitting is fully unicode grapheme aware.

    by default, str.split splits the string by unicode characters (graphemes)
]=]
str.split = str_internal.split :: (s: string, ...string) -> { string }

--[=[
	`str.splitaround` follows the same semantics as `str.split`, except that it splits in front of and behind the separator strings,
	keeping the separator strings in the final result.

	Like `str.split`, `str.splitaround` is fully unicode grapheme-aware and can operate on full strings (instead of just characters).

	## Usage
	
	```luau
	local line = `function Cat.meow(name: string, age: number)`
	local tokens = str.splitaround(line, " ", ".", "(", ":", ",", ")")
	--> { "function", " ", "Cat", ".", "meow", "(", "name", ":", " ", "string", ",", " ", "age", ":", " ", "number", ")"}
	```
]=]
str.splitaround = str_internal.splitaround :: (s: string, separators: string, ...string) -> { string }

str.splitbefore = str_internal.splitbefore :: (s: string, separators: string, ...string) -> { string }

str.splitafter = str_internal.splitafter :: (s: string, separators: string, ...string) -> { string }

--[=[
    Splits `s` by newlines, correctly handling carriage returns and trimming trailing whitespace
    (both by default). Empty lines are preserved - a leading empty line or an empty line between
    two others both show up as `""` in the result.

    The one exception is the string's final, unterminated remainder: if trimming reduces it to `""`,
    it's dropped by default (so `str.splitlines("a\n")` is `{"a"}`, not `{"a", ""}`). Pass
    `include_terminator = true` to keep that trailing `""` instead.
]=]
str.splitlines = str_internal.splitlines :: (s: string, trim_trailing_whitespace: boolean?, include_terminator: boolean?) -> { string }

--[=[
    Iterate over the human-readable characters (graphemes) of a string

    this function counts by 'characters', whereas `str.graphemes` provides byte indices for `string.sub`
]=]
function str.chars(s: string): (...any) -> (number, string)
    local current_index = 0
    local next_grapheme = str_internal.graphemes(s)
    return function()
        local bytes, grapheme = next_grapheme()
        if bytes ~= nil then
            current_index += 1
            return current_index, grapheme
        else
            return nil :: any, nil :: any
        end
    end
end

str.graphemes = str_internal.graphemes :: (s: string) -> (...any) -> (number, string)

export type Encoding = "Utf8" | "Utf8Bom" | "Utf16LE" | "Utf16LEBom" | "Utf16BE" | "Utf16BEBom" | "Binary"

--[=[
    Detects the encoding of arbitrary bytes `s` (string or buffer), returning one of:
    - `"Utf8"` / `"Utf8Bom"`
    - `"Utf16LE"` / `"Utf16LEBom"`
    - `"Utf16BE"` / `"Utf16BEBom"`
    - `"Binary"`, if `s` doesn't look like valid text in any of the above encodings

    See `str.convert` to convert `s` into a different encoding.
]=]
str.encoding = str_internal.encoding :: (s: string | buffer) -> Encoding

--[=[
    Converts `s` (string or buffer) from its `from` encoding (auto-detected via `str.encoding` if not given)
    into `to`, returning a string of the converted bytes.

    ## Usage
    ```luau
    local utf16 = str.convert("hi seals 🦭", "Utf16LEBom")
    local back = str.convert(utf16, "Utf8")
    ```
]=]
str.convert = str_internal.convert :: (s: string | buffer, to: Encoding, from: Encoding?) -> string

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
str.replace = str_internal.replace :: (s: string, old: string, new: string, max_replacements: number?, start_index: number?, end_index: number?) -> string

--- Reverses `s` by unicode grapheme, unlike luau's own `string.reverse` (which operates byte by
--- byte and corrupts multi-byte utf-8 content).
str.reverse = str_internal.reverse :: (s: string) -> string

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
str.splitfront = str_internal.splitfront :: (s: string, sep: string, max: number?) -> { string }

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
str.splitback = str_internal.splitback :: (s: string, sep: string, max: number?) -> { string }

-- -- stylua: ignore
-- local utf8_combining_marks: { [number]: boolean } = {
--     [0x0300] = true, [0x0301] = true, [0x0302] = true, [0x0303] = true, [0x0304] = true, [0x0305] = true,
--     [0x0306] = true, [0x0307] = true, [0x0308] = true, [0x0309] = true, [0x030A] = true, [0x030B] = true,
--     [0x030C] = true, [0x030D] = true, [0x030E] = true, [0x030F] = true, [0x0310] = true, [0x0311] = true,
--     [0x0312] = true, [0x0313] = true, [0x0314] = true, [0x0315] = true, [0x0316] = true, [0x0317] = true,
--     [0x0318] = true, [0x0319] = true, [0x031A] = true, [0x031B] = true, [0x031C] = true, [0x031D] = true,
--     [0x031E] = true, [0x031F] = true, [0x0320] = true, [0x0321] = true, [0x0322] = true, [0x0323] = true,
--     [0x0324] = true, [0x0325] = true, [0x0326] = true, [0x0327] = true, [0x0328] = true, [0x0329] = true,
--     [0x032A] = true, [0x032B] = true, [0x032C] = true, [0x032D] = true, [0x032E] = true, [0x032F] = true,
--     [0x0330] = true, [0x0331] = true, [0x0332] = true, [0x0333] = true, [0x0334] = true, [0x0335] = true,
--     [0x0336] = true, [0x0337] = true, [0x0338] = true, [0x0339] = true, [0x033A] = true, [0x033B] = true,
--     [0x033C] = true, [0x033D] = true, [0x033E] = true, [0x033F] = true, [0x0340] = true, [0x0341] = true,
--     [0x0342] = true, [0x0343] = true, [0x0344] = true, [0x0345] = true 
-- 	-- More combining marks exist, but these cover many.
-- }

-- -- stylua: ignore
-- local utf8_skin_tone_modifiers = {
--     [0x1F3FB] = true, [0x1F3FC] = true, [0x1F3FD] = true, [0x1F3FE] = true, [0x1F3FF] = true
-- }

-- -- stylua: ignore
-- local utf8_regional_indicators = {
--     [0x1F1E6] = true, [0x1F1E7] = true, [0x1F1E8] = true, [0x1F1E9] = true, [0x1F1EA] = true, [0x1F1EB] = true,
--     [0x1F1EC] = true, [0x1F1ED] = true, [0x1F1EE] = true, [0x1F1EF] = true, [0x1F1F0] = true, [0x1F1F1] = true,
--     [0x1F1F2] = true, [0x1F1F3] = true, [0x1F1F4] = true, [0x1F1F5] = true, [0x1F1F6] = true, [0x1F1F7] = true,
--     [0x1F1F8] = true, [0x1F1F9] = true, [0x1F1FA] = true, [0x1F1FB] = true, [0x1F1FC] = true, [0x1F1FD] = true,
--     [0x1F1FE] = true, [0x1F1FF] = true
-- }

-- --[=[
-- 	iterate over the human-readable characters (aka utf-8 graphemes) of a utf-8 encoded string
-- ]=]
-- function str.chars(s: string): () -> (number?, string?)
--     return coroutine.wrap(function()
--         local index = 1
--         local buffer = ""
--         local prev_codepoint = nil

--         for i, codepoint in utf8.codes(s) do
--             local char = utf8.char(codepoint)

--             -- combining marks, skin tone modifiers, zero-width joiners, and regional indicators
--             if utf8_combining_marks[codepoint] or utf8_skin_tone_modifiers[codepoint] or prev_codepoint == 0x200D then
--                 buffer ..= char
--             elseif prev_codepoint and utf8_regional_indicators[prev_codepoint] and utf8_regional_indicators[codepoint] then
--                 buffer ..= char -- keep flag emojis intact
--             else
--                 -- yield previous grapheme before starting a new one
--                 if buffer ~= "" then
--                     coroutine.yield(index, buffer)
--                     index += 1
--                 end
--                 buffer = char
--             end

--             prev_codepoint = codepoint
--         end

--         -- yield last grapheme if any
--         if buffer ~= "" then
--             coroutine.yield(index, buffer)
--         end
        
-- 		return nil, nil
--     end)
-- end

-- --[=[
--     Tries to *correctly* split `s` into human-readable characters (aka utf-8 graphemes), handling cases like:
--     - Multi-byte UTF-8 characters (e.g., "你好" or "á")
--     - Combining diacritical marks (e.g., "é" → keeps "e" + acute accent together)
--     - Emoji sequences with modifiers (e.g., "👋🏾" → keeps the emoji and skin tone modifier together)
--     - Zero-width joiner (ZWJ) sequences (e.g., "👨‍👩‍👧" → keeps family emoji intact)
--     - Regional indicator symbols for country flags (e.g., "🇺🇸" → keeps flag as one unit)
--     - Standard ASCII characters (e.g., "hello" → splits normally)
-- ]=]
-- function str.splitchars(s: string): { string }
--     local graphemes: { string } = {}
--     local buffer: string = ""
--     local prev_codepoint = nil

--     for _, codepoint in utf8.codes(s) do
--         local char = utf8.char(codepoint)

--         if utf8_combining_marks[codepoint] or utf8_skin_tone_modifiers[codepoint] then
--             -- Attach combining marks and skin tone modifiers to the previous character
--             buffer ..= char
--         elseif prev_codepoint and utf8_regional_indicators[prev_codepoint] and utf8_regional_indicators[codepoint] then
--             -- Attach regional indicator characters together (e.g., 🇺🇸)
--             buffer ..= char
--         elseif prev_codepoint == 0x200D then
--             -- Attach characters following a zero-width joiner (ZWJ), preserving emoji sequences like 👨‍👩‍👧
--             buffer ..= char
--         else
--             -- Store previous buffer and start a new grapheme
--             if buffer ~= "" then
--                 table.insert(graphemes, buffer)
--             end
--             buffer = char
--         end

--         prev_codepoint = codepoint
--     end

--     if buffer ~= "" then
--         table.insert(graphemes, buffer)
--     end

--     return graphemes
-- end

-- --[=[
-- 	Iterate over the utf-8 graphemes of a string, alias for `str.chars`.
-- ]=]
-- function str.graphemes(s: string)
-- 	return str.chars
-- end

local function _verytest()
    -- local t1 = "we are the dead"
    -- print(str.splitchars(t1))
    
    -- local willthiswork = "Hello, world!\n\twith tab characters.\r\n"
    --     .. "Here’s a mix: naïve café élan résumé!\n"
    --     .. "Whitespace test: \t\t\r\nMultiple spaces.\n"
    --     .. "Combining marks: é å ö\n"
    --     .. "Emoji sequences: 👨‍👩‍👧‍👦 👩🏽‍🚀 🏳️‍🌈 🇺🇸 🚀🌍❤️\n"
    --     .. "Special Unicode chars: Æ Ø ß ¿ ¡ ₿ € ©® ™\n"
    --     .. "Math symbols: ∑ ∞ ∫ √ ∂ Δ Π\n"
    
    -- print(str.splitchars(willthiswork))
    
    -- credit: song is Далекі Обрії by Ukranian metal band Ignea
    local daleki_obriyi_lyrics = [[
    "Далекі Обрії" by Ignea
    
    Неначе у золотій клітці
    Охоплена гамою барв
    Не cтерплять ми бідні зіниці
    Щодня - небуденний чар
    Міцні татуйовані лиця
    Боронять минуле своє
    Прощальне слово "верніться"
    Спіткає усюди мене
    Там де жодних листів не буває
    Скільки років кому не питають
    Полюю на кадри світу, що зникає
    Далекі обрії кличуть і манять
    Тремтіла, ламалась, благала
    Звивалась фільмова тасьма
    Дно пропасті - карна виправа
    Розсудить пралісна імла
    Коралеві скелі і мури
    Квітковий вівтар на ножах
    Тепер мене звати Теура
    Тепер я - багряний птах!
    Мої світлини не в змозі передати
    Неземної краси і омани
    Люди там не живуть - виживають
    Далекі обрії ваблять, жахають
    ]]
    
    local chars = {}
    local t0 = os.clock()
    for index, char in str.chars(daleki_obriyi_lyrics) do
        -- print(`index: {index}, grapheme: {char}`)
        table.insert(chars, char)
    end
    -- print(os.clock() - t0)
    local t1 = os.clock() - t0
    print(chars)
    local badchars = {}
    local t2 = os.clock()
    for i, bad in string.split(daleki_obriyi_lyrics, "") do
        table.insert(badchars, bad)
    end
    print(t1)
    print(os.clock() - t2)
    -- print(badchars)
end

return table.freeze(str)