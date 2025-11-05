<!-- markdownlint-disable MD033 -->

# Colors

``

<details>

<summary> See the docs </summary

The `@std/io/colors` lib, because if your terminal output isn't colorized, is it even output?

Usage:

```luau
local colors = require("@std/io/colors")
```

</details>

`function colors.magenta(text: string): string`

<details>

<summary> See the docs </summary

Turns the provided text magenta

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.magenta("this text is hereby colored magenta"))
```

</details>

`function colors.blue(text: string): string`

<details>

<summary> See the docs </summary

Turns the provided text blue

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.blue("this text is hereby colored blue"))
```

</details>

`function colors.cyan(text: string): string`

<details>

<summary> See the docs </summary

Turns the provided text cyan

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.cyan("this text is hereby colored cyan"))
```

</details>

`function colors.black(text: string): string`

<details>

<summary> See the docs </summary

Turns the provided text black

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.black("this text is hereby colored black"))
```

</details>

`function colors.green(text: string): string`

<details>

<summary> See the docs </summary

Turns the provided text green

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.green("this text is hereby colored green"))
```

</details>

`function colors.yellow(text: string): string`

<details>

<summary> See the docs </summary

Turns the provided text yellow

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.yellow("this text is hereby colored yellow"))
```

</details>

`function colors.white(text: string): string`

<details>

<summary> See the docs </summary

Turns the provided text white

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.white("this text is hereby colored white"))
```

</details>

`function colors.red(text: string): string`

<details>

<summary> See the docs </summary

Turns the provided text red

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.red("this text is hereby colored red"))
```

</details>

`function colors.bold.white(text: string): string`

<details>

<summary> See the docs </summary

Turns the provided text bold white

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.bold.white("this is now bold white"))
```

</details>

`function colors.bold.magenta(text: string): string`

<details>

<summary> See the docs </summary

Turns the provided text bold magenta

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.bold.magenta("this is now bold magenta"))
```

</details>

`function colors.bold.black(text: string): string`

<details>

<summary> See the docs </summary

Turns the provided text bold black

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.bold.black("this is now bold black"))
```

</details>

`function colors.bold.green(text: string): string`

<details>

<summary> See the docs </summary

Turns the provided text bold green

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.bold.green("this is now bold green"))
```

</details>

`function colors.bold.cyan(text: string): string`

<details>

<summary> See the docs </summary

Turns the provided text bold cyan

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.bold.cyan("this is now bold cyan"))
```

</details>

`function colors.bold.yellow(text: string): string`

<details>

<summary> See the docs </summary

Turns the provided text bold yellow

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.bold.yellow("this is now bold yellow"))
```

</details>

`function colors.bold.blue(text: string): string`

<details>

<summary> See the docs </summary

Turns the provided text bold blue

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.bold.blue("this is now bold blue"))
```

</details>

`function colors.bold.red(text: string): string`

<details>

<summary> See the docs </summary

Turns the provided text bold red

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.bold.red("this is now bold red"))
```

</details>

`colors.codes = {`

<details>

<summary> See the docs </summary

An assorted collection of ANSI color codes to help you colorize your text however you want!!

Don't forget to finish off your colorized strings with a `RESET` :p

Usage:

```luau
local colors = require("@std/io/colors")
print(colors.bold.red("this is now bold red"))
```

</details>
