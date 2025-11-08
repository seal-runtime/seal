<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# io.colors

`local colors = require("@std/io/colors")`

The `@std/io/colors` lib, because if your terminal output isn't colorized, is it even output?

Usage:

```luau
local colors = require("@std/io/colors")

print(colors.blue("my blue text"))
```

---

<h3>

```luau
colors.black: (text: string) -> string,
```

</h3>

---

<h3>

```luau
colors.red: (text: string) -> string,
```

</h3>

---

<h3>

```luau
colors.green: (text: string) -> string,
```

</h3>

---

<h3>

```luau
colors.yellow: (text: string) -> string,
```

</h3>

---

<h3>

```luau
colors.blue: (text: string) -> string,
```

</h3>

---

<h3>

```luau
colors.magenta: (text: string) -> string,
```

</h3>

---

<h3>

```luau
colors.cyan: (text: string) -> string,
```

</h3>

---

<h3>

```luau
colors.white: (text: string) -> string,
```

</h3>

---

<h3>

```luau
colors.bold.black: (text: string) -> string,
```

</h3>

---

<h3>

```luau
colors.bold.red: (text: string) -> string,
```

</h3>

---

<h3>

```luau
colors.bold.green: (text: string) -> string,
```

</h3>

---

<h3>

```luau
colors.bold.yellow: (text: string) -> string,
```

</h3>

---

<h3>

```luau
colors.bold.blue: (text: string) -> string,
```

</h3>

---

<h3>

```luau
colors.bold.magenta: (text: string) -> string,
```

</h3>

---

<h3>

```luau
colors.bold.cyan: (text: string) -> string,
```

</h3>

---

<h3>

```luau
colors.bold.white: (text: string) -> string,
```

</h3>

---

<h3>

```luau
colors.style.dim: (text: string) -> string,
```

</h3>

---

<h3>

```luau
colors.style.bold: (text: string) -> string,
```

</h3>

---

<h3>

```luau
colors.style.underline: (text: string) -> string,
```

</h3>

---

<h3>

```luau
colors.codes.RESET: "\x1b[0m",
```

</h3>

---

<h3>

```luau
colors.codes.BLACK: "\x1b[30m",
```

</h3>

---

<h3>

```luau
colors.codes.RED: "\x1b[31m",
```

</h3>

---

<h3>

```luau
colors.codes.GREEN: "\x1b[32m",
```

</h3>

---

<h3>

```luau
colors.codes.YELLOW: "\x1b[33m",
```

</h3>

---

<h3>

```luau
colors.codes.BLUE: "\x1b[34m",
```

</h3>

---

<h3>

```luau
colors.codes.MAGENTA: "\x1b[35m",
```

</h3>

---

<h3>

```luau
colors.codes.CYAN: "\x1b[36m",
```

</h3>

---

<h3>

```luau
colors.codes.WHITE: "\x1b[37m",
```

</h3>

---

<h3>

```luau
colors.codes.BOLD_BLACK: "\x1b[1;30m",
```

</h3>

---

<h3>

```luau
colors.codes.BOLD_RED: "\x1b[1;31m",
```

</h3>

---

<h3>

```luau
colors.codes.BOLD_GREEN: "\x1b[1;32m",
```

</h3>

---

<h3>

```luau
colors.codes.BOLD_YELLOW: "\x1b[1;33m",
```

</h3>

---

<h3>

```luau
colors.codes.BOLD_BLUE: "\x1b[1;34m",
```

</h3>

---

<h3>

```luau
colors.codes.BOLD_MAGENTA: "\x1b[1;35m",
```

</h3>

---

<h3>

```luau
colors.codes.BOLD_CYAN: "\x1b[1;36m",
```

</h3>

---

<h3>

```luau
colors.codes.BOLD_WHITE: "\x1b[1;37m",
```

</h3>

---

<h3>

```luau
colors.codes.BRIGHT_BLACK: "\x1b[90m",
```

</h3>

---

<h3>

```luau
colors.codes.BRIGHT_RED: "\x1b[91m",
```

</h3>

---

<h3>

```luau
colors.codes.BRIGHT_GREEN: "\x1b[92m",
```

</h3>

---

<h3>

```luau
colors.codes.BRIGHT_YELLOW: "\x1b[93m",
```

</h3>

---

<h3>

```luau
colors.codes.BRIGHT_BLUE: "\x1b[94m",
```

</h3>

---

<h3>

```luau
colors.codes.BRIGHT_MAGENTA: "\x1b[95m",
```

</h3>

---

<h3>

```luau
colors.codes.BRIGHT_CYAN: "\x1b[96m",
```

</h3>

---

<h3>

```luau
colors.codes.BRIGHT_WHITE: "\x1b[97m",
```

</h3>

---

<h3>

```luau
colors.codes.BLACK_BG: "\x1b[40m",
```

</h3>

---

<h3>

```luau
colors.codes.RED_BG: "\x1b[41m",
```

</h3>

---

<h3>

```luau
colors.codes.GREEN_BG: "\x1b[42m",
```

</h3>

---

<h3>

```luau
colors.codes.YELLOW_BG: "\x1b[43m",
```

</h3>

---

<h3>

```luau
colors.codes.BLUE_BG: "\x1b[44m",
```

</h3>

---

<h3>

```luau
colors.codes.MAGENTA_BG: "\x1b[45m",
```

</h3>

---

<h3>

```luau
colors.codes.CYAN_BG: "\x1b[46m",
```

</h3>

---

<h3>

```luau
colors.codes.WHITE_BG: "\x1b[47m",
```

</h3>

---

<h3>

```luau
colors.codes.BRIGHT_BLACK_BG: "\x1b[100m",
```

</h3>

---

<h3>

```luau
colors.codes.BRIGHT_RED_BG: "\x1b[101m",
```

</h3>

---

<h3>

```luau
colors.codes.BRIGHT_GREEN_BG: "\x1b[102m",
```

</h3>

---

<h3>

```luau
colors.codes.BRIGHT_YELLOW_BG: "\x1b[103m",
```

</h3>

---

<h3>

```luau
colors.codes.BRIGHT_BLUE_BG: "\x1b[104m",
```

</h3>

---

<h3>

```luau
colors.codes.BRIGHT_MAGENTA_BG: "\x1b[105m",
```

</h3>

---

<h3>

```luau
colors.codes.BRIGHT_CYAN_BG: "\x1b[106m",
```

</h3>

---

<h3>

```luau
colors.codes.BRIGHT_WHITE_BG: "\x1b[107m",
```

</h3>

---

<h3>

```luau
colors.codes.BOLD: "\x1b[1m",
```

</h3>

---

<h3>

```luau
colors.codes.DIM: "\x1b[2m",
```

</h3>

---

<h3>

```luau
colors.codes.UNDERLINE: "\x1b[4m",
```

</h3>

---
