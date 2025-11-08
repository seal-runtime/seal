<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# fs._types

`local _types = require("@std/fs/_types")`

---

### TreeBuilder.inner

<h4>

```luau
inner: DirectoryTree,
```

</h4>

 The `DirectoryTree` being constructed by the `TreeBuilder`.

---

### TreeBuilder.with_file

<h4>

```luau
function (self: TreeBuilder, name: string, content: string) -> TreeBuilder,
```

</h4>

 Add a file to the DirectoryTree by `name` with `content`

---

### TreeBuilder.with_tree

<h4>

```luau
function (self: TreeBuilder, name: string, builder: TreeBuilder) -> TreeBuilder,
```

</h4>

Add a new tree to the DirectoryTree; the second argument should be another `TreeBuilder` from `fs.tree()`

## Usage

```luau
local dir = fs.tree()
    :with_tree("subtree", fs.tree()
        :with_file("hi.json", '{"hi"}: true')
    )
```

---

## `export type` FileEntry

---

### FileEntry.name

<h4>

```luau
name: string,
```

</h4>

 The name of the file; also called basename, filename, etc. Can also be obtained by calling `fs.path.child` on a path.

---

### FileEntry.path

<h4>

```luau
path: string,
```

</h4>

 A filesystem path to the file; if the `FileEntry` was requested with an absolute path, then this path will be absolute, otherwise it'll be a relative path.

---

### FileEntry.type

<h4>

```luau
type: "File",
```

</h4>

---

### FileEntry.read

<h4>

```luau
function (self: FileEntry) -> string,
```

</h4>

<details>

<summary> See the docs </summary

Reads the file to string without performing utf-8 validation on the file's contents.

## Errors

- if `path` is not valid utf-8
- the file doesn't exist or is actually a directory
- you don't have permission to access `path`

Use `fs.file.try_read` instead if you want to handle`NotFound` and/or `PermissionDenied` explicitly without erroring.

This function blocks the current Luau VM. To use it in parallel, call it within a child thread from `@std/thread`.

</details>

---

### FileEntry.size

<h4>

```luau
function (self: FileEntry) -> number,
```

</h4>

 Returns the file's size (length) in bytes.

---

### FileEntry.readlines

<h4>

```luau
function (self: FileEntry) -> () -> (number, string),
```

</h4>

Iterate over the lines of the file without reading the whole file into memory.

## Usage

```luau
local csv = fs.file("./mybigdata.csv")
if csv then
    for line_number, line in csv:readlines() do
        print(line)
    end
end
```

## Errors

- if the file's contents are not valid utf-8
- user cannot access the file

---

### FileEntry.readbytes

<h4>

```luau
function (self: FileEntry, file_offset: number?, count: number?, target_buffer: buffer?, buffer_offset: number?) -> buffer,
```

</h4>

<details>

<summary> See the docs </summary

Reads the file into a buffer.

## Parameters

- `file_offset`: the number of bytes from the start of the file to the start of the portion you want to read. Default is `0` (start of file)
- `count`: the number of bytes you want to read (starting from `file_offset`) and copy into the buffer
- `target_buffer`: an optional buffer you want to write into; if not specified, a new buffer will be created for you
- `buffer_offset`: an optional number of bytes from the start of the `target_buffer` provided; this is useful if you're filling the same buffer from multiple calls

## Returns

- `target_buffer`: the exact same `target_buffer` provided, or a new buffer if not provided

## Errors

- if `path` is not a file, not valid utf-8, is actually a directory, not found or permission denied, etc.
- `file_offset`, `count`, or `buffer_offset` cannot be converted into positive mlua Integers
- provided `target_buffer` is too small (`buffer_offset` + `count` > buffer size)
- attempt to read a nonexistent portion of file (`file_offset` + `count` > file size)

This function blocks the current Luau VM. To use it in parallel, call it within a child thread from `@std/thread`.

</details>

---

### FileEntry.append

<h4>

```luau
function (self: FileEntry, content: buffer | string) -> (),
```

</h4>

<details>

<summary> See the docs </summary

Opens the file in append mode and appends `content` to the file.

Like `fs.writefile`, `content` does not have to be a valid utf-8 encoded string (though please just use a buffer instead)

## Errors

- if the file cannot be opened in append mode
- user does not have permission to append to the file
- unexpected error writing content to file

</details>

---

### FileEntry.is_valid_utf8

<h4>

```luau
function (self: FileEntry) -> boolean,
```

</h4>

---

### FileEntry.metadata

<h4>

```luau
function (self: FileEntry) -> FsMetadata,
```

</h4>

Returns a `FsMetadata` table containing timestamps for creation, modified, and access times, as well as permissions (depends on your operating system)

---

### FileEntry.copy_to

<h4>

```luau
function (self: FileEntry, to: string) -> (),
```

</h4>

---

### FileEntry.move_to

<h4>

```luau
function (self: FileEntry, to: string) -> (),
```

</h4>

---

### FileEntry.rename

<h4>

```luau
function (self: FileEntry, name: string) -> (),
```

</h4>

---

### FileEntry.remove

<h4>

```luau
function (self: FileEntry) -> (),
```

</h4>

 Removes the file at `FileEntry.path`.

---

## `export type` DirectoryEntry

---

### DirectoryEntry.name

<h4>

```luau
name: string,
```

</h4>

 The name of the directory; also called basename, etc. Can also be obtained by calling `fs.path.child` on a path.

---

### DirectoryEntry.path

<h4>

```luau
path: string,
```

</h4>

 A filesystem path to the directory; if the `DirectoryEntry` was requested with an absolute path, then this path will be absolute,
 otherwise it'll be a relative path.

---

### DirectoryEntry.type

<h4>

```luau
type: "Directory",
```

</h4>

---

### DirectoryEntry.list

<h4>

```luau
function (self: DirectoryEntry, recursive: boolean?, filter: ((path: string) -> boolean)?) -> { string },
```

</h4>

 Returns an an array of basenames of the directory's entries; pass `true` as the second argument to list all files recursively.
 Pass a filter function to narrow the returned list (to search for specific file names, extensions, etc.)

---

### DirectoryEntry.join

<h4>

```luau
function (self: DirectoryEntry, ...string) -> string,
```

</h4>

 Join the `DirectoryEntry`'s path with multiple paths in a cross-platform-compliant manner.
 Basically a wrapper around `fs.path.join(entry.path, a, b, c, ...)`

---

### DirectoryEntry.find

<h4>

```luau
function (self: DirectoryEntry, name: string, options: { follow_symlinks: boolean?, error_if_permission_denied: boolean? }?) -> FindResult,
```

</h4>

---

### DirectoryEntry.entries

<h4>

```luau
function (self: DirectoryEntry) -> { [string]: Entry },
```

</h4>

---

### DirectoryEntry.expect_file

<h4>

```luau
function (self: DirectoryEntry, name: string) -> FileEntry,
```

</h4>

 Expect that the directory contains file `name`, returning its `FileEntry` or otherwise error.

---

### DirectoryEntry.expect_dir

<h4>

```luau
function (self: DirectoryEntry, name: string) -> DirectoryEntry,
```

</h4>

 Expect that the directory contains directory `name`, returning its `DirectoryEntry` or otherwise error.

---

### DirectoryEntry.add_file

<h4>

```luau
function (self: DirectoryEntry, name: string, content: string | buffer) -> DirectoryEntry,
```

</h4>

<details>

<summary> See the docs </summary

Add or overwrite the file named `name` with `content`, returning the original `DirectoryEntry` for chaining.

## Usage

```luau
local src = fs.dir.ensure("./src")
    :add_file("main.luau", 'print("hi")')
    :add_file("byte.luau", 'print("munch")')
```

## Errors

- if name is not a valid, utf-8 encoded filename
- user does not have permission to write files in the directory

</details>

---

### DirectoryEntry.add_tree

<h4>

```luau
function (self: DirectoryEntry, name: string, builder: TreeBuilder) -> DirectoryEntry,
```

</h4>

<details>

<summary> See the docs </summary

Add a new directory tree to the directory from a `TreeBuilder`, returning the ***original*** `DirectoryEntry` for chaining.

## Usage

```luau
local src = fs.dir.ensure("./src")
    :add_tree("libraries", fs.tree()
        :with_file("Lists.luau", lists_src)
    )
```

## Errors

- if `name` is not a valid utf-8 encoded directory name
- the directory already contains a directory named `name` (or it's a file)

</details>

---

### DirectoryEntry.metadata

<h4>

```luau
function (self: DirectoryEntry) -> FsMetadata,
```

</h4>

Returns a `FsMetadata` table containing timestamps for creation, modified, and access times, as well as permissions (depends on your operating system)

---

### DirectoryEntry.copy_to

<h4>

```luau
function (self: DirectoryEntry, to: string) -> (),
```

</h4>

---

### DirectoryEntry.move_to

<h4>

```luau
function (self: DirectoryEntry, to: string) -> (),
```

</h4>

---

### DirectoryEntry.rename

<h4>

```luau
function (self: DirectoryEntry, name: string) -> (),
```

</h4>

---

### DirectoryEntry.remove

<h4>

```luau
function (self: DirectoryEntry) -> (),
```

</h4>

 Removes the directory at `DirectoryEntry.path`, alongside all its contents.

---

## `export type` Entry

---

## `export type` FsMetadata

---

### FsMetadata.created_at

<h4>

```luau
created_at: DateTime?,
```

</h4>

 A UTC DateTime representing when the `Entry` was created.
 This field is optional because it might not be available on all platforms.

---

### FsMetadata.modified_at

<h4>

```luau
modified_at: DateTime?,
```

</h4>

 A UTC DateTime representing when the `Entry` was last modified.
 This field is optional because it might not be available on all platforms.

---

### FsMetadata.accessed_at

<h4>

```luau
accessed_at: DateTime?,
```

</h4>

 A UTC DateTime representing when the `Entry` was last accessed.
 This field is optional because it might not be available on all platforms.

---

### FsMetadata.permissions.readonly

<h4>

```luau
readonly: boolean,
```

</h4>

 Whether the `Entry` is read-only or not. Should be accessible on both Windows and Unix-like operating systems.

---

### FsMetadata.permissions.unix_mode

<h4>

```luau
unix_mode: number?,
```

</h4>

 Represents the numeric Unix permission bits for the `Entry`, combining read, write, and execute permissions
 for owner, group, and others. This field is optional because it's not available on Windows.

---

## `export type` FindResult

---

### FindResult.ok

<h4>

```luau
ok: boolean,
```

</h4>

 `true` if the find operation succeeded ("File" | "Directory" | "Symlink"), otherwise `false` ("NotFound" | "PermissionDenied")

---

### FindResult.path

<h4>

```luau
path: string,
```

</h4>

---

### FindResult.type

<h4>

```luau
type: "File" | "Directory" | "Symlink" | "NotFound" | "PermissionDenied",
```

</h4>

---

### FindResult.exists

<h4>

```luau
function (self: FindResult) -> boolean,
```

</h4>

Checks if `FindResult.path` exists on the filesystem.

## Usage

```luau
if current_dir:find(".luaurc"):exists() then
    luaurc_path = path.join(current_dir.path, ".luaurc")
end
```

## Errors

- if permission denied

---

### FindResult.try_file

<h4>

```luau
function (self: FindResult) -> FileEntry?,
```

</h4>

Attempt to create a `FileEntry` from the `FindResult`, returning it or `nil` if unsuccessful.

## Usage

```luau
local main_luau = fs.find("./src/main.luau"):try_file()
if main_luau then
    process.shell(`seal {main_luau.path}`):unwrap()
end
```

---

### FindResult.try_dir

<h4>

```luau
function (self: FindResult) -> DirectoryEntry?,
```

</h4>

Attempt to create a `DirectoryEntry` from the `FindResult`, returning it or `nil` if unsuccessful.

## Usage

```luau
local src = fs.find("./src"):try_dir()
if src then
    print(src:list(--[[recursive = ]] true))
end
```

---

### FindResult.unwrap_file

<h4>

```luau
function (self: FindResult) -> FileEntry,
```

</h4>

 Create a `FileEntry` from the `FindResult`, erroring if the file doesn't exist.

---

### FindResult.unwrap_dir

<h4>

```luau
function (self: FindResult) -> DirectoryEntry,
```

</h4>

 Create a `DirectoryEntry` from the `FindResult`, erroring if the directory doesn't exist.

---

## `export type` FileBuilder

---

### FileBuilder.name

<h4>

```luau
name: string,
```

</h4>

---

### FileBuilder.type

<h4>

```luau
type: "File",
```

</h4>

---

### FileBuilder.content

<h4>

```luau
content: string,
```

</h4>

---

## `export type` DirectoryBuilder

---

### DirectoryBuilder.name

<h4>

```luau
name: string,
```

</h4>

---

### DirectoryBuilder.type

<h4>

```luau
type: "Directory",
```

</h4>

---

### DirectoryBuilder.children

<h4>

```luau
children: DirectoryTree,
```

</h4>

---

## `export type` DirectoryTree

---
