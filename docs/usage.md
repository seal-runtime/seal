# Usage

To run a codebase at its entry path, use `seal run`.

To run a single file, use `seal ./<filename>.luau`

To evaluate code from a string src, use `seal eval '<src>'`

To create a new codebase at the current directory, use one of:

- `seal setup project` short: `seal sp`
- `seal setup script` short: `seal ss`
- `seal setup custom` short: `seal sc`) (interactive)

For help, use `seal --help` or `seal help <command>`

## Codebases

*seal* codebases can be either *Projects*, *Scripts*, or single files.

The general setup for a codebase should follow:

1. Open a terminal
2. `mkdir/md ProjectName`
3. `cd ProjectName`
4. `seal sp` or `seal ss` (setup a project or script codebase)
5. `code .` or `zeditor .`

### Projects

Use a **Project** codebase when you want to use *seal* as the primary runtime for your project.

This option will generate a `.seal` directory containing seal's configuration and all typedefs locally for easy portability and standalone compilation, a `src` dir, a `.luaurc` Luau configuration file, a `.vscode/settings.json`, and will start a new `git` repository if one doesn't already exist.

### Scripts

Use a **Script** codebase when you want to add *seal* to an existing project to run build, glue, or extension scripts.

This option generates a `.seal` directory locally for seal configuration, but will otherwise link to user-wide typedefs in `~/.seal/typedefs/*`.

Additionally, the project's `.vscode/settings.json` and `.luaurc` will also be created or updated to include *seal*'s typedefs and default config.

#### Configuring codebases

Both Project and Script codebases should have a `.seal/config.luau` file, which you can modify to set a codebase entry path, test runner path, etc.

The default config is:

```luau
local config = {
    entry_path = "./src/main.luau",
    test_path = "./tests/run.luau",
    seal_version = "<SEAL_VERSION_REPLACE>"
}

export type SealConfig = {
    --- Script that `seal run` runs; usually the entrypoint to your codebase.
    --- Defaults to `./src/main.luau`.
    entry_path: string?,
    --- Script that `seal test` runs; usually a test runner.
    test_path: string?,
    --- semver version of seal this project/typedefs expects to run on
    seal_version: string,
}

return config :: SealConfig

```

### Running single files

To run a `.luau` file with seal, use `seal <filename_with_ext>` (like `seal ./get_the_endpoint.luau`).

### Evaluating code from the command line

To evaluate code with seal, use `seal eval '<string src>'`. `seal eval` comes with the `fs`, `http`, and `process` libs loaded in for convenience. An interactive REPL is planned for the future.

## Compiling to a standalone application

*seal* supports limited project compilation. For the vast majority of well-formatted projects, standalone compiliation should work just fine.

Run `seal compile` to compile the seal project at your cwd, `seal compile -o binname` to compile it to a binary called `binname`, or `seal compile -o filename.luau` to bundle the entire codebase into a single Luau file.

Keep in mind that the behavior of `script:path()` *will change* in the bundled application! For example, if you check that `script:path() == script.entry_path` in a file called `setup.luau`, required by `main.luau` in your codebase, it will not trigger when you run `seal run` or `seal ./src/main.luau`, but it will *always* trigger when you run the project once bundled/compiled. Additionally, be careful to package all necessary files non-Luau files alongside your standalone application because calls to `fs.readfile` and similar will not be inlined in the compilation process.

To check if the currently running program is a standalone application or not, use the `@interop/standalone` library.

To ensure your project compiles successfully, make sure each module returns a single value, and that the module's top-level require is completely unindented--fully aligned to the left. After everything's transformed in a module, *seal* goes bottom-to-top, looking for the first `return` that's fully unindented to treat as the module return, replacing it with a `local` variable in the final bundled src.

Dynamic requires nor circular requires are not allowed in bundled or standalone programs.

These should compile correctly:

### mod1.luau

```luau
local mod = {}
-- do things
return mod
```

### mod2.luau

```luau
local function api1()
end
local function api2()
end

return {
    api1 = api1,
    api2 = api2,
}
```

### @somewhere/mod3.luau

```luau
local mod3 = {}
local mod2 = require("./mod2")

local thread = require("@std/thread")

local function createLib()
    -- seal compile inlines thread.spawn path into thread.spawn src
    -- this only works when `path = ` is one line below the call to thread.spawn
    local handle = thread.spawn({
        path = "./somewhere.luau", 
        data = { idk = true },
    })
end

function mod3.api1()
end

return function()
    -- this compiles correctly, because we ignore the return that isn't
    -- fully unindented to the left
    return createLib()
end
```

### main.luau

```luau
local fs = require("@std/fs") -- @std is an internal seal alias
local mod3 = require("@somewhere/mod3")
local mod1 = require("./mod1")
```

If you encounter a syntax error running `seal compile -o binname`, you might have to manually fix the Luau output. To do this, bundle the codebase into a Luau file first with `seal compile -o filename.luau`, fix the errors, then compile the fixed bundled file to a binary.
