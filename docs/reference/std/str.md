<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# str

`local str = require("@std/str")`

Features ergonomic methods like `str.startwith`, `str.trimfront/trimback`, etc.

This library features utf-8-aware string handling, including easy access to splitting utf-8 strings,
iterating over the graphemes of a string, etc.

Unlike many seal standard libraries, inputs to `str` library functions don't necessarily have
to be valid utf-8 encoded strings.
 check if a string starts with `prefix`

---

### str.endswith

<h4>

```luau
function str.endswith(s: string, suffix: string): boolean
```

</h4>

 check if a string ends with `suffix`

---

### str.starts

<h4>

```luau
function str.starts(s: string, ...: string): boolean
```

</h4>

 like str.startswith, but accepts multiple prefixes

---

### str.ends

<h4>

```luau
function str.ends(s: string, ...: string): boolean
```

</h4>

 like str.endswith, but accepts multiple suffixes

---

### str.trimfront

<h4>

```luau
function str.trimfront(s: string, ...: string): string
```

</h4>

 trims any of the provided strings/characters from the front of the string `s`

 if no strings provided as ..., `str.trimfront` will trim whitespace (" ", "\n", etc.)

---

### str.trimback

<h4>

```luau
function str.trimback(s: string, ...: string): string
```

</h4>

 trims any of the provided strings/characters/patterns from the back of the string `s`

 if no strings provided as ..., `str.trimback` will trim whitespace (" ", "\n", etc.)

---

### str.trim

<h4>

```luau
function str.trim(s: string, ...: string): string
```

</h4>

 trims one or many strings/characters/patterns from both front and back of string `s`

 if no strings provided to `...`, then default is whitespace

---

### str.splitlines

<h4>

```luau
function str.splitlines(s: string, trim_trailing_whitespace: boolean?): { string }
```

</h4>

 splits `s` by newlines, correctly handling carriage returns, trimming trailing whitespace,
 without an extra empty string, etc.

---

### str.len

<h4>

```luau
function str.len(s: string): number
```

</h4>

 returns the utf-8 length if `s` is utf-8 or the regular string length #

---

### str.width

<h4>

```luau
function str.width(s: string): number
```

</h4>

<details>

<summary> See the docs </summary

`str.width` estimates the number of monospace space characters required to correctly format/pad a utf8-encoded string.

## Handles (or attempts to)

- **ASCII** characters and strings.
- Adjusts for **CJK (Chinese, Japanese, and Korean) characters**, which often take up double width.
- Accounts for **emoji width**, ensuring proper alignment in terminal/monospace output.

## Simple usage

```luau
print(str.width("Hello")) -- 5
print(str.width("田中良")) -- 6 (each character takes 2 spaces)
print(str.width("🔥🎉")) -- 4 (each emoji takes 2 spaces)
```

## Actual example

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

</details>

---

### str.leftpad

<h4>

```luau
function str.leftpad(s: string, width: number, pad: string?): string
```

</h4>

 left pads `s` to make it at least `width` characters long, using `pad` as the padding character.

---

### str.escape

<h4>

```luau
function str.escape(s: string): string
```

</h4>

 escapes special characters like `\n`, `\t`, `\\` for easier debugging

---

### str.unescape

<h4>

```luau
function str.unescape(s: string): string
```

</h4>

 reverts `str.escape`

---

### str.slice

<h4>

```luau
function str.slice(s: string, first: number, final: number)
```

</h4>

 alias for string.sub

---

### str.indent

<h4>

```luau
function str.indent(s: string, whitespace_type: "Tabs" | "Spaces", count: number, sep: ("\n" | "\r\n")?): string
```

</h4>

 indents multiline string `count` characters; lines separated by `sep` (default "\n")

---

### str.unindent

<h4>

```luau
function str.unindent(s: string, whitespace_type: "Tabs" | "Spaces", count: number, sep: ("\n" | "\r\n")?): string
```

</h4>

 unindents multiline string by `count` characters; lines separated by `sep` (default "\n")

---

### str.split

<h4>

```luau
function str.split(s: string, ...: string): { string }
```

</h4>

<details>

<summary> See the docs </summary

`str.split` is an improvement on luau's `string.split` in that it can split by multiple different strings (not just one single character)
at the same time and that the splitting is fully unicode grapheme aware.

If no separators are passed, `str.split` splits the string by graphemes (human-readable unicode characters);
otherwise, splitting is performed by the Aho-Corasick algorithm, which allows for efficient string splitting
with multiple separator strings.

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

### Edge cases

- Sometimes simple characters directly to the right of emojis don't render when printed (example `print[["🦭"]]`)
- Some Hindi graphemes (like हा) don't render properly in terminals :(

</details>

---

### str.splitaround

<h4>

```luau
function str.splitaround(s: string, seps: string, ...: string): { string }
```

</h4>

<details>

<summary> See the docs </summary

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

</details>

---

### str.splitbefore

<h4>

```luau
function str.splitbefore(s: string, seps: string, ...: string): { string }
```

</h4>

<details>

<summary> See the docs </summary

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

</details>

---

### str.splitafter

<h4>

```luau
function str.splitafter(s: string, seps: string, ...: string): { string }
```

</h4>

<details>

<summary> See the docs </summary

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

</details>

---

### str.chars

<h4>

```luau
function str.chars(s: string): (...any) -> (number, string)
```

</h4>

Iterate over the human-readable characters (graphemes) of a string

This function counts by 'characters', whereas `str.graphemes` provides byte indices for `string.sub`/`str.slice`

---

### str.graphemes

<h4>

```luau
function str.graphemes(s: string): (...any) -> (number, string)
```

</h4>

<details>

<summary> See the docs </summary

Iterate over the utf-8 graphemes of `s` with indices useful for `str.slice` or `string.sub`

## Usage

```luau
local str = require("@std/str")

local utf8_string = "सील hi i am a seal 🦭"
for offset, grapheme in str.graphemes(utf8_string) do
    print(`found '{grapheme}' starting at {offset} and ending at {offset + #grapheme}`)
end
```

### Edge cases

- Sometimes simple characters directly to the right of emojis don't render when printed (example "🦭")
- Some Hindi graphemes (like हा) don't render properly in terminals :(

</details>

---

Autogenerated from [std/str.luau](/.seal/typedefs/std/str.luau).

*seal* is best experienced with inline, in-editor documentation. Please see the linked typedefs file if this documentation is confusing, too verbose, or inaccurate.
