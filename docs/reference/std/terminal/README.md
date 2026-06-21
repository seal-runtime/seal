<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# terminal

`local terminal = require("@std/terminal")`

This library is for writing TUI applications with *seal*;
if you're looking to run terminal commands, you probably want the `@std/process` library instead.

---

### terminal.size

<h4>

```luau
function terminal.size() -> vector,
```

</h4>

 Returns the terminal screen buffer size as a vector of <columns, rows, 0>

---

### terminal.tty

<h4>

```luau
function terminal.tty(stream: "Stdout" | "Stderr" | "Stdin"?) -> boolean,
```

</h4>

<details>

<summary> See the docs </summary

Determine whether we're connected to a rich [TTY](https://en.wikipedia.org/wiki/Tty_(Unix)) terminal.

This can be used to detect if users can provide rich input at runtime (`io.prompt.*` or `io.input.readline`) or if we're limited to basic stdin (`io.input.rawline`)

- If `stream` is nil or unspecified, returns true if *all* streams are TTY.
- Otherwise, returns `true` if `stream` is a TTY.

If *seal* is being run in a child process, this will almost always return `false` (because users can't easily write input).

</details>

---

### terminal.write

<h4>

```luau
function terminal.write(content: string) -> TerminalAction,
```

</h4>

<details>

<summary> See the docs </summary

Creates a <kbd>Write</kbd> `TerminalAction` that once invoked, writes
to stdout without a trailing newline.

Please note that this function only accepts *valid* UTF-8 encoded `content`,
if you want  to write arbitrary bytes to stdout or stderr, use `output.write`
and `output.ewrite` from `@std/io/output` instead.

`TerminalActions` do nothing until invoked; pass the action to `terminal.execute`
to queue and execute multiple commands, or call its `:execute()` method to directly invoke.

</details>

---

### terminal.title

<h4>

```luau
function terminal.title(title: string) -> TerminalAction,
```

</h4>

Creates a <kbd>SetTitle</kbd> `TerminalAction` that sets the terminal window title.

`TerminalActions` do nothing until invoked; pass the action to `terminal.execute`
to queue and execute multiple commands, or call its `:execute()` method to directly invoke.

---

### terminal.clear

<h4>

```luau
function terminal.clear(mode: ClearMode?) -> TerminalAction,
```

</h4>

<details>

<summary> See the docs </summary

Creates a <kbd>Clear</kbd> `TerminalAction`.

Once invoked, visually clears the terminal by writing escape codes
to move existing content into the terminal scrollback buffer. Unlike
`io.output.clear`, this `TerminalAction` doesn't actually run `clear`
or `cls`, making it faster but less strong.

If you really want to clear the terminal and prevent the user from
scrolling back, use `io.output.clear` from `@std/io/output` instead.

`TerminalActions` do nothing until invoked; pass the action to `terminal.execute`
to queue and execute multiple commands, or call its `:execute()` method to directly invoke.

</details>

---

### terminal.switch

<h4>

```luau
function terminal.switch(screen: "Main" | "Alternate") -> TerminalAction,
```

</h4>

<details>

<summary> See the docs </summary

Creates a <kbd>Switch</kbd> `TerminalAction` that once invoked, switches
between crossterm's Main and Alternate terminal screens.

If you want to write a TUI but preserve existing content, you should
switch to the Alternate screen and switch back when done.

`TerminalActions` do nothing until invoked; pass the action to `terminal.execute`
to queue and execute multiple commands, or call its `:execute()` method to directly invoke.

## Usage

```luau
local terminal = require("@std/terminal")
local cursor = require("@std/terminal/cursor")
if not terminal.tty() then
    error("expected terminal to be a tty")
end
if not terminal.rawmode.enabled() then
    terminal.rawmode.enable()
end
terminal.execute(
    terminal.switch("Alternate"),
    cursor.right(2),
    terminal.write("Options:"),
)
```

</details>

---

### terminal.linewrap

<h4>

```luau
function terminal.linewrap(enabled: boolean) -> TerminalAction,
```

</h4>

Creates a `TerminalAction` that when invoked, enables or disables terminal linewrapping.

`TerminalActions` do nothing until invoked; pass the action to `terminal.execute`
to queue and execute multiple commands, or call its `:execute()` method to directly invoke.

---

### terminal.scroll

<h4>

```luau
function terminal.scroll(lines: number) -> TerminalAction,
```

</h4>

Creates a <kbd>Scroll</kbd> `TerminalAction` that when invoked, scrolls the terminal.

Negative values scroll up, positive values scroll down.

`TerminalActions` do nothing until invoked; pass the action to `terminal.execute`
to queue and execute multiple commands, or call its `:execute()` method to directly invoke.

---

### terminal.rawmode

<h4>

```luau
rawmode: {
```

</h4>

<details>

<summary> See the docs </summary

The [terminal mode](https://en.wikipedia.org/wiki/Terminal_mode) controls
how incoming keypresses are handled.

There are two terminal modes on compliant terminals:

- **Raw**: programs have full control over keystrokes, can intercept <kbd>Ctrl+C</kbd>
and <kbd>Ctrl+D</kbd>, and must manually print characters for them to be seen.
- **Cooked**: the default; intercepts incoming keystrokes, handles <kbd>Ctrl+C</kbd> and
other terminal signals, formats output in a way consistent for a line-by-line command-line
interface, and automatically echoes (prints) incoming input to terminal output.

To write a TUI, you'll want to switch to rawmode to get full control over all
incoming keystrokes. Keep in mind that when using rawmode, keystrokes are not
automatically printed to the terminal.

</details>

---

### terminal.rawmode.enabled

<h4>

```luau
function terminal.rawmode.enabled() -> boolean,
```

</h4>

 Determine if the terminal is currently in rawmode.

---

### terminal.rawmode.enable

<h4>

```luau
function terminal.rawmode.enable() -> (),
```

</h4>

<details>

<summary> See the docs </summary

Switches the [terminal mode](https://en.wikipedia.org/wiki/Terminal_mode) to rawmode.

Always ensure that `terminal.tty()` returns `true` before changing the terminal mode.

## ⚠️ Caution

- Do ***not*** switch the terminal mode in multithreaded programs (`@std/thread`) when
another thread might be writing to stdout or reading from stdin at the same time.
Doing so may cause unexpected behavior.
- Enabling this in a `ChildProcess (@std/process)` will somehow cause `print`
and `output.write` to write to the parent process' stdout (at least on Linux).
- Different operating systems, terminal emulators, or shells may handle modes differently.

</details>

---

### terminal.rawmode.disable

<h4>

```luau
function terminal.rawmode.disable() -> (),
```

</h4>

Switches back to **cooked** mode.

---

```luau
  }, -- closes rawmode
```

---

### terminal.interrupt

<h4>

```luau
interrupt: {
```

</h4>

---

### terminal.interrupt.sigint

<h4>

```luau
function terminal.interrupt.sigint() -> interrupt,
```

</h4>

 Returns an interrupt for Ctrl+C (SIGINT).

---

### terminal.interrupt.eof

<h4>

```luau
function terminal.interrupt.eof() -> interrupt,
```

</h4>

 Returns an interrupt for Ctrl+D (EOF).

---

### terminal.interrupt.check

<h4>

```luau
function terminal.interrupt.check(event: TerminalEvent) -> interrupt?,
```

</h4>

<details>

<summary> See the docs </summary

Checks if a TerminalEvent is a Ctrl+C (SIGINT) or Ctrl+D (EOF) interrupt.
Returns the corresponding interrupt object if the event is an interrupt,
otherwise returns nil. Use this in your event loop to simplify interrupt handling.

## Example

```luau
for event in terminal.events(time.milliseconds(50)) do
    local interrupt = terminal.interrupt.check(event)
    if interrupt then
        -- user pressed Ctrl+C or Ctrl+D
        terminal.reset()
        process.exit(1)
    end
end
```

</details>

---

```luau
  }, -- closes interrupt
```

---

### terminal.capture

<h4>

```luau
capture: {
```

</h4>

---

### terminal.capture.mouse

<h4>

```luau
function terminal.capture.mouse(enabled: boolean) -> (),
```

</h4>

 Allows `MouseEvents` to be reported by `terminal.events()`.

---

### terminal.capture.focus

<h4>

```luau
function terminal.capture.focus(enabled: boolean) -> (),
```

</h4>

 Allows `FocusGained` and `FocusLost` events to be reported by `terminal.events()`.

---

### terminal.capture.paste

<h4>

```luau
function terminal.capture.paste(enabled: boolean) -> (),
```

</h4>

 Allows `Paste` events to be reported by `terminal.events()`.<br>
 Might not work correctly when multiple lines are copied.

---

```luau
  }, -- closes capture
```

---

### terminal.events

<h4>

```luau
function terminal.events(poll: Duration) -> (() -> TerminalEvent),
```

</h4>

<details>

<summary> See the docs </summary

Listen for raw terminal events, returning an iterator over the events that blocks
the VM until the next event is detected or the `poll` duration elapses.

Use this function alongside `terminal.execute` and `TerminalActions`
to write interactive TUIs that immediately redraw and react to user input.

When the poll duration elapses without any input, an `Empty` event is returned,
allowing your TUI loop to continue without blocking indefinitely. This is useful
for situations in which you want to keep track of time, have a stable 'heartbeat',
early-exit from the event loop, or update the display without user input.

To enable `Mouse`, `Focus`, and clipboard `Paste` events, check out the `terminal.capture.*` apis.

When writing a TUI, you generally want to follow these steps:

1. Always make sure stdin and stdout are TTYs with `terminal.tty()`.
2. Switch to rawmode with `terminal.rawmode.enable()`.
3. If you're writing a separate TUI application and terminal history doesn't need
to be visible, switch to the Alternate terminal screen with `terminal.switch("Alternate")`.
This prevents scrolling from affecting terminal state.
4. Hide the cursor when you don't need it (such as in a dropdown picker) or when the user
isn't currently typing.
5. Set up your internal state logic that lives throughout all iterations. Keep in mind that
in rawmode, characters aren't automatically printed to output so you have to track internal
and visual state separately.
6. Listen for terminal events with this function, `terminal.events()`, making sure you
handle <kbd>Ctrl+C</kbd> and <kbd>Ctrl+D</kbd> events to allow users to gracefully exit
your program.
7. In each iteration (or as necessary), update your visual state by passing `TerminalActions`
to `terminal.execute`. Try to execute as many actions as you can in the same `terminal.execute`
call to reduce syscalls for better performance.
8. When you're done, call `terminal.reset()` or a custom cleanup function to ensure all actions
you've taken have been cleaned up.

## ⚠️ Caution

- Stdin **must** be a valid TTY; use `terminal.tty()` to check.
- Rawmode **should** be enabled before calling this function; set it with `terminal.rawmode.enable()`.
- Check for/intercept Ctrl-C and Ctrl-D events, otherwise users might not be able to cancel your program.
- If you forget to clean up actions you've taken before exiting, you can break the user's terminal.
When exiting, call`terminal.reset()`, which handles disabling raw mode, switching
back to Main screen, and restoring cursor state for you, or call a custom cleanup function that
reverts the changes you've made to terminal state.

## Usage

A simple arrow key prompt picker on the Alternate screen.

```luau
local process = require("@std/process")
local terminal = require("@std/terminal")
local cursor = require("@std/terminal/cursor")
local time = require("@std/time")

local animals = { "seal", "monkey", "rhino", "dolphin", "crab", "gopher" }

local function draw(selected: number, start_row: number)
    local actions = { cursor.to(0, start_row) }
    for i, animal in animals do
        if i == selected then
            table.insert(actions, terminal.write("> " .. animal))
        else
            table.insert(actions, terminal.write("  " .. animal))
        end
        if i < #animals then
            table.insert(actions, cursor.down(1))
            table.insert(actions, cursor.column(0))
        end
    end
    terminal.execute(table.unpack(actions))
end

if terminal.tty() then
    if not terminal.rawmode.enabled() then
        terminal.rawmode.enable()
    end

    terminal.execute(
        terminal.switch("Alternate"),
        cursor.hide(),
        cursor.to(0, 0),
        terminal.write("Pick an animal:")
    )

    local selected: (string | interrupt)?
    local current_selection = 1
    local start_row = 1

    draw(current_selection, start_row)

    while selected == nil do
        for event in terminal.events(time.milliseconds(50)) do
            local interrupt = terminal.interrupt.check(event)
            if interrupt then
                selected = interrupt
                break
            end

            if event.type == "Key" then
                if event.key == "Up" then
                    current_selection = if current_selection > 1 then current_selection - 1 else #animals
                    draw(current_selection, start_row)
                elseif event.key == "Down" then
                    current_selection = if current_selection < #animals then current_selection + 1 else 1
                    draw(current_selection, start_row)
                elseif event.key == "Enter" then
                    selected = animals[current_selection]
                    break
                end
            end
        end
    end

    if typeof(selected) == "interrupt" then
        terminal.reset()
        warn(`exiting with {selected.code}`)
        process.exit(1)
    end

    terminal.reset()
    print(`You selected: {selected}`)
end
```

</details>

---

### terminal.reset

<h4>

```luau
function terminal.reset() -> (),
```

</h4>

<details>

<summary> See the docs </summary

Resets the terminal to its default state.

Disables raw mode (if enabled), switches back to the Main screen,
enables line wrap, resets the cursor to column 0, sets the cursor
to the default style, makes the cursor visible, and disables mouse,
paste, and focus-changed event capturing.
This is useful for cleanup when exiting a TUI.

This function is idempotent and safe to call multiple times.

</details>

---

### terminal.execute

<h4>

```luau
function terminal.execute(...TerminalAction) -> (),
```

</h4>

<details>

<summary> See the docs </summary

Queue and execute multiple `TerminalActions` efficiently when writing a TUI.

Uses crossterm's queue API to schedule all `TerminalActions` and invoke them
with a single flush operation, thereby reducing syscalls and IO overhead.
Wraps all actions in a synchronized output block to eliminate flicker on supporting terminals.

## Usage

A simple spinner.

```luau
local terminal = require("@std/terminal")
local cursor = require("@std/terminal/cursor")
local time = require("@std/time")

local keyframes = { "⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏" }

local function spinner(message: string, duration_seconds: number)
    local pos = cursor.position()
    -- Hide cursor during animation
    cursor.hide():execute()

    local start = os.clock()
    local frame = 0
    while (os.clock() - start) < duration_seconds do
        frame = (frame % #keyframes) + 1
        -- Move back to start position and redraw frame
        terminal.execute(
            cursor.to(pos),
            terminal.write(keyframes[frame] .. " " .. message)
        )
        time.wait(0.08)
    end

    -- Clear the spinner and restore cursor
    terminal.execute(
        cursor.to(pos),
        terminal.write(string.rep(" ", #message + 2)),
        cursor.show(),
        cursor.column(0)
    )
end

if terminal.tty() then
    spinner("Processing", 3)
end
```

</details>

---

### terminal.background

<h4>

```luau
function terminal.background() -> vector?
```

</h4>

Queries the terminal for its background color via the OSC 11 escape sequence.

Returns a vector of `<r, g, b>` with each channel in the 0–255 range,
or `nil` if the terminal doesn't respond within 150ms or doesn't support the query.

Temporarily enables raw mode if not already active.

---

## `export type` ClearMode

<h4>

```luau
type ClearMode =
```

</h4>

---

```luau
  | "All"
```

---

```luau
  | "Purge"
```

---

```luau
  | "FromCursorDown"
```

---

```luau
  | "FromCursorUp"
```

---

```luau
  | "CurrentLine"
```

---

```luau
  | "UntilNewLine"
```

---

## `export type` TerminalEvent

<h4>

```luau
export type TerminalEvent =
```

</h4>

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
  | FocusGained
```

---

```luau
  | FocusLost
```

---

```luau
  | Empty
```

---

## `export type` KeyModifiers

<h4>

```luau
export type KeyModifiers = {
```

</h4>

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

```luau
} -- closes KeyModifiers
```

---

## `export type` ResizeEvent

<h4>

```luau
export type ResizeEvent = {
```

</h4>

---

### ResizeEvent.type

<h4>

```luau
  type: "Resize",
```

</h4>

---

### ResizeEvent.size

<h4>

```luau
  size: vector,
```

</h4>

 The new terminal size; a vector of `<columns, rows, 0>`; access via `size.x` (columns) and `size.y` (rows)

---

```luau
} -- closes ResizeEvent
```

---

## `export type` FocusGained

<h4>

```luau
export type FocusGained = {
```

</h4>

---

### FocusGained.type

<h4>

```luau
  type: "FocusGained",
```

</h4>

---

```luau
} -- closes FocusGained
```

---

## `export type` FocusLost

<h4>

```luau
export type FocusLost = {
```

</h4>

---

### FocusLost.type

<h4>

```luau
  type: "FocusLost",
```

</h4>

---

```luau
} -- closes FocusLost
```

---

## `export type` PasteEvent

<h4>

```luau
export type PasteEvent = {
```

</h4>

---

### PasteEvent.type

<h4>

```luau
  type: "Paste",
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

```luau
} -- closes PasteEvent
```

---

## `export type` Empty

<h4>

```luau
export type Empty = {
```

</h4>

 Sent if `poll` duration elapses and no new events have been recorded. Use this to guarantee your event loop updates
 at least once per `poll` duration.

---

### Empty.type

<h4>

```luau
  type: "Empty",
```

</h4>

---

```luau
} -- closes Empty
```

---

## `export type` KeyEvent

<h4>

```luau
export type KeyEvent = {
```

</h4>

---

### KeyEvent.type

<h4>

```luau
  type: "Key",
```

</h4>

---

### KeyEvent.key

<h4>

```luau
  key: KeyCode,
```

</h4>

<details>

<summary> See the docs </summary

*seal* normalizes all keys to a single canonical name for cross-platform compatibility:

## Navigation & Control Keys

- `"Backspace"`: <kbd>Backspace</kbd> (Linux/Windows), <kbd>Delete</kbd> (macOS)
- `"Enter"`: <kbd>Enter</kbd> (Linux/Windows), <kbd>Return</kbd> (macOS)
- `"Delete"`: <kbd>Del</kbd> (Linux/Windows), <kbd>Fwd Del</kbd> (macOS)

## Modifier Keys

- `"Left Ctrl"`: <kbd>Left Ctrl</kbd> (Linux/Windows), <kbd>Left Control</kbd> (macOS)
- `"Right Ctrl"`: <kbd>Right Ctrl</kbd> (Linux/Windows), <kbd>Right Control</kbd> (macOS)
- `"Left Alt"`: <kbd>Left Alt</kbd> (Linux/Windows), <kbd>Left Option</kbd> (macOS)
- `"Right Alt"`: <kbd>Right Alt</kbd> (Linux/Windows), <kbd>Right Option</kbd> (macOS)
- `"Left Super"`: <kbd>Left Super</kbd> (Linux/other), <kbd>Left Windows</kbd> (Windows), <kbd>Left Command</kbd> (macOS)
- `"Right Super"`: <kbd>Right Super</kbd> (Linux/other), <kbd>Right Windows</kbd> (Windows), <kbd>Right Command</kbd> (macOS)

All other keys (arrows, function keys, characters, media keys, etc.) are reported consistently
across all platforms.

The spacebar key is `"Space"`.

</details>

---

### KeyEvent.kind

<h4>

```luau
  kind: KeyEventKind,
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

```luau
} -- closes KeyEvent
```

---

## `export type` KeyCode

<h4>

```luau
export type KeyCode =
```

</h4>

---

```luau
  | "Backspace"
```

 Navigation and control keys

---

```luau
  | "Enter"
```

---

```luau
  | "Left"
```

---

```luau
  | "Right"
```

---

```luau
  | "Up"
```

---

```luau
  | "Down"
```

---

```luau
  | "Home"
```

---

```luau
  | "End"
```

---

```luau
  | "Page Up"
```

---

```luau
  | "Page Down"
```

---

```luau
  | "Tab"
```

---

```luau
  | "Back Tab"
```

---

```luau
  | "Delete"
```

---

```luau
  | "Insert"
```

---

```luau
  | "F1"
```

 Function keys F1 through F24

---

```luau
  | "F2"
```

---

```luau
  | "F3"
```

---

```luau
  | "F4"
```

---

```luau
  | "F5"
```

---

```luau
  | "F6"
```

---

```luau
  | "F7"
```

---

```luau
  | "F8"
```

---

```luau
  | "F9"
```

---

```luau
  | "F10"
```

---

```luau
  | "F11"
```

---

```luau
  | "F12"
```

---

```luau
  | "F13"
```

---

```luau
  | "F14"
```

---

```luau
  | "F15"
```

---

```luau
  | "F16"
```

---

```luau
  | "F17"
```

---

```luau
  | "F18"
```

---

```luau
  | "F19"
```

---

```luau
  | "F20"
```

---

```luau
  | "F21"
```

---

```luau
  | "F22"
```

---

```luau
  | "F23"
```

---

```luau
  | "F24"
```

---

```luau
  | "a"
```

 Character keys: lowercase letters (all printable characters normalized to lowercase)

---

```luau
  | "b"
```

---

```luau
  | "c"
```

---

```luau
  | "d"
```

---

```luau
  | "e"
```

---

```luau
  | "f"
```

---

```luau
  | "g"
```

---

```luau
  | "h"
```

---

```luau
  | "i"
```

---

```luau
  | "j"
```

---

```luau
  | "k"
```

---

```luau
  | "l"
```

---

```luau
  | "m"
```

---

```luau
  | "n"
```

---

```luau
  | "o"
```

---

```luau
  | "p"
```

---

```luau
  | "q"
```

---

```luau
  | "r"
```

---

```luau
  | "s"
```

---

```luau
  | "t"
```

---

```luau
  | "u"
```

---

```luau
  | "v"
```

---

```luau
  | "w"
```

---

```luau
  | "x"
```

---

```luau
  | "y"
```

---

```luau
  | "z"
```

---

```luau
  | "0"
```

 Character keys: digits

---

```luau
  | "1"
```

---

```luau
  | "2"
```

---

```luau
  | "3"
```

---

```luau
  | "4"
```

---

```luau
  | "5"
```

---

```luau
  | "6"
```

---

```luau
  | "7"
```

---

```luau
  | "8"
```

---

```luau
  | "9"
```

---

```luau
  | "Space"
```

 Special character keys

---

```luau
  | "Null"
```

---

```luau
  | "Esc"
```

---

```luau
  | "Caps Lock"
```

---

```luau
  | "Scroll Lock"
```

---

```luau
  | "Num Lock"
```

---

```luau
  | "Print Screen"
```

---

```luau
  | "Pause"
```

---

```luau
  | "Menu"
```

---

```luau
  | "Begin"
```

---

```luau
  | "Play"
```

 Media keys (cross-platform normalized)

---

```luau
  | "Pause"
```

---

```luau
  | "Play/Pause"
```

---

```luau
  | "Reverse"
```

---

```luau
  | "Stop"
```

---

```luau
  | "Fast Forward"
```

---

```luau
  | "Rewind"
```

---

```luau
  | "Next Track"
```

---

```luau
  | "Previous Track"
```

---

```luau
  | "Record"
```

---

```luau
  | "Lower Volume"
```

---

```luau
  | "Raise Volume"
```

---

```luau
  | "Mute Volume"
```

---

```luau
  | "Left Shift"
```

 Modifier keys (cross-platform normalized)

---

```luau
  | "Right Shift"
```

---

```luau
  | "Left Ctrl"
```

---

```luau
  | "Right Ctrl"
```

---

```luau
  | "Left Alt"
```

---

```luau
  | "Right Alt"
```

---

```luau
  | "Left Super"
```

---

```luau
  | "Right Super"
```

---

```luau
  | "Left Hyper"
```

---

```luau
  | "Right Hyper"
```

---

```luau
  | "Left Meta"
```

---

```luau
  | "Right Meta"
```

---

```luau
  | "Iso Level 3 Shift"
```

---

```luau
  | "Iso Level 5 Shift"
```

---

## `export type` KeyEventKind

<h4>

```luau
export type KeyEventKind =
```

</h4>

---

```luau
  | "Press"
```

---

```luau
  | "Release"
```

---

```luau
  | "Repeat"
```

---

## `export type` MouseEvent

<h4>

```luau
export type MouseEvent = {
```

</h4>

---

### MouseEvent.type

<h4>

```luau
  type: "Mouse",
```

</h4>

---

### MouseEvent.kind

<h4>

```luau
  kind: MouseEventKind,
```

</h4>

---

### MouseEvent.position

<h4>

```luau
  position: vector,
```

</h4>

 The mouse position where the event occurred as vector of `<column, row, 0>`; access via `position.x` (column) and `position.y` (row)

---

### MouseEvent.modifiers

<h4>

```luau
  modifiers: KeyModifiers,
```

</h4>

---

```luau
} -- closes MouseEvent
```

---

## `export type` MouseEventKind

<h4>

```luau
export type MouseEventKind =
```

</h4>

---

```luau
  | "Down(Left)"
```

 User started left clicking

---

```luau
  | "Down(Right)"
```

 User started right clicking

---

```luau
  | "Down(Middle)"
```

 User started middle clicking

---

```luau
  | "Up(Left)"
```

 User released a left click

---

```luau
  | "Up(Right)"
```

 User released a right click

---

```luau
  | "Up(Middle)"
```

 User released a middle click

---

```luau
  | "Drag(Left)"
```

 User moved the mouse while left click is down

---

```luau
  | "Drag(Right)"
```

 User moved the mouse while right click is down

---

```luau
  | "Drag(Middle)"
```

 User moved the mouse while middle click is down

---

```luau
  | "Moved"
```

 User moved the mouse without a mouse button being down

---

```luau
  | "ScrollDown"
```

 User scrolled the mouse wheel/touchpad downwards (towards the user's lap)

---

```luau
  | "ScrollUp"
```

 User scrolled the mouse wheel/touchpad upwards (away from the user's lap)

---

```luau
  | "ScrollLeft"
```

 User scrolled to the left (usually reported moreso on laptop touchpads)

---

```luau
  | "ScrollRight"
```

 User scrolled to the right (usually reported on laptop touchpads)

---

Autogenerated from [std/terminal/init.luau](/.seal/typedefs/std/terminal/init.luau).

*seal* is best experienced with inline, in-editor documentation. Please see the linked typedefs file if this documentation is confusing, too verbose, or inaccurate.
