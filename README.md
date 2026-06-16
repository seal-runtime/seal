# *seal*, make shell scripts readable

<!-- markdownlint-disable MD033 -->
<table>
  <tr>
    <td><img src="assets/seal-smaller.png" width="400" alt="seal mascot reading reference book" /></td>
    <td valign="top">
      <em>seal, the cutest scripting runtime</em><br /><br />
      <strong>Usecases:</strong>
      <ul>
        <li>scripting tool with autocomplete</li>
        <li>faster Python replacement</li>
        <li>write cute TUIs</li>
        <li>quickly deploy on major platforms</li>
      </ul>
      <a href="/docs/usage.md">Usage</a> | <a href="/docs/libraries_and_programming.md">Programming</a> | <a href="/docs/reference/">API Reference</a>
    </td>
  </tr>
</table>
<!-- markdownlint-enable MD033 -->

Use *seal* to write anything from self-contained scripts to cute TUIs to full GUI applications in Luau. *seal* provides a useful set of libraries builtin so you can get to writing dependable, type-safe projects with good inline documentation and tooling.

## Install

Grab the [latest release](https://github.com/deviaze/seal/releases/latest) or check out [these install instructions](docs/install.md) for a detailed walkthrough.

To get started, you just need:

1. A text editor (VSCode, Zed, and nvim are supported by Luau Language Server).
2. [Luau Language Server](https://github.com/JohnnyMorganz/luau-lsp) installed in your editor.
3. The *seal* executable in your `$PATH`

## Usage

To start a new project with *seal*, make a new directory, run `seal setup project` inside it, and open it up with `code .`

- `seal ./filename.luau` runs a Luau file with *seal*.
- `seal run` runs the project at your current working directory.
- `seal compile` bundles and compiles the project at your current working directory into a standalone executable for your platform.

Check out the full [usage instructions](docs/usage.md) for more.

## Programming

Check out [the programming intro](docs/libraries_and_programming.md) to get started
or the [standard library reference](/docs/reference/) for all current features and APIs.

A quick example (calling an API with an API key):

```luau
local args = require("@std/args")
local vars = require("@std/env/vars")
local http = require("@std/net/http")
local json = require("@std/serde/json")
local colors = require("@std/io/colors")

-- automatically loads from .env
local API_KEY = vars.get("SOME_API_KEY") or error("can't find api key")
local OUTPUT_FILE = "endpoint_results.json"

local parsed = args.parse("endpointer", "get the api"):simple(
    args.positional("name", "what param name to search for"),
    args.positional("value", "parameter value")
)

local name = parsed:expect<<string>>("name", "missing parameter name")
local value = parsed:expect<<string>>("value", "missing parameter value")

local response = http.get({
    url = "https://some_endpoint.dev/api",
    params = {
        [name] = value
    },
    headers = {
        Authorization = `Bearer {API_KEY}`
    }
})
type EndpointResult = {
    detections: { string },
    updated: string,
}

if response.ok then
    local contents = response:expect_json<<EndpointResult>>()
    json.writefile(OUTPUT_FILE, contents :: json.JsonData, {
        pretty = false,
        sorted = false,
    })
    print(`Saved {colors.blue(tostring(#contents.detections))} detections to {colors.green(OUTPUT_FILE)}`)
elseif response.status.code >= 400 and response.status.code < 500 then
    error("invalid parameter name or value?")
end
```

## Roadmap

- More featureful `@extra` library.
- Custom and customizable `seal setup` scripts.
- Ecosystem of [external libraries](/docs/external_libraries.md) to expand *seal*'s functionality with native bindings.

## Community

[Join the Discord](https://discord.gg/3MJ37CFNWh) if you need help or want to contribute!
