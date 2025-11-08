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

## io.input

<h4>

```luau
io.input: typeof(require("@self/input")),
```

</h4>

---

## io.output

<h4>

```luau
io.output: typeof(require("@self/output")),
```

</h4>

---

## io.colors

<h4>

```luau
io.colors: typeof(require("@self/colors")),
```

</h4>

---

## io.format

<h4>

```luau
io.format: typeof(require("@self/format")),
```

</h4>

---

## io.prompt

<h4>

```luau
io.prompt: typeof(require("@self/prompt")),
```

</h4>

---
