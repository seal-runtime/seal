<!-- markdownlint-disable MD033 -->

# Err

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
