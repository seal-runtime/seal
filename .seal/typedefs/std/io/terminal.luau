export type terminal = {
    --- Clears the terminal screen buffer through crossterm.
    --- For a stronger way to clear the terminal (typically by running `cls` or `clear` directly), use `output.clear` instead.
    clear: (mode: ClearMode?) -> (),
    --- Returns the terminal screen buffer size as a vector <columns, rows, 0>
    size: () -> vector,
    --- Switches between crossterm's Main and Alternate terminal screens.
    switch: (screen: "Main" | "Alternate") -> (),
    --- Enables or disables linewrap in the terminal
    linewrap: (enabled: boolean) -> (),
    --- Resizes the terminal screen buffer.
    resize: (columns: number, rows: number) -> (),
    --- Scroll up or down `lines` lines; negative values scroll up, positive values scroll down.
    scroll: (lines: number) -> (),
    --[=[
        Determine whether we're connected to a rich [TTY](https://en.wikipedia.org/wiki/Tty_(Unix)) terminal.

        This can be used to detect if users can provide rich input at runtime (`io.prompt.*` or `io.input.readline`) or if we're limited to basic stdin (`io.input.rawline`)

        - If `stream` is nil or unspecified, returns true if *all* streams are TTY.
        - Otherwise, returns `true` if `stream` is a TTY.

        If *seal* is being run in a child process, this will almost always return `false` (because users can't easily write input).
    ]=]
    tty: (stream: "Stdout" | "Stderr" | "Stdin") -> boolean,

    --[=[
        The [terminal mode](https://en.wikipedia.org/wiki/Terminal_mode) controls
        how incoming keypresses are handled.

        There are two terminal modes on compliant terminals:
        - **Raw**: programs have full control over keystrokes, can intercept <kbd>Ctrl+C</kbd>
        and <kbd>Ctrl+D</kbd>, and must manually print characters for them to be seen.
        - **Cooked**: the default; intercepts incoming keystrokes, handles <kbd>Ctrl+C</kbd> and
        other terminal signals, formats output in a way consistent for a line-by-line command-line
        interface, and automatically echoes (prints) incoming input to terminal output.

        To write a TUI, you'll want to switch to `"Raw"` mode to get full control over all
        incoming keystrokes. Keep in mind that when using raw mode, keystrokes are not 
        automatically printed to the terminal.
    ]=]
    mode: {
        --[=[
            Change the [terminal mode](https://en.wikipedia.org/wiki/Terminal_mode).

            Always ensure that `terminal.tty()` returns `true` before changing the terminal mode.

            ## Usage

            ```luau
            local terminal = require("@std/io/terminal")
            local time = require("@std/time")

            if not terminal.tty() then
                error("incompatible terminal")
            end

            if terminal.mode.current() == "Cooked" then
                terminal.mode.set("Raw")
                for event in terminal.events(time.milliseconds(10)) do
                    -- insert tui dropdown and selection logic
                end
                terminal.mode.set("Cooked")
            end
            ```

            ## ⚠️ Caution
            
            - Do ***not*** switch the terminal mode in multithreaded programs (`@std/thread`) when
            another thread might be writing to stdout or reading from stdin at the same time. 
            Doing so may cause unexpected behavior.
            - Enabling this in a `ChildProcess (@std/process)` will somehow cause `print` 
            and `output.write` to write to the parent process' stdout (at least on Linux).
            - Different operating systems, terminal emulators, or shells may handle modes differently.
        ]=]
        set: (mode: "Raw" | "Cooked") -> (),
        --[=[
            Returns the current [terminal mode](https://en.wikipedia.org/wiki/Terminal_mode), either `"Raw"` or `"Cooked`".
        ]=]
        current: () -> "Raw" | "Cooked",
    },
    --- Returns an `interrupt` userdata object. For reasons. Maybe control flow.
    interrupt: (key: "CtrlC" | "CtrlD") -> interrupt,
    capture: {
        --- Allows `MouseEvents` to be reported by `terminal.events()`.
        mouse: (enabled: boolean) -> (),
        --- Allows `FocusGained` and `FocusLost` events to be reported by `terminal.events()`.
        focus: (enabled: boolean) -> (),
        --- Allows `Paste` events to be reported by `terminal.events()`.<br>
        --- Might not work correctly when multiple lines are copied.
        paste: (enabled: boolean) -> (),
    },

    events: (poll: Duration) -> (() -> TerminalEvent),
}

type ClearMode =
    | "All"
    | "Purge"
    | "FromCursorDown"
    | "FromCursorUp"
    | "CurrentLine"
    | "UntilNewLine"

--- Note this modifier table is ***REUSED*** across all iterations. Don't try to store it in a table or anything please.
export type KeyModifiers = {
    ctrl: boolean,
    shift: boolean,
    alt: boolean,
}

export type KeyEvent = {
    type: "Key",
    key: string,
    modifiers: KeyModifiers,
}
export type MouseEvent = {
    type: "Mouse",
    kind: string,
    column: number,
    row: number,
    modifiers: KeyModifiers,
}
export type ResizeEvent = {
    type: "Resize",
    columns: number,
    rows: number,
}
export type FocusGained = {
    type: "FocusGained",
}
export type FocusLost = {
    type: "FocusLost",
}
export type PasteEvent = {
    type: "Paste",
    contents: string,
}
export type Empty = {
    type: "Empty",
}

export type TerminalEvent = 
    | KeyEvent 
    | MouseEvent
    | ResizeEvent
    | PasteEvent
    | FocusGained 
    | FocusLost 
    | Empty

return {} :: terminal