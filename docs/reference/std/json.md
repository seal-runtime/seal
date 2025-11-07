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

json.encode: `(t: JsonData, options: EncodeOptions?) -> string`

 encodes a table as json; by default this encodes as a pretty-formatted string; use `json.raw` for a condensed version instead

json.raw: `(t: JsonData) -> string`

 encodes a table as json in a condensed fashion for passing as data (without newlines, not as easily readable)

json.decode: `(json: string) -> JsonData`

json.readfile: `(path: string) -> JsonData`

json.writefile: `(path: string, data: JsonData, options: EncodeOptions?) -> ()`

json.writefile_raw: `(path: string, data: JsonData) -> ()`

json.null: `() -> any`

 returns an object that serializes to json's `null`

json.array: `<T>(t: { T }?) -> { T }`

 Treat `t` as an array (will always encode as `[]` even when empty).

 If `t` isn't provided, returns a new array-like table that will serialize to a json array.

`export type` EncodeOptions

EncodeOptions.pretty: `boolean?`

 with tabs/newlines/spaces for easy reading (default true)

EncodeOptions.sorted: `boolean?`

 sorted in alphanumeric order (default false)

`export type` JsonData
