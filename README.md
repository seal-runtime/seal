# *seal*, make shell scripts readable

<!-- markdownlint-disable MD033 -->
<div align="center">
    <img src="assets/seal-smaller.png" width="320" alt="seal mascot reading reference book" /><br />
    <em>the cutest scripting runtime</em><br />
    <!-- Start-Precommit-Marker-2 --><img src="https://img.shields.io/badge/seal-0.0.8--rc.2-f0f8ff" alt="seal version" /><!-- End-Precommit-Marker-2 --> <!-- Start-Precommit-Marker-3 --><img src="https://img.shields.io/badge/Luau-0.715-4f99ba" alt="Luau version" /><!-- End-Precommit-Marker-3 --><br />
    <a href="/docs/usage.md">Usage</a> | <a href="/docs/libraries_and_programming.md">Programming</a> | <a href="/docs/reference/">API Reference</a>
</div>

**Usecases:**

- cross-platform scripting tool with autocomplete
- automation & task runner
- faster Python replacement
- write cute TUIs
- quickly deploy to major platforms
- data is pretty—just print it (colors included)
<!-- markdownlint-enable MD033 -->

*seal* provides a useful set of libraries builtin so you can start writing dependable, type-safe projects with good inline documentation and tooling.

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

## Reliability

*seal* should be consistent, reliable, and have an easy-to-understand API. In most cases, runtime errors will be expressly documented or returned as an `error` type to facilitate nonthrowing error handling. In any case, if you encounter an error at runtime, you should easily be able to resolve it quickly thanks to *seal*'s <!-- Start-Precommit-Marker-1 -->1089<!-- End-Precommit-Marker-1 --> handcrafted error messages.

If you encounter a bug, panic, or security vulnerability, please make an issue in this repo right away; you may attach a repro or send one privately to [dev@deviaze.com](mailto:dev@deviaze.com) or `@deviaze` on Discord.

## Roadmap

- More featureful `@extra` library.
- Custom and customizable `seal setup` scripts.
- Ecosystem of [external libraries](/docs/external_libraries.md) to expand *seal*'s functionality with native bindings.

## Community

[Join the Discord](https://discord.gg/3MJ37CFNWh) if you need help or want to contribute!
