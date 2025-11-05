<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# Err

 Simple library for managing `error` types from seal.

`function err.message(string): error`

<details>

<summary> See the docs </summary

Create an `error` with a custom error message. This allows you to return result-like unions that can be `typeof` checked.

## Usage

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
```

</details>

`function err.wrap(message: string): error`

Wraps an error message with the stack traceback at the location this function was called.

Unlike `err.message`, errors created with this function are red colored and contain their own error tracebacks.

`function err.format(err: error): string`

 Prettifies an `error` from `pcall` or any of the error returning functions.

`function err.traceback(): string`

 Gets and formats the stack traceback at the current location.

`function err.throw(err: error): never`

 Takes any error and throws it (causing an error).

 If the error already has stack traceback information, throwing the error causes two stack tracebacks to appear (one from the error itself, and a new one because we're causing an error).
