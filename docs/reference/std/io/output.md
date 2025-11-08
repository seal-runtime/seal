<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# io.output

`local output = require("@std/io/output")`

Write to the terminal's stdout/stderr.

---

<h3>
```luau
output.write: (contents: string | buffer, flush: boolean?) -> error?,
```
</h3>

<details>

<summary> See the docs </summary

Writes `contents` to stdout without any intermediate utf-8 validation, flushing the stream immediately.

## Windows Portability Concerns

On Windows, this function may return an error if `contents` is invalid utf-8.

Additionally, on Windows it may fail silently when used in a child process.

## Returns

- An `error` instance if either writing to the stream failed or flushing the stream failed.

## Usage

```luau
local err = output.write("idk")
if err then
    warn(`error writing to stdout: {err}`)
end
```

</details>

---

<h3>
```luau
output.ewrite: (contents: string | buffer, flush: boolean?) -> error?,
```
</h3>

<details>

<summary> See the docs </summary

Writes `contents` to stderr without any intermediate utf-8 validation, flushing the stream immediately.

## Windows Portability Concerns

On Windows, this function may return an error if `contents` is invalid utf-8.

Additionally, on Windows it may fail silently when used in a child process.

## Returns

- An `error` instance if either writing to the stream failed or flushing the stream failed.

## Usage

```luau
local err = output.ewrite("error message\n")
```

</details>

---

<h3>
```luau
output.clear: () -> (),
```
</h3>

 clears stdout akin to `cls` or `clear`.

---
