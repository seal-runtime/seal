<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# io.format

`local format = require("@std/io/format")`

Format objects for pretty printing to stdout/stderr.

---

<h3>

```luau
format.pretty: (item: unknown) -> string,
```

</h3>

Formats `item` in the same way as `print` or `pp`.

---

<h3>

```luau
format.simple: (item: unknown) -> string,
```

</h3>

Like pretty printing but without colors.

---

<h3>

```luau
format.debug: (item: unknown) -> string,
```

</h3>

Prints the debug representation of `item`, equivalent to using `{:?}` in Rust.

---

<h3>

```luau
format.uncolor: (s: string) -> string,
```

</h3>

Removes ANSI color codes from a pretty formatted string.

---

<h3>

```luau
format.__call: (self: any, item: unknown) -> string,
```

</h3>

---
