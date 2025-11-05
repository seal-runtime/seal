<!-- markdownlint-disable MD033 -->

# Json

` pretty: boolean?`

` sorted: boolean?`

`type json = {`

[=[
Easily manipulate JSON (JavaScript Object Notation) data.

## Usage

```luau
local json = require("@std/json")
local data = json.readfile("./animals.json") :: { cats: number, dogs: number }
data.cats += 1
json.writefile("./animals.json", data)
```

`function JsonData.encode(t: JsonData, options: EncodeOptions?): string`

`function JsonData.raw(t: JsonData): string`

`function JsonData.null(): any`

`function JsonData.array<T>(t: { T }?): { T }`
