--!nonstrict
--[=[
The `@std/io/colors` lib, because if your terminal output isn't colorized, is it even output?

Usage:

```luau
local colors = require("@std/io/colors")

print(colors.blue("my blue text"))
```
]=]
export type colors = {
    black: (text: string) -> string,
    red: (text: string) -> string,
    green: (text: string) -> string,
    yellow: (text: string) -> string,
    blue: (text: string) -> string,
    magenta: (text: string) -> string,
    cyan: (text: string) -> string,
    white: (text: string) -> string,

    bold: {
        black: (text: string) -> string,
        red: (text: string) -> string,
        green: (text: string) -> string,
        yellow: (text: string) -> string,
        blue: (text: string) -> string,
        magenta: (text: string) -> string,
        cyan: (text: string) -> string,
        white: (text: string) -> string,
    },

    style: {
        dim: (text: string) -> string,
        bold: (text: string) -> string,
        underline: (text: string) -> string,
    },

    codes: {
        RESET: "\x1b[0m",
        BLACK: "\x1b[30m",
        RED: "\x1b[31m",
        GREEN: "\x1b[32m",
        YELLOW: "\x1b[33m",
        BLUE: "\x1b[34m",
        MAGENTA: "\x1b[35m",
        CYAN: "\x1b[36m",
        WHITE: "\x1b[37m",
        BOLD_BLACK: "\x1b[1;30m",
        BOLD_RED: "\x1b[1;31m",
        BOLD_GREEN: "\x1b[1;32m",
        BOLD_YELLOW: "\x1b[1;33m",
        BOLD_BLUE: "\x1b[1;34m",
        BOLD_MAGENTA: "\x1b[1;35m",
        BOLD_CYAN: "\x1b[1;36m",
        BOLD_WHITE: "\x1b[1;37m",
        BRIGHT_BLACK: "\x1b[90m",
        BRIGHT_RED: "\x1b[91m",
        BRIGHT_GREEN: "\x1b[92m",
        BRIGHT_YELLOW: "\x1b[93m",
        BRIGHT_BLUE: "\x1b[94m",
        BRIGHT_MAGENTA: "\x1b[95m",
        BRIGHT_CYAN: "\x1b[96m",
        BRIGHT_WHITE: "\x1b[97m",
        BLACK_BG: "\x1b[40m",
        RED_BG: "\x1b[41m",
        GREEN_BG: "\x1b[42m",
        YELLOW_BG: "\x1b[43m",
        BLUE_BG: "\x1b[44m",
        MAGENTA_BG: "\x1b[45m",
        CYAN_BG: "\x1b[46m",
        WHITE_BG: "\x1b[47m",
        BRIGHT_BLACK_BG: "\x1b[100m",
        BRIGHT_RED_BG: "\x1b[101m",
        BRIGHT_GREEN_BG: "\x1b[102m",
        BRIGHT_YELLOW_BG: "\x1b[103m",
        BRIGHT_BLUE_BG: "\x1b[104m",
        BRIGHT_MAGENTA_BG: "\x1b[105m",
        BRIGHT_CYAN_BG: "\x1b[106m",
        BRIGHT_WHITE_BG: "\x1b[107m",
        BOLD: "\x1b[1m",
        DIM: "\x1b[2m",
        UNDERLINE: "\x1b[4m",
    }
}

return {} :: colors