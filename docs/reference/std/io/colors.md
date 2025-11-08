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

colors.black: `(text: string) -> string`

---

colors.red: `(text: string) -> string`

---

colors.green: `(text: string) -> string`

---

colors.yellow: `(text: string) -> string`

---

colors.blue: `(text: string) -> string`

---

colors.magenta: `(text: string) -> string`

---

colors.cyan: `(text: string) -> string`

---

colors.white: `(text: string) -> string`

---

colors.bold.black: `(text: string) -> string`

---

colors.bold.red: `(text: string) -> string`

---

colors.bold.green: `(text: string) -> string`

---

colors.bold.yellow: `(text: string) -> string`

---

colors.bold.blue: `(text: string) -> string`

---

colors.bold.magenta: `(text: string) -> string`

---

colors.bold.cyan: `(text: string) -> string`

---

colors.bold.white: `(text: string) -> string`

---

colors.style.dim: `(text: string) -> string`

---

colors.style.bold: `(text: string) -> string`

---

colors.style.underline: `(text: string) -> string`

---

colors.codes.RESET: `"\x1b[0m"`

---

colors.codes.BLACK: `"\x1b[30m"`

---

colors.codes.RED: `"\x1b[31m"`

---

colors.codes.GREEN: `"\x1b[32m"`

---

colors.codes.YELLOW: `"\x1b[33m"`

---

colors.codes.BLUE: `"\x1b[34m"`

---

colors.codes.MAGENTA: `"\x1b[35m"`

---

colors.codes.CYAN: `"\x1b[36m"`

---

colors.codes.WHITE: `"\x1b[37m"`

---

colors.codes.BOLD_BLACK: `"\x1b[1;30m"`

---

colors.codes.BOLD_RED: `"\x1b[1;31m"`

---

colors.codes.BOLD_GREEN: `"\x1b[1;32m"`

---

colors.codes.BOLD_YELLOW: `"\x1b[1;33m"`

---

colors.codes.BOLD_BLUE: `"\x1b[1;34m"`

---

colors.codes.BOLD_MAGENTA: `"\x1b[1;35m"`

---

colors.codes.BOLD_CYAN: `"\x1b[1;36m"`

---

colors.codes.BOLD_WHITE: `"\x1b[1;37m"`

---

colors.codes.BRIGHT_BLACK: `"\x1b[90m"`

---

colors.codes.BRIGHT_RED: `"\x1b[91m"`

---

colors.codes.BRIGHT_GREEN: `"\x1b[92m"`

---

colors.codes.BRIGHT_YELLOW: `"\x1b[93m"`

---

colors.codes.BRIGHT_BLUE: `"\x1b[94m"`

---

colors.codes.BRIGHT_MAGENTA: `"\x1b[95m"`

---

colors.codes.BRIGHT_CYAN: `"\x1b[96m"`

---

colors.codes.BRIGHT_WHITE: `"\x1b[97m"`

---

colors.codes.BLACK_BG: `"\x1b[40m"`

---

colors.codes.RED_BG: `"\x1b[41m"`

---

colors.codes.GREEN_BG: `"\x1b[42m"`

---

colors.codes.YELLOW_BG: `"\x1b[43m"`

---

colors.codes.BLUE_BG: `"\x1b[44m"`

---

colors.codes.MAGENTA_BG: `"\x1b[45m"`

---

colors.codes.CYAN_BG: `"\x1b[46m"`

---

colors.codes.WHITE_BG: `"\x1b[47m"`

---

colors.codes.BRIGHT_BLACK_BG: `"\x1b[100m"`

---

colors.codes.BRIGHT_RED_BG: `"\x1b[101m"`

---

colors.codes.BRIGHT_GREEN_BG: `"\x1b[102m"`

---

colors.codes.BRIGHT_YELLOW_BG: `"\x1b[103m"`

---

colors.codes.BRIGHT_BLUE_BG: `"\x1b[104m"`

---

colors.codes.BRIGHT_MAGENTA_BG: `"\x1b[105m"`

---

colors.codes.BRIGHT_CYAN_BG: `"\x1b[106m"`

---

colors.codes.BRIGHT_WHITE_BG: `"\x1b[107m"`

---

colors.codes.BOLD: `"\x1b[1m"`

---

colors.codes.DIM: `"\x1b[2m"`

---

colors.codes.UNDERLINE: `"\x1b[4m"`

---
