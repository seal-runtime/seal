<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# io.format

`local format = require("@std/io/format")`

Format objects for pretty printing to stdout/stderr.

format.pretty: `(item: unknown) -> string`

Formats `item` in the same way as `print` or `pp`.

---

format.simple: `(item: unknown) -> string`

Like pretty printing but without colors.

---

format.debug: `(item: unknown) -> string`

Prints the debug representation of `item`, equivalent to using `{:?}` in Rust.

---

format.uncolor: `(s: string) -> string`

Removes ANSI color codes from a pretty formatted string.

---

format.__call: `(self: any, item: unknown) -> string`

---
