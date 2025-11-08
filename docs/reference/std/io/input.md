<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# io.input

`local input = require("@std/io/input")`

 input handling lib
 gets input with an optional `raw_prompt` to display before getting said input.

---

### .stream

```luau
`function` .streaminput.tty: (stream: "Stdout" | "Stderr" | "Stdin"?) -> boolean
```

<details>

<summary> See the docs </summary

Determine whether we're connected to a sane [TTY](https://en.wikipedia.org/wiki/Tty_(Unix)) terminal.

This can be used to detect if users can provide rich input at runtime (`io.prompt.*` or `io.input.readline`) or if we're limited to basic stdin (`io.input.rawline`)

- If `stream` is nil or unspecified, returns true if *all* streams are TTY.
- Otherwise, returns `true` if `stream` is a TTY.

If *seal* is being run in a child process, this will almost always return `false` (because users can't easily write input).

</details>

---

### .prompt

```luau
`function` .promptinput.rawline: (prompt: string?) -> string
```

Gets a line directly from stdout in a way that doesn't properly handle editing text (going back and forward with arrow keys), etc.

But works with stdin in a child process/works while piped, making it a fallback for automated solutions or cursed ancient terminals.

---

### .prompt

```luau
`function` .promptinput.readline: (prompt: string) -> string | interrupt | error
```

<details>

<summary> See the docs </summary

Prompts the user for one line of text with proper text buffer handling and error handling.

For a higher-level prompting API, use `@std/io/prompt` instead.

Falls back to `io.input.rawline` if called in a non-TTY (like a piped child process)

## Returns

- A `string` once the user types something in and presses <kbd>Enter</kbd>.
- An `interrupt` userdata, which contains a `code` (`"CtrlC"` or `"CtrlD"`) if the user presses either.
- An `error` userdata, if some other IO error occurs (like a broken pipe)

## Errors

- *Throws* an error if some weird low level Errno code occurs.

## Usage

```luau
local line: string?
local result = input.readline("what's your name?: ")
if typeof(result) == "string" then
    line = result
elseif typeof(result) == "interrupt" then
    if result.code == "CtrlC" then
        process.exit(0)
    elseif result.code == "CtrlD" then
        line = ""
    end
elseif typeof(result) == "error" then
    print(`got error {result}`)
end
```

</details>

---

### .key

```luau
`function` .keyinput.interrupt: (key: "CtrlC" | "CtrlD") -> interrupt
```

Returns an `interrupt` userdata object. For reasons. Maybe control flow.

---

### .enabled

```luau
`function` .enabledinput.rawmode: (enabled: boolean)
```

<details>

<summary> See the docs </summary

Set stdin to raw mode, allowing you direct control over incoming keypresses.

Use this with `input.events` to write a TUI.

## ⚠️ Safety

- Do ***not*** use this in multithreaded programs (`@std/thread`) where another thread
might be writing to stdout or reading from stdin at the same time. This may cause unexpected behavior.
- Enabling this in a `ChildProcess (@std/process)` will somehow cause `output.write` to write to the parent process' stdout.

</details>

---

### .mouse

```luau
`function` .mousemouse: : (enabled: boolean) -> (),
```

 Allows `MouseEvents` to be reported by `input.events()`.

---

### .focus

```luau
`function` .focusfocus: : (enabled: boolean) -> (),
```

 Allows `FocusGained` and `FocusLost` events to be reported by `input.events()`.

---

### .paste

```luau
`function` .pastepaste: : (enabled: boolean) -> (),
```

 Allows `Paste` events to be reported by `input.events()`.<br>Might not work correctly when multiple lines are copied.

---

### .poll

```luau
`function` .pollinput.events: (poll: Duration) -> () -> TerminalEvent
```

<details>

<summary> See the docs </summary

Listens for raw terminal events from stdin, returning an iterator over those events.

Use this function to write interactive TUIs that immediately redraw and respond to user input.

## ⚠️ Safety

This function has specific usage requirements:

- Stdin **must** be a valid TTY; use `input.tty()` to check.
- Rawmode **must** be enabled before calling this function; set it with `input.rawmode(true)`.
- Remember to check for/intercept Ctrl-C and Ctrl-D events otherwise users might not be able to cancel or exit your program.
- Remember to disable rawmode once you're done listening to terminal events, otherwise you might break
the user's terminal, prevent them from exiting your program, or worse.

## Usage

To enable `Mouse`, `Focus`, and clipboard `Paste` events, check out the `input.capture` apis.

```luau
if input.tty() then -- MUST be checked
    input.rawmode(true)
    input.capture.paste(true)
    output.write("\27[?25l") -- hide cursor

    local interrupted: interrupt?

    for event in input.events(time.milliseconds(40)) do
        if event.is == "Key" then
            if event.modifiers.ctrl and event.key == "c" then
                interrupted = input.interrupt("CtrlC")
                break -- user pressed Ctrl + C
            elseif event.modifiers.ctrl and event.key == "d" then
                interrupted = input.interrupt("CtrlD")
                break -- user pressed Ctrl + D
            end

            if event.key == "Up" then
                -- up arrow key
            elseif event.key == "Left" then
                -- left arrow key
            elseif event.key == "Enter" then
                -- user pressed Enter or Return
            elseif event.key == "Space" then
                -- user pressed spacebar
            else
                print(event.key)
            end
        elseif event.is == "Paste" then
            print(`user pasted {event.contents}`)
        end
    end

    output.write("\27[?25h") -- show cursor
    input.capture.paste(false)
    input.rawmode(false)
end
```

</details>

---

### `export type` KeyModifiers

```luau

```

 Note this modifier table is ***REUSED*** across all iterations. Don't try to store it in a table or anything please.

---

### KeyModifiers.ctrl

```luau
KeyModifiers.ctrl: boolean,
```

---

### KeyModifiers.shift

```luau
KeyModifiers.shift: boolean,
```

---

### KeyModifiers.alt

```luau
KeyModifiers.alt: boolean,
```

---

### `export type` KeyEvent

```luau

```

---

### KeyEvent.is

```luau
KeyEvent.is: "Key",
```

---

### KeyEvent.key

```luau
KeyEvent.key: string,
```

---

### KeyEvent.modifiers

```luau
KeyEvent.modifiers: KeyModifiers,
```

---

### `export type` MouseEvent

```luau

```

---

### MouseEvent.is

```luau
MouseEvent.is: "Mouse",
```

---

### MouseEvent.kind

```luau
MouseEvent.kind: string,
```

---

### MouseEvent.column

```luau
MouseEvent.column: number,
```

---

### MouseEvent.row

```luau
MouseEvent.row: number,
```

---

### MouseEvent.modifiers

```luau
MouseEvent.modifiers: KeyModifiers,
```

---

### `export type` ResizeEvent

```luau

```

---

### ResizeEvent.is

```luau
ResizeEvent.is: "Resize",
```

---

### ResizeEvent.columns

```luau
ResizeEvent.columns: number,
```

---

### ResizeEvent.rows

```luau
ResizeEvent.rows: number,
```

---

### `export type` FocusGained

```luau

```

---

### FocusGained.is

```luau
FocusGained.is: "FocusGained",
```

---

### `export type` FocusLost

```luau

```

---

### FocusLost.is

```luau
FocusLost.is: "FocusLost",
```

---

### `export type` PasteEvent

```luau

```

---

### PasteEvent.is

```luau
PasteEvent.is: "Paste",
```

---

### PasteEvent.contents

```luau
PasteEvent.contents: string,
```

---

### `export type` Empty

```luau

```

---

### Empty.is

```luau
Empty.is: "Empty",
```

---

### `export type` TerminalEvent

```luau

```

---

### TerminalEvent

```luau
| KeyEvent
```

---

### TerminalEvent

```luau
| MouseEvent
```

---

### TerminalEvent

```luau
| ResizeEvent
```

---

### TerminalEvent

```luau
| PasteEvent
```

---

### TerminalEvent

```luau
| FocusGained | FocusLost
```

---

### TerminalEvent

```luau
| Empty
```

---

### `export type` input

```luau

```

---
