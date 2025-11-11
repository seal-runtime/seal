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

### colors.black

<h4>

```luau
function colors.black(text: string) -> string,
```

</h4>

---

### colors.red

<h4>

```luau
function colors.red(text: string) -> string,
```

</h4>

---

### colors.green

<h4>

```luau
function colors.green(text: string) -> string,
```

</h4>

---

### colors.yellow

<h4>

```luau
function colors.yellow(text: string) -> string,
```

</h4>

---

### colors.blue

<h4>

```luau
function colors.blue(text: string) -> string,
```

</h4>

---

### colors.magenta

<h4>

```luau
function colors.magenta(text: string) -> string,
```

</h4>

---

### colors.cyan

<h4>

```luau
function colors.cyan(text: string) -> string,
```

</h4>

---

### colors.white

<h4>

```luau
function colors.white(text: string) -> string,
```

</h4>

---

### colors.bold.black

<h4>

```luau
function colors.bold.black(text: string) -> string,
```

</h4>

---

### colors.bold.red

<h4>

```luau
function colors.bold.red(text: string) -> string,
```

</h4>

---

### colors.bold.green

<h4>

```luau
function colors.bold.green(text: string) -> string,
```

</h4>

---

### colors.bold.yellow

<h4>

```luau
function colors.bold.yellow(text: string) -> string,
```

</h4>

---

### colors.bold.blue

<h4>

```luau
function colors.bold.blue(text: string) -> string,
```

</h4>

---

### colors.bold.magenta

<h4>

```luau
function colors.bold.magenta(text: string) -> string,
```

</h4>

---

### colors.bold.cyan

<h4>

```luau
function colors.bold.cyan(text: string) -> string,
```

</h4>

---

### colors.bold.white

<h4>

```luau
function colors.bold.white(text: string) -> string,
```

</h4>

---

### colors.style.dim

<h4>

```luau
function colors.style.dim(text: string) -> string,
```

</h4>

---

### colors.style.bold

<h4>

```luau
function colors.style.bold(text: string) -> string,
```

</h4>

---

### colors.style.underline

<h4>

```luau
function colors.style.underline(text: string) -> string,
```

</h4>

---

### colors.codes.RESET

<h4>

```luau
RESET: "\x1b[0m",
```

</h4>

---

### colors.codes.BLACK

<h4>

```luau
BLACK: "\x1b[30m",
```

</h4>

---

### colors.codes.RED

<h4>

```luau
RED: "\x1b[31m",
```

</h4>

---

### colors.codes.GREEN

<h4>

```luau
GREEN: "\x1b[32m",
```

</h4>

---

### colors.codes.YELLOW

<h4>

```luau
YELLOW: "\x1b[33m",
```

</h4>

---

### colors.codes.BLUE

<h4>

```luau
BLUE: "\x1b[34m",
```

</h4>

---

### colors.codes.MAGENTA

<h4>

```luau
MAGENTA: "\x1b[35m",
```

</h4>

---

### colors.codes.CYAN

<h4>

```luau
CYAN: "\x1b[36m",
```

</h4>

---

### colors.codes.WHITE

<h4>

```luau
WHITE: "\x1b[37m",
```

</h4>

---

### colors.codes.BOLD_BLACK

<h4>

```luau
BOLD_BLACK: "\x1b[1;30m",
```

</h4>

---

### colors.codes.BOLD_RED

<h4>

```luau
BOLD_RED: "\x1b[1;31m",
```

</h4>

---

### colors.codes.BOLD_GREEN

<h4>

```luau
BOLD_GREEN: "\x1b[1;32m",
```

</h4>

---

### colors.codes.BOLD_YELLOW

<h4>

```luau
BOLD_YELLOW: "\x1b[1;33m",
```

</h4>

---

### colors.codes.BOLD_BLUE

<h4>

```luau
BOLD_BLUE: "\x1b[1;34m",
```

</h4>

---

### colors.codes.BOLD_MAGENTA

<h4>

```luau
BOLD_MAGENTA: "\x1b[1;35m",
```

</h4>

---

### colors.codes.BOLD_CYAN

<h4>

```luau
BOLD_CYAN: "\x1b[1;36m",
```

</h4>

---

### colors.codes.BOLD_WHITE

<h4>

```luau
BOLD_WHITE: "\x1b[1;37m",
```

</h4>

---

### colors.codes.BRIGHT_BLACK

<h4>

```luau
BRIGHT_BLACK: "\x1b[90m",
```

</h4>

---

### colors.codes.BRIGHT_RED

<h4>

```luau
BRIGHT_RED: "\x1b[91m",
```

</h4>

---

### colors.codes.BRIGHT_GREEN

<h4>

```luau
BRIGHT_GREEN: "\x1b[92m",
```

</h4>

---

### colors.codes.BRIGHT_YELLOW

<h4>

```luau
BRIGHT_YELLOW: "\x1b[93m",
```

</h4>

---

### colors.codes.BRIGHT_BLUE

<h4>

```luau
BRIGHT_BLUE: "\x1b[94m",
```

</h4>

---

### colors.codes.BRIGHT_MAGENTA

<h4>

```luau
BRIGHT_MAGENTA: "\x1b[95m",
```

</h4>

---

### colors.codes.BRIGHT_CYAN

<h4>

```luau
BRIGHT_CYAN: "\x1b[96m",
```

</h4>

---

### colors.codes.BRIGHT_WHITE

<h4>

```luau
BRIGHT_WHITE: "\x1b[97m",
```

</h4>

---

### colors.codes.BLACK_BG

<h4>

```luau
BLACK_BG: "\x1b[40m",
```

</h4>

---

### colors.codes.RED_BG

<h4>

```luau
RED_BG: "\x1b[41m",
```

</h4>

---

### colors.codes.GREEN_BG

<h4>

```luau
GREEN_BG: "\x1b[42m",
```

</h4>

---

### colors.codes.YELLOW_BG

<h4>

```luau
YELLOW_BG: "\x1b[43m",
```

</h4>

---

### colors.codes.BLUE_BG

<h4>

```luau
BLUE_BG: "\x1b[44m",
```

</h4>

---

### colors.codes.MAGENTA_BG

<h4>

```luau
MAGENTA_BG: "\x1b[45m",
```

</h4>

---

### colors.codes.CYAN_BG

<h4>

```luau
CYAN_BG: "\x1b[46m",
```

</h4>

---

### colors.codes.WHITE_BG

<h4>

```luau
WHITE_BG: "\x1b[47m",
```

</h4>

---

### colors.codes.BRIGHT_BLACK_BG

<h4>

```luau
BRIGHT_BLACK_BG: "\x1b[100m",
```

</h4>

---

### colors.codes.BRIGHT_RED_BG

<h4>

```luau
BRIGHT_RED_BG: "\x1b[101m",
```

</h4>

---

### colors.codes.BRIGHT_GREEN_BG

<h4>

```luau
BRIGHT_GREEN_BG: "\x1b[102m",
```

</h4>

---

### colors.codes.BRIGHT_YELLOW_BG

<h4>

```luau
BRIGHT_YELLOW_BG: "\x1b[103m",
```

</h4>

---

### colors.codes.BRIGHT_BLUE_BG

<h4>

```luau
BRIGHT_BLUE_BG: "\x1b[104m",
```

</h4>

---

### colors.codes.BRIGHT_MAGENTA_BG

<h4>

```luau
BRIGHT_MAGENTA_BG: "\x1b[105m",
```

</h4>

---

### colors.codes.BRIGHT_CYAN_BG

<h4>

```luau
BRIGHT_CYAN_BG: "\x1b[106m",
```

</h4>

---

### colors.codes.BRIGHT_WHITE_BG

<h4>

```luau
BRIGHT_WHITE_BG: "\x1b[107m",
```

</h4>

---

### colors.codes.BOLD

<h4>

```luau
BOLD: "\x1b[1m",
```

</h4>

---

### colors.codes.DIM

<h4>

```luau
DIM: "\x1b[2m",
```

</h4>

---

### colors.codes.UNDERLINE

<h4>

```luau
UNDERLINE: "\x1b[4m",
```

</h4>

---

Autogenerated from [std/io/colors.luau](/.seal/typedefs/std/io/colors.luau).Please see that file if this documentation is confusing, inaccurate, or too verbose.
