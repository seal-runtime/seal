--[=[
    Format objects for pretty printing to stdout/stderr.
]=]
export type format = setmetatable<{
    --[=[
        Formats `item` in the same way as `print` or `pp`.

        Pass `options` to change the behavior of the formatter.
        If you want to change ALL print/format.pretty/etc. formatting
        throughout seal, call `format.defaults`.
    ]=]
    pretty: (item: unknown, options: FormatOptions?, current_depth: number?) -> string,
    --[=[
        Sets the default `FormatOptions` applied to all subsequent `print`, `format.pretty`, `format`, and `pp` calls.
        Pass `nil` to revert to built-in defaults.
    ]=]
    defaults: (options: FormatOptions?) -> (),
    --[=[
        Like pretty printing but without colors.
    ]=]
    simple: (item: unknown) -> string,
    --[=[
        Prints the debug representation of `item`, equivalent to using `{:?}` in Rust.
    ]=]
    debug: (item: unknown) -> string,
    --[=[
        Removes ANSI color codes from a pretty formatted string.
    ]=]
    uncolor: (s: string) -> string,
    --[=[
        Returns a hexdump-formatted string of the provided string or buffer, similar to `dp`.
    ]=]
    hexdump: (data: string | buffer) -> string,
}, { 
    __call: (self: any, item: unknown, options: FormatOptions?, current_depth: number?) -> string,
}>

--- Customize how your output looks.
export type FormatOptions = {
    --- How many levels deep into a nested table before showing `{ ... }`; 1 shows top level table, defaults to 7.
    max_depth: number?,
    --- How many spaces to indent, defaults to 4
    indent_spaces: number?,
    --- Limit the number of elements printed in an array; defaults to 42
    max_elements_in_array: number?,
    --- Do you want to see `[1] = value`? Defaults to true
    show_array_indices: boolean?,
    --- Show `@metatable = ...` when a table has a metatable. Defaults to true
    show_metatables: boolean?,
    --- Show vertical guide lines at each indent level, colored by depth. Defaults to true when indent_spaces < 3
    guidelines: boolean?,
    --- Show the total number of items at the opening brace. Defaults to true when in a TTY
    show_array_length: boolean?,
}

return {} :: format