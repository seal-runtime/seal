<!-- markdownlint-disable MD033 -->

# Format

`export type format = setmetatable<{`

<details>

<summary> Docs </summary

Format objects for pretty printing to stdout/stderr.

</details>

`function format.pretty(item: unknown): string`

<details>

<summary> Docs </summary

Formats `item` in the same way as `print` or `pp`.

</details>

`function format.simple(item: unknown): string`

<details>

<summary> Docs </summary

Like pretty printing but without colors.

</details>

`function format.debug(item: unknown): string`

<details>

<summary> Docs </summary

Prints the debug representation of `item`, equivalent to using `{:?}` in Rust.

</details>

`function format.uncolor(s: string): string`

<details>

<summary> Docs </summary

Removes ANSI color codes from a pretty formatted string.

</details>
