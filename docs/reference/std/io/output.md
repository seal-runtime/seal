<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# io.output

`local output = require("@std/io/output")`

$\hspace{5pt}$ Write to the terminal's stdout/stderr.

output.write: `(contents: string | buffer, flush: boolean?) -> error?`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ Writes `contents` to stdout without any intermediate utf-8 validation, flushing the stream immediately.
$\hspace{5pt}$
$\hspace{5pt}$ ## Windows Portability Concerns
$\hspace{5pt}$
$\hspace{5pt}$ On Windows, this function may return an error if `contents` is invalid utf-8.
$\hspace{5pt}$
$\hspace{5pt}$ Additionally, on Windows it may fail silently when used in a child process.
$\hspace{5pt}$
$\hspace{5pt}$ ## Returns
$\hspace{5pt}$
$\hspace{5pt}$ - An `error` instance if either writing to the stream failed or flushing the stream failed.
$\hspace{5pt}$
$\hspace{5pt}$ ## Usage
$\hspace{5pt}$
$\hspace{5pt}$ ```luau
$\hspace{5pt}$ local err = output.write("idk")
$\hspace{5pt}$ if err then
$\hspace{5pt}$     warn(`error writing to stdout: {err}`)
$\hspace{5pt}$ end
$\hspace{5pt}$```

</details>

output.ewrite: `(contents: string | buffer, flush: boolean?) -> error?`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ Writes `contents` to stderr without any intermediate utf-8 validation, flushing the stream immediately.
$\hspace{5pt}$
$\hspace{5pt}$ ## Windows Portability Concerns
$\hspace{5pt}$
$\hspace{5pt}$ On Windows, this function may return an error if `contents` is invalid utf-8.
$\hspace{5pt}$
$\hspace{5pt}$ Additionally, on Windows it may fail silently when used in a child process.
$\hspace{5pt}$
$\hspace{5pt}$ ## Returns
$\hspace{5pt}$
$\hspace{5pt}$ - An `error` instance if either writing to the stream failed or flushing the stream failed.
$\hspace{5pt}$
$\hspace{5pt}$ ## Usage
$\hspace{5pt}$
$\hspace{5pt}$ ```luau
$\hspace{5pt}$ local err = output.ewrite("error message\n")
$\hspace{5pt}$```

</details>

output.clear: `() -> ()`

$\hspace{5pt}$  clears stdout akin to `cls` or `clear`.
