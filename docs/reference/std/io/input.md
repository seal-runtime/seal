<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# io.input

`local input = require("@std/io/input")`

 input handling lib
 gets input with an optional `raw_prompt` to display before getting said input.

---

### io.input.tty

<h4>

```luau
tty: (stream: "Stdout" | "Stderr" | "Stdin"?): boolean
```

</h4>

<details>

<summary> See the docs </summary

Determine whether we're connected to a sane [TTY](https://en.wikipedia.org/wiki/Tty_(Unix)) terminal.

This can be used to detect if users can provide rich input at runtime (`io.prompt.*` or `io.input.readline`) or if we're limited to basic stdin (`io.input.rawline`)

- If `stream` is nil or unspecified, returns true if *all* streams are TTY.
- Otherwise, returns `true` if `stream` is a TTY.

If *seal* is being run in a child process, this will almost always return `false` (because users can't easily write input).

</details>

---

### io.input.rawline

<h4>

```luau
rawline: (prompt: string?): string
```

</h4>

Gets a line directly from stdout in a way that doesn't properly handle editing text (going back and forward with arrow keys), etc.

But works with stdin in a child process/works while piped, making it a fallback for automated solutions or cursed ancient terminals.

---

### io.input.readline

<h4>

```luau
readline: (prompt: string): string | interrupt | error
```

</h4>

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

### io.input.interrupt

<h4>

```luau
interrupt: (key: "CtrlC" | "CtrlD"): interrupt
```

</h4>

Returns an `interrupt` userdata object. For reasons. Maybe control flow.

---

### io.input.rawmode

<h4>

```luau
rawmode: (enabled: boolean)
```

</h4>

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

### io.input.mouse

<h4>

```luau
mouse: (enabled: boolean) -> (),
```

</h4>

 Allows `MouseEvents` to be reported by `input.events()`.

---

### io.input.focus

<h4>

```luau
focus: (enabled: boolean) -> (),
```

</h4>

 Allows `FocusGained` and `FocusLost` events to be reported by `input.events()`.

---

### io.input.paste

<h4>

```luau
paste: (enabled: boolean) -> (),
```

</h4>

 Allows `Paste` events to be reported by `input.events()`.<br>Might not work correctly when multiple lines are copied.

---

### io.input.events

<h4>

```luau
events: (poll: Duration): () -> TerminalEvent
```

</h4>

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

## `export type` KeyModifiers

 Note this modifier table is ***REUSED*** across all iterations. Don't try to store it in a table or anything please.

---

### KeyModifiers.ctrl

<h4>

```luau
ctrl: boolean,
```

</h4>

---

### KeyModifiers.shift

<h4>

```luau
shift: boolean,
```

</h4>

---

### KeyModifiers.alt

<h4>

```luau
alt: boolean,
```

</h4>

---

## `export type` KeyEvent

---

### KeyEvent.is

<h4>

```luau
is: "Key",
```

</h4>

---

### KeyEvent.key

<h4>

```luau
key: string,
```

</h4>

---

### KeyEvent.modifiers

<h4>

```luau
modifiers: KeyModifiers,
```

</h4>

---

## `export type` MouseEvent

---

### MouseEvent.is

<h4>

```luau
is: "Mouse",
```

</h4>

---

### MouseEvent.kind

<h4>

```luau
kind: string,
```

</h4>

---

### MouseEvent.column

<h4>

```luau
column: number,
```

</h4>

---

### MouseEvent.row

<h4>

```luau
row: number,
```

</h4>

---

### MouseEvent.modifiers

<h4>

```luau
modifiers: KeyModifiers,
```

</h4>

---

## `export type` ResizeEvent

---

### ResizeEvent.is

<h4>

```luau
is: "Resize",
```

</h4>

---

### ResizeEvent.columns

<h4>

```luau
columns: number,
```

</h4>

---

### ResizeEvent.rows

<h4>

```luau
rows: number,
```

</h4>

---

## `export type` FocusGained

---

### FocusGained.is

<h4>

```luau
is: "FocusGained",
```

</h4>

---

## `export type` FocusLost

---

### FocusLost.is

<h4>

```luau
is: "FocusLost",
```

</h4>

---

## `export type` PasteEvent

---

### PasteEvent.is

<h4>

```luau
is: "Paste",
```

</h4>

---

### PasteEvent.contents

<h4>

```luau
contents: string,
```

</h4>

---

## `export type` Empty

---

### Empty.is

<h4>

```luau
is: "Empty",
```

</h4>

---

## `export type` TerminalEvent

---

```luau
| KeyEvent
```

---

```luau
| MouseEvent
```

---

```luau
| ResizeEvent
```

---

```luau
| PasteEvent
```

---

```luau
| FocusGained | FocusLost
```

---

```luau
| Empty
```

---

## `export type` input

---
