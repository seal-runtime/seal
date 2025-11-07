<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# fs._types

`local _types = require("@std/fs/_types")`

TreeBuilder.inner: `DirectoryTree`

$\hspace{5pt}$ --- The `DirectoryTree` being constructed by the `TreeBuilder`.

TreeBuilder.with_file: `(self: TreeBuilder, name: string, content: string) -> TreeBuilder`

$\hspace{5pt}$ --- Add a file to the DirectoryTree by `name` with `content`

TreeBuilder.with_tree: `(self: TreeBuilder, name: string, builder: TreeBuilder) -> TreeBuilder`

$\hspace{5pt}$ Add a new tree to the DirectoryTree; the second argument should be another `TreeBuilder` from `fs.tree()`
$\hspace{5pt}$
$\hspace{5pt}$ ## Usage

```luau
local dir = fs.tree()
    :with_tree("subtree", fs.tree()
        :with_file("hi.json", '{"hi"}: true')
    )
$\hspace{5pt}$ ```

`export type` FileEntry

FileEntry.name: `string`

$\hspace{5pt}$ --- The name of the file; also called basename, filename, etc. Can also be obtained by calling `fs.path.child` on a path.

FileEntry.path: `string`

$\hspace{5pt}$ --- A filesystem path to the file; if the `FileEntry` was requested with an absolute path, then this path will be absolute, otherwise it'll be a relative path.

FileEntry.type: `"File"`

FileEntry.read: `(self: FileEntry) -> string`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ Reads the file to string without performing utf-8 validation on the file's contents.
$\hspace{5pt}$ 
$\hspace{5pt}$ ## Errors
$\hspace{5pt}$ - if `path` is not valid utf-8
$\hspace{5pt}$ - the file doesn't exist or is actually a directory
$\hspace{5pt}$ - you don't have permission to access `path`
$\hspace{5pt}$ 
$\hspace{5pt}$ Use `fs.file.try_read` instead if you want to handle`NotFound` and/or `PermissionDenied` explicitly without erroring.
$\hspace{5pt}$ 
$\hspace{5pt}$ This function blocks the current Luau VM. To use it in parallel, call it within a child thread from `@std/thread`.

</details>


FileEntry.size: `(self: FileEntry) -> number`

$\hspace{5pt}$ --- Returns the file's size (length) in bytes.

FileEntry.readlines: `(self: FileEntry) -> () -> (number, string)`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ Iterate over the lines of the file without reading the whole file into memory.
$\hspace{5pt}$ 
$\hspace{5pt}$ ## Usage
```luau
local csv = fs.file("./mybigdata.csv")
if csv then
    for line_number, line in csv:readlines() do
        print(line)
    end
end
$\hspace{5pt}$ ```
$\hspace{5pt}$ 
$\hspace{5pt}$ ## Errors
$\hspace{5pt}$ - if the file's contents are not valid utf-8
$\hspace{5pt}$ - user cannot access the file

</details>


FileEntry.readbytes: `(self: FileEntry, file_offset: number?, count: number?, target_buffer: buffer?, buffer_offset: number?) -> buffer`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ Reads the file into a buffer.
$\hspace{5pt}$ 
$\hspace{5pt}$ ## Parameters
$\hspace{5pt}$ - `file_offset`: the number of bytes from the start of the file to the start of the portion you want to read. Default is `0` (start of file)
$\hspace{5pt}$ - `count`: the number of bytes you want to read (starting from `file_offset`) and copy into the buffer
$\hspace{5pt}$ - `target_buffer`: an optional buffer you want to write into; if not specified, a new buffer will be created for you
$\hspace{5pt}$ - `buffer_offset`: an optional number of bytes from the start of the `target_buffer` provided; this is useful if you're filling the same buffer from multiple calls
$\hspace{5pt}$ 
$\hspace{5pt}$ ## Returns
$\hspace{5pt}$ - `target_buffer`: the exact same `target_buffer` provided, or a new buffer if not provided
$\hspace{5pt}$ 
$\hspace{5pt}$ ## Errors
$\hspace{5pt}$ - if `path` is not a file, not valid utf-8, is actually a directory, not found or permission denied, etc.
$\hspace{5pt}$ - `file_offset`, `count`, or `buffer_offset` cannot be converted into positive mlua Integers
$\hspace{5pt}$ - provided `target_buffer` is too small (`buffer_offset` + `count` > buffer size)
$\hspace{5pt}$ - attempt to read a nonexistent portion of file (`file_offset` + `count` > file size)
$\hspace{5pt}$ 
$\hspace{5pt}$ This function blocks the current Luau VM. To use it in parallel, call it within a child thread from `@std/thread`.

</details>


FileEntry.append: `(self: FileEntry, content: buffer | string) -> ()`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ Opens the file in append mode and appends `content` to the file.
$\hspace{5pt}$ 
$\hspace{5pt}$ Like `fs.writefile`, `content` does not have to be a valid utf-8 encoded string (though please just use a buffer instead)
$\hspace{5pt}$ 
$\hspace{5pt}$ ## Errors
$\hspace{5pt}$ - if the file cannot be opened in append mode
$\hspace{5pt}$ - user does not have permission to append to the file
$\hspace{5pt}$ - unexpected error writing content to file

</details>


FileEntry.is_valid_utf8: `(self: FileEntry) -> boolean`

FileEntry.metadata: `(self: FileEntry) -> FsMetadata`

$\hspace{5pt}$ Returns a `FsMetadata` table containing timestamps for creation, modified, and access times, as well as permissions (depends on your operating system)

FileEntry.copy_to: `(self: FileEntry, to: string) -> ()`

FileEntry.move_to: `(self: FileEntry, to: string) -> ()`

FileEntry.rename: `(self: FileEntry, name: string) -> ()`

FileEntry.remove: `(self: FileEntry) -> ()`

$\hspace{5pt}$ --- Removes the file at `FileEntry.path`.

`export type` DirectoryEntry

DirectoryEntry.name: `string`

$\hspace{5pt}$ --- The name of the directory; also called basename, etc. Can also be obtained by calling `fs.path.child` on a path.

DirectoryEntry.path: `string`

$\hspace{5pt}$ --- A filesystem path to the directory; if the `DirectoryEntry` was requested with an absolute path, then this path will be absolute,
$\hspace{5pt}$ --- otherwise it'll be a relative path.

DirectoryEntry.type: `"Directory"`

DirectoryEntry.list: `(self: DirectoryEntry, recursive: boolean?, filter: ((path: string) -> boolean)?) -> { string }`

$\hspace{5pt}$ --- Returns an an array of basenames of the directory's entries; pass `true` as the second argument to list all files recursively.
$\hspace{5pt}$ --- Pass a filter function to narrow the returned list (to search for specific file names, extensions, etc.)

DirectoryEntry.join: `(self: DirectoryEntry, ...string) -> string`

$\hspace{5pt}$ --- Join the `DirectoryEntry`'s path with multiple paths in a cross-platform-compliant manner.
$\hspace{5pt}$ --- Basically a wrapper around `fs.path.join(entry.path, a, b, c, ...)`

DirectoryEntry.find: `(self: DirectoryEntry, name: string, options: { follow_symlinks: boolean?, error_if_permission_denied: boolean? }?) -> FindResult`

DirectoryEntry.entries: `(self: DirectoryEntry) -> { [string]: Entry }`

DirectoryEntry.expect_file: `(self: DirectoryEntry, name: string) -> FileEntry`

$\hspace{5pt}$ --- Expect that the directory contains file `name`, returning its `FileEntry` or otherwise error.

DirectoryEntry.expect_dir: `(self: DirectoryEntry, name: string) -> DirectoryEntry`

$\hspace{5pt}$ --- Expect that the directory contains directory `name`, returning its `DirectoryEntry` or otherwise error.

DirectoryEntry.add_file: `(self: DirectoryEntry, name: string, content: string | buffer) -> DirectoryEntry`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ Add or overwrite the file named `name` with `content`, returning the original `DirectoryEntry` for chaining.
$\hspace{5pt}$ 
$\hspace{5pt}$ ## Usage
```luau
local src = fs.dir.ensure("./src")
    :add_file("main.luau", 'print("hi")')
    :add_file("byte.luau", 'print("munch")')
$\hspace{5pt}$ ```
$\hspace{5pt}$ 
$\hspace{5pt}$ ## Errors
$\hspace{5pt}$ - if name is not a valid, utf-8 encoded filename
$\hspace{5pt}$ - user does not have permission to write files in the directory

</details>


DirectoryEntry.add_tree: `(self: DirectoryEntry, name: string, builder: TreeBuilder) -> DirectoryEntry`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ Add a new directory tree to the directory from a `TreeBuilder`, returning the ***original*** `DirectoryEntry` for chaining.
$\hspace{5pt}$ 
$\hspace{5pt}$ ## Usage
```luau
local src = fs.dir.ensure("./src")
    :add_tree("libraries", fs.tree()
        :with_file("Lists.luau", lists_src)
    )
$\hspace{5pt}$ ```
$\hspace{5pt}$ 
$\hspace{5pt}$ ## Errors
$\hspace{5pt}$ - if `name` is not a valid utf-8 encoded directory name
$\hspace{5pt}$ - the directory already contains a directory named `name` (or it's a file)

</details>


DirectoryEntry.metadata: `(self: DirectoryEntry) -> FsMetadata`

$\hspace{5pt}$ Returns a `FsMetadata` table containing timestamps for creation, modified, and access times, as well as permissions (depends on your operating system)

DirectoryEntry.copy_to: `(self: DirectoryEntry, to: string) -> ()`

DirectoryEntry.move_to: `(self: DirectoryEntry, to: string) -> ()`

DirectoryEntry.rename: `(self: DirectoryEntry, name: string) -> ()`

DirectoryEntry.remove: `(self: DirectoryEntry) -> ()`

$\hspace{5pt}$ --- Removes the directory at `DirectoryEntry.path`, alongside all its contents.

`export type` Entry

`export type` FsMetadata

FsMetadata.created_at: `DateTime?`

$\hspace{5pt}$ --- A UTC DateTime representing when the `Entry` was created.
$\hspace{5pt}$ --- This field is optional because it might not be available on all platforms.

FsMetadata.modified_at: `DateTime?`

$\hspace{5pt}$ --- A UTC DateTime representing when the `Entry` was last modified.
$\hspace{5pt}$ --- This field is optional because it might not be available on all platforms.

FsMetadata.accessed_at: `DateTime?`

$\hspace{5pt}$ --- A UTC DateTime representing when the `Entry` was last accessed.
$\hspace{5pt}$ --- This field is optional because it might not be available on all platforms.

FsMetadata.permissions.readonly: `boolean`

$\hspace{5pt}$ --- Whether the `Entry` is read-only or not. Should be accessible on both Windows and Unix-like operating systems.

FsMetadata.permissions.unix_mode: `number?`

$\hspace{5pt}$ --- Represents the numeric Unix permission bits for the `Entry`, combining read, write, and execute permissions
$\hspace{5pt}$ --- for owner, group, and others. This field is optional because it's not available on Windows.

`export type` FindResult

FindResult.ok: `boolean`

$\hspace{5pt}$ --- `true` if the find operation succeeded ("File" | "Directory" | "Symlink"), otherwise `false` ("NotFound" | "PermissionDenied")

FindResult.path: `string`

FindResult.type: `"File" | "Directory" | "Symlink" | "NotFound" | "PermissionDenied"`

FindResult.exists: `(self: FindResult) -> boolean`

$\hspace{5pt}$ Checks if `FindResult.path` exists on the filesystem.
$\hspace{5pt}$ 
$\hspace{5pt}$ ## Usage
```luau
if current_dir:find(".luaurc"):exists() then
    luaurc_path = path.join(current_dir.path, ".luaurc")
end
$\hspace{5pt}$ ```
$\hspace{5pt}$ 
$\hspace{5pt}$ ## Errors
$\hspace{5pt}$ - if permission denied

FindResult.try_file: `(self: FindResult) -> FileEntry?`

$\hspace{5pt}$ Attempt to create a `FileEntry` from the `FindResult`, returning it or `nil` if unsuccessful.
$\hspace{5pt}$ 
$\hspace{5pt}$ ## Usage
```luau
local main_luau = fs.find("./src/main.luau"):try_file()
if main_luau then
    process.shell(`seal {main_luau.path}`):unwrap()
end
$\hspace{5pt}$ ```

FindResult.try_dir: `(self: FindResult) -> DirectoryEntry?`

$\hspace{5pt}$ Attempt to create a `DirectoryEntry` from the `FindResult`, returning it or `nil` if unsuccessful.
$\hspace{5pt}$ 
$\hspace{5pt}$ ## Usage
```luau
local src = fs.find("./src"):try_dir()
if src then
    print(src:list(--[[recursive = ]] true))
end
$\hspace{5pt}$ ```

FindResult.unwrap_file: `(self: FindResult) -> FileEntry`

$\hspace{5pt}$ --- Create a `FileEntry` from the `FindResult`, erroring if the file doesn't exist.

FindResult.unwrap_dir: `(self: FindResult) -> DirectoryEntry`

$\hspace{5pt}$ --- Create a `DirectoryEntry` from the `FindResult`, erroring if the directory doesn't exist.

`export type` FileBuilder

FileBuilder.name: `string`

FileBuilder.type: `"File"`

FileBuilder.content: `string`

`export type` DirectoryBuilder

DirectoryBuilder.name: `string`

DirectoryBuilder.type: `"Directory"`

DirectoryBuilder.children: `DirectoryTree`

`export type` DirectoryTree
