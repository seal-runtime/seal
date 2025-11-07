<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# fs.file

`local file = require("@std/fs/file")`

`export type` FileBuilder

`export type` FileLib

FileLib.from: `(path: string) -> FileEntry`

$hspace{5pt}$Create a `FileEntry` from `path`; errors if unable to create the `FileEntry` if a file is not found or permission was denied, etc.

FileLib.build: `(name: string, content: string) -> FileBuilder`

$hspace{5pt}$ Returns a `FileBuilder` for use with `fs.readtree` and `fs.writetree`

FileLib.create: `(path: string) -> FileEntry`

$hspace{5pt}$ Creates a *new*, empty file at `path` using Rust's `fs::File::create_new`; errors if a file or other entry already exists at that path.

FileLib.try_read: `(path: string) -> (string?, "Ok" | "NotFound" | "PermissionDenied")`

<details>

<summary> See the docs </summary

$hspace{5pt}$Tries to read a file to string from `path` when errors such as `NotFound` or `PermissionDenied` are expected and should be handled explicitly.
$hspace{5pt}$
$hspace{5pt}$This is a better and TOCTOU-safer variant to `local content = if fs.path.exists(path) then fs.readfile(path) else nil`
$hspace{5pt}$
$hspace{5pt}$## Usage
$hspace{5pt}$```luau
$hspace{5pt}$local content, result = fs.file.try_read("./mymaybefile.txt")
$hspace{5pt}$if typeof(content) == "string" and result == "Ok" then
$hspace{5pt}$    -- success case
$hspace{5pt}$elseif result == "NotFound" then
$hspace{5pt}$elseif result == "PermissionDenied" then
$hspace{5pt}$    print("i don't have access to this path!!")
$hspace{5pt}$else
$hspace{5pt}$    print(`unexpected extremely rare error: {result}`)
$hspace{5pt}$end
$hspace{5pt}$```

</details>

FileLib.try_readbytes: `(path: string, file_offset: number?, count: number?, target_buffer: buffer?, buffer_offset: number?) -> (buffer?, "Ok" | "NotFound" | "PermissionDenied")`

<details>

<summary> See the docs </summary

$hspace{5pt}$Tries to read a file to buffer from `path` when errors such as `NotFound` or `PermissionDenied` are expected and should be handled explicitly.
$hspace{5pt}$
$hspace{5pt}$This is a better and TOCTOU-safer variant to `local content = if fs.path.exists(path) then fs.readbytes(path) else nil`
$hspace{5pt}$
$hspace{5pt}$## Usage
$hspace{5pt}$```luau
$hspace{5pt}$local content, result = fs.file.try_readbytes("./mymaybefile.txt", 0, 120)
$hspace{5pt}$if typeof(content) == "buffer" and result == "Ok" then
$hspace{5pt}$    -- success case
$hspace{5pt}$elseif result == "NotFound" then
$hspace{5pt}$elseif result == "PermissionDenied" then
$hspace{5pt}$    print("i don't have access to this path!!")
$hspace{5pt}$else
$hspace{5pt}$    print(`unexpected extremely rare error: {result}`)
$hspace{5pt}$end
$hspace{5pt}$```
$hspace{5pt}$
$hspace{5pt}$## Errors
$hspace{5pt}$- if attempt to read a file into an incorrectly-sized buffer,
$hspace{5pt}$- invalid file or buffer offset (too big for file size, negative, etc.),
$hspace{5pt}$- error trying to Seek to the file offset (on Windows)

</details>

FileLib.try_write: `(path: string, content: string | buffer) -> (boolean, "Ok" | "PermissionDenied")`

<details>

<summary> See the docs </summary

$hspace{5pt}$Try to write `content` (string or buffer) into file at `path`, overwriting an existing file if present.
$hspace{5pt}$
$hspace{5pt}$Use this if `PermissionDenied` is an expected result for your usecase.
$hspace{5pt}$
$hspace{5pt}$## Usage
$hspace{5pt}$```luau
$hspace{5pt}$local success, result = fs.file.try_write("/opt/meow.txt", "meow")
$hspace{5pt}$if result == "PermissionDenied" then
$hspace{5pt}$    print("Can't write to file! Run me with sudo!!")
$hspace{5pt}$end
$hspace{5pt}$```

</details>

FileLib.try_remove: `(path: string) -> (boolean, "Ok" | "PermissionDenied" | "NotFound" | "IsADirectory")`

$hspace{5pt}$Try to remove a file at `path` without erroring if the file doesn't exist or if the user doesn't have access to it.
$hspace{5pt}$
$hspace{5pt}$Doesn't follow symlinks.
$hspace{5pt}$
$hspace{5pt}$## Errors
$hspace{5pt}$- if `path` is not a valid utf-8 encoded path that could possibly exist on the filesystem

FileLib.__call: `(self: any, path: string) -> FileEntry?`

$hspace{5pt}$Convenient and slightly more efficient alternative to `fs.find(path):try_file()`
$hspace{5pt}$
$hspace{5pt}$## Usage
$hspace{5pt}$```luau
$hspace{5pt}$local myfile = fs.file("./myfile.txt")
$hspace{5pt}$if myfile then
$hspace{5pt}$    print(myfile:metadata())
$hspace{5pt}$end
$hspace{5pt}$```
