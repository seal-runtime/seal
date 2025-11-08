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

<h3>
```luau
json.encode: (t: JsonData, options: EncodeOptions?) -> string,
```
</h3>

 encodes a table as json; by default this encodes as a pretty-formatted string; use `json.raw` for a condensed version instead

---

<h3>
```luau
json.raw: (t: JsonData) -> string,
```
</h3>

 encodes a table as json in a condensed fashion for passing as data (without newlines, not as easily readable)

---

<h3>
```luau
json.decode: (json: string) -> JsonData,
```
</h3>

---

<h3>
```luau
json.readfile: (path: string) -> JsonData,
```
</h3>

---

<h3>
```luau
json.writefile: (path: string, data: JsonData, options: EncodeOptions?) -> (),
```
</h3>

---

<h3>
```luau
json.writefile_raw: (path: string, data: JsonData) -> (),
```
</h3>

---

<h3>
```luau
json.null: () -> any,
```
</h3>

 returns an object that serializes to json's `null`

---

<h3>
```luau
json.array: <T>(t: { T }?) -> { T }
```
</h3>

 Treat `t` as an array (will always encode as `[]` even when empty).

 If `t` isn't provided, returns a new array-like table that will serialize to a json array.

---

---

<h3>
```luau
EncodeOptions.pretty: boolean?,
```
</h3>

 with tabs/newlines/spaces for easy reading (default true)

---

<h3>
```luau
EncodeOptions.sorted: boolean?,
```
</h3>

 sorted in alphanumeric order (default false)

---

---
