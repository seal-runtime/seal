<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# fs.dir

`local dir = require("@std/fs/dir")`

`export type` DirectoryEntry

`export type` DirectoryBuilder

`export type` DirLib

DirLib.from: `(path: string) -> DirectoryEntry`

$\hspace{5pt}$  Creates a `DirectoryEntry` from the directory at `path`, erroring if the directory is NotFound/PermissionDenied, etc.

DirLib.build: `(name: string, tree: DirectoryTree) -> DirectoryBuilder`

$\hspace{5pt}$  Returns a `DirectoryBuilder` table for `fs.readtree`, `fs.writetree`, etc.

DirLib.create: `(path: string) -> DirectoryEntry`

$\hspace{5pt}$  Creates a *new* directory at `path`, erroring if an entry already exists there.

DirLib.ensure: `(path: string, create_missing: boolean?) -> DirectoryEntry`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ Ensures that a directory exists at `path` by trying to create it, catching any AlreadyExists error, and returning a `DirectoryEntry` at that path.
$\hspace{5pt}$
$\hspace{5pt}$ Similar to `fs.makedir(path, { error_if_exists = false }); fs.dir.from(path)`
$\hspace{5pt}$
$\hspace{5pt}$ ## Usage
$\hspace{5pt}$ ```luau
$\hspace{5pt}$     -- doesn't replace .vscode if it already exists, but creates it if it doesn't
$\hspace{5pt}$     local dot_vscode = fs.dir.ensure(".vscode")
$\hspace{5pt}$     local settings_json = dot_vscode:find("settings.json"):try_file()
$\hspace{5pt}$```

</details>

DirLib.try_remove: `(path: string) -> (boolean, "Ok" | "PermissionDenied" | "NotFound" | "NotADirectory" | "Other", string?)`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ Try to remove directory at `path` using Rust's `fs::remove_dir_all` without erroring in common cases.
$\hspace{5pt}$
$\hspace{5pt}$ If this function partially fails (removes some but not all subtrees/files in `path`), `try_remove` will return false
$\hspace{5pt}$ with result "Other", as well as an error kind string that describes what went wrong.
$\hspace{5pt}$
$\hspace{5pt}$ ## Errors
$\hspace{5pt}$ - if provided invalid arguments (`path` is not a valid utf-8 encoded string that could exist on the filesystem)

</details>

DirLib.home: `() -> DirectoryEntry`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ Returns a `DirectoryEntry` corresponding to the user's home directory, erroring if not found.
$\hspace{5pt}$
$\hspace{5pt}$ ## Usage
$\hspace{5pt}$
$\hspace{5pt}$ ```luau
$\hspace{5pt}$ local zip_downloads = fs.dir.home()
$\hspace{5pt}$     :expect_dir("Downloads")
$\hspace{5pt}$     :list(false, function(path: string)
$\hspace{5pt}$         return str.endswith(path, ".zip")
$\hspace{5pt}$     end)
$\hspace{5pt}$```

</details>

DirLib.cwd: `() -> DirectoryEntry`

$\hspace{5pt}$ Constructs a `DirectoryEntry` from the user's current working directory (cwd)
$\hspace{5pt}$
$\hspace{5pt}$ If you're looking for project-relative pathing, I recommend using `fs.dir.project()` instead, as those will work no matter
$\hspace{5pt}$ where the user is when they execute your code.

DirLib.project: `(n: number?) -> DirectoryEntry`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ Constructs a `DirectoryEntry` from the script's current *seal* project.
$\hspace{5pt}$
$\hspace{5pt}$ Use this for most project-relative paths instead of `fs.path.cwd` or `fs.dir.cwd` usages.
$\hspace{5pt}$
$\hspace{5pt}$ ## Errors
$\hspace{5pt}$
$\hspace{5pt}$ - if a *seal* project couldn't be found exactly `n` project parents relative to `script:path()`.
$\hspace{5pt}$ - use `fs.path.project` instead if you want to get the *seal* project without erroring.
$\hspace{5pt}$
$\hspace{5pt}$ ## Usage
$\hspace{5pt}$
$\hspace{5pt}$ ```luau
$\hspace{5pt}$ local fs = require("@std/fs")
$\hspace{5pt}$ local str = require("@std/str")
$\hspace{5pt}$
$\hspace{5pt}$ local input_files = fs.dir.project()
$\hspace{5pt}$     :expect_dir("input")
$\hspace{5pt}$     :list(false, function(path: string)
$\hspace{5pt}$         return str.endswith(path, ".csv")
$\hspace{5pt}$     end)

</details>

DirLib.__call: `(self: any, path: string) -> DirectoryEntry?`

<details>

<summary> See the docs </summary

$\hspace{5pt}$     Convenient and slightly more efficient alternative to `fs.find(path):try_dir()`
$\hspace{5pt}$
$\hspace{5pt}$     ## Usage
$\hspace{5pt}$ ```luau
$\hspace{5pt}$ local src_dir = fs.dir("./src")
$\hspace{5pt}$ if src_dir then
$\hspace{5pt}$     local main_luau = src_dir:expect_file("main.luau")
$\hspace{5pt}$     main_luau:append('print("meow")')
$\hspace{5pt}$ end
$\hspace{5pt}$```

</details>
