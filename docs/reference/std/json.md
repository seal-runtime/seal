<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# json

`local json = require("@std/json")`

Easily manipulate JSON (JavaScript Object Notation) data.

## Usage

```luau
local json = require("@std/json")
local data = json.readfile("./animals.json") :: { cats: number, dogs: number }
data.cats += 1
json.writefile("./animals.json", data)
```

---

## json.encode

<h4>

```luau
json.encode: (t: JsonData, options: EncodeOptions?) -> string,
```

</h4>

 encodes a table as json; by default this encodes as a pretty-formatted string; use `json.raw` for a condensed version instead

---

## json.raw

<h4>

```luau
json.raw: (t: JsonData) -> string,
```

</h4>

 encodes a table as json in a condensed fashion for passing as data (without newlines, not as easily readable)

---

## json.decode

<h4>

```luau
json.decode: (json: string) -> JsonData,
```

</h4>

---

## json.readfile

<h4>

```luau
json.readfile: (path: string) -> JsonData,
```

</h4>

---

## json.writefile

<h4>

```luau
json.writefile: (path: string, data: JsonData, options: EncodeOptions?) -> (),
```

</h4>

---

## json.writefile_raw

<h4>

```luau
json.writefile_raw: (path: string, data: JsonData) -> (),
```

</h4>

---

## json.null

<h4>

```luau
json.null: () -> any,
```

</h4>

 returns an object that serializes to json's `null`

---

## json.array

<h4>

```luau
json.array: <T>(t: { T }?) -> { T }
```

</h4>

 Treat `t` as an array (will always encode as `[]` even when empty).

 If `t` isn't provided, returns a new array-like table that will serialize to a json array.

---

## `export type` EncodeOptions

---

## EncodeOptions.pretty

<h4>

```luau
EncodeOptions.pretty: boolean?,
```

</h4>

 with tabs/newlines/spaces for easy reading (default true)

---

## EncodeOptions.sorted

<h4>

```luau
EncodeOptions.sorted: boolean?,
```

</h4>

 sorted in alphanumeric order (default false)

---

## `export type` JsonData

---
