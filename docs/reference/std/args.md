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

<h3>

```luau
args.parse: (program: string, tagline: string, info: ProgramInfo?) -> {
```

</h3>

---

<h3>

```luau
args.simple: (self: any, ...Arg) -> Parsed,
```

</h3>

 Parse only arguments; pass in args with `args.positional`, `args.flag`, etc.

---

<h3>

```luau
args.commands: (self: any, ...Command) -> Parsed,
```

</h3>

 Parse more than one command; pass in `args.default(...)` and `args.command(...)` to
 generate commands.

---

<h3>

```luau
args.positional: (name: string, help: string) -> Positional,
```

</h3>

 Add a positional argument

---

<h3>

```luau
args.named: (name: string, help: string) -> Named,
```

</h3>

 Add a named argument `--name=value` (or when aliased to -n, `-n value`). Named arguments must start with `--`

---

<h3>

```luau
args.command: (name: string, help: string) -> Command,
```

</h3>

 Add a new top-level command, must be used with `args.parse(program, desc, info):commands(...)`

---

<h3>

```luau
args.flag: (name: string, help: string) -> Flag,
```

</h3>

 Add a new flag argument like `--verbose` or `--override`. Flags must start with `--` and cannot be `--help` or `--commands`.

---

<h3>

```luau
args.list: (name: string, help: string) -> ArgList,
```

</h3>

 Add a new list (tail) argument that collects all remaining positional arguments into a `{ string }`

---

<h3>

```luau
args.default: (...Arg) -> Command,
```

</h3>

 Add a default command.

---

---

<h3>

```luau
ProgramInfo.description: string?,
```

</h3>

 if provided, goes below program name/tagline in `--help`

---

<h3>

```luau
ProgramInfo.examples: { string }?,
```

</h3>

 examples of arguments *following* program and path (already pre-filled)

---

<h3>

```luau
ProgramInfo.footer: string?
```

</h3>

 put authors and/or repository link here

---

---

<h3>

```luau
Command.name: string,
```

</h3>

---

<h3>

```luau
Command.is: "Command",
```

</h3>

---

<h3>

```luau
Command.help: string,
```

</h3>

---

<h3>

```luau
Command._args: { Arg },
```

</h3>

---

<h3>

```luau
Command.args: (self: Command, ...Arg) -> Command,
```

</h3>

---

<h3>

```luau
Command._aliases: { [string]: true? },
```

</h3>

---

<h3>

```luau
Command.aliases: (self: Command, ...string) -> Command,
```

</h3>

 Aliases for your command, like `seal r -> seal run`

---

---

<h3>

```luau
Parsed.command: string | "default",
```

</h3>

---

<h3>

```luau
Parsed.get: <T>(self: Parsed, name: string, default: T?) -> T?,
```

</h3>

---

<h3>

```luau
Parsed.expect: <T>(self: Parsed, name: string, assertion: string?) -> T,
```

</h3>

---

<h3>

```luau
Parsed.help: (self: Parsed) -> string,
```

</h3>

---

<h3>

```luau
Parsed.flags: { [string]: true? },
```

</h3>

---

---

<h3>

```luau
ArgList.name: string,
```

</h3>

---

<h3>

```luau
ArgList.is: "ArgList",
```

</h3>

---

<h3>

```luau
ArgList.help: string,
```

</h3>

---

<h3>

```luau
ArgList.values: { string }?,
```

</h3>

---

---

---

<h3>

```luau
| Positional
```

</h3>

---

<h3>

```luau
| Flag
```

</h3>

---

<h3>

```luau
| Named
```

</h3>

---

<h3>

```luau
| ArgList
```

</h3>

---

---

<h3>

```luau
Positional.name: string,
```

</h3>

---

<h3>

```luau
Positional.is: "Positional",
```

</h3>

---

<h3>

```luau
Positional.help: string,
```

</h3>

---

<h3>

```luau
Positional._default: any,
```

</h3>

---

<h3>

```luau
Positional.default: (any) -> Positional,
```

</h3>

---

<h3>

```luau
Positional._optional: boolean,
```

</h3>

---

<h3>

```luau
Positional.optional: (self: Positional) -> Positional,
```

</h3>

 call this to turn the positional argument into an optional positional argument

---

<h3>

```luau
Positional._validator: Validator?,
```

</h3>

---

<h3>

```luau
Positional.validate: (self: Positional, validator: Validator) -> Positional,
```

</h3>

 validate the argument's input by passing a function that returns either the transformed
 validated input (such as converting input strings from p -> project) or an error object.

---

<h3>

```luau
Positional.value: any,
```

</h3>

---

---

<h3>

```luau
Flag.name: string,
```

</h3>

 Must start with `--` and cannot be `--help` or `--commands`

---

<h3>

```luau
Flag.is: "Flag",
```

</h3>

---

<h3>

```luau
Flag.help: string,
```

</h3>

---

<h3>

```luau
Flag._aliases: { [string]: true? },
```

</h3>

---

<h3>

```luau
Flag.aliases: (self: Flag, ...string) -> Flag,
```

</h3>

 flag aliases must start with `-` and cannot be `-h` (reserved for help)

---

<h3>

```luau
Flag._default: boolean?,
```

</h3>

---

<h3>

```luau
Flag.default: (self: Flag, boolean) -> Flag,
```

</h3>

---

<h3>

```luau
Flag.value: boolean,
```

</h3>

---

---

<h3>

```luau
Named.name: string,
```

</h3>

 Must start with `--` and cannot be `--help` or `--commands`

---

<h3>

```luau
Named.is: "Named",
```

</h3>

---

<h3>

```luau
Named.help: string,
```

</h3>

---

<h3>

```luau
Named._default: any,
```

</h3>

---

<h3>

```luau
Named.default: (self: Named, any) -> Named,
```

</h3>

---

<h3>

```luau
Named._aliases: { [string]: true? },
```

</h3>

---

<h3>

```luau
Named.aliases: (self: Named, ...string) -> Named,
```

</h3>

 aliases must start with `-` and cannot be `-h` (reserved for help)

---

<h3>

```luau
Named._required: boolean,
```

</h3>

---

<h3>

```luau
Named.required: (self: Named) -> Named,
```

</h3>

---

<h3>

```luau
Named._validator: Validator?,
```

</h3>

---

<h3>

```luau
Named.validate: (self: Named, validator: Validator) -> Named,
```

</h3>

---

<h3>

```luau
Named.value: any,
```

</h3>

---
