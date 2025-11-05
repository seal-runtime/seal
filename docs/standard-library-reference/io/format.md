<!-- markdownlint-disable MD033 -->

# Format

[=[
Format objects for pretty printing to stdout/stderr.

`function format.pretty(item: unknown): string`

--[=[
Formats `item` in the same way as `print` or `pp`.

`function format.simple(item: unknown): string`

--[=[
Like pretty printing but without colors.

`function format.debug(item: unknown): string`

--[=[
Prints the debug representation of `item`, equivalent to using `{:?}` in Rust.

`function format.uncolor(s: string): string`

--[=[
Removes ANSI color codes from a pretty formatted string.
