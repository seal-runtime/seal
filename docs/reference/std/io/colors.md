<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# io.colors

`local colors = require("@std/io/colors")`

The `@std/io/colors` lib, because if your terminal output isn't colorized, is it even output?

Usage:

```luau
local colors = require("@std/io/colors")
```

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

colors.bold.style.dim: `(text: string) -> string`

---

colors.bold.style.bold: `(text: string) -> string`

---

colors.bold.style.underline: `(text: string) -> string`

---

colors.bold.style.codes.RESET: `"\x1b[0m"`

---

colors.bold.style.codes.BLACK: `"\x1b[30m"`

---

colors.bold.style.codes.RED: `"\x1b[31m"`

---

colors.bold.style.codes.GREEN: `"\x1b[32m"`

---

colors.bold.style.codes.YELLOW: `"\x1b[33m"`

---

colors.bold.style.codes.BLUE: `"\x1b[34m"`

---

colors.bold.style.codes.MAGENTA: `"\x1b[35m"`

---

colors.bold.style.codes.CYAN: `"\x1b[36m"`

---

colors.bold.style.codes.WHITE: `"\x1b[37m"`

---

colors.bold.style.codes.BOLD_BLACK: `"\x1b[1;30m"`

---

colors.bold.style.codes.BOLD_RED: `"\x1b[1;31m"`

---

colors.bold.style.codes.BOLD_GREEN: `"\x1b[1;32m"`

---

colors.bold.style.codes.BOLD_YELLOW: `"\x1b[1;33m"`

---

colors.bold.style.codes.BOLD_BLUE: `"\x1b[1;34m"`

---

colors.bold.style.codes.BOLD_MAGENTA: `"\x1b[1;35m"`

---

colors.bold.style.codes.BOLD_CYAN: `"\x1b[1;36m"`

---

colors.bold.style.codes.BOLD_WHITE: `"\x1b[1;37m"`

---

colors.bold.style.codes.BRIGHT_BLACK: `"\x1b[90m"`

---

colors.bold.style.codes.BRIGHT_RED: `"\x1b[91m"`

---

colors.bold.style.codes.BRIGHT_GREEN: `"\x1b[92m"`

---

colors.bold.style.codes.BRIGHT_YELLOW: `"\x1b[93m"`

---

colors.bold.style.codes.BRIGHT_BLUE: `"\x1b[94m"`

---

colors.bold.style.codes.BRIGHT_MAGENTA: `"\x1b[95m"`

---

colors.bold.style.codes.BRIGHT_CYAN: `"\x1b[96m"`

---

colors.bold.style.codes.BRIGHT_WHITE: `"\x1b[97m"`

---

colors.bold.style.codes.BLACK_BG: `"\x1b[40m"`

---

colors.bold.style.codes.RED_BG: `"\x1b[41m"`

---

colors.bold.style.codes.GREEN_BG: `"\x1b[42m"`

---

colors.bold.style.codes.YELLOW_BG: `"\x1b[43m"`

---

colors.bold.style.codes.BLUE_BG: `"\x1b[44m"`

---

colors.bold.style.codes.MAGENTA_BG: `"\x1b[45m"`

---

colors.bold.style.codes.CYAN_BG: `"\x1b[46m"`

---

colors.bold.style.codes.WHITE_BG: `"\x1b[47m"`

---

colors.bold.style.codes.BRIGHT_BLACK_BG: `"\x1b[100m"`

---

colors.bold.style.codes.BRIGHT_RED_BG: `"\x1b[101m"`

---

colors.bold.style.codes.BRIGHT_GREEN_BG: `"\x1b[102m"`

---

colors.bold.style.codes.BRIGHT_YELLOW_BG: `"\x1b[103m"`

---

colors.bold.style.codes.BRIGHT_BLUE_BG: `"\x1b[104m"`

---

colors.bold.style.codes.BRIGHT_MAGENTA_BG: `"\x1b[105m"`

---

colors.bold.style.codes.BRIGHT_CYAN_BG: `"\x1b[106m"`

---

colors.bold.style.codes.BRIGHT_WHITE_BG: `"\x1b[107m"`

---

colors.bold.style.codes.BOLD: `"\x1b[1m"`

---

colors.bold.style.codes.DIM: `"\x1b[2m"`

---

colors.bold.style.codes.UNDERLINE: `"\x1b[4m"`

---

colors.bold.style.codes.function colors.magenta(text: `string): string`

Turns the provided text magenta

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.magenta("this text is hereby colored magenta"))
```

---

colors.bold.style.codes.function colors.blue(text: `string): string`

Turns the provided text blue

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.blue("this text is hereby colored blue"))
```

---

colors.bold.style.codes.function colors.cyan(text: `string): string`

Turns the provided text cyan

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.cyan("this text is hereby colored cyan"))
```

---

colors.bold.style.codes.function colors.black(text: `string): string`

Turns the provided text black

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.black("this text is hereby colored black"))
```

---

colors.bold.style.codes.function colors.green(text: `string): string`

Turns the provided text green

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.green("this text is hereby colored green"))
```

---

colors.bold.style.codes.function colors.yellow(text: `string): string`

Turns the provided text yellow

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.yellow("this text is hereby colored yellow"))
```

---

colors.bold.style.codes.function colors.white(text: `string): string`

Turns the provided text white

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.white("this text is hereby colored white"))
```

---

colors.bold.style.codes.function colors.red(text: `string): string`

Turns the provided text red

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.red("this text is hereby colored red"))
```

---

colors.bold.style.codes.function colors.bold.white(text: `string): string`

Turns the provided text bold white

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.bold.white("this is now bold white"))
```

---

colors.bold.style.codes.function colors.bold.magenta(text: `string): string`

Turns the provided text bold magenta

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.bold.magenta("this is now bold magenta"))
```

---

colors.bold.style.codes.function colors.bold.black(text: `string): string`

Turns the provided text bold black

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.bold.black("this is now bold black"))
```

---

colors.bold.style.codes.function colors.bold.green(text: `string): string`

Turns the provided text bold green

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.bold.green("this is now bold green"))
```

---

colors.bold.style.codes.function colors.bold.cyan(text: `string): string`

Turns the provided text bold cyan

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.bold.cyan("this is now bold cyan"))
```

---

colors.bold.style.codes.function colors.bold.yellow(text: `string): string`

Turns the provided text bold yellow

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.bold.yellow("this is now bold yellow"))
```

---

colors.bold.style.codes.function colors.bold.blue(text: `string): string`

Turns the provided text bold blue

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.bold.blue("this is now bold blue"))
```

---

colors.bold.style.codes.function colors.bold.red(text: `string): string`

Turns the provided text bold red

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.bold.red("this is now bold red"))
```

---

colors.bold.style.codes.dim = function(text: `string): string return nil :: any end`

 Use different styles such as dim or bold
 dim style

---

colors.bold.style.codes.bold = function(text: `string): string return nil :: any end`

 bold style

---

colors.bold.style.codes.underline = function(text: `string): string return nil :: any end`

 underline your text

---
