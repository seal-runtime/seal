<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# env

`local env = require("@std/env")`

A stdlib to interact with the script's running environment.

---

<h3>

```luau
env.args: {string},
```

</h3>

 a list of arguments passed to the program

---

<h3>

```luau
env.os: "Windows" | "Linux" | "Android" | "MacOS" | "Other",
```

</h3>

 your operating system

---

<h3>

```luau
env.executable_path: string,
```

</h3>

 the path of the executable

---

<h3>

```luau
env.cwd: () -> string,
```

</h3>

Get the current working directory of the running process.

Errors if the `cwd` doesn't exist or otherwise isn't accessible (permission denied).

---

<h3>

```luau
env.getvar: (key: string) -> string?,
```

</h3>

Gets an environment variable in the current process.

---

<h3>

```luau
env.setvar: (key: string, value: string) -> string,
```

</h3>

Sets an environment variable in the current process.

Note, this function is **unsafe** in multithreaded contexts on Linux.

---

<h3>

```luau
env.removevar: (key: string) -> nil,
```

</h3>

Removes an environment variable in the current process.

Note, this function is **unsafe** in multithreaded contexts on Linux.

---
