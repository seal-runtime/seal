<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# io.prompt

`local prompt = require("@std/io/prompt")`

Prompt users for personal information :)

## Usage

```luau
local prompt = require("@std/io/prompt")

-- confirm (defaults to true)
if prompt.confirm(`Switch to branch {branch}`) then
    -- displays "Switch to branch branchname (Y/n): "
    switch_branch(branch)
end

-- Ask the user to provide one line of arbitrary text, supporting normal text buffer operations:
local result = prompt.text("What's your name?")
-- displays "What's your name?: "
```

---

### prompt.text

<h4>

```luau
function prompt.text(message: string) -> string,
```

</h4>

<details>

<summary> See the docs </summary

Ask the user to provide one line of text, trimming the resulting text on both ends and supporting normal text buffer operations.

- A colon and a space `": "` will be appended to the passed `message` if it isn't an empty string and doesn't already contain `": "`
- If `message` already contains `": "`, *seal* will assume you've already typed your entire prompt (including punctuation) and will display your prompt unchanged.
- If you don't want this behavior for any reason but still want sane text buffer handling, use `io.output.write(your_message)` and then `prompt.text("")`
- Falls back to `io.input.rawline` semantics if called in a non-TTY (like a piped child process)

If you want to apply *custom validation*, use `prompt.validate` instead!

## Usage

```luau
local name = prompt.text("What's your name?")
-- displays "What's your name?: "
if name == "deviaze" then
    print("no that's me")
end

-- a very bold question
local ssn = prompt.text(colors.bold.white("whats your ssn???: "))
-- seal doesn't display an extra `: ` after your message
```

</details>

---

### prompt.confirm

<h4>

```luau
function prompt.confirm(message: string, default: boolean?) -> boolean,
```

</h4>

<details>

<summary> See the docs </summary

Ask the user to confirm an action, defaulting to `true`, and displaying a y/n after the `message` prompt according to the usual CLI application conventions.

- If `default` is unspecified or `true`, displays `[Y/n]:` after the message, demonstrating that pressing <kbd>Enter</kbd> signifies Yes.
- If `default` is `false,` displays `[y/N]:` after the message, demonstrating that pressing <kbd>Enter</kbd> signifies No.

To confirm, the user may send any of `Y`, `y`, `n`, or `N` to explicitly signify confirmation/denial or nothing (just <kbd>Enter</kbd>) for the default.

## Usage

```luau
if prompt.confirm("are roses red") then
    -- displays "are roses red [Y/n]: "
    print("violets are blue")
end
```

</details>

---

### prompt.validate

<h4>

```luau
function prompt.validate(message: string, v: (response: string) -> true | string) -> string,
```

</h4>

<details>

<summary> See the docs </summary

Like `prompt.text`, but validates the response with a custom validation function.

- Return a `string` to tell users why validation failed.
- Retries until `v(response)` returns `true`.

## Usage

```luau
local response = prompt.validate("Favorite animal that starts with 's'",
    function(response: string): true | string
        return
            if response == "seal" then
                true
            elseif not str.startswith(response, "s") then
                `'{response}' does not start with s!`
            else "nope not the answer :)"
    end
)
```

</details>

---

### prompt.password

<h4>

```luau
function prompt.password(message: string, style: "Hidden" | "*"?) -> string,
```

</h4>

<details>

<summary> See the docs </summary

Prompts a hidden password prompt.

By default, any characters typed in the password prompt will be fully hidden according to unix conventions.

Note that users might get confused when they try typing in a password and nothing shows up. That's why *seal* also
provides the `"*"` option, which replaces characters typed with astricks.

- When `message` isn't an empty string and `style == "Hidden"`, displays `(hidden):` after the prompt.
- If `message` already contains a colon (`:`), *seal* will not display `(hidden):` after the prompt.
- If `message` is an empty string, no visual indicator will be displayed. Use this with `io.output.write`
to override the default behavior for custom password prompts.

## ⚠️ Safety

- Do not use when rawmode (`input.rawmode`) is already enabled.
- Do not use in multithreaded programs while another thread may be reading from stdin.
- Using the default `"Hidden"` mode might be slightly more secure;
seal makes a best-effort attempt to clean up/erase passwords in astrick mode
but it may not be perfectly safe from introspection.

</details>

---

### prompt.pick

<h4>

```luau
function prompt.pick(message: string, options: { string }, default: number?) -> number,
```

</h4>

<details>

<summary> See the docs </summary

Prompts users to pick one of a list of options by index.

- If `default` is unspecified, assumes no default and retries until the user picks something.
- Automatically retries until the user provides a valid option.
- Handles `message` like `prompt.text`, appending `": "` to the prompt message if it isn't an empty string and doesn't already include `": "`.

## Usage

```luau
local opt = prompt.pick("Pick an editor", {"vscode/code", "zed", "nvim"}, 1)
if opt == 1 then
    -- handle vscode stuff
elseif opt == 2 then
    -- handle zed stuff
elseif opt == 3 then
    -- handle nvim stuff
end

-- or dynamically
local file = files[prompt.pick("Pick a file", files)]
```

</details>

---

Autogenerated from [std/io/prompt.luau](/.seal/typedefs/std/io/prompt.luau).

*seal* is best experienced with inline, in-editor documentation. Please see the linked typedefs file if this documentation is confusing, too verbose, or inaccurate.
