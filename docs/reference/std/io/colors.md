<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# io.colors

`local colors = require("@std/io/colors")`

$hspace{5pt}$The `@std/io/colors` lib, because if your terminal output isn't colorized, is it even output?
$hspace{5pt}$
$hspace{5pt}$Usage:
$hspace{5pt}$
$hspace{5pt}$```luau
$hspace{5pt}$local colors = require("@std/io/colors")
$hspace{5pt}$```

colors.black: `(text: string) -> string`

colors.red: `(text: string) -> string`

colors.green: `(text: string) -> string`

colors.yellow: `(text: string) -> string`

colors.blue: `(text: string) -> string`

colors.magenta: `(text: string) -> string`

colors.cyan: `(text: string) -> string`

colors.white: `(text: string) -> string`

colors.bold.black: `(text: string) -> string`

colors.bold.red: `(text: string) -> string`

colors.bold.green: `(text: string) -> string`

colors.bold.yellow: `(text: string) -> string`

colors.bold.blue: `(text: string) -> string`

colors.bold.magenta: `(text: string) -> string`

colors.bold.cyan: `(text: string) -> string`

colors.bold.white: `(text: string) -> string`

colors.style.dim: `(text: string) -> string`

colors.style.bold: `(text: string) -> string`

colors.style.underline: `(text: string) -> string`

colors.codes.RESET: `"\x1b[0m"`

colors.codes.BLACK: `"\x1b[30m"`

colors.codes.RED: `"\x1b[31m"`

colors.codes.GREEN: `"\x1b[32m"`

colors.codes.YELLOW: `"\x1b[33m"`

colors.codes.BLUE: `"\x1b[34m"`

colors.codes.MAGENTA: `"\x1b[35m"`

colors.codes.CYAN: `"\x1b[36m"`

colors.codes.WHITE: `"\x1b[37m"`

colors.codes.BOLD_BLACK: `"\x1b[1;30m"`

colors.codes.BOLD_RED: `"\x1b[1;31m"`

colors.codes.BOLD_GREEN: `"\x1b[1;32m"`

colors.codes.BOLD_YELLOW: `"\x1b[1;33m"`

colors.codes.BOLD_BLUE: `"\x1b[1;34m"`

colors.codes.BOLD_MAGENTA: `"\x1b[1;35m"`

colors.codes.BOLD_CYAN: `"\x1b[1;36m"`

colors.codes.BOLD_WHITE: `"\x1b[1;37m"`

colors.codes.BRIGHT_BLACK: `"\x1b[90m"`

colors.codes.BRIGHT_RED: `"\x1b[91m"`

colors.codes.BRIGHT_GREEN: `"\x1b[92m"`

colors.codes.BRIGHT_YELLOW: `"\x1b[93m"`

colors.codes.BRIGHT_BLUE: `"\x1b[94m"`

colors.codes.BRIGHT_MAGENTA: `"\x1b[95m"`

colors.codes.BRIGHT_CYAN: `"\x1b[96m"`

colors.codes.BRIGHT_WHITE: `"\x1b[97m"`

colors.codes.BLACK_BG: `"\x1b[40m"`

colors.codes.RED_BG: `"\x1b[41m"`

colors.codes.GREEN_BG: `"\x1b[42m"`

colors.codes.YELLOW_BG: `"\x1b[43m"`

colors.codes.BLUE_BG: `"\x1b[44m"`

colors.codes.MAGENTA_BG: `"\x1b[45m"`

colors.codes.CYAN_BG: `"\x1b[46m"`

colors.codes.WHITE_BG: `"\x1b[47m"`

colors.codes.BRIGHT_BLACK_BG: `"\x1b[100m"`

colors.codes.BRIGHT_RED_BG: `"\x1b[101m"`

colors.codes.BRIGHT_GREEN_BG: `"\x1b[102m"`

colors.codes.BRIGHT_YELLOW_BG: `"\x1b[103m"`

colors.codes.BRIGHT_BLUE_BG: `"\x1b[104m"`

colors.codes.BRIGHT_MAGENTA_BG: `"\x1b[105m"`

colors.codes.BRIGHT_CYAN_BG: `"\x1b[106m"`

colors.codes.BRIGHT_WHITE_BG: `"\x1b[107m"`

colors.codes.BOLD: `"\x1b[1m"`

colors.codes.DIM: `"\x1b[2m"`

colors.codes.UNDERLINE: `"\x1b[4m"`

colors.function colors.magenta(text: `string): string`

$hspace{5pt}$Turns the provided text magenta
$hspace{5pt}$
$hspace{5pt}$Usage:
$hspace{5pt}$
$hspace{5pt}$```luau
$hspace{5pt}$local colors = require("@std/io/colors")
$hspace{5pt}$print(colors.magenta("this text is hereby colored magenta"))
$hspace{5pt}$```

colors.function colors.blue(text: `string): string`

$hspace{5pt}$Turns the provided text blue
$hspace{5pt}$
$hspace{5pt}$Usage:
$hspace{5pt}$
$hspace{5pt}$```luau
$hspace{5pt}$local colors = require("@std/io/colors")
$hspace{5pt}$print(colors.blue("this text is hereby colored blue"))
$hspace{5pt}$```

colors.function colors.cyan(text: `string): string`

$hspace{5pt}$Turns the provided text cyan
$hspace{5pt}$
$hspace{5pt}$Usage:
$hspace{5pt}$
$hspace{5pt}$```luau
$hspace{5pt}$local colors = require("@std/io/colors")
$hspace{5pt}$print(colors.cyan("this text is hereby colored cyan"))
$hspace{5pt}$```

colors.function colors.black(text: `string): string`

$hspace{5pt}$Turns the provided text black
$hspace{5pt}$
$hspace{5pt}$Usage:
$hspace{5pt}$
$hspace{5pt}$```luau
$hspace{5pt}$local colors = require("@std/io/colors")
$hspace{5pt}$print(colors.black("this text is hereby colored black"))
$hspace{5pt}$```

colors.function colors.green(text: `string): string`

$hspace{5pt}$Turns the provided text green
$hspace{5pt}$
$hspace{5pt}$Usage:
$hspace{5pt}$
$hspace{5pt}$```luau
$hspace{5pt}$local colors = require("@std/io/colors")
$hspace{5pt}$print(colors.green("this text is hereby colored green"))
$hspace{5pt}$```

colors.function colors.yellow(text: `string): string`

$hspace{5pt}$Turns the provided text yellow
$hspace{5pt}$
$hspace{5pt}$Usage:
$hspace{5pt}$
$hspace{5pt}$```luau
$hspace{5pt}$local colors = require("@std/io/colors")
$hspace{5pt}$print(colors.yellow("this text is hereby colored yellow"))
$hspace{5pt}$```

colors.function colors.white(text: `string): string`

$hspace{5pt}$Turns the provided text white
$hspace{5pt}$
$hspace{5pt}$Usage:
$hspace{5pt}$
$hspace{5pt}$```luau
$hspace{5pt}$local colors = require("@std/io/colors")
$hspace{5pt}$print(colors.white("this text is hereby colored white"))
$hspace{5pt}$```

colors.function colors.red(text: `string): string`

$hspace{5pt}$Turns the provided text red
$hspace{5pt}$
$hspace{5pt}$Usage:
$hspace{5pt}$
$hspace{5pt}$```luau
$hspace{5pt}$local colors = require("@std/io/colors")
$hspace{5pt}$print(colors.red("this text is hereby colored red"))
$hspace{5pt}$```

colors.function colors.bold.white(text: `string): string`

$hspace{5pt}$Turns the provided text bold white
$hspace{5pt}$
$hspace{5pt}$Usage:
$hspace{5pt}$
$hspace{5pt}$```luau
$hspace{5pt}$local colors = require("@std/io/colors")
$hspace{5pt}$print(colors.bold.white("this is now bold white"))
$hspace{5pt}$```

colors.function colors.bold.magenta(text: `string): string`

$hspace{5pt}$Turns the provided text bold magenta
$hspace{5pt}$
$hspace{5pt}$Usage:
$hspace{5pt}$
$hspace{5pt}$```luau
$hspace{5pt}$local colors = require("@std/io/colors")
$hspace{5pt}$print(colors.bold.magenta("this is now bold magenta"))
$hspace{5pt}$```

colors.function colors.bold.black(text: `string): string`

$hspace{5pt}$Turns the provided text bold black
$hspace{5pt}$
$hspace{5pt}$Usage:
$hspace{5pt}$
$hspace{5pt}$```luau
$hspace{5pt}$local colors = require("@std/io/colors")
$hspace{5pt}$print(colors.bold.black("this is now bold black"))
$hspace{5pt}$```

colors.function colors.bold.green(text: `string): string`

$hspace{5pt}$Turns the provided text bold green
$hspace{5pt}$
$hspace{5pt}$Usage:
$hspace{5pt}$
$hspace{5pt}$```luau
$hspace{5pt}$local colors = require("@std/io/colors")
$hspace{5pt}$print(colors.bold.green("this is now bold green"))
$hspace{5pt}$```

colors.function colors.bold.cyan(text: `string): string`

$hspace{5pt}$Turns the provided text bold cyan
$hspace{5pt}$
$hspace{5pt}$Usage:
$hspace{5pt}$
$hspace{5pt}$```luau
$hspace{5pt}$local colors = require("@std/io/colors")
$hspace{5pt}$print(colors.bold.cyan("this is now bold cyan"))
$hspace{5pt}$```

colors.function colors.bold.yellow(text: `string): string`

$hspace{5pt}$Turns the provided text bold yellow
$hspace{5pt}$
$hspace{5pt}$Usage:
$hspace{5pt}$
$hspace{5pt}$```luau
$hspace{5pt}$local colors = require("@std/io/colors")
$hspace{5pt}$print(colors.bold.yellow("this is now bold yellow"))
$hspace{5pt}$```

colors.function colors.bold.blue(text: `string): string`

$hspace{5pt}$Turns the provided text bold blue
$hspace{5pt}$
$hspace{5pt}$Usage:
$hspace{5pt}$
$hspace{5pt}$```luau
$hspace{5pt}$local colors = require("@std/io/colors")
$hspace{5pt}$print(colors.bold.blue("this is now bold blue"))
$hspace{5pt}$```

colors.function colors.bold.red(text: `string): string`

$hspace{5pt}$Turns the provided text bold red
$hspace{5pt}$
$hspace{5pt}$Usage:
$hspace{5pt}$
$hspace{5pt}$```luau
$hspace{5pt}$local colors = require("@std/io/colors")
$hspace{5pt}$print(colors.bold.red("this is now bold red"))
$hspace{5pt}$```

colors.dim = function(text: `string): string return nil :: any end`

$hspace{5pt}$ Use different styles such as dim or bold
$hspace{5pt}$ dim style

colors.bold = function(text: `string): string return nil :: any end`

$hspace{5pt}$ bold style

colors.underline = function(text: `string): string return nil :: any end`

$hspace{5pt}$ underline your text
