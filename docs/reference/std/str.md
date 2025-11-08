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

### .s

```luau
`function` .sstr.endswith: (s: string, suffix: string) -> boolean
```

 check if a string ends with `suffix`

---

### .s

```luau
`function` .sstr.starts: (s: string, ...: string) -> boolean
```

 like str.startswith, but accepts multiple prefixes

---

### .s

```luau
`function` .sstr.ends: (s: string, ...: string) -> boolean
```

 like str.endswith, but accepts multiple suffixes

---

### .s

```luau
`function` .sstr.trimfront: (s: string, ...: string) -> string
```

 trims any of the provided strings/characters from the front of the string `s`

 if no strings provided as ..., `str.trimfront` will trim whitespace (" ", "\n", etc.)

---

### .s

```luau
`function` .sstr.trimback: (s: string, ...: string) -> string
```

 trims any of the provided strings/characters/patterns from the back of the string `s`

 if no strings provided as ..., `str.trimback` will trim whitespace (" ", "\n", etc.)

---

### .s

```luau
`function` .sstr.trim: (s: string, ...: string) -> string
```

 trims one or many strings/characters/patterns from both front and back of string `s`

 if no strings provided to `...`, then default is whitespace

---

### .s

```luau
`function` .sstr.splitlines: (s: string, trim_trailing_whitespace: boolean?) -> { string }
```

 splits `s` by newlines, correctly handling carriage returns, trimming trailing whitespace,
 without an extra empty string, etc.

---

### .s

```luau
`function` .sstr.len: (s: string) -> number
```

 returns the utf-8 length if `s` is utf-8 or the regular string length #

---

### .s

```luau
`function` .sstr.width: (s: string) -> number
```

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

### .s

```luau
`function` .sstr.leftpad: (s: string, width: number, pad: string?) -> string
```

 left pads `s` to make it at least `width` characters long, using `pad` as the padding character.

---

### .s

```luau
`function` .sstr.escape: (s: string) -> string
```

 escapes special characters like `\n`, `\t`, `\\` for easier debugging

---

### .s

```luau
`function` .sstr.unescape: (s: string) -> string
```

 reverts `str.escape`

---

### .s

```luau
`function` .sstr.slice: (s: string, first: number, final: number)
```

 alias for string.sub

---

### .s

```luau
`function` .sstr.indent: (s: string, whitespace_type: "Tabs" | "Spaces", count: number, sep: ("\n" | "\r\n")?) -> string
```

 indents multiline string `count` characters; lines separated by `sep` (default "\n")

---

### .s

```luau
`function` .sstr.unindent: (s: string, whitespace_type: "Tabs" | "Spaces", count: number, sep: ("\n" | "\r\n")?) -> string
```

 unindents multiline string by `count` characters; lines separated by `sep` (default "\n")

---

### .s

```luau
`function` .sstr.split = internal.split :: : (s: string, ...string) -> { string }
```

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

### .s

```luau
`function` .sstr.splitaround = internal.splitaround :: : (s: string, seps: string, ...string) -> { string }
```

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

### .s

```luau
`function` .sstr.splitbefore = internal.splitbefore :: : (s: string, seps: string, ...string) -> { string }
```

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

### .s

```luau
`function` .sstr.splitafter = internal.splitafter :: : (s: string, seps: string, ...string) -> { string }
```

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

### .s

```luau
`function` .sstr.chars: (s: string) -> (...any) -> (number, string)
```

Iterate over the human-readable characters (graphemes) of a string

This function counts by 'characters', whereas `str.graphemes` provides byte indices for `string.sub`/`str.slice`

---

### .s

```luau
`function` .sstr.graphemes = internal.graphemes :: : (s: string) -> (...any) -> (number, string)
```

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
