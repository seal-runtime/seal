<!-- markdownlint-disable MD033 -->

# Json

` pretty: boolean?`

 with tabs/newlines/spaces for easy reading (default true)

` sorted: boolean?`

 sorted in alphanumeric order (default false)

`type json = {`

Easily manipulate JSON (JavaScript Object Notation) data.

## Usage

```luau
local json = require("@std/json")
local data = json.readfile("./animals.json") :: { cats: number, dogs: number }
data.cats += 1
json.writefile("./animals.json", data)
```

`function JsonData.encode(t: JsonData, options: EncodeOptions?): string`

 encodes a table as json; by default this encodes as a pretty-formatted string; use `json.raw` for a condensed version instead

`function JsonData.raw(t: JsonData): string`

 encodes a table as json in a condensed fashion for passing as data (without newlines, not as easily readable)

`function JsonData.null(): any`

 returns an object that serializes to json's `null`

`function JsonData.array<T>(t: { T }?): { T }`

 Treat `t` as an array (will always encode as `[]` even when empty).

 If `t` isn't provided, returns a new array-like table that will serialize to a json array.
