<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# args

`local args = require("@std/args")`

$hspace{5pt}$# CLI Argument Parsing
$hspace{5pt}$
$hspace{5pt}$This implementation supports
$hspace{5pt}$- a single level of commands (such as `run` in `seal run`),
$hspace{5pt}$- positional arguments (`./some/path` in `seal ./some/path`),
$hspace{5pt}$- named arguments (`--name=value`),
$hspace{5pt}$- flags (--verbose),
$hspace{5pt}$- lists of multiple positional arguments, as long as they come after all other arguments
$hspace{5pt}$
$hspace{5pt}$The flags `--help`, its alias `-h`, and `--commands` are reserved for builtins and may not be used.
$hspace{5pt}$
$hspace{5pt}$## Usage
$hspace{5pt}$
$hspace{5pt}$For programs with a single command, use `args.parse(program_name, desc):simple(...)`:
$hspace{5pt}$
$hspace{5pt}$```luau
$hspace{5pt}$local parsed = args.parse("scripty", "scripty mc scriptface", {
$hspace{5pt}$    description = "runs simple scripts",
$hspace{5pt}$    examples = {
$hspace{5pt}$        "myscript.script",
$hspace{5pt}$        "myscript.script --verbose",
$hspace{5pt}$    },
$hspace{5pt}$    footer = "github repo: <link>",
$hspace{5pt}$}):simple(
$hspace{5pt}$    args.positional("script", "the script you want to run, must end in .script")
$hspace{5pt}$        :default("scripty.script"),
$hspace{5pt}$    args.flag("--verbose", "display debug info alongside the script")
$hspace{5pt}$)
$hspace{5pt}$
$hspace{5pt}$local script_path = parsed:expect("script") :: string
$hspace{5pt}$if parsed.flags["verbose"] then -- "--verbose" also works
$hspace{5pt}$    print("am verbose")
$hspace{5pt}$end
$hspace{5pt}$```
$hspace{5pt}$
$hspace{5pt}$For programs with multiple commands, use `args.parse(program_name, desc):commands(...)`:
$hspace{5pt}$
$hspace{5pt}$```luau
$hspace{5pt}$    -- With a default command and validation
$hspace{5pt}$    local args = require("@std/args")
$hspace{5pt}$    local err = require("@std/err")
$hspace{5pt}$
$hspace{5pt}$    local parsed = args.parse("seal", "the cutest luau runtime", {
$hspace{5pt}$        description = "A highly reliable scripting and automation-focused Luau runtime",
$hspace{5pt}$        examples = {
$hspace{5pt}$            "./myfile.luau" ,
$hspace{5pt}$            "run",
$hspace{5pt}$            "setup",
$hspace{5pt}$        },
$hspace{5pt}$        footer = "See the repository at deviaze/seal",
$hspace{5pt}$    }):commands(
$hspace{5pt}$        args.default(
$hspace{5pt}$            args.positional("file", "the luau file to run"),
$hspace{5pt}$            args.list("args", "arguments to pass to the file")
$hspace{5pt}$        ),
$hspace{5pt}$        args.command("run", "run the project at the cwd"):args(
$hspace{5pt}$            args.list("args", "arguments to pass to project")
$hspace{5pt}$        ),
$hspace{5pt}$        args.command("setup", "setup typedefs for a new project in the directory at the cwd"):args(
$hspace{5pt}$            args.positional("codebase", "codebase style: project or script")
$hspace{5pt}$                :validate(function(s): err.Result<string>
$hspace{5pt}$                    if s == "project" or s == "p" then
$hspace{5pt}$                        return "project"
$hspace{5pt}$                    elseif s == "script" or s == "s" then
$hspace{5pt}$                        return "script"
$hspace{5pt}$                    else
$hspace{5pt}$                        return err.message("invalid codebase style")
$hspace{5pt}$                    end
$hspace{5pt}$                end)
$hspace{5pt}$        )
$hspace{5pt}$    )
$hspace{5pt}$
$hspace{5pt}$    if parsed.command == "default" then
$hspace{5pt}$        local filename = parsed:expect("file") :: string
$hspace{5pt}$        local args_to_pass = parsed:get("args") :: { string }
$hspace{5pt}$        print(`default filename {filename} with args {table.concat(args_to_pass, ", ")}`)
$hspace{5pt}$    elseif parsed.command == "run" then
$hspace{5pt}$        print(`run with args {table.concat(parsed:get("args") :: { string }, ", ")}`)
$hspace{5pt}$    elseif parsed.command == "setup" then
$hspace{5pt}$        local codebase_style = parsed:get("codebase")
$hspace{5pt}$        print(`codebase style {codebase_style}`)
$hspace{5pt}$    end
$hspace{5pt}$
$hspace{5pt}$    -- Without a default command
$hspace{5pt}$    local parsed = args.parse("lgti", "Let's get that imported!", {
$hspace{5pt}$        description = "Add libraries directly from GitHub without a full package management solution.",
$hspace{5pt}$        examples = {
$hspace{5pt}$            "add 'https://github.com/luaulover/mouseauto.git'",
$hspace{5pt}$            "add 'luaulover/mouseauto' mouseauto --folder='./dependencies'",
$hspace{5pt}$            "remove mouseauto",
$hspace{5pt}$        },
$hspace{5pt}$        footer = `See {colors.style.underline("https://github.com/deviaze/lgti")} for support & documentation`
$hspace{5pt}$    }):commands(
$hspace{5pt}$        args.command("add", "Add/import a repository from GitHub"):aliases("a"):args(
$hspace{5pt}$            args.positional(
$hspace{5pt}$                "repo",
$hspace{5pt}$                "Permalink to the GitHub repository to import, can either be the full link or just owner/repo"
$hspace{5pt}$            ),
$hspace{5pt}$            args.named("--rename", "Rename the repository after importing?"),
$hspace{5pt}$            args.named("--src", "Path (relative to repo root) to grab src from")
$hspace{5pt}$                :default("./src"),
$hspace{5pt}$            args.named("--folder", "Folder in this repository you want to import the repo to")
$hspace{5pt}$                :default("./libraries")
$hspace{5pt}$        ),
$hspace{5pt}$        args.command("remove", "Remove a repository added by lgti"):aliases("r"):args(
$hspace{5pt}$            args.positional("name", "Name of the repo to remove")
$hspace{5pt}$        )
$hspace{5pt}$    )
$hspace{5pt}$
$hspace{5pt}$    if parsed.command == "add" then
$hspace{5pt}$        print("Got command add")
$hspace{5pt}$        local repo = parsed:expect("repo") :: string
$hspace{5pt}$        print(`Got unparsed repo name {repo}`)
$hspace{5pt}$        local project_src = parsed:expect("src")
$hspace{5pt}$
$hspace{5pt}$        local repo_name = "" do
$hspace{5pt}$            local repo_splits = str.split(repo, "/")
$hspace{5pt}$            repo_name = repo_splits[#repo_splits]
$hspace{5pt}$            repo_name = parsed:get("rename", repo_name) :: string
$hspace{5pt}$        end
$hspace{5pt}$        print(`Got parsed repo name {repo_name}`)
$hspace{5pt}$
$hspace{5pt}$        local folder = parsed:expect("folder")
$hspace{5pt}$        print(`Got folder name {folder}`)
$hspace{5pt}$    elseif parsed.command == "remove" then
$hspace{5pt}$        print("Got command remove")
$hspace{5pt}$        print(`Got repo name {parsed:expect("repo")}`)
$hspace{5pt}$    end
$hspace{5pt}$```

args.parse: `(program: string, tagline: string, info: ProgramInfo?) -> {`

args.simple: `(self: any, ...Arg) -> Parsed`

$hspace{5pt}$ Parse only arguments; pass in args with `args.positional`, `args.flag`, etc.

args.commands: `(self: any, ...Command) -> Parsed`

$hspace{5pt}$ Parse more than one command; pass in `args.default(...)` and `args.command(...)` to
$hspace{5pt}$ generate commands.

args.positional: `(name: string, help: string) -> Positional`

$hspace{5pt}$ Add a positional argument

args.named: `(name: string, help: string) -> Named`

$hspace{5pt}$ Add a named argument `--name=value` (or when aliased to -n, `-n value`). Named arguments must start with `--`

args.command: `(name: string, help: string) -> Command`

$hspace{5pt}$ Add a new top-level command, must be used with `args.parse(program, desc, info):commands(...)`

args.flag: `(name: string, help: string) -> Flag`

$hspace{5pt}$ Add a new flag argument like `--verbose` or `--override`. Flags must start with `--` and cannot be `--help` or `--commands`.

args.list: `(name: string, help: string) -> ArgList`

$hspace{5pt}$ Add a new list (tail) argument that collects all remaining positional arguments into a `{ string }`

args.default: `(...Arg) -> Command`

$hspace{5pt}$ Add a default command.

`export type` ProgramInfo

ProgramInfo.description: `string?`

$hspace{5pt}$ if provided, goes below program name/tagline in `--help`

ProgramInfo.examples: `{ string }?`

$hspace{5pt}$ examples of arguments *following* program and path (already pre-filled)

ProgramInfo.footer: `string?`

$hspace{5pt}$ put authors and/or repository link here

`export type` Command

Command.name: `string`

Command.is: `"Command"`

Command.help: `string`

Command._args: `{ Arg }`

Command.args: `(self: Command, ...Arg) -> Command`

Command._aliases: `{ [string]: true? }`

Command.aliases: `(self: Command, ...string) -> Command`

$hspace{5pt}$ Aliases for your command, like `seal r -> seal run`

`export type` Parsed

Parsed.command: `string | "default"`

Parsed.get: `<T>(self: Parsed, name: string, default: T?) -> T?`

Parsed.expect: `<T>(self: Parsed, name: string, assertion: string?) -> T`

Parsed.help: `(self: Parsed) -> string`

Parsed.flags: `{ [string]: true? }`

`export type` ArgList

ArgList.name: `string`

ArgList.is: `"ArgList"`

ArgList.help: `string`

ArgList.values: `{ string }?`

`export type` Validator

`export type` Arg

`export type` Positional

Positional.name: `string`

Positional.is: `"Positional"`

Positional.help: `string`

Positional._default: `any`

Positional.default: `(any) -> Positional`

Positional._optional: `boolean`

Positional.optional: `(self: Positional) -> Positional`

$hspace{5pt}$ call this to turn the positional argument into an optional positional argument

Positional._validator: `Validator?`

Positional.validate: `(self: Positional, validator: Validator) -> Positional`

$hspace{5pt}$ validate the argument's input by passing a function that returns either the transformed
$hspace{5pt}$ validated input (such as converting input strings from p -> project) or an error object.

Positional.value: `any`

`export type` Flag

Flag.name: `string`

$hspace{5pt}$ Must start with `--` and cannot be `--help` or `--commands`

Flag.is: `"Flag"`

Flag.help: `string`

Flag._aliases: `{ [string]: true? }`

Flag.aliases: `(self: Flag, ...string) -> Flag`

$hspace{5pt}$ flag aliases must start with `-` and cannot be `-h` (reserved for help)

Flag._default: `boolean?`

Flag.default: `(self: Flag, boolean) -> Flag`

Flag.value: `boolean`

`export type` Named

Named.name: `string`

$hspace{5pt}$ Must start with `--` and cannot be `--help` or `--commands`

Named.is: `"Named"`

Named.help: `string`

Named._default: `any`

Named.default: `(self: Named, any) -> Named`

Named._aliases: `{ [string]: true? }`

Named.aliases: `(self: Named, ...string) -> Named`

$hspace{5pt}$ aliases must start with `-` and cannot be `-h` (reserved for help)

Named._required: `boolean`

Named.required: `(self: Named) -> Named`

Named._validator: `Validator?`

Named.validate: `(self: Named, validator: Validator) -> Named`

Named.value: `any`
