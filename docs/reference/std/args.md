<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# args

`local args = require("@std/args")`

# CLI Argument Parsing

This implementation supports

- a single level of commands (such as `run` in `seal run`),
- positional arguments (`./some/path` in `seal ./some/path`),
- named arguments (`--name=value`),
- flags (--verbose),
- lists of multiple positional arguments, as long as they come after all other arguments

The flags `--help`, its alias `-h`, and `--commands` are reserved for builtins and may not be used.

## Usage

For programs with a single command, use `args.parse(program_name, desc):simple(...)`:

```luau
local parsed = args.parse("scripty", "scripty mc scriptface", {
    description = "runs simple scripts",
    examples = {
        "myscript.script",
        "myscript.script --verbose",
    },
    footer = "github repo: <link>",
}):simple(
    args.positional("script", "the script you want to run, must end in .script")
        :default("scripty.script"),
    args.flag("--verbose", "display debug info alongside the script")
)

local script_path = parsed:expect("script") :: string
if parsed.flags["verbose"] then -- "--verbose" also works
    print("am verbose")
end
```

For programs with multiple commands, use `args.parse(program_name, desc):commands(...)`:

```luau
-- With a default command and validation
local args = require("@std/args")
local err = require("@std/err")

local parsed = args.parse("seal", "the cutest luau runtime", {
    description = "A highly reliable scripting and automation-focused Luau runtime",
    examples = {
        "./myfile.luau" ,
        "run",
        "setup",
    },
    footer = "See the repository at deviaze/seal",
}):commands(
    args.default(
        args.positional("file", "the luau file to run"),
        args.list("args", "arguments to pass to the file")
    ),
    args.command("run", "run the project at the cwd"):args(
        args.list("args", "arguments to pass to project")
    ),
    args.command("setup", "setup typedefs for a new project in the directory at the cwd"):args(
        args.positional("codebase", "codebase style: project or script")
            :validate(function(s): err.Result<string>
                if s == "project" or s == "p" then
                    return "project"
                elseif s == "script" or s == "s" then
                    return "script"
                else
                    return err.message("invalid codebase style")
                end
            end)
    )
)

if parsed.command == "default" then
    local filename = parsed:expect("file") :: string
    local args_to_pass = parsed:get("args") :: { string }
    print(`default filename {filename} with args {table.concat(args_to_pass, ", ")}`)
elseif parsed.command == "run" then
    print(`run with args {table.concat(parsed:get("args") :: { string }, ", ")}`)
elseif parsed.command == "setup" then
    local codebase_style = parsed:get("codebase")
    print(`codebase style {codebase_style}`)
end

-- Without a default command
local parsed = args.parse("lgti", "Let's get that imported!", {
    description = "Add libraries directly from GitHub without a full package management solution.",
    examples = {
        "add 'https://github.com/luaulover/mouseauto.git'",
        "add 'luaulover/mouseauto' mouseauto --folder='./dependencies'",
        "remove mouseauto",
    },
    footer = `See {colors.style.underline("https://github.com/deviaze/lgti")} for support & documentation`
}):commands(
    args.command("add", "Add/import a repository from GitHub"):aliases("a"):args(
        args.positional(
            "repo",
            "Permalink to the GitHub repository to import, can either be the full link or just owner/repo"
        ),
        args.named("--rename", "Rename the repository after importing?"),
        args.named("--src", "Path (relative to repo root) to grab src from")
            :default("./src"),
        args.named("--folder", "Folder in this repository you want to import the repo to")
            :default("./libraries")
    ),
    args.command("remove", "Remove a repository added by lgti"):aliases("r"):args(
        args.positional("name", "Name of the repo to remove")
    )
)

if parsed.command == "add" then
    print("Got command add")
    local repo = parsed:expect("repo") :: string
    print(`Got unparsed repo name {repo}`)
    local project_src = parsed:expect("src")

    local repo_name = "" do
        local repo_splits = str.split(repo, "/")
        repo_name = repo_splits[#repo_splits]
        repo_name = parsed:get("rename", repo_name) :: string
    end
    print(`Got parsed repo name {repo_name}`)

    local folder = parsed:expect("folder")
    print(`Got folder name {folder}`)
elseif parsed.command == "remove" then
    print("Got command remove")
    print(`Got repo name {parsed:expect("repo")}`)
end
```

---

### args.parse

<h4>

```luau
args.parse: (program: string, tagline: string, info: ProgramInfo?) -> {
```

</h4>

---

### args.simple

<h4>

```luau
args.simple: (self: any, ...Arg) -> Parsed,
```

</h4>

 Parse only arguments; pass in args with `args.positional`, `args.flag`, etc.

---

### args.commands

<h4>

```luau
args.commands: (self: any, ...Command) -> Parsed,
```

</h4>

 Parse more than one command; pass in `args.default(...)` and `args.command(...)` to
 generate commands.

---

### args.positional

<h4>

```luau
args.positional: (name: string, help: string) -> Positional,
```

</h4>

 Add a positional argument

---

### args.named

<h4>

```luau
args.named: (name: string, help: string) -> Named,
```

</h4>

 Add a named argument `--name=value` (or when aliased to -n, `-n value`). Named arguments must start with `--`

---

### args.command

<h4>

```luau
args.command: (name: string, help: string) -> Command,
```

</h4>

 Add a new top-level command, must be used with `args.parse(program, desc, info):commands(...)`

---

### args.flag

<h4>

```luau
args.flag: (name: string, help: string) -> Flag,
```

</h4>

 Add a new flag argument like `--verbose` or `--override`. Flags must start with `--` and cannot be `--help` or `--commands`.

---

### args.list

<h4>

```luau
args.list: (name: string, help: string) -> ArgList,
```

</h4>

 Add a new list (tail) argument that collects all remaining positional arguments into a `{ string }`

---

### args.default

<h4>

```luau
args.default: (...Arg) -> Command,
```

</h4>

 Add a default command.

---

## `export type` ProgramInfo

---

### ProgramInfo.description

<h4>

```luau
ProgramInfo.description: string?,
```

</h4>

 if provided, goes below program name/tagline in `--help`

---

### ProgramInfo.examples

<h4>

```luau
ProgramInfo.examples: { string }?,
```

</h4>

 examples of arguments *following* program and path (already pre-filled)

---

### ProgramInfo.footer

<h4>

```luau
ProgramInfo.footer: string?
```

</h4>

 put authors and/or repository link here

---

## `export type` Command

---

### Command.name

<h4>

```luau
Command.name: string,
```

</h4>

---

### Command.is

<h4>

```luau
Command.is: "Command",
```

</h4>

---

### Command.help

<h4>

```luau
Command.help: string,
```

</h4>

---

### Command._args

<h4>

```luau
Command._args: { Arg },
```

</h4>

---

### Command.args

<h4>

```luau
Command.args: (self: Command, ...Arg) -> Command,
```

</h4>

---

### Command._aliases

<h4>

```luau
Command._aliases: { [string]: true? },
```

</h4>

---

### Command.aliases

<h4>

```luau
Command.aliases: (self: Command, ...string) -> Command,
```

</h4>

 Aliases for your command, like `seal r -> seal run`

---

## `export type` Parsed

---

### Parsed.command

<h4>

```luau
Parsed.command: string | "default",
```

</h4>

---

### Parsed.get

<h4>

```luau
Parsed.get: <T>(self: Parsed, name: string, default: T?) -> T?,
```

</h4>

---

### Parsed.expect

<h4>

```luau
Parsed.expect: <T>(self: Parsed, name: string, assertion: string?) -> T,
```

</h4>

---

### Parsed.help

<h4>

```luau
Parsed.help: (self: Parsed) -> string,
```

</h4>

---

### Parsed.flags

<h4>

```luau
Parsed.flags: { [string]: true? },
```

</h4>

---

## `export type` ArgList

---

### ArgList.name

<h4>

```luau
ArgList.name: string,
```

</h4>

---

### ArgList.is

<h4>

```luau
ArgList.is: "ArgList",
```

</h4>

---

### ArgList.help

<h4>

```luau
ArgList.help: string,
```

</h4>

---

### ArgList.values

<h4>

```luau
ArgList.values: { string }?,
```

</h4>

---

## `export type` Validator

---

## `export type` Arg

---

```luau
| Positional
```

---

```luau
| Flag
```

---

```luau
| Named
```

---

```luau
| ArgList
```

---

## `export type` Positional

---

### Positional.name

<h4>

```luau
Positional.name: string,
```

</h4>

---

### Positional.is

<h4>

```luau
Positional.is: "Positional",
```

</h4>

---

### Positional.help

<h4>

```luau
Positional.help: string,
```

</h4>

---

### Positional._default

<h4>

```luau
Positional._default: any,
```

</h4>

---

### Positional.default

<h4>

```luau
Positional.default: (any) -> Positional,
```

</h4>

---

### Positional._optional

<h4>

```luau
Positional._optional: boolean,
```

</h4>

---

### Positional.optional

<h4>

```luau
Positional.optional: (self: Positional) -> Positional,
```

</h4>

 call this to turn the positional argument into an optional positional argument

---

### Positional._validator

<h4>

```luau
Positional._validator: Validator?,
```

</h4>

---

### Positional.validate

<h4>

```luau
Positional.validate: (self: Positional, validator: Validator) -> Positional,
```

</h4>

 validate the argument's input by passing a function that returns either the transformed
 validated input (such as converting input strings from p -> project) or an error object.

---

### Positional.value

<h4>

```luau
Positional.value: any,
```

</h4>

---

## `export type` Flag

---

### Flag.name

<h4>

```luau
Flag.name: string,
```

</h4>

 Must start with `--` and cannot be `--help` or `--commands`

---

### Flag.is

<h4>

```luau
Flag.is: "Flag",
```

</h4>

---

### Flag.help

<h4>

```luau
Flag.help: string,
```

</h4>

---

### Flag._aliases

<h4>

```luau
Flag._aliases: { [string]: true? },
```

</h4>

---

### Flag.aliases

<h4>

```luau
Flag.aliases: (self: Flag, ...string) -> Flag,
```

</h4>

 flag aliases must start with `-` and cannot be `-h` (reserved for help)

---

### Flag._default

<h4>

```luau
Flag._default: boolean?,
```

</h4>

---

### Flag.default

<h4>

```luau
Flag.default: (self: Flag, boolean) -> Flag,
```

</h4>

---

### Flag.value

<h4>

```luau
Flag.value: boolean,
```

</h4>

---

## `export type` Named

---

### Named.name

<h4>

```luau
Named.name: string,
```

</h4>

 Must start with `--` and cannot be `--help` or `--commands`

---

### Named.is

<h4>

```luau
Named.is: "Named",
```

</h4>

---

### Named.help

<h4>

```luau
Named.help: string,
```

</h4>

---

### Named._default

<h4>

```luau
Named._default: any,
```

</h4>

---

### Named.default

<h4>

```luau
Named.default: (self: Named, any) -> Named,
```

</h4>

---

### Named._aliases

<h4>

```luau
Named._aliases: { [string]: true? },
```

</h4>

---

### Named.aliases

<h4>

```luau
Named.aliases: (self: Named, ...string) -> Named,
```

</h4>

 aliases must start with `-` and cannot be `-h` (reserved for help)

---

### Named._required

<h4>

```luau
Named._required: boolean,
```

</h4>

---

### Named.required

<h4>

```luau
Named.required: (self: Named) -> Named,
```

</h4>

---

### Named._validator

<h4>

```luau
Named._validator: Validator?,
```

</h4>

---

### Named.validate

<h4>

```luau
Named.validate: (self: Named, validator: Validator) -> Named,
```

</h4>

---

### Named.value

<h4>

```luau
Named.value: any,
```

</h4>

---
