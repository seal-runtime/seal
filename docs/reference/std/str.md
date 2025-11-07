<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# str

`local str = require("@std/str")`

$\hspace{5pt}$ Features ergonomic methods like `str.startwith`, `str.trimfront/trimback`, etc.
$\hspace{5pt}$
$\hspace{5pt}$ This library features utf-8-aware string handling, including easy access to splitting utf-8 strings,
$\hspace{5pt}$ iterating over the graphemes of a string, etc.
$\hspace{5pt}$
$\hspace{5pt}$ Unlike many seal standard libraries, inputs to `str` library functions don't necessarily have
$\hspace{5pt}$ to be valid utf-8 encoded strings.
$\hspace{5pt}$  check if a string starts with `prefix`

.function str.endswith(s: `string, suffix: string): boolean`

$\hspace{5pt}$  check if a string ends with `suffix`

.function str.starts(s: `string, ...: string): boolean`

$\hspace{5pt}$  like str.startswith, but accepts multiple prefixes

.function str.ends(s: `string, ...: string): boolean`

$\hspace{5pt}$  like str.endswith, but accepts multiple suffixes

.function str.trimfront(s: `string, ...: string): string`

$\hspace{5pt}$  trims any of the provided strings/characters from the front of the string `s`
$\hspace{5pt}$
$\hspace{5pt}$  if no strings provided as ..., `str.trimfront` will trim whitespace (" ", "\n", etc.)

.function str.trimback(s: `string, ...: string): string`

$\hspace{5pt}$  trims any of the provided strings/characters/patterns from the back of the string `s`
$\hspace{5pt}$
$\hspace{5pt}$  if no strings provided as ..., `str.trimback` will trim whitespace (" ", "\n", etc.)

.function str.trim(s: `string, ...: string): string`

$\hspace{5pt}$  trims one or many strings/characters/patterns from both front and back of string `s`
$\hspace{5pt}$
$\hspace{5pt}$  if no strings provided to `...`, then default is whitespace

.function str.splitlines(s: `string, trim_trailing_whitespace: boolean?): { string }`

$\hspace{5pt}$  splits `s` by newlines, correctly handling carriage returns, trimming trailing whitespace,
$\hspace{5pt}$  without an extra empty string, etc.

.function str.len(s: `string): number`

$\hspace{5pt}$  returns the utf-8 length if `s` is utf-8 or the regular string length #

.function str.width(s: `string): number`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ `str.width` estimates the number of monospace space characters required to correctly format/pad a utf8-encoded string.
$\hspace{5pt}$
$\hspace{5pt}$ ## Handles (or attempts to):
$\hspace{5pt}$ - **ASCII** characters and strings.
$\hspace{5pt}$ - Adjusts for **CJK (Chinese, Japanese, and Korean) characters**, which often take up double width.
$\hspace{5pt}$ - Accounts for **emoji width**, ensuring proper alignment in terminal/monospace output.
$\hspace{5pt}$
$\hspace{5pt}$ ## Simple usage:
$\hspace{5pt}$ ```luau
$\hspace{5pt}$ print(str.width("Hello")) -- 5
$\hspace{5pt}$ print(str.width("田中良")) -- 6 (each character takes 2 spaces)
$\hspace{5pt}$ print(str.width("🔥🎉")) -- 4 (each emoji takes 2 spaces)
$\hspace{5pt}$```
$\hspace{5pt}$
$\hspace{5pt}$ ## Actual example:
$\hspace{5pt}$ ```luau
$\hspace{5pt}$ local students = {
$\hspace{5pt}$     { name = "Alex Johnson", score = 95 },
$\hspace{5pt}$     { name = "田中良", score = 88 },
$\hspace{5pt}$     { name = "🔥🎉 Emily Carter", score = 92 },
$\hspace{5pt}$     { name = "Nadiya Kovalenko", score = 85 },
$\hspace{5pt}$ }
$\hspace{5pt}$
$\hspace{5pt}$ -- Calculate column widths dynamically using `str.width`
$\hspace{5pt}$ local max_name_width = 0
$\hspace{5pt}$ for _, student in students do
$\hspace{5pt}$     max_name_width = math.max(max_name_width, str.width(student.name))
$\hspace{5pt}$ end
$\hspace{5pt}$
$\hspace{5pt}$ -- Print formatted table
$\hspace{5pt}$ print("Name" .. string.rep(" ", max_name_width - str.width("Name")) .. " | Score")
$\hspace{5pt}$ print(string.rep("-", max_name_width) .. "-|------")
$\hspace{5pt}$
$\hspace{5pt}$ for _, student in students do
$\hspace{5pt}$     print(
$\hspace{5pt}$         student.name
$\hspace{5pt}$         .. string.rep(" ", max_name_width - str.width(student.name))
$\hspace{5pt}$         .. " | " .. student.score
$\hspace{5pt}$     )
$\hspace{5pt}$ end
$\hspace{5pt}$```

</details>

.function str.leftpad(s: `string, width: number, pad: string?): string`

$\hspace{5pt}$  left pads `s` to make it at least `width` characters long, using `pad` as the padding character.

.function str.escape(s: `string): string`

$\hspace{5pt}$  escapes special characters like `\n`, `\t`, `\\` for easier debugging

.function str.unescape(s: `string): string`

$\hspace{5pt}$  reverts `str.escape`

.function str.slice(s: `string, first: number, final: number)`

$\hspace{5pt}$  alias for string.sub

.function str.indent(s: `string, whitespace_type: "Tabs" | "Spaces", count: number, sep: ("\n" | "\r\n")?): string`

$\hspace{5pt}$  indents multiline string `count` characters; lines separated by `sep` (default "\n")

.function str.unindent(s: `string, whitespace_type: "Tabs" | "Spaces", count: number, sep: ("\n" | "\r\n")?): string`

$\hspace{5pt}$  unindents multiline string by `count` characters; lines separated by `sep` (default "\n")

.str.split = internal.split : `: (s: string, ...string) -> { string }`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ `str.split` is an improvement on luau's `string.split` in that it can split by multiple different strings (not just one single character)
$\hspace{5pt}$ at the same time and that the splitting is fully unicode grapheme aware.
$\hspace{5pt}$
$\hspace{5pt}$ If no separators are passed, `str.split` splits the string by graphemes (human-readable unicode characters);
$\hspace{5pt}$ otherwise, splitting is performed by the Aho-Corasick algorithm, which allows for efficient string splitting
$\hspace{5pt}$ with multiple separator strings.
$\hspace{5pt}$
$\hspace{5pt}$ ## Usage
$\hspace{5pt}$ ```luau
$\hspace{5pt}$ local chars = str.split("seals 🦭 ")
$\hspace{5pt}$ --> { "s", "e", "a", "l", "s", " ", "🦭", " "  }
$\hspace{5pt}$ local words = str.split("seals 🦭 say hi", " ")
$\hspace{5pt}$ --> { "seals", "🦭", "say", "hi" }
$\hspace{5pt}$ local omit_hi = str.split("seals 🦭 say hi", " ", "hi")
$\hspace{5pt}$ --> { "seals", "🦭", "say" }
$\hspace{5pt}$```
$\hspace{5pt}$
$\hspace{5pt}$ ### Notes
$\hspace{5pt}$ - Like with Luau's `string.split`, passing an empty separator string (`""`) to `str.split` will split the string by bytes instead of graphemes.
$\hspace{5pt}$ - splits that result in an empty string are not included in the returned array.
$\hspace{5pt}$ - `str.split` does not allow for overlapping splits when split with multiple separators.
$\hspace{5pt}$ - Separators are evaluated in left-to-right order, meaning that separators in front have higher priority than those in the back.
$\hspace{5pt}$
$\hspace{5pt}$
$\hspace{5pt}$ ### Edge cases:
$\hspace{5pt}$ - Sometimes simple characters directly to the right of emojis don't render when printed (example `print[["🦭"]]`)
$\hspace{5pt}$ - Some Hindi graphemes (like हा) don't render properly in terminals :(

</details>

.str.splitaround = internal.splitaround : `: (s: string, seps: string, ...string) -> { string }`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ Splits string `s` *around* one or more separator strings, keeping the separators in the final result.
$\hspace{5pt}$ This is especially useful for parsing and tokenizing text!
$\hspace{5pt}$
$\hspace{5pt}$ `str.splitaround` otherwise follows the same semantics as `str.split`.
$\hspace{5pt}$
$\hspace{5pt}$ Separators are evaluated in left-to-right order, meaning that separators in front have higher priority than those in the back.
$\hspace{5pt}$
$\hspace{5pt}$ Like `str.split`, `str.splitaround` is fully unicode grapheme-aware and can operate on full strings (instead of just characters).
$\hspace{5pt}$
$\hspace{5pt}$ ## Usage
$\hspace{5pt}$
$\hspace{5pt}$ ```luau
$\hspace{5pt}$ local line = `function Cat.meow(name: string, age: number)`
$\hspace{5pt}$ local tokens = str.splitaround(line, " ", ".", "(", ":", ",", ")")
$\hspace{5pt}$ --> { "function", " ", "Cat", ".", "meow", "(", "name", ":", " ", "string", ",", " ", "age", ":", " ", "number", ")"}
$\hspace{5pt}$```

</details>

.str.splitbefore = internal.splitbefore : `: (s: string, seps: string, ...string) -> { string }`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ Splits `s` in front of any passed separator strings, keeping the separator in the subsequent element of the returned array.
$\hspace{5pt}$
$\hspace{5pt}$ Otherwise has the same semantics as `str.split`.
$\hspace{5pt}$
$\hspace{5pt}$ ## Usage
$\hspace{5pt}$
$\hspace{5pt}$ ```luau
$\hspace{5pt}$ local messages = "[INFO] message\nnext line of message\n[WARN] bad warning\n[ERROR] message\n stack traceback"
$\hspace{5pt}$ local splitted = str.splitbefore(messages, "[INFO]", "[WARN]", "[ERROR]")
$\hspace{5pt}$ print(splitted) -->
$\hspace{5pt}$ {
$\hspace{5pt}$     "[INFO] message\nnext line of message\n",
$\hspace{5pt}$     "[WARN] bad warning\n",
$\hspace{5pt}$     "[ERROR] message\n stack traceback",
$\hspace{5pt}$ }
$\hspace{5pt}$```

</details>

.str.splitafter = internal.splitafter : `: (s: string, seps: string, ...string) -> { string }`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ Splits `s` after every occurrence of a separator string, keeping the separator in the current element of the returned array.
$\hspace{5pt}$
$\hspace{5pt}$ Otherwise has the same semantics as `str.split`.
$\hspace{5pt}$
$\hspace{5pt}$ ## Usage
$\hspace{5pt}$
$\hspace{5pt}$ ```luau
$\hspace{5pt}$ local httpheaders = "Content-Type: text/html\r\nContent-Length: 123\r\nConnection: keep-alive\r\n"
$\hspace{5pt}$ local splitted = str.splitafter(httpheaders, "\r\n") -->
$\hspace{5pt}$ {
$\hspace{5pt}$     "Content-Type: text/html\r\n",
$\hspace{5pt}$     "Content-Length: 123\r\n",
$\hspace{5pt}$     "Connection: keep-alive\r\n"
$\hspace{5pt}$ }
$\hspace{5pt}$```

</details>

.function str.chars(s: `string): (...any) -> (number, string)`

$\hspace{5pt}$ Iterate over the human-readable characters (graphemes) of a string
$\hspace{5pt}$
$\hspace{5pt}$ This function counts by 'characters', whereas `str.graphemes` provides byte indices for `string.sub`/`str.slice`

.str.graphemes = internal.graphemes : `: (s: string) -> (...any) -> (number, string)`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ Iterate over the utf-8 graphemes of `s` with indices useful for `str.slice` or `string.sub`
$\hspace{5pt}$
$\hspace{5pt}$ ## Usage
$\hspace{5pt}$ ```luau
$\hspace{5pt}$ local str = require("@std/str")
$\hspace{5pt}$
$\hspace{5pt}$ local utf8_string = "सील hi i am a seal 🦭"
$\hspace{5pt}$ for offset, grapheme in str.graphemes(utf8_string) do
$\hspace{5pt}$     print(`found '{grapheme}' starting at {offset} and ending at {offset + #grapheme}`)
$\hspace{5pt}$ end
$\hspace{5pt}$```
$\hspace{5pt}$
$\hspace{5pt}$ ### Edge cases:
$\hspace{5pt}$ - Sometimes simple characters directly to the right of emojis don't render when printed (example "🦭")
$\hspace{5pt}$ - Some Hindi graphemes (like हा) don't render properly in terminals :(

</details>
