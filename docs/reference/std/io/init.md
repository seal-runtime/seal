<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# io

`local io = require("@std/io")`

$hspace{5pt}$Standard library for handling **terminal** input/output.
$hspace{5pt}$
$hspace{5pt}$You can require the whole `@std/io` lib at once but it's recommended you require individual
$hspace{5pt}$libraries as you need them instead.
$hspace{5pt}$
$hspace{5pt}$- To read input from the terminal, use `@std/io/prompt` or if you need lower-level control, `@std/io/input`.
$hspace{5pt}$- To write directly to stdout/stderr, use `@std/io/output`.
$hspace{5pt}$- Want to format your tables in the same pretty way as `print`? Use `@std/io/format`.
$hspace{5pt}$- If you want to be colorful, use `@std/io/colors`, which is aliased to just `@std/colors` as well (due to high traffic).

io.input: `typeof(require("@self/input"))`

io.output: `typeof(require("@self/output"))`

io.colors: `typeof(require("@self/colors"))`

io.format: `typeof(require("@self/format"))`

io.prompt: `typeof(require("@self/prompt"))`
