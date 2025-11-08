<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# io

`local io = require("@std/io")`

Standard library for handling **terminal** input/output.

You can require the whole `@std/io` lib at once but it's recommended you require individual
libraries as you need them instead.

- To read input from the terminal, use `@std/io/prompt` or if you need lower-level control, `@std/io/input`.
- To write directly to stdout/stderr, use `@std/io/output`.
- Want to format your tables in the same pretty way as `print`? Use `@std/io/format`.
- If you want to be colorful, use `@std/io/colors`, which is aliased to just `@std/colors` as well (due to high traffic).

---

<h3>

```luau
io.input: typeof(require("@self/input")),
```

</h3>

---

<h3>

```luau
io.output: typeof(require("@self/output")),
```

</h3>

---

<h3>

```luau
io.colors: typeof(require("@self/colors")),
```

</h3>

---

<h3>

```luau
io.format: typeof(require("@self/format")),
```

</h3>

---

<h3>

```luau
io.prompt: typeof(require("@self/prompt")),
```

</h3>

---
