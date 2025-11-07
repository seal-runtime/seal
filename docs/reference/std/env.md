<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# env

`local env = require("@std/env")`

$\hspace{5pt}$ A stdlib to interact with the script's running environment.

.args: `{string}`

$\hspace{5pt}$ --- a list of arguments passed to the program

.os: `"Windows" | "Linux" | "Android" | "MacOS" | "Other"`

$\hspace{5pt}$ --- your operating system

.executable_path: `string`

$\hspace{5pt}$ --- the path of the executable

.cwd: `() -> string`

$\hspace{5pt}$ Get the current working directory of the running process.
$\hspace{5pt}$
$\hspace{5pt}$ Errors if the `cwd` doesn't exist or otherwise isn't accessible (permission denied).

.getvar: `(key: string) -> string?`

$\hspace{5pt}$ Gets an environment variable in the current process.

.setvar: `(key: string, value: string) -> string`

$\hspace{5pt}$ Sets an environment variable in the current process.
$\hspace{5pt}$
$\hspace{5pt}$ Note, this function is **unsafe** in multithreaded contexts on Linux.

.removevar: `(key: string) -> nil`

$\hspace{5pt}$ Removes an environment variable in the current process.
$\hspace{5pt}$
$\hspace{5pt}$ Note, this function is **unsafe** in multithreaded contexts on Linux.
