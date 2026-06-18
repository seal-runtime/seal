<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# terminal.cursor

`local cursor = require("@std/terminal/cursor")`

---

### cursor.position

<h4>

```luau
function cursor.position() -> vector,
```

</h4>

Returns the current cursor position as a vector of `<column, row, 0>`.

---

### cursor.show

<h4>

```luau
function cursor.show() -> TerminalAction,
```

</h4>

Creates a <kbd>ShowCursor</kbd> `TerminalAction`.

Once invoked, shows the terminal cursor if hidden.

`TerminalActions` do nothing until invoked; pass the action to `terminal.execute`
to queue and execute multiple commands, or call its `:execute()` method to directly invoke.

---

### cursor.hide

<h4>

```luau
function cursor.hide() -> TerminalAction,
```

</h4>

Creates a <kbd>HideCursor</kbd> `TerminalAction`.

Once invoked, hides the terminal cursor if visible.

`TerminalActions` do nothing until invoked; pass the action to `terminal.execute`
to queue and execute multiple commands, or call its `:execute()` method to directly invoke.

---

### cursor.style

<h4>

```luau
function cursor.style(mode: CursorStyle) -> TerminalAction,
```

</h4>

<details>

<summary> See the docs </summary

Creates a <kbd>SetStyle</kbd> `TerminalAction`.

Once invoked, changes the cursor's style to the specified `CursorStyle` mode.

Supported modes include:

```luau
export type CursorStyle =
    | "Default"
    | "BlinkingBlock"
    | "SteadyBlock"
    | "BlinkingUnderScore"
    | "SteadyUnderScore"
    | "BlinkingBar"
    | "SteadyBar"
```

`TerminalActions` do nothing until invoked; pass the action to `terminal.execute`
to queue and execute multiple commands, or call its `:execute()` method to directly invoke.

</details>

---

### cursor.save

<h4>

```luau
function cursor.save() -> TerminalAction,
```

</h4>

<details>

<summary> See the docs </summary

Creates a <kbd>SavePosition</kbd> `TerminalAction`.

Once invoked, saves the current cursor position so you can move back to it later.
Only one position may be saved at a time.

To move back to the saved position, invoke the <kbd>RestorePosition</kbd> action from `cursor.restore()`.

`TerminalActions` do nothing until invoked; pass the action to `terminal.execute`
to queue and execute multiple commands, or call its `:execute()` method to directly invoke.

</details>

---

### cursor.restore

<h4>

```luau
function cursor.restore() -> TerminalAction,
```

</h4>

<details>

<summary> See the docs </summary

Creates a <kbd>RestorePosition</kbd> `TerminalAction`.

Once invoked, returns the cursor to the last position saved via invoking <kbd>SavePosition</kbd>.
If no position is saved, nothing should happen.

`TerminalActions` do nothing until invoked; pass the action to `terminal.execute`
to queue and execute multiple commands, or call its `:execute()` method to directly invoke.

</details>

---

### cursor.up

<h4>

```luau
function cursor.up(r: number) -> TerminalAction,
```

</h4>

Creates a <kbd>MoveUp</kbd> `TerminalAction`.

Once invoked, moves the cursor up `r` rows.

`TerminalActions` do nothing until invoked; pass the action to `terminal.execute`
to queue and execute multiple commands, or call its `:execute()` method to directly invoke.

---

### cursor.down

<h4>

```luau
function cursor.down(r: number) -> TerminalAction,
```

</h4>

Creates a <kbd>MoveDown</kbd> `TerminalAction`.

Once invoked, moves the cursor down `r` rows.

`TerminalActions` do nothing until invoked; pass the action to `terminal.execute`
to queue and execute multiple commands, or call its `:execute()` method to directly invoke.

---

### cursor.left

<h4>

```luau
function cursor.left(c: number) -> TerminalAction,
```

</h4>

Creates a <kbd>MoveLeft</kbd> `TerminalAction`.

Once invoked, moves the cursor to the left `c` columns.

`TerminalActions` do nothing until invoked; pass the action to `terminal.execute`
to queue and execute multiple commands, or call its `:execute()` method to directly invoke.

---

### cursor.right

<h4>

```luau
function cursor.right(c: number) -> TerminalAction,
```

</h4>

Creates a <kbd>MoveRight</kbd> `TerminalAction`.

Once invoked, moves the cursor to the right `c` columns.

`TerminalActions` do nothing until invoked; pass the action to `terminal.execute`
to queue and execute multiple commands, or call its `:execute()` method to directly invoke.

---

### cursor.to

<h4>

```luau
to: CursorMoveToColumnsAndRows & CursorMoveToPositionVector,
```

</h4>

<details>

<summary> See the docs </summary

Creates a <kbd>MoveTo</kbd> `TerminalAction` that moves the cursor to a specific position.

Once invoked, moves the cursor to the given position.
The position can be either specified as two arguments or as a vector.

This function has two overloads, and can be called with either 1 or 2 arguments:

## Overloads

### Numbers

`(column: number, row: number) -> TerminalAction`

- **column**: must be a positive whole number.
- **row**: must be a positive whole number.

### Vector

`(position: vector) -> TerminalAction`

- **position**: a vector of `<column, row, 0>`; vectors may be created
by `cursor.position()` or `vector.create(x, y, z)`.

`position.x` and `position.y` should both be positive whole numbers.

`TerminalActions` do nothing until invoked; pass the action to `terminal.execute`
to queue and execute multiple commands, or call its `:execute()` method to directly invoke.

</details>

---

### cursor.column

<h4>

```luau
function cursor.column(c: number) -> TerminalAction,
```

</h4>

<details>

<summary> See the docs </summary

Creates a <kbd>MoveTo</kbd> `TerminalAction` that moves the cursor to a specific column.

Once invoked, this action moves the cursor to column `c` of the current row.

`TerminalActions` do nothing until invoked; pass the action to `terminal.execute`
to queue and execute multiple commands, or call its `:execute()` method to directly invoke.

</details>

---

### cursor.row

<h4>

```luau
function cursor.row(r: number) -> TerminalAction,
```

</h4>

Creates a <kbd>MoveTo</kbd> `TerminalAction` that moves the cursor to a specific row.

Once invoked, this action moves the cursor to row `r` at the current column.

`TerminalActions` do nothing until invoked; pass the action to `terminal.execute`
to queue and execute multiple commands, or call its `:execute()` method to directly invoke.

---

### cursor.nextline

<h4>

```luau
function cursor.nextline(n: number?) -> TerminalAction,
```

</h4>

Creates a <kbd>NextLine</kbd> `TerminalAction`.

Once invoked, moves the cursor down `l` lines and resets it to the first column.

`TerminalActions` do nothing until invoked; pass the action to `terminal.execute`
to queue and execute multiple commands, or call its `:execute()` method to directly invoke.

---

### cursor.prevline

<h4>

```luau
function cursor.prevline(n: number?) -> TerminalAction,
```

</h4>

Creates a <kbd>PreviousLine</kbd> `TerminalAction`.

Once invoked, moves the cursor up `l` lines and resets it to the first column.

`TerminalActions` do nothing until invoked; pass the action to `terminal.execute`
to queue and execute multiple commands, or call its `:execute()` method to directly invoke.

---

## `export type` CursorMoveToColumnsAndRows

<h4>

```luau
export type CursorMoveToColumnsAndRows = (column: number, row: number) -> TerminalAction
```

</h4>

---

## `export type` CursorMoveToPositionVector

<h4>

```luau
export type CursorMoveToPositionVector = (position: vector) -> TerminalAction
```

</h4>

---

## `export type` CursorStyle

<h4>

```luau
export type CursorStyle =
```

</h4>

---

```luau
  | "Default"
```

---

```luau
  | "BlinkingBlock"
```

---

```luau
  | "SteadyBlock"
```

---

```luau
  | "BlinkingUnderScore"
```

---

```luau
  | "SteadyUnderScore"
```

---

```luau
  | "BlinkingBar"
```

---

```luau
  | "SteadyBar"
```

---

Autogenerated from [std/terminal/cursor.luau](/.seal/typedefs/std/terminal/cursor.luau).

*seal* is best experienced with inline, in-editor documentation. Please see the linked typedefs file if this documentation is confusing, too verbose, or inaccurate.
