<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# io.prompt

`local prompt = require("@std/io/prompt")`

$hspace{5pt}$Prompt users for personal information :)
$hspace{5pt}$
$hspace{5pt}$## Usage
$hspace{5pt}$
$hspace{5pt}$```luau
$hspace{5pt}$local prompt = require("@std/io/prompt")
$hspace{5pt}$
$hspace{5pt}$-- confirm (defaults to true)
$hspace{5pt}$if prompt.confirm(`Switch to branch {branch}`) then
$hspace{5pt}$    -- displays "Switch to branch branchname (Y/n): "
$hspace{5pt}$    switch_branch(branch)
$hspace{5pt}$end
$hspace{5pt}$
$hspace{5pt}$-- Ask the user to provide one line of arbitrary text, supporting normal text buffer operations:
$hspace{5pt}$local result = prompt.text("What's your name?")
$hspace{5pt}$-- displays "What's your name?: "
$hspace{5pt}$```

prompt.text: `(message: string) -> string`

<details>

<summary> See the docs </summary

$hspace{5pt}$Ask the user to provide one line of text, trimming the resulting text on both ends and supporting normal text buffer operations.
$hspace{5pt}$
$hspace{5pt}$- A colon and a space `": "` will be appended to the passed `message` if it isn't an empty string and doesn't already contain `": "`
$hspace{5pt}$- If `message` already contains `": "`, *seal* will assume you've already typed your entire prompt (including punctuation) and will display your prompt unchanged.
$hspace{5pt}$- If you don't want this behavior for any reason but still want sane text buffer handling, use `io.output.write(your_message)` and then `prompt.text("")`
$hspace{5pt}$- Falls back to `io.input.rawline` semantics if called in a non-TTY (like a piped child process)
$hspace{5pt}$
$hspace{5pt}$If you want to apply *custom validation*, use `prompt.validate` instead!
$hspace{5pt}$
$hspace{5pt}$## Usage
$hspace{5pt}$
$hspace{5pt}$```luau
$hspace{5pt}$local name = prompt.text("What's your name?")
$hspace{5pt}$-- displays "What's your name?: "
$hspace{5pt}$if name == "deviaze" then
$hspace{5pt}$    print("no that's me")
$hspace{5pt}$end
$hspace{5pt}$
$hspace{5pt}$-- a very bold question
$hspace{5pt}$local ssn = prompt.text(colors.bold.white("whats your ssn???: "))
$hspace{5pt}$-- seal doesn't display an extra `: ` after your message
$hspace{5pt}$```

</details>

prompt.confirm: `(message: string, default: boolean?) -> boolean`

<details>

<summary> See the docs </summary

$hspace{5pt}$Ask the user to confirm an action, defaulting to `true`, and displaying a y/n after the `message` prompt according to the usual CLI application conventions.
$hspace{5pt}$
$hspace{5pt}$- If `default` is unspecified or `true`, displays `[Y/n]:` after the message, demonstrating that pressing <kbd>Enter</kbd> signifies Yes.
$hspace{5pt}$- If `default` is `false,` displays `[y/N]:` after the message, demonstrating that pressing <kbd>Enter</kbd> signifies No.
$hspace{5pt}$
$hspace{5pt}$To confirm, the user may send any of `Y`, `y`, `n`, or `N` to explicitly signify confirmation/denial or nothing (just <kbd>Enter</kbd>) for the default.
$hspace{5pt}$
$hspace{5pt}$## Usage
$hspace{5pt}$
$hspace{5pt}$```luau
$hspace{5pt}$if prompt.confirm("are roses red") then
$hspace{5pt}$    -- displays "are roses red [Y/n]: "
$hspace{5pt}$    print("violets are blue")
$hspace{5pt}$end
$hspace{5pt}$```

</details>

prompt.validate: `(message: string, v: (response: string) -> true | string) -> string`

<details>

<summary> See the docs </summary

$hspace{5pt}$Like `prompt.text`, but validates the response with a custom validation function.
$hspace{5pt}$
$hspace{5pt}$- Return a `string` to tell users why validation failed.
$hspace{5pt}$- Retries until `v(response)` returns `true`.
$hspace{5pt}$
$hspace{5pt}$## Usage
$hspace{5pt}$
$hspace{5pt}$```luau
$hspace{5pt}$local response = prompt.validate("Favorite animal that starts with 's'",
$hspace{5pt}$    function(response: string): true | string
$hspace{5pt}$        return
$hspace{5pt}$            if response == "seal" then
$hspace{5pt}$                true
$hspace{5pt}$            elseif not str.startswith(response, "s") then
$hspace{5pt}$                `'{response}' does not start with s!`
$hspace{5pt}$            else "nope not the answer :)"
$hspace{5pt}$    end
$hspace{5pt}$)
$hspace{5pt}$```

</details>

prompt.password: `(message: string, style: "Hidden" | "*"?) -> string`

<details>

<summary> See the docs </summary

$hspace{5pt}$Prompts a hidden password prompt.
$hspace{5pt}$
$hspace{5pt}$By default, any characters typed in the password prompt will be fully hidden according to unix conventions.
$hspace{5pt}$
$hspace{5pt}$Note that users might get confused when they try typing in a password and nothing shows up. That's why *seal* also
$hspace{5pt}$provides the `"*"` option, which replaces characters typed with astricks.
$hspace{5pt}$
$hspace{5pt}$- When `message` isn't an empty string and `style == "Hidden"`, displays `(hidden):` after the prompt.
$hspace{5pt}$- If `message` already contains a colon (`:`), *seal* will not display `(hidden):` after the prompt.
$hspace{5pt}$- If `message` is an empty string, no visual indicator will be displayed. Use this with `io.output.write`
$hspace{5pt}$to override the default behavior for custom password prompts.
$hspace{5pt}$
$hspace{5pt}$## ⚠️ Safety
$hspace{5pt}$
$hspace{5pt}$- Do not use when rawmode (`input.rawmode`) is already enabled.
$hspace{5pt}$- Do not use in multithreaded programs while another thread may be reading from stdin.
$hspace{5pt}$- Using the default `"Hidden"` mode might be slightly more secure;
$hspace{5pt}$seal makes a best-effort attempt to clean up/erase passwords in astrick mode
$hspace{5pt}$but it may not be perfectly safe from introspection.

</details>

prompt.pick: `(message: string, options: { string }, default: number?) -> number`

<details>

<summary> See the docs </summary

$hspace{5pt}$Prompts users to pick one of a list of options by index.
$hspace{5pt}$
$hspace{5pt}$- If `default` is unspecified, assumes no default and retries until the user picks something.
$hspace{5pt}$- Automatically retries until the user provides a valid option.
$hspace{5pt}$- Handles `message` like `prompt.text`, appending `": "` to the prompt message if it isn't an empty string and doesn't already include `": "`.
$hspace{5pt}$
$hspace{5pt}$## Usage
$hspace{5pt}$
$hspace{5pt}$```luau
$hspace{5pt}$local opt = prompt.pick("Pick an editor", {"vscode/code", "zed", "nvim"}, 1)
$hspace{5pt}$if opt == 1 then
$hspace{5pt}$    -- handle vscode stuff
$hspace{5pt}$elseif opt == 2 then
$hspace{5pt}$    -- handle zed stuff
$hspace{5pt}$elseif opt == 3 then
$hspace{5pt}$    -- handle nvim stuff
$hspace{5pt}$end
$hspace{5pt}$
$hspace{5pt}$-- or dynamically
$hspace{5pt}$local file = files[prompt.pick("Pick a file", files)]
$hspace{5pt}$```

</details>
