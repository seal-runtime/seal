<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# io.input

`local input = require("@std/io/input")`

$hspace{5pt}$ input handling lib
$hspace{5pt}$ gets input with an optional `raw_prompt` to display before getting said input.

.function input.tty(stream: `"Stdout" | "Stderr" | "Stdin"?): boolean`

<details>

<summary> See the docs </summary

$hspace{5pt}$Determine whether we're connected to a sane [TTY](https://en.wikipedia.org/wiki/Tty_(Unix)) terminal.
$hspace{5pt}$
$hspace{5pt}$This can be used to detect if users can provide rich input at runtime (`io.prompt.*` or `io.input.readline`) or if we're limited to basic stdin (`io.input.rawline`)
$hspace{5pt}$
$hspace{5pt}$- If `stream` is nil or unspecified, returns true if *all* streams are TTY.
$hspace{5pt}$- Otherwise, returns `true` if `stream` is a TTY.
$hspace{5pt}$
$hspace{5pt}$If *seal* is being run in a child process, this will almost always return `false` (because users can't easily write input).

</details>

.function input.rawline(prompt: `string?): string`

$hspace{5pt}$Gets a line directly from stdout in a way that doesn't properly handle editing text (going back and forward with arrow keys), etc.
$hspace{5pt}$
$hspace{5pt}$But works with stdin in a child process/works while piped, making it a fallback for automated solutions or cursed ancient terminals.

.function input.readline(prompt: `string): string | interrupt | error`

<details>

<summary> See the docs </summary

$hspace{5pt}$Prompts the user for one line of text with proper text buffer handling and error handling.
$hspace{5pt}$
$hspace{5pt}$For a higher-level prompting API, use `@std/io/prompt` instead.
$hspace{5pt}$
$hspace{5pt}$Falls back to `io.input.rawline` if called in a non-TTY (like a piped child process)
$hspace{5pt}$
$hspace{5pt}$## Returns
$hspace{5pt}$
$hspace{5pt}$- A `string` once the user types something in and presses <kbd>Enter</kbd>.
$hspace{5pt}$- An `interrupt` userdata, which contains a `code` (`"CtrlC"` or `"CtrlD"`) if the user presses either.
$hspace{5pt}$- An `error` userdata, if some other IO error occurs (like a broken pipe)
$hspace{5pt}$
$hspace{5pt}$## Errors
$hspace{5pt}$
$hspace{5pt}$- *Throws* an error if some weird low level Errno code occurs.
$hspace{5pt}$
$hspace{5pt}$## Usage
$hspace{5pt}$
$hspace{5pt}$```luau
$hspace{5pt}$local line: string?
$hspace{5pt}$local result = input.readline("what's your name?: ")
$hspace{5pt}$if typeof(result) == "string" then
$hspace{5pt}$    line = result
$hspace{5pt}$elseif typeof(result) == "interrupt" then
$hspace{5pt}$    if result.code == "CtrlC" then
$hspace{5pt}$        process.exit(0)
$hspace{5pt}$    elseif result.code == "CtrlD" then
$hspace{5pt}$        line = ""
$hspace{5pt}$    end
$hspace{5pt}$elseif typeof(result) == "error" then
$hspace{5pt}$    print(`got error {result}`)
$hspace{5pt}$end
$hspace{5pt}$```

</details>

.function input.interrupt(key: `"CtrlC" | "CtrlD"): interrupt`

$hspace{5pt}$Returns an `interrupt` userdata object. For reasons. Maybe control flow.

.function input.rawmode(enabled: `boolean)`

<details>

<summary> See the docs </summary

$hspace{5pt}$Set stdin to raw mode, allowing you direct control over incoming keypresses.
$hspace{5pt}$
$hspace{5pt}$Use this with `input.events` to write a TUI.
$hspace{5pt}$
$hspace{5pt}$## ⚠️ Safety
$hspace{5pt}$
$hspace{5pt}$- Do ***not*** use this in multithreaded programs (`@std/thread`) where another thread
$hspace{5pt}$might be writing to stdout or reading from stdin at the same time. This may cause unexpected behavior.
$hspace{5pt}$- Enabling this in a `ChildProcess (@std/process)` will somehow cause `output.write` to write to the parent process' stdout.

</details>

.mouse: `(enabled: boolean) -> ()`

$hspace{5pt}$ Allows `MouseEvents` to be reported by `input.events()`.

.focus: `(enabled: boolean) -> ()`

$hspace{5pt}$ Allows `FocusGained` and `FocusLost` events to be reported by `input.events()`.

.paste: `(enabled: boolean) -> ()`

$hspace{5pt}$ Allows `Paste` events to be reported by `input.events()`.<br>Might not work correctly when multiple lines are copied.

.function input.events(poll: `Duration): () -> TerminalEvent`

<details>

<summary> See the docs </summary

$hspace{5pt}$Listens for raw terminal events from stdin, returning an iterator over those events.
$hspace{5pt}$
$hspace{5pt}$Use this function to write interactive TUIs that immediately redraw and respond to user input.
$hspace{5pt}$
$hspace{5pt}$## ⚠️ Safety
$hspace{5pt}$
$hspace{5pt}$This function has specific usage requirements:
$hspace{5pt}$
$hspace{5pt}$- Stdin **must** be a valid TTY; use `input.tty()` to check.
$hspace{5pt}$- Rawmode **must** be enabled before calling this function; set it with `input.rawmode(true)`.
$hspace{5pt}$- Remember to check for/intercept Ctrl-C and Ctrl-D events otherwise users might not be able to cancel or exit your program.
$hspace{5pt}$- Remember to disable rawmode once you're done listening to terminal events, otherwise you might break
$hspace{5pt}$the user's terminal, prevent them from exiting your program, or worse.
$hspace{5pt}$
$hspace{5pt}$## Usage
$hspace{5pt}$
$hspace{5pt}$To enable `Mouse`, `Focus`, and clipboard `Paste` events, check out the `input.capture` apis.
$hspace{5pt}$
$hspace{5pt}$```luau
$hspace{5pt}$if input.tty() then -- MUST be checked
$hspace{5pt}$    input.rawmode(true)
$hspace{5pt}$    input.capture.paste(true)
$hspace{5pt}$    output.write("\27[?25l") -- hide cursor
$hspace{5pt}$
$hspace{5pt}$    local interrupted: interrupt?
$hspace{5pt}$
$hspace{5pt}$    for event in input.events(time.milliseconds(40)) do
$hspace{5pt}$        if event.is == "Key" then
$hspace{5pt}$            if event.modifiers.ctrl and event.key == "c" then
$hspace{5pt}$                interrupted = input.interrupt("CtrlC")
$hspace{5pt}$                break -- user pressed Ctrl + C
$hspace{5pt}$            elseif event.modifiers.ctrl and event.key == "d" then
$hspace{5pt}$                interrupted = input.interrupt("CtrlD")
$hspace{5pt}$                break -- user pressed Ctrl + D
$hspace{5pt}$            end
$hspace{5pt}$
$hspace{5pt}$            if event.key == "Up" then
$hspace{5pt}$                -- up arrow key
$hspace{5pt}$            elseif event.key == "Left" then
$hspace{5pt}$                -- left arrow key
$hspace{5pt}$            elseif event.key == "Enter" then
$hspace{5pt}$                -- user pressed Enter or Return
$hspace{5pt}$            elseif event.key == "Space" then
$hspace{5pt}$                -- user pressed spacebar
$hspace{5pt}$            else
$hspace{5pt}$                print(event.key)
$hspace{5pt}$            end
$hspace{5pt}$        elseif event.is == "Paste" then
$hspace{5pt}$            print(`user pasted {event.contents}`)
$hspace{5pt}$        end
$hspace{5pt}$    end
$hspace{5pt}$
$hspace{5pt}$    output.write("\27[?25h") -- show cursor
$hspace{5pt}$    input.capture.paste(false)
$hspace{5pt}$    input.rawmode(false)
$hspace{5pt}$end
$hspace{5pt}$```

</details>

`export type` KeyModifiers

$hspace{5pt}$ Note this modifier table is ***REUSED*** across all iterations. Don't try to store it in a table or anything please.

KeyModifiers.ctrl: `boolean`

KeyModifiers.shift: `boolean`

KeyModifiers.alt: `boolean`

`export type` KeyEvent

KeyEvent.is: `"Key"`

KeyEvent.key: `string`

KeyEvent.modifiers: `KeyModifiers`

`export type` MouseEvent

MouseEvent.is: `"Mouse"`

MouseEvent.kind: `string`

MouseEvent.column: `number`

MouseEvent.row: `number`

MouseEvent.modifiers: `KeyModifiers`

`export type` ResizeEvent

ResizeEvent.is: `"Resize"`

ResizeEvent.columns: `number`

ResizeEvent.rows: `number`

`export type` FocusGained

FocusGained.is: `"FocusGained"`

`export type` FocusLost

FocusLost.is: `"FocusLost"`

`export type` PasteEvent

PasteEvent.is: `"Paste"`

PasteEvent.contents: `string`

`export type` Empty

Empty.is: `"Empty"`

`export type` TerminalEvent

`export type` input
