<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# err

`local err = require("@std/err")`

$\hspace{5pt}$  Simple library for managing `error` types from seal.

err.message: `(string) -> error`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ Create an `error` with a custom error message. This allows you to return result-like unions that can be `typeof` checked.
$\hspace{5pt}$
$\hspace{5pt}$ ## Usage
$\hspace{5pt}$ ```luau
$\hspace{5pt}$ local err = require("@std/err")
$\hspace{5pt}$
$\hspace{5pt}$ local function canfail(): string | error
$\hspace{5pt}$     if not somecheck() then
$\hspace{5pt}$         return err.message("whoops we failed")
$\hspace{5pt}$     end
$\hspace{5pt}$     return "success"
$\hspace{5pt}$ end
$\hspace{5pt}$
$\hspace{5pt}$ local res = canfail()
$\hspace{5pt}$ if typeof(res) == "error" then
$\hspace{5pt}$     -- error handling
$\hspace{5pt}$ else
$\hspace{5pt}$     -- res should be narrowed to `string`
$\hspace{5pt}$ end
$\hspace{5pt}$```

</details>

err.wrap: `(message: string) -> error`

$\hspace{5pt}$ Wraps an error message with the stack traceback at the location this function was called.
$\hspace{5pt}$
$\hspace{5pt}$ Unlike `err.message`, errors created with this function are red colored and contain their own error tracebacks.

err.format: `(err: error) -> string`

$\hspace{5pt}$  Prettifies an `error` from `pcall` or any of the error returning functions.

err.traceback: `() -> string`

$\hspace{5pt}$  Gets and formats the stack traceback at the current location.

err.throw: `(err: error) -> never`

$\hspace{5pt}$  Takes any error and throws it (causing an error).
$\hspace{5pt}$
$\hspace{5pt}$  If the error already has stack traceback information, throwing the error causes two stack tracebacks to appear (one from the error itself, and a new one because we're causing an error).
