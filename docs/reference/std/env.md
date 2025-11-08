<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# env

`local env = require("@std/env")`

A stdlib to interact with the script's running environment.

---

### env.args

<h4>

```luau
env.args: {string},
```

</h4>

 a list of arguments passed to the program

---

### env.os

<h4>

```luau
env.os: "Windows" | "Linux" | "Android" | "MacOS" | "Other",
```

</h4>

 your operating system

---

### env.executable_path

<h4>

```luau
env.executable_path: string,
```

</h4>

 the path of the executable

---

### env.cwd

<h4>

```luau
env.cwd: () -> string,
```

</h4>

Get the current working directory of the running process.

Errors if the `cwd` doesn't exist or otherwise isn't accessible (permission denied).

---

### env.getvar

<h4>

```luau
env.getvar: (key: string) -> string?,
```

</h4>

Gets an environment variable in the current process.

---

### env.setvar

<h4>

```luau
env.setvar: (key: string, value: string) -> string,
```

</h4>

Sets an environment variable in the current process.

Note, this function is **unsafe** in multithreaded contexts on Linux.

---

### env.removevar

<h4>

```luau
env.removevar: (key: string) -> nil,
```

</h4>

Removes an environment variable in the current process.

Note, this function is **unsafe** in multithreaded contexts on Linux.

---
