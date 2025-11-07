<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# json

`local json = require("@std/json")`

$hspace{5pt}$Easily manipulate JSON (JavaScript Object Notation) data.
$hspace{5pt}$
$hspace{5pt}$## Usage
$hspace{5pt}$```luau
$hspace{5pt}$local json = require("@std/json")
$hspace{5pt}$local data = json.readfile("./animals.json") :: { cats: number, dogs: number }
$hspace{5pt}$data.cats += 1
$hspace{5pt}$json.writefile("./animals.json", data)
$hspace{5pt}$```

json.encode: `(t: JsonData, options: EncodeOptions?) -> string`

$hspace{5pt}$ encodes a table as json; by default this encodes as a pretty-formatted string; use `json.raw` for a condensed version instead

json.raw: `(t: JsonData) -> string`

$hspace{5pt}$ encodes a table as json in a condensed fashion for passing as data (without newlines, not as easily readable)

json.decode: `(json: string) -> JsonData`

json.readfile: `(path: string) -> JsonData`

json.writefile: `(path: string, data: JsonData, options: EncodeOptions?) -> ()`

json.writefile_raw: `(path: string, data: JsonData) -> ()`

json.null: `() -> any`

$hspace{5pt}$ returns an object that serializes to json's `null`

json.array: `<T>(t: { T }?) -> { T }`

$hspace{5pt}$ Treat `t` as an array (will always encode as `[]` even when empty).
$hspace{5pt}$
$hspace{5pt}$ If `t` isn't provided, returns a new array-like table that will serialize to a json array.

`export type` EncodeOptions

EncodeOptions.pretty: `boolean?`

$hspace{5pt}$ with tabs/newlines/spaces for easy reading (default true)

EncodeOptions.sorted: `boolean?`

$hspace{5pt}$ sorted in alphanumeric order (default false)

`export type` JsonData
