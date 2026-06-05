--[=[
    This library is for writing TUI applications with *seal*;
    if you're looking to run terminal commands, you probably want the `@std/process` library instead.
]=]
export type terminal = {
    --- Returns the terminal screen buffer size as a vector of <columns, rows, 0>
    size: () -> vector,
    --[=[
        Determine whether we're connected to a rich [TTY](https://en.wikipedia.org/wiki/Tty_(Unix)) terminal.

        This can be used to detect if users can provide rich input at runtime (`io.prompt.*` or `io.input.readline`) or if we're limited to basic stdin (`io.input.rawline`)

        - If `stream` is nil or unspecified, returns true if *all* streams are TTY.
        - Otherwise, returns `true` if `stream` is a TTY.

        If *seal* is being run in a child process, this will almost always return `false` (because users can't easily write input).
    ]=]
    tty: (stream: "Stdout" | "Stderr" | "Stdin"?) -> boolean,

    --[=[
        Creates a <kbd>Write</kbd> `TerminalAction` that once invoked, writes
        to stdout without a trailing newline.

        Please note that this function only accepts *valid* UTF-8 encoded `content`,
        if you want  to write arbitrary bytes to stdout or stderr, use `output.write` 
        and `output.ewrite` from `@std/io/output` instead.

        `TerminalActions` do nothing until invoked; pass the action to `terminal.execute` 
        to queue and execute multiple commands, or call its `:execute()` method to directly invoke.
    ]=]
    write: (content: string) -> TerminalAction,
    --[=[
        Creates a <kbd>Clear</kbd> `TerminalAction`.

        Once invoked, visually clears the terminal by writing escape codes
        to move existing content into the terminal scrollback buffer. Unlike 
        `io.output.clear`, this `TerminalAction` doesn't actually run `clear`
        or `cls`, making it faster but less strong.

        If you really want to clear the terminal and prevent the user from
        scrolling back, use `io.output.clear` from `@std/io/output` instead.

        `TerminalActions` do nothing until invoked; pass the action to `terminal.execute` 
        to queue and execute multiple commands, or call its `:execute()` method to directly invoke.
    ]=]
    clear: (mode: ClearMode?) -> TerminalAction,
    --[=[
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
    ]=]
    switch: (screen: "Main" | "Alternate") -> TerminalAction,
    --[=[
        Creates a `TerminalAction` that when invoked, enables or disables terminal linewrapping.
        
        `TerminalActions` do nothing until invoked; pass the action to `terminal.execute` 
        to queue and execute multiple commands, or call its `:execute()` method to directly invoke.
    ]=]
    linewrap: (enabled: boolean) -> TerminalAction,
    --[=[
        Creates a <kbd>Scroll</kbd> `TerminalAction` that when invoked, scrolls the terminal.

        Negative values scroll up, positive values scroll down.

        `TerminalActions` do nothing until invoked; pass the action to `terminal.execute` 
        to queue and execute multiple commands, or call its `:execute()` method to directly invoke.
    ]=]
    scroll: (lines: number) -> TerminalAction,

    --[=[
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
    ]=]
    rawmode: {
        --- Determine if the terminal is currently in rawmode.
        enabled: () -> boolean,
        --[=[
            Switches the [terminal mode](https://en.wikipedia.org/wiki/Terminal_mode) to rawmode.

            Always ensure that `terminal.tty()` returns `true` before changing the terminal mode.

            ## ⚠️ Caution
            
            - Do ***not*** switch the terminal mode in multithreaded programs (`@std/thread`) when
            another thread might be writing to stdout or reading from stdin at the same time. 
            Doing so may cause unexpected behavior.
            - Enabling this in a `ChildProcess (@std/process)` will somehow cause `print` 
            and `output.write` to write to the parent process' stdout (at least on Linux).
            - Different operating systems, terminal emulators, or shells may handle modes differently.
        ]=]
        enable: () -> (),
        --[=[
            Switches back to **cooked** mode.
        ]=]
        disable: () -> (),
    },
    interrupt: {
        --- Returns an interrupt for Ctrl+C (SIGINT).
        sigint: () -> interrupt,
        --- Returns an interrupt for Ctrl+D (EOF).
        eof: () -> interrupt,
        --[=[
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
        ]=]
        check: (event: TerminalEvent) -> interrupt?,
    },
    capture: {
        --- Allows `MouseEvents` to be reported by `terminal.events()`.
        mouse: (enabled: boolean) -> (),
        --- Allows `FocusGained` and `FocusLost` events to be reported by `terminal.events()`.
        focus: (enabled: boolean) -> (),
        --- Allows `Paste` events to be reported by `terminal.events()`.<br>
        --- Might not work correctly when multiple lines are copied.
        paste: (enabled: boolean) -> (),
    },
    --[=[
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
    ]=]
    events: (poll: Duration) -> (() -> TerminalEvent),
    --[=[
        Resets the terminal to its default state.

        Disables raw mode (if enabled), switches back to the Main screen,
        enables line wrap, resets the cursor to column 0, sets the cursor
        to the default style, makes the cursor visible, and disables mouse,
        paste, and focus-changed event capturing.
        This is useful for cleanup when exiting a TUI.

        This function is idempotent and safe to call multiple times.
    ]=]
    reset: () -> (),
    --[=[
        Queue and execute multiple `TerminalActions` efficiently when writing a TUI.

        Uses crossterm's queue API to schedule all `TerminalActions` and invoke them
        with a single flush operation, thereby reducing syscalls and IO overhead.
        
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
                    cursor.to(pos.x, pos.y),
                    terminal.write(keyframes[frame] .. " " .. message)
                )
                time.wait(0.08)
            end

            -- Clear the spinner and restore cursor
            terminal.execute(
                cursor.to(pos.x, pos.y),
                terminal.write(string.rep(" ", #message + 2)),
                cursor.show(),
                cursor.column(0)
            )
        end

        if terminal.tty() then
            spinner("Processing", 3)
        end
        ```
    ]=]
    execute: (...TerminalAction) -> ()
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