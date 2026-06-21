# 0.9.0

## Configurable colors and formatting

*seal* now respects the [NO_COLOR](https://no-color.org/) specification.

### New features in std/io/format

Added `format.FormatOptions` which you can pass to `format.pretty` and `format()`; pass it to `format.defaults` to override the behavior of `print` and `pp`.

These options now include these new features:

- print guidelines (automatically enabled when indent < 3)
- configurable number of spaces to indent over
- enable/disable printing metatables
- limit the amount of array elements shown
- enable/disable array indices (if you only care about seeing the values)
- max table depth after which the formatter stops recursing.

General formatter improvements:

- Formatter now prints function names when available and not equivalent to their table key value.
- Formatter now prints snowflakes around frozen tables.

### New features in std/terminal

- `terminal.background` which gets the current background color of the terminal if supported. Might not be supported on all platforms/terminals.

### Changes to std/io/colors

- `colors.rgb` which allows generating arbitrary terminal colors.
- `colors.override` enables or disables colors at runtime.
- `colors.enabled` checks if colors are enabled or disabled for whatever reason (NO_COLOR/etc.)

## Implement `_RUNTIME` and `_LUAU` specification, modify `_VERSION`

I implemented [Bottersnike's specification](https://gist.github.com/Bottersnike/001470cbbb0cd63d9790a542ed5be1bf) so it's easier for portable code to branch on Luau runtimes.

### Breaking change to `_VERSION`

`_VERSION` is now in the form `seal <_RUNTIME.version.display>+<_LUAU.version.display>` (`seal 0.0.8-rc.2+0.709`) to better match other runtimes such as Lune and Zune.

## Implement `@std/terminal` and `@std/terminal/cursor`

Implements new TUI libraries `@std/terminal` and its sublib `@std/terminal/cursor`, moving relevant functions from `@std/io/input` and `@std/io/output` to the new library, most importantly TUI-related functions and terminal event watching.

The biggest addition here is the ability to queue and execute terminal TUI commands with crossterm, giving us our first manual-flush-control terminal writing API in *seal* with `terminal.execute(terminal.write(content))`.

Many of the existing auto-executing crossterm commands were converted into ones that instead return a `TerminalAction` extern type, which represents a queued crossterm `Command`; these must be invoked to be used and may be invoked by passing them to `terminal.execute` or by invoking their `:execute` method directly. From an API user standpoint, this is a bit more work than just having functions but allows for a more composable API that also provides flush control for efficiency reasons -- flushing is a relatively expensive syscall so we don't want to invoke it more than necessary. `terminal.execute` also executes all commands in a synchronized output block to eliminate visual flicker and bugs (on supported terminals).

### Breaking changes (APIs moved)

| Feature | Old location | New location |
| ------------ | ------------- | -------------- |
| TTY check | `io.input.tty()` | `terminal.tty()` |
| Raw mode | `io.input.rawmode(enabled)` | `terminal.rawmode.enable()` / `.disable()` |
| Event loop | `io.input.events(poll)` | `terminal.events(poll)` |
| Mouse capture | `io.input.capture.mouse()` | `terminal.capture.mouse()` |
| Focus capture | `io.input.capture.focus()` | `terminal.capture.focus()` |
| Paste capture | `io.input.capture.paste()` | `terminal.capture.paste()` |
| Terminal size | `io.output.size()` | `terminal.size()` |
| Switch screen | `io.output.switch(screen)` | `terminal.execute(terminal.switch(screen))` |
| Resize terminal | `io.output.resize(cols, rows)` | *(removed, see note)* |
| Cursor position | `io.output.cursor()` | `cursor.position()` |
| Interrupt (CtrlC) | `io.input.interrupt("CtrlC")` | `terminal.interrupt.sigint()` |
| Interrupt (CtrlD) | `io.input.interrupt("CtrlD")` | `terminal.interrupt.eof()` |

`output.resize` sent `crossterm::terminal::SetSize` to ask the terminal emulator to physically resize its own window. Most terminal emulators ignore this escape sequence entirely. It has been removed with no replacement — if you genuinely need this, write the escape sequence directly via `output.write`.

### Breaking changes (API surface changed)

All terminal event types previously exported from `@std/io/input` are now defined in `@std/terminal`. The event enum type discriminant field has been **renamed from `.is` to `.type`** across all event variants:

| Old field | New field |
| ----------- | ----------- |
| `event.is == "Key"` | `event.type == "Key"` |
| `event.is == "Mouse"` | `event.type == "Mouse"` |
| `event.is == "Resize"` | `event.type == "Resize"` |
| `event.is == "Paste"` | `event.type == "Paste"` |
| `event.is == "FocusGained"` | `event.type == "FocusGained"` |
| `event.is == "FocusLost"` | `event.type == "FocusLost"` |
| `event.is == "Empty"` | `event.type == "Empty"` |

#### Additional structural changes to specific event types

**`KeyEvent`**

- `event.key` keys are now canonicalized and standardized across all operating systems, so you don't have to check `key == "Enter" or key == "Return"` for macOS compatibility. I've noted all such OS-specific keys in the documentation so you know what to expect.
- `key: string` is now typed as `key: KeyCode`; `KeyCode` is a fully typed union of all keys expected.
- Key press state is now exposed as `event.kind: KeyEventKind` (`"Press" | "Release" | "Repeat"`).

**`MouseEvent`**

- `column: number` and `row: number` fields removed and replaced with `position: vector` (access as `event.position.x` for column, `event.position.y` for row) for consistency with vector-returning APIs such as `screen.size()` and `cursor.position()`. It's easier to do math on vectors that store columns and rows in the same datatype instead of needing to handle such manually.
- `event.kind` is now strictly typed as `kind: MouseEventKind` where `MouseEventKind` is a enum type union of the possible mouse event kinds (e.g. `"Down(Left)`, `"ScrollUp"`, `"Moved"`, ...). These event kinds should be identical to those produced in the old implementation but now are directly specified instead of relying on crossterm's debug representation of MouseEventKind staying stable.

**`ResizeEvent`**

- Similarly to `MouseEvent`, `ResizeEvent` has its `columns: number` and `rows: number` fields removed and replaced with `size: vector` (access as `event.size.x` for columns, `event.size.y` for rows) for consistency with vector APIs such as `MouseEvent.position`, `screen.size()`, and `cursor.position()`.

### New features

- `terminal.execute(...TerminalAction)` - queue and execute multiple terminal actions with a single flush
- `terminal.write(content: string) -> TerminalAction` - write to stdout without a newline; unlike `io.output.write`, accepts only valid UTF-8 and returns a `TerminalAction`
- `terminal.clear(mode: ClearMode?) -> TerminalAction` - clear the terminal via escape codes; unlike `io.output.clear`, does not invoke `clear`/`cls`
- `terminal.scroll(lines: number) -> TerminalAction` - scroll the terminal up (negative) or down (positive)
- `terminal.linewrap(enabled: boolean) -> TerminalAction` - enable or disable terminal line wrapping
- `terminal.title(name: string) -> TerminalAction` - set the terminal window title
- `terminal.rawmode.enabled() -> boolean` - check whether the terminal is currently in raw mode
- `terminal.interrupt.check(event: TerminalEvent) -> interrupt?` - check if an event is a Ctrl+C or Ctrl+D interrupt; simplifies interrupt handling inside event loops
- `terminal.reset()` - restore the terminal to its default state (disables raw mode, switches to Main screen, re-enables line wrap, resets and shows cursor, disables all captures); safe to call multiple times

#### `@std/terminal/cursor` library

The cursor sublib handles highly requested cursor positioning and styling functionality. This means you no longer need to rely on your own ANSI code libraries to write TUIs in *seal* :)

| Function | Description |
| -------- | ----------- |
| `cursor.position() -> vector` | Returns the current cursor position as `<column, row, 0>` |
| `cursor.show() -> TerminalAction` | Shows the terminal cursor if hidden |
| `cursor.hide() -> TerminalAction` | Hides the terminal cursor if visible |
| `cursor.style(mode: CursorStyle) -> TerminalAction` | Changes cursor style (block, bar, underline; blinking or steady) |
| `cursor.save() -> TerminalAction` | Saves the current cursor position (one slot) |
| `cursor.restore() -> TerminalAction` | Restores cursor to the last saved position |
| `cursor.up(r: number) -> TerminalAction` | Moves cursor up `r` rows |
| `cursor.down(r: number) -> TerminalAction` | Moves cursor down `r` rows |
| `cursor.left(c: number) -> TerminalAction` | Moves cursor left `c` columns |
| `cursor.right(c: number) -> TerminalAction` | Moves cursor right `c` columns |
| `cursor.to(column: number, row: number) -> TerminalAction` | Moves cursor to an absolute position |
| `cursor.to(position: vector) -> TerminalAction` | Moves cursor to an absolute position (vector overload) |
| `cursor.column(c: number) -> TerminalAction` | Moves cursor to column `c` of the current row |
| `cursor.row(r: number) -> TerminalAction` | Moves cursor to row `r` at the current column |
| `cursor.nextline(n: number?) -> TerminalAction` | Moves cursor down `n` lines and resets to first column |
| `cursor.prevline(n: number?) -> TerminalAction` | Moves cursor up `n` lines and resets to first column |

#### `prompt.pick`

Rewritten completely to take advantage of the new `@std/terminal` libraries; the function is now much more featureful with a much more responsive TUI interface, more UX modes to select options and with better handling of non-ASCII UTF-8 text. You can now use the scrollwheel to select options, and now you can scroll up and down inside the picker which correctly handles vertical resizing to fit more/fewer options depending on space available.

#### New functions added to existing libraries

- `io.output.writeln(content: string | buffer) -> error?` - write to stdout with a trailing newline
- `io.output.ewriteln(content: string | buffer) -> error?` - write to stderr with a trailing newline
- `io.input.read() -> string?` - reads from stdin until reaching EOF, similar to Lune's `stdio.readToEnd`. Returns `nil` if stdin was empty.

### Fixes

- Standard output and stderr-writing functions `output.write` and `output.ewrite` no longer are documented to have a `flush` parameter - the parameter was useless and didn't work. For manual control over flushes, use `terminal.write` and `terminal.execute` instead.
- `prompt.pick` from `@std/io/prompt` was completely rewritten to be more stable.
- `io.input.rawline` now accepts invalid utf-8 input.

### Known issues

- `prompt.pick` does not correctly handle fast horizontal resizing, sometimes clobbering previously-outputted text above the picker's `top_position`. This is a limitation of TUIs that don't occupy the full terminal screen and cannot be fixed. Users are advised not to quickly horizontally resize the prompt picker.
