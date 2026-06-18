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

<h4>

```luau
function args.parse(program: string, tagline: string, info: ProgramInfo?) -> ArgParser,
```

</h4>

### args.parse

 Parse arguments, call either `:simple` or `:commands` on what this returns.

---

<h4>

```luau
function args.positional(name: string, help: string) -> Positional,
```

</h4>

### args.positional

 Add a positional argument

---

<h4>

```luau
function args.named(name: string, help: string) -> Named,
```

</h4>

### args.named

 Add a named argument `--name=value` (or when aliased to -n, `-n value`). Named arguments must start with `--`

---

<h4>

```luau
function args.command(name: string, help: string) -> Command,
```

</h4>

### args.command

 Add a new top-level command, must be used with `args.parse(program, desc, info):commands(...)`

---

<h4>

```luau
function args.flag(name: string, help: string) -> Flag,
```

</h4>

### args.flag

 Add a new flag argument like `--verbose` or `--override`. Flags must start with `--` and cannot be `--help` or `--commands`.

---

<h4>

```luau
function args.list(name: string, help: string) -> ArgList,
```

</h4>

### args.list

 Add a new list (tail) argument that collects all remaining positional arguments into a `{ string }`

---

<h4>

```luau
function args.default(...Arg) -> Command,
```

</h4>

### args.default

 Add a default command.

```luau
}
```

---

## `export type` ArgParser

<h4>

```luau
export type ArgParser = {
```

</h4>

---

<h4>

```luau
function ArgParser.simple(self: any, ...Arg) -> Parsed,
```

</h4>

#### ArgParser.simple

 Parse only arguments; pass in args with `args.positional`, `args.flag`, etc.

---

<h4>

```luau
function ArgParser.commands(self: any, ...Command) -> Parsed,
```

</h4>

#### ArgParser.commands

 Parse more than one command; pass in `args.default(...)` and `args.command(...)` to
 generate commands.

```luau
}
```

---

## `export type` ProgramInfo

<h4>

```luau
export type ProgramInfo = {
```

</h4>

---

<h4>

```luau
  description: string?,
```

</h4>

#### ProgramInfo.description

 if provided, goes below program name/tagline in `--help`

---

<h4>

```luau
  examples: { string }?,
```

</h4>

#### ProgramInfo.examples

 examples of arguments *following* program and path (already pre-filled)

---

<h4>

```luau
  footer: string?
```

</h4>

#### ProgramInfo.footer

 put authors and/or repository link here

```luau
}
```

---

## `export type` Command

<h4>

```luau
export type Command = {
```

</h4>

---

<h4>

```luau
  name: string,
```

</h4>

#### Command.name

---

<h4>

```luau
  is: "Command",
```

</h4>

#### Command.is

---

<h4>

```luau
  help: string,
```

</h4>

#### Command.help

---

<h4>

```luau
function Command.args(self: Command, ...Arg) -> Command,
```

</h4>

#### Command.args

---

<h4>

```luau
function Command.aliases(self: Command, ...string) -> Command,
```

</h4>

#### Command.aliases

 Aliases for your command, like `seal r -> seal run`

```luau
}
```

---

## `export type` Parsed

<h4>

```luau
export type Parsed = {
```

</h4>

---

<h4>

```luau
  command: string | "default",
```

</h4>

#### Parsed.command

---

<h4>

```luau
function Parsed.get<T>(self: Parsed, name: string, default: T?) -> T?,
```

</h4>

#### Parsed.get

---

<h4>

```luau
function Parsed.expect<T>(self: Parsed, name: string, assertion: string?) -> T,
```

</h4>

#### Parsed.expect

---

<h4>

```luau
function Parsed.help(self: Parsed) -> string,
```

</h4>

#### Parsed.help

---

<h4>

```luau
  flags: { [string]: true? },
```

</h4>

#### Parsed.flags

---

<h4>

```luau
  args: { Arg },
```

</h4>

#### Parsed.args

```luau
}
```

---

## `export type` ArgList

<h4>

```luau
export type ArgList = {
```

</h4>

---

<h4>

```luau
  name: string,
```

</h4>

#### ArgList.name

---

<h4>

```luau
  is: "ArgList",
```

</h4>

#### ArgList.is

---

<h4>

```luau
  help: string,
```

</h4>

#### ArgList.help

---

<h4>

```luau
  values: { string }?,
```

</h4>

#### ArgList.values

```luau
}
```

---

## `export type` Validator

<h4>

```luau
export type Validator = (arg: string) -> any | error
```

</h4>

---

## `export type` Arg

<h4>

```luau
export type Arg =
```

</h4>

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

<h4>

```luau
export type Positional = {
```

</h4>

---

<h4>

```luau
  name: string,
```

</h4>

#### Positional.name

---

<h4>

```luau
  is: "Positional",
```

</h4>

#### Positional.is

---

<h4>

```luau
  help: string,
```

</h4>

#### Positional.help

---

<h4>

```luau
function Positional.default(any) -> Positional,
```

</h4>

#### Positional.default

---

<h4>

```luau
function Positional.optional(self: Positional) -> Positional,
```

</h4>

#### Positional.optional

 call this to turn the positional argument into an optional positional argument

---

<h4>

```luau
function Positional.validate(self: Positional, validator: Validator) -> Positional,
```

</h4>

#### Positional.validate

 validate the argument's input by passing a function that returns either the transformed
 validated input (such as converting input strings from p -> project) or an error object.

---

<h4>

```luau
  value: any,
```

</h4>

#### Positional.value

```luau
}
```

---

## `export type` Flag

<h4>

```luau
export type Flag = {
```

</h4>

---

<h4>

```luau
  name: string,
```

</h4>

#### Flag.name

 Must start with `--` and cannot be `--help` or `--commands`

---

<h4>

```luau
  is: "Flag",
```

</h4>

#### Flag.is

---

<h4>

```luau
  help: string,
```

</h4>

#### Flag.help

---

<h4>

```luau
function Flag.aliases(self: Flag, ...string) -> Flag,
```

</h4>

#### Flag.aliases

 flag aliases must start with `-` and cannot be `-h` (reserved for help)

---

<h4>

```luau
function Flag.default(self: Flag, boolean) -> Flag,
```

</h4>

#### Flag.default

---

<h4>

```luau
  value: boolean,
```

</h4>

#### Flag.value

```luau
}
```

---

## `export type` Named

<h4>

```luau
export type Named = {
```

</h4>

---

<h4>

```luau
  name: string,
```

</h4>

#### Named.name

 Must start with `--` and cannot be `--help` or `--commands`

---

<h4>

```luau
  is: "Named",
```

</h4>

#### Named.is

---

<h4>

```luau
  help: string,
```

</h4>

#### Named.help

---

<h4>

```luau
function Named.default(self: Named, any) -> Named,
```

</h4>

#### Named.default

---

<h4>

```luau
function Named.aliases(self: Named, ...string) -> Named,
```

</h4>

#### Named.aliases

 aliases must start with `-` and cannot be `-h` (reserved for help)

---

<h4>

```luau
function Named.required(self: Named) -> Named,
```

</h4>

#### Named.required

---

<h4>

```luau
function Named.validate(self: Named, validator: Validator) -> Named,
```

</h4>

#### Named.validate

---

<h4>

```luau
  value: any,
```

</h4>

#### Named.value

```luau
}
```

---

Autogenerated from [std/args.luau](/.seal/typedefs/std/args.luau).

*seal* is best experienced with inline, in-editor documentation. Please see the linked typedefs file if this documentation is confusing, too verbose, or inaccurate.
