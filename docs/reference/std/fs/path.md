<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# fs.path

`local path = require("@std/fs/path")`

PathLib.join: `(...string) -> string`

<details>

<summary> See the docs </summary

$hspace{5pt}$Joins path components together in a cross-platform-safe manner.
$hspace{5pt}$
$hspace{5pt}$The default separator is `/`, except when dealing with absolute paths on Windows.
$hspace{5pt}$
$hspace{5pt}$On Windows, pass `.\` as the first component to `path.join` to use `\` in relative paths.
$hspace{5pt}$
$hspace{5pt}$## Usage
$hspace{5pt}$```luau
$hspace{5pt}$local srcpath = path.join(path.cwd(), "src")
$hspace{5pt}$local main_luau = path.join(srcpath, "main.luau")
$hspace{5pt}$local main_content = fs.readfile(main_luau)
$hspace{5pt}$
$hspace{5pt}$local otherfile_in_script_dir = path.join(script:parent(), "otherfile.txt")
$hspace{5pt}$```

</details>

PathLib.exists: `(path: string) -> boolean`

<details>

<summary> See the docs </summary

$hspace{5pt}$Checks if `path` exists on the filesystem using Rust's `std::fs::exists`.
$hspace{5pt}$
$hspace{5pt}$Note this function is ***not* TOCTOU (Time Of Check to Time Of Use)-safe**!
$hspace{5pt}$
$hspace{5pt}$In security-critical applications, use relatively error-safe functions like `fs.file.try_read`, `fs.file.try_write`, etc., which allow you to
$hspace{5pt}$handle cases like `NotFound` and `PermissionDenied` without wrapping error-throwing functions like `fs.readbytes` in a pcall.

</details>

PathLib.canonicalize: `(path: string) -> string`

$hspace{5pt}$Returns the canonical (absolute) form of `path` using Rust's `std::fs::canonicalize`, resolving symlinks and intermediate components.
$hspace{5pt}$
$hspace{5pt}$Errors if the requested path doesn't exist on the filesystem or is invalid.

PathLib.absolutize: `(path: string) -> string`

$hspace{5pt}$Returns the absolute path of `path` without checking the filesystem.
$hspace{5pt}$
$hspace{5pt}$Use this function if your path may or may not exist (yet).

PathLib.normalize: `(path: string) -> string`

<details>

<summary> See the docs </summary

$hspace{5pt}$Returns a normalized (cleaned) version of `path` with a consistent path separator and with duplicate separators and unneeded relative path symbol removed.
$hspace{5pt}$
$hspace{5pt}$By default, uses '/' as the path separator unless `path` is a Windows-style absolute path, in which case it'll use a backslash instead.
$hspace{5pt}$
$hspace{5pt}$```luau
$hspace{5pt}$local mixed_path = [[./hi/im/a\./file.txt]]
$hspace{5pt}$print(path.normalize(mixed_path)) --> "./hi/im/a/file.txt"
$hspace{5pt}$
$hspace{5pt}$-- absolute paths on windows use \
$hspace{5pt}$local windows_path = [[C:\Users\Example\Documents\project/main.luau]]
$hspace{5pt}$print(path.normalize(windows_path)) --> "C:\Users\Example\Documents\project\main.luau"
$hspace{5pt}$
$hspace{5pt}$-- paths with redundant separators get cleaned
$hspace{5pt}$local redundant_separators = [[C:\\Users\\Example//Documents////project\main.luau]]
$hspace{5pt}$print(path.normalize(redundant_separators)) --> "C:\Users\Example\Documents\project\main.luau"
$hspace{5pt}$```
$hspace{5pt}$
$hspace{5pt}$For Windows-style absolute paths, `path.normalize` handles both drive letter paths like `"C:\Users\Username\Documents\..."`
$hspace{5pt}$as well as UNC paths like `"\\network\share\text.txt"` or `"\\?\wsl\mnt\..."`.

</details>

PathLib.parent: `(path: string, n: number?) -> string?`

<details>

<summary> See the docs </summary

$hspace{5pt}$Returns the path of the parent directory `n` (default = 1) parents to the left of `path`
$hspace{5pt}$
$hspace{5pt}$## Usage
$hspace{5pt}$```luau
$hspace{5pt}$local fs = require("@std/fs")
$hspace{5pt}$local env = require("@std/env")
$hspace{5pt}$local path = fs.path
$hspace{5pt}$
$hspace{5pt}$local cwd = env.current_working_directory
$hspace{5pt}$local parent_dir = path.parent(cwd)
$hspace{5pt}$```

</details>

PathLib.child: `(path: string) -> string?`

$hspace{5pt}$ the farthest child/leaf/node of the path, ex. `path.child("./src/main.luau") == "main.luau"`

PathLib.home: `() -> string`

$hspace{5pt}$ returns the user's home directory, also known as `~`

PathLib.cwd: `() -> string`

$hspace{5pt}$ returns the current working directory, errors if not found or invalid utf-8.
$hspace{5pt}$
$hspace{5pt}$ Consider using `fs.path.project()` or `fs.dir.project()` instead if you want paths to be relative
$hspace{5pt}$ to the current project instead of relying on the user's cwd.

PathLib.project: `(n: number?, script_path: string?) -> string?`

<details>

<summary> See the docs </summary

$hspace{5pt}$Returns the *seal* project directory `n` projects up, relative to `script_path` or the current `script:path()` if unspecified.
$hspace{5pt}$
$hspace{5pt}$To get the closest project directory to the current file, use `fs.path.project()`.
$hspace{5pt}$
$hspace{5pt}$Returns the project directory if found, or `nil` if no project directory was found exactly `n` projects up.

</details>
