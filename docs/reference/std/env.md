<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# env

`local env = require("@std/env")`

A stdlib to interact with the script's running environment.

---

### env.args

<h4>

```luau
args: {string},
```

</h4>

 a list of arguments passed to the program

---

### env.os

<h4>

```luau
os: "Windows" | "Linux" | "Android" | "MacOS" | "Other",
```

</h4>

 your operating system

---

### env.executable_path

<h4>

```luau
executable_path: string,
```

</h4>

 the path of the executable

---

<h3>

```luau
function env.cwd() -> string,
```

</h3>

Get the current working directory of the running process.

Errors if the `cwd` doesn't exist or otherwise isn't accessible (permission denied).

---

<h3>

```luau
function env.getvar(key: string) -> string?,
```

</h3>

Gets an environment variable in the current process.

---

<h3>

```luau
function env.setvar(key: string, value: string) -> string,
```

</h3>

Sets an environment variable in the current process.

Note, this function is **unsafe** in multithreaded contexts on Linux.

---

<h3>

```luau
function env.removevar(key: string) -> nil,
```

</h3>

Removes an environment variable in the current process.

Note, this function is **unsafe** in multithreaded contexts on Linux.

---
