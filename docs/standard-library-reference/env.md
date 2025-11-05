<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# Env

A stdlib to interact with the script's running environment.

`env.args: {string}`

 a list of arguments passed to the program

`env.os: "Windows" | "Linux" | "Android" | "MacOS" | "Other"`

 your operating system

`env.executable_path: string`

 the path of the executable

`function env.cwd(): string`

Get the current working directory of the running process.

Errors if the `cwd` doesn't exist or otherwise isn't accessible (permission denied).

`function env.getvar(key: string): string?`

Gets an environment variable in the current process.

`function env.setvar(key: string, value: string): string`

Sets an environment variable in the current process.

Note, this function is **unsafe** in multithreaded contexts on Linux.

`function env.removevar(key: string): nil`

Removes an environment variable in the current process.

Note, this function is **unsafe** in multithreaded contexts on Linux.
