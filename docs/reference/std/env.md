<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# env

`local env = require("@std/env")`

A stdlib to interact with the script's running environment.

---

### env.args

```luau
env.args: {string},
```

 a list of arguments passed to the program

---

### env.os

```luau
env.os: "Windows" | "Linux" | "Android" | "MacOS" | "Other",
```

 your operating system

---

### env.executable_path

```luau
env.executable_path: string,
```

 the path of the executable

---

### env.cwd

```luau
env.cwd: () -> string,
```

Get the current working directory of the running process.

Errors if the `cwd` doesn't exist or otherwise isn't accessible (permission denied).

---

### env.getvar

```luau
env.getvar: (key: string) -> string?,
```

Gets an environment variable in the current process.

---

### env.setvar

```luau
env.setvar: (key: string, value: string) -> string,
```

Sets an environment variable in the current process.

Note, this function is **unsafe** in multithreaded contexts on Linux.

---

### env.removevar

```luau
env.removevar: (key: string) -> nil,
```

Removes an environment variable in the current process.

Note, this function is **unsafe** in multithreaded contexts on Linux.

---
