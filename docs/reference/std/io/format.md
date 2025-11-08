<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# io.format

`local format = require("@std/io/format")`

Format objects for pretty printing to stdout/stderr.

---

### format.pretty

<h4>

```luau
format.pretty: (item: unknown) -> string,
```

</h4>

Formats `item` in the same way as `print` or `pp`.

---

### format.simple

<h4>

```luau
format.simple: (item: unknown) -> string,
```

</h4>

Like pretty printing but without colors.

---

### format.debug

<h4>

```luau
format.debug: (item: unknown) -> string,
```

</h4>

Prints the debug representation of `item`, equivalent to using `{:?}` in Rust.

---

### format.uncolor

<h4>

```luau
format.uncolor: (s: string) -> string,
```

</h4>

Removes ANSI color codes from a pretty formatted string.

---

### format.__call

<h4>

```luau
format.__call: (self: any, item: unknown) -> string,
```

</h4>

---
