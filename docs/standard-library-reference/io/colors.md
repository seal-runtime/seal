<!-- markdownlint-disable MD033 -->

# Colors

The `@std/io/colors` lib, because if your terminal output isn't colorized, is it even output?

Usage:

```luau
local colors = require("@std/io/colors")
```

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

`colors.codes = {`

<details>

<summary> See the docs </summary

--- Use different styles such as dim or bold
colors.style = {
--- dim style
dim = function(text: string): string return nil :: any end,
--- bold style
bold = function(text: string): string return nil :: any end,
--- underline your text
underline = function(text: string): string return nil :: any end,
}

An assorted collection of ANSI color codes to help you colorize your text however you want!!

Don't forget to finish off your colorized strings with a `RESET` :p

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.bold.red("this is now bold red"))
```

</details>
