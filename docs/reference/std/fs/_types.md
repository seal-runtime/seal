<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# fs._types

`local _types = require("@std/fs/_types")`

---

### TreeBuilder.inner

```luau
TreeBuilder.inner: DirectoryTree,
```

 The `DirectoryTree` being constructed by the `TreeBuilder`.

---

### TreeBuilder.with_file

```luau
TreeBuilder.with_file: (self: TreeBuilder, name: string, content: string) -> TreeBuilder,
```

 Add a file to the DirectoryTree by `name` with `content`

---

### TreeBuilder.with_tree

```luau
TreeBuilder.with_tree: (self: TreeBuilder, name: string, builder: TreeBuilder) -> TreeBuilder,
```

Add a new tree to the DirectoryTree; the second argument should be another `TreeBuilder` from `fs.tree()`

## Usage

```luau
local dir = fs.tree()
    :with_tree("subtree", fs.tree()
        :with_file("hi.json", '{"hi"}: true')
    )
```

---

### `export type` FileEntry

```luau

```

---

### FileEntry.name

```luau
FileEntry.name: string,
```

 The name of the file; also called basename, filename, etc. Can also be obtained by calling `fs.path.child` on a path.

---

### FileEntry.path

```luau
FileEntry.path: string,
```

 A filesystem path to the file; if the `FileEntry` was requested with an absolute path, then this path will be absolute, otherwise it'll be a relative path.

---

### FileEntry.type

```luau
FileEntry.type: "File",
```

---

### FileEntry.read

```luau
FileEntry.read: (self: FileEntry) -> string,
```

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

```luau
FileEntry.size: (self: FileEntry) -> number,
```

 Returns the file's size (length) in bytes.

---

### FileEntry.readlines

```luau
FileEntry.readlines: (self: FileEntry) -> () -> (number, string),
```

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

```luau
FileEntry.readbytes: (self: FileEntry, file_offset: number?, count: number?, target_buffer: buffer?, buffer_offset: number?) -> buffer,
```

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

```luau
FileEntry.append: (self: FileEntry, content: buffer | string) -> (),
```

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

```luau
FileEntry.is_valid_utf8: (self: FileEntry) -> boolean,
```

---

### FileEntry.metadata

```luau
FileEntry.metadata: (self: FileEntry) -> FsMetadata,
```

Returns a `FsMetadata` table containing timestamps for creation, modified, and access times, as well as permissions (depends on your operating system)

---

### FileEntry.copy_to

```luau
FileEntry.copy_to: (self: FileEntry, to: string) -> (),
```

---

### FileEntry.move_to

```luau
FileEntry.move_to: (self: FileEntry, to: string) -> (),
```

---

### FileEntry.rename

```luau
FileEntry.rename: (self: FileEntry, name: string) -> (),
```

---

### FileEntry.remove

```luau
FileEntry.remove: (self: FileEntry) -> (),
```

 Removes the file at `FileEntry.path`.

---

### `export type` DirectoryEntry

```luau

```

---

### DirectoryEntry.name

```luau
DirectoryEntry.name: string,
```

 The name of the directory; also called basename, etc. Can also be obtained by calling `fs.path.child` on a path.

---

### DirectoryEntry.path

```luau
DirectoryEntry.path: string,
```

 A filesystem path to the directory; if the `DirectoryEntry` was requested with an absolute path, then this path will be absolute,
 otherwise it'll be a relative path.

---

### DirectoryEntry.type

```luau
DirectoryEntry.type: "Directory",
```

---

### DirectoryEntry.list

```luau
DirectoryEntry.list: (self: DirectoryEntry, recursive: boolean?, filter: ((path: string) -> boolean)?) -> { string },
```

 Returns an an array of basenames of the directory's entries; pass `true` as the second argument to list all files recursively.
 Pass a filter function to narrow the returned list (to search for specific file names, extensions, etc.)

---

### DirectoryEntry.join

```luau
DirectoryEntry.join: (self: DirectoryEntry, ...string) -> string,
```

 Join the `DirectoryEntry`'s path with multiple paths in a cross-platform-compliant manner.
 Basically a wrapper around `fs.path.join(entry.path, a, b, c, ...)`

---

### DirectoryEntry.find

```luau
DirectoryEntry.find: (self: DirectoryEntry, name: string, options: { follow_symlinks: boolean?, error_if_permission_denied: boolean? }?) -> FindResult,
```

---

### DirectoryEntry.entries

```luau
DirectoryEntry.entries: (self: DirectoryEntry) -> { [string]: Entry },
```

---

### DirectoryEntry.expect_file

```luau
DirectoryEntry.expect_file: (self: DirectoryEntry, name: string) -> FileEntry,
```

 Expect that the directory contains file `name`, returning its `FileEntry` or otherwise error.

---

### DirectoryEntry.expect_dir

```luau
DirectoryEntry.expect_dir: (self: DirectoryEntry, name: string) -> DirectoryEntry,
```

 Expect that the directory contains directory `name`, returning its `DirectoryEntry` or otherwise error.

---

### DirectoryEntry.add_file

```luau
DirectoryEntry.add_file: (self: DirectoryEntry, name: string, content: string | buffer) -> DirectoryEntry,
```

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

```luau
DirectoryEntry.add_tree: (self: DirectoryEntry, name: string, builder: TreeBuilder) -> DirectoryEntry,
```

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

```luau
DirectoryEntry.metadata: (self: DirectoryEntry) -> FsMetadata,
```

Returns a `FsMetadata` table containing timestamps for creation, modified, and access times, as well as permissions (depends on your operating system)

---

### DirectoryEntry.copy_to

```luau
DirectoryEntry.copy_to: (self: DirectoryEntry, to: string) -> (),
```

---

### DirectoryEntry.move_to

```luau
DirectoryEntry.move_to: (self: DirectoryEntry, to: string) -> (),
```

---

### DirectoryEntry.rename

```luau
DirectoryEntry.rename: (self: DirectoryEntry, name: string) -> (),
```

---

### DirectoryEntry.remove

```luau
DirectoryEntry.remove: (self: DirectoryEntry) -> (),
```

 Removes the directory at `DirectoryEntry.path`, alongside all its contents.

---

### `export type` Entry

```luau

```

---

### `export type` FsMetadata

```luau

```

---

### FsMetadata.created_at

```luau
FsMetadata.created_at: DateTime?,
```

 A UTC DateTime representing when the `Entry` was created.
 This field is optional because it might not be available on all platforms.

---

### FsMetadata.modified_at

```luau
FsMetadata.modified_at: DateTime?,
```

 A UTC DateTime representing when the `Entry` was last modified.
 This field is optional because it might not be available on all platforms.

---

### FsMetadata.accessed_at

```luau
FsMetadata.accessed_at: DateTime?,
```

 A UTC DateTime representing when the `Entry` was last accessed.
 This field is optional because it might not be available on all platforms.

---

### FsMetadata.permissions.readonly

```luau
FsMetadata.permissions.readonly: boolean,
```

 Whether the `Entry` is read-only or not. Should be accessible on both Windows and Unix-like operating systems.

---

### FsMetadata.permissions.unix_mode

```luau
FsMetadata.permissions.unix_mode: number?,
```

 Represents the numeric Unix permission bits for the `Entry`, combining read, write, and execute permissions
 for owner, group, and others. This field is optional because it's not available on Windows.

---

### `export type` FindResult

```luau

```

---

### FindResult.ok

```luau
FindResult.ok: boolean,
```

 `true` if the find operation succeeded ("File" | "Directory" | "Symlink"), otherwise `false` ("NotFound" | "PermissionDenied")

---

### FindResult.path

```luau
FindResult.path: string,
```

---

### FindResult.type

```luau
FindResult.type: "File" | "Directory" | "Symlink" | "NotFound" | "PermissionDenied",
```

---

### FindResult.exists

```luau
FindResult.exists: (self: FindResult) -> boolean,
```

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

```luau
FindResult.try_file: (self: FindResult) -> FileEntry?,
```

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

```luau
FindResult.try_dir: (self: FindResult) -> DirectoryEntry?,
```

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

```luau
FindResult.unwrap_file: (self: FindResult) -> FileEntry,
```

 Create a `FileEntry` from the `FindResult`, erroring if the file doesn't exist.

---

### FindResult.unwrap_dir

```luau
FindResult.unwrap_dir: (self: FindResult) -> DirectoryEntry,
```

 Create a `DirectoryEntry` from the `FindResult`, erroring if the directory doesn't exist.

---

### `export type` FileBuilder

```luau

```

---

### FileBuilder.name

```luau
FileBuilder.name: string,
```

---

### FileBuilder.type

```luau
FileBuilder.type: "File",
```

---

### FileBuilder.content

```luau
FileBuilder.content: string,
```

---

### `export type` DirectoryBuilder

```luau

```

---

### DirectoryBuilder.name

```luau
DirectoryBuilder.name: string,
```

---

### DirectoryBuilder.type

```luau
DirectoryBuilder.type: "Directory",
```

---

### DirectoryBuilder.children

```luau
DirectoryBuilder.children: DirectoryTree,
```

---

### `export type` DirectoryTree

```luau

```

---
