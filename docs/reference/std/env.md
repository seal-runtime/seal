<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# env

`local env = require("@std/env")`

A stdlib to interact with the script's running environment.

.args: `{string}`

 a list of arguments passed to the program

.os: `"Windows" | "Linux" | "Android" | "MacOS" | "Other"`

 your operating system

.executable_path: `string`

 the path of the executable

.cwd: `() -> string`

Get the current working directory of the running process.

Errors if the `cwd` doesn't exist or otherwise isn't accessible (permission denied).

.getvar: `(key: string) -> string?`

Gets an environment variable in the current process.

.setvar: `(key: string, value: string) -> string`

Sets an environment variable in the current process.

Note, this function is **unsafe** in multithreaded contexts on Linux.

.removevar: `(key: string) -> nil`

Removes an environment variable in the current process.

Note, this function is **unsafe** in multithreaded contexts on Linux.
