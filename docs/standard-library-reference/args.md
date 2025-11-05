<!-- markdownlint-disable MD033 -->

# Args

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

`function args.simple(self: any, ...Arg): Parsed`

`function args.commands(self: any, ...Command): Parsed`

`function args.positional(name: string, help: string): Positional`

`function args.named(name: string, help: string): Named`

`function args.command(name: string, help: string): Command`

`function args.flag(name: string, help: string): Flag`

`function args.list(name: string, help: string): ArgList`

`function args.default(...Arg): Command`

`description: string?`

`examples: { string }?`

`footer: string?`

`function Command.aliases(self: Command, ...string): Command`

`function Positional.optional(self: Positional): Positional`

`function Positional.validate(self: Positional, validator: Validator): Positional`

`name: string`

`function Flag.aliases(self: Flag, ...string): Flag`

`name: string`

`function Named.aliases(self: Named, ...string): Named`
