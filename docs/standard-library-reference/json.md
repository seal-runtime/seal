<!-- markdownlint-disable MD033 -->

# Json

`type json = {`

<details>

<summary> See the docs </summary

Easily manipulate JSON (JavaScript Object Notation) data.

## Usage

```luau
local json = require("@std/json")
local data = json.readfile("./animals.json") :: { cats: number, dogs: number }
data.cats += 1
json.writefile("./animals.json", data)
```

</details>
