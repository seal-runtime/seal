<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# fs.file

`local file = require("@std/fs/file")`

---

## `export type` FileBuilder

---

## `export type` FileLib

---

### FileLib.from

<h4>

```luau
from: (path: string) -> FileEntry,
```

</h4>

Create a `FileEntry` from `path`; errors if unable to create the `FileEntry` if a file is not found or permission was denied, etc.

---

### FileLib.build

<h4>

```luau
build: (name: string, content: string) -> FileBuilder,
```

</h4>

 Returns a `FileBuilder` for use with `fs.readtree` and `fs.writetree`

---

### FileLib.create

<h4>

```luau
create: (path: string) -> FileEntry,
```

</h4>

 Creates a *new*, empty file at `path` using Rust's `fs::File::create_new`; errors if a file or other entry already exists at that path.

---

### FileLib.try_read

<h4>

```luau
try_read: (path: string) -> (string?, "Ok" | "NotFound" | "PermissionDenied"),
```

</h4>

<details>

<summary> See the docs </summary

Tries to read a file to string from `path` when errors such as `NotFound` or `PermissionDenied` are expected and should be handled explicitly.

This is a better and TOCTOU-safer variant to `local content = if fs.path.exists(path) then fs.readfile(path) else nil`

## Usage

```luau
local content, result = fs.file.try_read("./mymaybefile.txt")
if typeof(content) == "string" and result == "Ok" then
    -- success case
elseif result == "NotFound" then
elseif result == "PermissionDenied" then
    print("i don't have access to this path!!")
else
    print(`unexpected extremely rare error: {result}`)
end
```

</details>

---

### FileLib.try_readbytes

<h4>

```luau
try_readbytes: (path: string, file_offset: number?, count: number?, target_buffer: buffer?, buffer_offset: number?) -> (buffer?, "Ok" | "NotFound" | "PermissionDenied"),
```

</h4>

<details>

<summary> See the docs </summary

Tries to read a file to buffer from `path` when errors such as `NotFound` or `PermissionDenied` are expected and should be handled explicitly.

This is a better and TOCTOU-safer variant to `local content = if fs.path.exists(path) then fs.readbytes(path) else nil`

## Usage

```luau
local content, result = fs.file.try_readbytes("./mymaybefile.txt", 0, 120)
if typeof(content) == "buffer" and result == "Ok" then
    -- success case
elseif result == "NotFound" then
elseif result == "PermissionDenied" then
    print("i don't have access to this path!!")
else
    print(`unexpected extremely rare error: {result}`)
end
```

## Errors

- if attempt to read a file into an incorrectly-sized buffer,
- invalid file or buffer offset (too big for file size, negative, etc.),
- error trying to Seek to the file offset (on Windows)

</details>

---

### FileLib.try_write

<h4>

```luau
try_write: (path: string, content: string | buffer) -> (boolean, "Ok" | "PermissionDenied"),
```

</h4>

<details>

<summary> See the docs </summary

Try to write `content` (string or buffer) into file at `path`, overwriting an existing file if present.

Use this if `PermissionDenied` is an expected result for your usecase.

## Usage

```luau
local success, result = fs.file.try_write("/opt/meow.txt", "meow")
if result == "PermissionDenied" then
    print("Can't write to file! Run me with sudo!!")
end
```

</details>

---

### FileLib.try_remove

<h4>

```luau
try_remove: (path: string) -> (boolean, "Ok" | "PermissionDenied" | "NotFound" | "IsADirectory")
```

</h4>

Try to remove a file at `path` without erroring if the file doesn't exist or if the user doesn't have access to it.

Doesn't follow symlinks.

## Errors

- if `path` is not a valid utf-8 encoded path that could possibly exist on the filesystem

---

### FileLib.__call

<h4>

```luau
__call: (self: any, path: string) -> FileEntry?,
```

</h4>

Convenient and slightly more efficient alternative to `fs.find(path):try_file()`

## Usage

```luau
local myfile = fs.file("./myfile.txt")
if myfile then
    print(myfile:metadata())
end
```

---
