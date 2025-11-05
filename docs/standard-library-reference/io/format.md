<!-- markdownlint-disable MD033 -->

# Format

Format objects for pretty printing to stdout/stderr.

`function format.pretty(item: unknown): string`

<details>

<summary> See the docs </summary

Formats `item` in the same way as `print` or `pp`.

</details>

`function format.simple(item: unknown): string`

<details>

<summary> See the docs </summary

Like pretty printing but without colors.

</details>

`function format.debug(item: unknown): string`

<details>

<summary> See the docs </summary

Prints the debug representation of `item`, equivalent to using `{:?}` in Rust.

</details>

`function format.uncolor(s: string): string`

<details>

<summary> See the docs </summary

Removes ANSI color codes from a pretty formatted string.

</details>
