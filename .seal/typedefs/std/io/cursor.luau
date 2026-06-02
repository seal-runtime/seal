export type cursor = {
    --- Returns the current cursor position as a vector `<column, row, 0>`.
    position: () -> vector,
    --- Shows the cursor if it was hidden.
    show: () -> (),
    --- Hides the cursor.
    hide: () -> (),
    --- Changes the cursor style/shape.
    style: (mode: CursorStyle) -> (),
    --[=[
        Queues and flushes one or more cursor movement commands via 
        crossterm's queue API.

        `cursor.move` batches all commands and flushes stdout only once,
        avoiding the overhead of a system call per command.

        ## Usage

        ```luau
        local cursor = require("@std/io/cursor")
        local output = require("@std/io/output")

        cursor.move(
            cursor.commands.save(),
            cursor.commands.row(3),
            cursor.commands.right(2)
        )
        output.writeln("something")
        cursor.move(
            cursor.commands.restore(),
            cursor.commands.nextline()
        )
        ```
    ]=]
    move: (...CursorCommand) -> (),
    commands: {
        --- Save the current cursor position.
        save: () -> CursorCommand,
        --- Restore the previously saved cursor position.
        restore: () -> CursorCommand,
        --- Move cursor up `n` rows.
        up: (n: number) -> CursorCommand,
        --- Move cursor down `n` rows.
        down: (n: number) -> CursorCommand,
        --- Move cursor left `cols` columns.
        left: (cols: number) -> CursorCommand,
        --- Move cursor right `cols` columns.
        right: (cols: number) -> CursorCommand,
        --[=[
            Move the cursor to the given position, either specified as arguments
            or in a vector.

            This function has two overloads, and can be called with either 1 or 2 arguments:

            ## Overloads

            ### Numbers
            `(columns: number, rows: number) -> CursorCommand`

            - **column**: must be a positive whole number.
            - **row**: must be a positive whole number.

            ### Vector
            `(position: vector) -> CursorCommand`

            - **position**: a vector of `<columns, rows, 0>`; vectors may be created
            by `cursor.position()` or `vector.create(x, y, z)`.
            
            `position.x` and `position.y` should both be positive whole numbers.
        ]=]
        to: CursorMoveToColumnsAndRows & CursorMoveToPositionVector,
        --- Move cursor to column `c`.
        column: (c: number) -> CursorCommand,
        --- Move cursor to row `r`.
        row: (r: number) -> CursorCommand,
        --- Move down `n` (default 1) lines, resetting cursor to the first column.
        nextline: (n: number?) -> CursorCommand,
        --- Move up `n` (default 1) lines, resetting cursor to the first column.
        prevline: (n: number?) -> CursorCommand,
    },
}

export type CursorMoveToColumnsAndRows = (column: number, row: number) -> CursorCommand
export type CursorMoveToPositionVector = (position: vector) -> CursorCommand

export type CursorStyle =
    | "Default"
    | "BlinkingBlock"
    | "SteadyBlock"
    | "BlinkingUnderScore"
    | "SteadyUnderScore"
    | "BlinkingBar"
    | "SteadyBar"

return {} :: cursor
