<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# err

`local err = require("@std/err")`

$\hspace{5pt}$ --- Simple library for managing `error` types from seal.

err.message: `(string) -> error`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ Create an `error` with a custom error message. This allows you to return result-like unions that can be `typeof` checked.
$\hspace{5pt}$
$\hspace{5pt}$ ## Usage

```luau
local err = require("@std/err")

local function canfail(): string | error
    if not somecheck() then
        return err.message("whoops we failed")
    end
    return "success"
end

local res = canfail()
if typeof(res) == "error" then
    -- error handling
else
    -- res should be narrowed to `string`
end
$\hspace{5pt}$ ```

</details>


err.wrap: `(message: string) -> error`

$\hspace{5pt}$ Wraps an error message with the stack traceback at the location this function was called.
$\hspace{5pt}$ 
$\hspace{5pt}$ Unlike `err.message`, errors created with this function are red colored and contain their own error tracebacks.

err.format: `(err: error) -> string`

$\hspace{5pt}$ --- Prettifies an `error` from `pcall` or any of the error returning functions.

err.traceback: `() -> string`

$\hspace{5pt}$ --- Gets and formats the stack traceback at the current location.

err.throw: `(err: error) -> never`

$\hspace{5pt}$ --- Takes any error and throws it (causing an error).
$\hspace{5pt}$ ---
$\hspace{5pt}$ --- If the error already has stack traceback information, throwing the error causes two stack tracebacks to appear (one from the error itself, and a new one because we're causing an error).
