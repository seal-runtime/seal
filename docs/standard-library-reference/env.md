<!-- markdownlint-disable MD033 -->

# Env

`type env = {`

<details>

<summary> Docs </summary

A stdlib to interact with the script's running environment.

</details>

` cwd: () -> string`

<details>

<summary> Docs </summary

Get the current working directory of the running process.

Errors if the `cwd` doesn't exist or otherwise isn't accessible (permission denied).

</details>

` getvar: (key: string) -> string?`

<details>

<summary> Docs </summary

Gets an environment variable in the current process.

</details>

` setvar: (key: string, value: string) -> string`

<details>

<summary> Docs </summary

Sets an environment variable in the current process.

Note, this function is **unsafe** in multithreaded contexts on Linux.

</details>

` removevar: (key: string) -> nil`

<details>

<summary> Docs </summary

Removes an environment variable in the current process.

Note, this function is **unsafe** in multithreaded contexts on Linux.

</details>
