<!-- markdownlint-disable MD033 -->

# Env

A stdlib to interact with the script's running environment.

`function env.cwd(): string`

<details>

<summary> See the docs </summary

--- a list of arguments passed to the program
args: {string},
--- your operating system
os: "Windows" | "Linux" | "Android" | "MacOS" | "Other",
--- the path of the executable
executable_path: string,
Get the current working directory of the running process.

Errors if the `cwd` doesn't exist or otherwise isn't accessible (permission denied).

</details>

`function env.getvar(key: string): string?`

Gets an environment variable in the current process.

`function env.setvar(key: string, value: string): string`

Sets an environment variable in the current process.

Note, this function is **unsafe** in multithreaded contexts on Linux.

`function env.removevar(key: string): nil`

Removes an environment variable in the current process.

Note, this function is **unsafe** in multithreaded contexts on Linux.
