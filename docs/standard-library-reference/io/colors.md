<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# Colors

The `@std/io/colors` lib, because if your terminal output isn't colorized, is it even output?

Usage:

```luau
local colors = require("@std/io/colors")
```

`function colors.black(text: string): string`

`function colors.red(text: string): string`

`function colors.green(text: string): string`

`function colors.yellow(text: string): string`

`function colors.blue(text: string): string`

`function colors.magenta(text: string): string`

`function colors.cyan(text: string): string`

`function colors.white(text: string): string`

`bold: {`

`function bold.black(text: string): string`

`function bold.red(text: string): string`

`function bold.green(text: string): string`

`function bold.yellow(text: string): string`

`function bold.blue(text: string): string`

`function bold.magenta(text: string): string`

`function bold.cyan(text: string): string`

`function bold.white(text: string): string`

`style: {`

`function style.dim(text: string): string`

`function style.bold(text: string): string`

`function style.underline(text: string): string`

`codes: {`

`codes.RESET: "\x1b[0m"`

`codes.BLACK: "\x1b[30m"`

`codes.RED: "\x1b[31m"`

`codes.GREEN: "\x1b[32m"`

`codes.YELLOW: "\x1b[33m"`

`codes.BLUE: "\x1b[34m"`

`codes.MAGENTA: "\x1b[35m"`

`codes.CYAN: "\x1b[36m"`

`codes.WHITE: "\x1b[37m"`

`codes.BOLD_BLACK: "\x1b[1;30m"`

`codes.BOLD_RED: "\x1b[1;31m"`

`codes.BOLD_GREEN: "\x1b[1;32m"`

`codes.BOLD_YELLOW: "\x1b[1;33m"`

`codes.BOLD_BLUE: "\x1b[1;34m"`

`codes.BOLD_MAGENTA: "\x1b[1;35m"`

`codes.BOLD_CYAN: "\x1b[1;36m"`

`codes.BOLD_WHITE: "\x1b[1;37m"`

`codes.BRIGHT_BLACK: "\x1b[90m"`

`codes.BRIGHT_RED: "\x1b[91m"`

`codes.BRIGHT_GREEN: "\x1b[92m"`

`codes.BRIGHT_YELLOW: "\x1b[93m"`

`codes.BRIGHT_BLUE: "\x1b[94m"`

`codes.BRIGHT_MAGENTA: "\x1b[95m"`

`codes.BRIGHT_CYAN: "\x1b[96m"`

`codes.BRIGHT_WHITE: "\x1b[97m"`

`codes.BLACK_BG: "\x1b[40m"`

`codes.RED_BG: "\x1b[41m"`

`codes.GREEN_BG: "\x1b[42m"`

`codes.YELLOW_BG: "\x1b[43m"`

`codes.BLUE_BG: "\x1b[44m"`

`codes.MAGENTA_BG: "\x1b[45m"`

`codes.CYAN_BG: "\x1b[46m"`

`codes.WHITE_BG: "\x1b[47m"`

`codes.BRIGHT_BLACK_BG: "\x1b[100m"`

`codes.BRIGHT_RED_BG: "\x1b[101m"`

`codes.BRIGHT_GREEN_BG: "\x1b[102m"`

`codes.BRIGHT_YELLOW_BG: "\x1b[103m"`

`codes.BRIGHT_BLUE_BG: "\x1b[104m"`

`codes.BRIGHT_MAGENTA_BG: "\x1b[105m"`

`codes.BRIGHT_CYAN_BG: "\x1b[106m"`

`codes.BRIGHT_WHITE_BG: "\x1b[107m"`

`codes.BOLD: "\x1b[1m"`

`codes.DIM: "\x1b[2m"`

`codes.UNDERLINE: "\x1b[4m"`

`function colors.magenta(text: string): string`

Turns the provided text magenta

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.magenta("this text is hereby colored magenta"))
```

`function colors.blue(text: string): string`

Turns the provided text blue

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.blue("this text is hereby colored blue"))
```

`function colors.cyan(text: string): string`

Turns the provided text cyan

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.cyan("this text is hereby colored cyan"))
```

`function colors.black(text: string): string`

Turns the provided text black

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.black("this text is hereby colored black"))
```

`function colors.green(text: string): string`

Turns the provided text green

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.green("this text is hereby colored green"))
```

`function colors.yellow(text: string): string`

Turns the provided text yellow

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.yellow("this text is hereby colored yellow"))
```

`function colors.white(text: string): string`

Turns the provided text white

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.white("this text is hereby colored white"))
```

`function colors.red(text: string): string`

Turns the provided text red

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.red("this text is hereby colored red"))
```

`function colors.bold.white(text: string): string`

Turns the provided text bold white

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.bold.white("this is now bold white"))
```

`function colors.bold.magenta(text: string): string`

Turns the provided text bold magenta

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.bold.magenta("this is now bold magenta"))
```

`function colors.bold.black(text: string): string`

Turns the provided text bold black

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.bold.black("this is now bold black"))
```

`function colors.bold.green(text: string): string`

Turns the provided text bold green

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.bold.green("this is now bold green"))
```

`function colors.bold.cyan(text: string): string`

Turns the provided text bold cyan

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.bold.cyan("this is now bold cyan"))
```

`function colors.bold.yellow(text: string): string`

Turns the provided text bold yellow

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.bold.yellow("this is now bold yellow"))
```

`function colors.bold.blue(text: string): string`

Turns the provided text bold blue

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.bold.blue("this is now bold blue"))
```

`function colors.bold.red(text: string): string`

Turns the provided text bold red

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.bold.red("this is now bold red"))
```

`colors.style = {`

 Use different styles such as dim or bold

`colors.codes = {`

 dim style
 bold style
 underline your text
An assorted collection of ANSI color codes to help you colorize your text however you want!!

Don't forget to finish off your colorized strings with a `RESET` :p

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.bold.red("this is now bold red"))
```
