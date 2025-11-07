<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# fs

`local fs = require("@std/fs")`

fs.readfile: `(path: string) -> string`

<details>

<summary> See the docs </summary

$hspace{5pt}$Reads the file at `path` to string, without performing utf-8 validation on the file's contents.
$hspace{5pt}$
$hspace{5pt}$## Errors
$hspace{5pt}$- if `path` is not valid utf-8
$hspace{5pt}$- the file doesn't exist or is actually a directory
$hspace{5pt}$- you don't have permission to access `path`
$hspace{5pt}$
$hspace{5pt}$Use `fs.file.try_read` instead if you want to handle`NotFound` and/or `PermissionDenied` explicitly without erroring.
$hspace{5pt}$
$hspace{5pt}$This function blocks the current Luau VM. To use it in parallel, call it within a child thread from `@std/thread`.

</details>

fs.readbytes: `(path: string, file_offset: number?, count: number?, target_buffer: buffer?, buffer_offset: number?) -> buffer`

<details>

<summary> See the docs </summary

$hspace{5pt}$Reads the file at `path` into a buffer.
$hspace{5pt}$
$hspace{5pt}$This function has 3 common variants:
$hspace{5pt}$- Read the whole file into a new buffer: `fs.readbytes(path: string)`
$hspace{5pt}$- Partially read a file into a new buffer: `fs.readbytes(path: string, file_offset: number, count: number)`
$hspace{5pt}$- Partially read a file into an existing buffer: `fs.readbytes(path: string, file_offset: number, count: number, target_buffer: buffer, buffer_offset: number?)
$hspace{5pt}$
$hspace{5pt}$## Parameters
$hspace{5pt}$-`path`: must be a valid, utf-8 encoded string representing an accessible file's path on your filesystem
$hspace{5pt}$-`file_offset`: the number of bytes from the start of the file to the start of the portion you want to read. Default is`0` (start of file)
$hspace{5pt}$- `count`: the number of bytes you want to read (starting from`file_offset`) and copy into the buffer
$hspace{5pt}$-`target_buffer`: an optional buffer you want to write into; if not specified, a new buffer will be created for you
$hspace{5pt}$-`buffer_offset`: an optional number of bytes from the start of the`target_buffer` provided; this is useful if you're filling the same buffer from multiple calls
$hspace{5pt}$
$hspace{5pt}$## Returns
$hspace{5pt}$- `target_buffer`: the exact same`target_buffer` provided, or a new buffer if not provided
$hspace{5pt}$
$hspace{5pt}$## Errors
$hspace{5pt}$- if `path` is not a file, not valid utf-8, is actually a directory, not found or permission denied, etc.
$hspace{5pt}$- `file_offset`,`count`, or`buffer_offset` cannot be converted into positive mlua Integers
$hspace{5pt}$- provided `target_buffer`is too small (`buffer_offset` + `count`> buffer size)
$hspace{5pt}$- attempt to read a nonexistent portion of file (`file_offset` + `count` > file size)
$hspace{5pt}$
$hspace{5pt}$This function blocks the current Luau VM. To use it in parallel, call it within a child thread from `@std/thread`.

</details>

fs.readlines: `(path: string) -> () -> (number, string)`

<details>

<summary> See the docs </summary

$hspace{5pt}$Iterate over the lines of a file without reading the whole file into memory.
$hspace{5pt}$
$hspace{5pt}$This function returns a normal iterator function, so if you save the return of `fs.readlines` to a variable, you can keep calling it for the next line!
$hspace{5pt}$
$hspace{5pt}$## Errors
$hspace{5pt}$- if `path` is not valid utf-8, doesn't point to a file, not found or permission denied, etc.
$hspace{5pt}$
$hspace{5pt}$## Usage
$hspace{5pt}$```luau
$hspace{5pt}$for line_number, line in fs.readlines("./myfile.txt") do
$hspace{5pt}$    print(`{line_number} says {line}`)
$hspace{5pt}$end
$hspace{5pt}$
$hspace{5pt}$local nextline = fs.readlines("./myfile.txt")
$hspace{5pt}$local _, line1 = nextline()
$hspace{5pt}$local _, line2 = nextline()
$hspace{5pt}$```

</details>

fs.writefile: `(path: string, content: string | buffer) -> ()`

<details>

<summary> See the docs </summary

$hspace{5pt}$Writes `content` to the file at `path`, overwriting any file that already exists there.
$hspace{5pt}$
$hspace{5pt}$Note that `content` may be either a string or a buffer; in either case, `content` does not need to be utf-8 encoded.
$hspace{5pt}$
$hspace{5pt}$## Usage
$hspace{5pt}$```luau
$hspace{5pt}$local content = getcontent()
$hspace{5pt}$fs.writefile("./myfile.txt",content)
$hspace{5pt}$```
$hspace{5pt}$## Errors
$hspace{5pt}$- if `path` is not a valid, utf-8-encoded path to a file or empty location on the filesystem
$hspace{5pt}$- path already exists on the filesystem and is a directory
$hspace{5pt}$- the user does not have permission to access `path`
$hspace{5pt}$
$hspace{5pt}$This function blocks the current Luau VM. To use it in parallel, call it within a child thread from `@std/thread`.

</details>

fs.removefile: `(path: string) -> ()`

$hspace{5pt}$Removes a regular file at `path` without following symlinks.
$hspace{5pt}$
$hspace{5pt}$## Usage
$hspace{5pt}$```luau
$hspace{5pt}$fs.removefile("./bad.exe")
$hspace{5pt}$```
$hspace{5pt}$
$hspace{5pt}$This function blocks the current Luau VM. To use it in parallel, call it within a child thread from `@std/thread`.

fs.is: `(path: string) -> PathIs`

<details>

<summary> See the docs </summary

$hspace{5pt}$Check what's at `path`.
$hspace{5pt}$
$hspace{5pt}$## Usage
$hspace{5pt}$
$hspace{5pt}$Check if something's a file:
$hspace{5pt}$```luau
$hspace{5pt}$if fs.is(path) == "File" then
$hspace{5pt}$    print(fs.readfile(path)) -- not TOCTOU safe
$hspace{5pt}$end
$hspace{5pt}$```
$hspace{5pt}$
$hspace{5pt}$A more exhaustive check:
$hspace{5pt}$```luau
$hspace{5pt}$for _, path in fs.listdir(directory) do
$hspace{5pt}$    local path_is = fs.is(path)
$hspace{5pt}$
$hspace{5pt}$    if path_is == "File" then
$hspace{5pt}$        print(`{path} is a file!`)
$hspace{5pt}$    elseif path_is == "Directory" then
$hspace{5pt}$        print(`{path} is a directory!`)
$hspace{5pt}$    elseif path_is == "Symlink" then
$hspace{5pt}$        print(`{path} is a symlink!`)
$hspace{5pt}$    elseif path_is == "NotFound" then
$hspace{5pt}$        print(`{path} not found!`)
$hspace{5pt}$    elseif path_is == "PermissionDenied" then
$hspace{5pt}$        print(`We don't have permission to access {path} (try sudo)`)
$hspace{5pt}$    else
$hspace{5pt}$        print(`{path} is weird one! got {path_is}`)
$hspace{5pt}$    end
$hspace{5pt}$end
$hspace{5pt}$```

</details>

fs.symlink: `(target: string, link: string) -> boolean`

<details>

<summary> See the docs </summary

$hspace{5pt}$Creates a symlink from `link` to `target`, possibly overwriting any already-existing symlink at `link`.
$hspace{5pt}$
$hspace{5pt}$If you're on Windows, you need to run this program with Administrator permissions to create a symlink.
$hspace{5pt}$
$hspace{5pt}$## Errors
$hspace{5pt}$- if `link` already exists on the filesystem and isn't a symlink,
$hspace{5pt}$- there's nothing to link to at `target`
$hspace{5pt}$- you don't have access to `link`

</details>

fs.unsymlink: `(link: string) -> boolean`

$hspace{5pt}$Removes the symlink at `link`.
$hspace{5pt}$
$hspace{5pt}$## Returns
$hspace{5pt}$- `true` if the symlink was successfully removed
$hspace{5pt}$
$hspace{5pt}$## Errors
$hspace{5pt}$- If `link` points to something that isn't a symlink.
$hspace{5pt}$- If the symlink at `link` is not found or permission denied.

fs.readlink: `(symlink: string) -> string`

$hspace{5pt}$Follows `symlink` and returns the *path* targeted by the symlink.
$hspace{5pt}$
$hspace{5pt}$## Errors
$hspace{5pt}$
$hspace{5pt}$- if `symlink` is not a symlink, does not exist on the filesystem, or is permission denied

fs.watch: `(paths: string | { string }, options: WatchOptions?) -> () -> (WatchEventCategory, WatchEventInfo)`

<details>

<summary> See the docs </summary

$hspace{5pt}$Watch for filesystem changes on one or more `paths`.
$hspace{5pt}$
$hspace{5pt}$- `WatchOptions.recursive`: defaults `true`; may produce duplicate events if any `paths` overlap,
$hspace{5pt}$- `WatchOptions.timeout_ms`: in milliseconds, after which the `fs.watch` iterator will return a `"None"` event instead of blocking the VM.
$hspace{5pt}$Defaults to 10 milliseconds.
$hspace{5pt}$
$hspace{5pt}$Note that filesystem watching is inherently *messy* and *platform specific*!
$hspace{5pt}$
$hspace{5pt}$In many cases, instead of just relying on the exact events this api provides to know which
$hspace{5pt}$files were created or removed, it may be better just to check the filesystem yourself
$hspace{5pt}$with `fs.listdir/entries`, and use `fs.watch` as a trigger for knowing when to check.
$hspace{5pt}$
$hspace{5pt}$Because you can get duplicated events, it's better to add your own debounces and if you're relying on counts,
$hspace{5pt}$
$hspace{5pt}$Don't expect that you'll get the same events every time, or that the same operation will map to the same specific `WatchKind`
$hspace{5pt}$on all platforms (though they *should* be similar.)
$hspace{5pt}$
$hspace{5pt}$## Usage
$hspace{5pt}$
$hspace{5pt}$- You probably want to ignore `"Access"` since it's noisy (unless you need to know which files are currently open)
$hspace{5pt}$- If you want to listen for files getting added/modified, don't rely on certain kinds/categories.
$hspace{5pt}$- Check for `WatchEventInfo.is_write` instead of relying on `Modify::Data` or `Create` or `Close::Write`.
$hspace{5pt}$- If you don't want to loop forever, `break` if when you keep encountering `category == "None"` for a while.
$hspace{5pt}$- To make this function nonblocking (as best as possible), pass 0 milliseconds to `timeout_ms`.
$hspace{5pt}$- You'll probably still want to loop it/put it in a function and watch out for `"None"` events.
$hspace{5pt}$
$hspace{5pt}$## Examples
$hspace{5pt}$
$hspace{5pt}$Run a Lune script whenever a file in ./src/** changes:
$hspace{5pt}$```luau
$hspace{5pt}$local fs = require("@std/fs")
$hspace{5pt}$local str = require("@std/str")
$hspace{5pt}$local process = require("@std/process")
$hspace{5pt}$
$hspace{5pt}$local serializer_script = fs.path.join(".", ".lune", "instance_serializer.luau")
$hspace{5pt}$
$hspace{5pt}$for category, event in fs.watch("./src") do
$hspace{5pt}$    if category == "Access" or category == "None" then
$hspace{5pt}$        continue -- ignore these, we only need "None" if we want to break loop
$hspace{5pt}$    elseif event.is_write then
$hspace{5pt}$        local modified_path = event.paths[1]
$hspace{5pt}$        local result = process.run {
$hspace{5pt}$            program = "lune",
$hspace{5pt}$            args = {"run", serializer_script, modified_path}
$hspace{5pt}$        }
$hspace{5pt}$    end
$hspace{5pt}$end
$hspace{5pt}$```
$hspace{5pt}$
$hspace{5pt}$Watch only .json config files for changes:
$hspace{5pt}$
$hspace{5pt}$```luau
$hspace{5pt}$local options: fs.WatchOptions = {
$hspace{5pt}$    recursive = false,
$hspace{5pt}$    timeout_ms = 2, -- blocks vm for max of 2 milliseconds
$hspace{5pt}$}
$hspace{5pt}$
$hspace{5pt}$local files = fs.listdir(
$hspace{5pt}$    "./src",
$hspace{5pt}$    true, -- recursive
$hspace{5pt}$    function(path: string) -- filter
$hspace{5pt}$        return if string.match(path, "%.json$") or string.match(path, "%.luaurc")
$hspace{5pt}$            then true
$hspace{5pt}$            else false
$hspace{5pt}$    end
$hspace{5pt}$)
$hspace{5pt}$
$hspace{5pt}$for _, event in fs.watch(files, options) do
$hspace{5pt}$    if event.is_write then
$hspace{5pt}$        local modified = fs.path.child(event.paths[1])
$hspace{5pt}$        if modified then
$hspace{5pt}$            print(`Config file modified: {modified}!`)
$hspace{5pt}$        end
$hspace{5pt}$    end
$hspace{5pt}$end
$hspace{5pt}$```
$hspace{5pt}$
$hspace{5pt}$Manually poll a few times:
$hspace{5pt}$
$hspace{5pt}$```luau
$hspace{5pt}$local options = {
$hspace{5pt}$    recursive = true,
$hspace{5pt}$    timeout_ms = 2, -- check for 2 seconds
$hspace{5pt}$}
$hspace{5pt}$time.wait(1)
$hspace{5pt}$local poll = fs.watch("./src", options)
$hspace{5pt}$local cat, event = poll()
$hspace{5pt}$if cat == "None" then
$hspace{5pt}$    time.wait(1) -- retry
$hspace{5pt}$    cat, event = poll()
$hspace{5pt}$end
$hspace{5pt}$if cat == "None" then
$hspace{5pt}$    print("not found")
$hspace{5pt}$end
$hspace{5pt}$```
$hspace{5pt}$
$hspace{5pt}$With a custom timeout:
$hspace{5pt}$
$hspace{5pt}$```luau
$hspace{5pt}$local start_time = os.clock()
$hspace{5pt}$for category, event in fs.watch(script:parent()) do
$hspace{5pt}$    if category == "None" and os.clock() - start_time > 5 then
$hspace{5pt}$        break
$hspace{5pt}$    end
$hspace{5pt}$    if event.is_write then
$hspace{5pt}$        print(event)
$hspace{5pt}$    end
$hspace{5pt}$end
$hspace{5pt}$print("hi after 5 seconds")
$hspace{5pt}$```
$hspace{5pt}$
$hspace{5pt}$This function uses the Rust `notify` crate as its backend; please refer to its documentation for more specifics.

</details>

fs.readtree: `(path: string) -> DirectoryTree`

$hspace{5pt}$Recursively read contents of directory at `path` into a `fs.DirectoryTree` that can be passed into `fs.writetree` and `DirectoryEntry:add_tree` apis.

fs.writetree: `(path: string, tree: TreeBuilder | DirectoryTree) -> ()`

<details>

<summary> See the docs </summary

$hspace{5pt}$Writes a new directory tree at `path` (which includes the directory's name) from `tree`:
$hspace{5pt}$
$hspace{5pt}$## Usage
$hspace{5pt}$```luau
$hspace{5pt}$        -- using TreeBuilders from fs.tree()
$hspace{5pt}$        fs.writetree("./tests", fs.tree()
$hspace{5pt}$            :with_file("run.luau", test_runner_src)
$hspace{5pt}$            :with_tree("simple-tests", fs.tree()
$hspace{5pt}$                :with_file("cats.spec.luau", cats_src)
$hspace{5pt}$                :with_file("seals.spec.luau", seals_src)
$hspace{5pt}$            )
$hspace{5pt}$        )
$hspace{5pt}$        -- or using a return from fs.readtree:
$hspace{5pt}$        local all_tests = fs.readtree("./all_tests")
$hspace{5pt}$        local applicable_tests: fs.DirectoryTree = {} do
$hspace{5pt}$            for _, entry in all_tests do
$hspace{5pt}$                if entry.type == "File" and string.find(entry.name, "spec%.luau$") then
$hspace{5pt}$                    table.insert(applicable_tests, entry)
$hspace{5pt}$                end
$hspace{5pt}$            end
$hspace{5pt}$        end
$hspace{5pt}$        fs.writetree("./applicable_tests", applicable_tests)
$hspace{5pt}$```
$hspace{5pt}$
$hspace{5pt}$## Errors
$hspace{5pt}$- if `path` not a valid utf-8 string representing a path on the filesystem
$hspace{5pt}$- an entry already exists at `path` or user does not have permission to access `path`
$hspace{5pt}$- `tree` is not a valid `fs.TreeBuilder` or `fs.DirectoryTree` (`{ fs.FileBuilder | fs.DirectoryBuilder }`)
$hspace{5pt}$
$hspace{5pt}$Use fs.makedir instead if you  just want to make an empty directory.
$hspace{5pt}$
$hspace{5pt}$This function blocks the current Luau VM. To use it in parallel, call it within a child thread from `@std/thread`.

</details>

fs.removetree: `(path: string) -> ()`

<details>

<summary> See the docs </summary

$hspace{5pt}$Removes a directory tree or an empty directory at `path` by calling Rust's `fs::remove_dir_all`, without following symlinks.
$hspace{5pt}$
$hspace{5pt}$```luau
$hspace{5pt}$local victim_folder = fs.path.join(fs.path.cwd(), "badfolder")
$hspace{5pt}$fs.makedir(victim_folder, { error_if_exists = false })
$hspace{5pt}$fs.removetree(victim_folder)
$hspace{5pt}$```
$hspace{5pt}$
$hspace{5pt}$Please use this function carefully.
$hspace{5pt}$
$hspace{5pt}$## Errors
$hspace{5pt}$- if `path` is not a valid utf-8 encoded path to a directory on the filesystem
$hspace{5pt}$- user does not have permission to access `path`
$hspace{5pt}$- `fs.removetree` fails to remove some (or all) files and directories within `path`

</details>

fs.makedir: `(path: string, options: { create_missing: boolean?, error_if_exists: boolean? }?) -> boolean`

<details>

<summary> See the docs </summary

$hspace{5pt}$Create an empty directory at `path` according to (an optional) `options` table.
$hspace{5pt}$
$hspace{5pt}$By default, `create_missing` is set to `false` and `error_if_exists` is set to `true`.
$hspace{5pt}$
$hspace{5pt}$- Enable `create_missing` to create any missing intermediate directories (such as `"recipes"` in `"./food/recipes/pumpkin_pie.md"`) using Rust's `fs::create_dir_all`.
$hspace{5pt}$- Disable `error_if_exists` if you expect the directory to already exist in normal use and only want to make the directory if it doesn't.
$hspace{5pt}$
$hspace{5pt}$If you want to ensure that a directory exists (like `fs.makedir(d, { error_if_exists = false })`), and get a `DirectoryEntry`, use `fs.dir.ensure` instead.
$hspace{5pt}$
$hspace{5pt}$## Usage
$hspace{5pt}$```luau
$hspace{5pt}$fs.makedir(fs.path.join(fs.path.cwd(), "Config", "Editor", "Formatting"), {
$hspace{5pt}$    create_missing = true,
$hspace{5pt}$    error_if_exists = false,
$hspace{5pt}$})
$hspace{5pt}$```
$hspace{5pt}$
$hspace{5pt}$# Errors
$hspace{5pt}$- if `path` is not a valid utf-8 encoded path
$hspace{5pt}$- a directory already exists at `path` and `options.error_if_exists` is omitted or set to `true`
$hspace{5pt}$- user does not have permission to access or to create a directory at `path`
$hspace{5pt}$- a file unexpectedly exists at `path`
$hspace{5pt}$- an intermediate component directory of `path` is missing and `create_missing` is omitted or set to `false`

</details>

fs.listdir: `(path: string, recursive: boolean?, filter: ((path: string) -> boolean)?) -> { string }`

<details>

<summary> See the docs </summary

$hspace{5pt}$Returns an array of all child paths of directory `path`, relative to the passed path.
$hspace{5pt}$
$hspace{5pt}$This means paths from `fs.listdir` can be directly passed into other `fs` library functions.
$hspace{5pt}$
$hspace{5pt}$Pass `true` as the second parameter to recursively enumerate all files in the directory tree.
$hspace{5pt}$
$hspace{5pt}$If a filter function is passed, only paths that pass the filter are included.
$hspace{5pt}$
$hspace{5pt}$## Usage
$hspace{5pt}$```luau
$hspace{5pt}$local test_files: { string } = fs.listdir("./tests", --[[recursive =]] true)
$hspace{5pt}$
$hspace{5pt}$-- all .luau files
$hspace{5pt}$local luau_files = fs.listdir("./tests", true, function(path: string)
$hspace{5pt}$    return if string.match(path, "%.luau$") then true else false
$hspace{5pt}$end)
$hspace{5pt}$```
$hspace{5pt}$
$hspace{5pt}$## Errors
$hspace{5pt}$- if `path` is not a valid, utf-8 encoded string
$hspace{5pt}$- `path` does not exist in the filesystem or is not a directory
$hspace{5pt}$- user does not have permission to access `path`

</details>

fs.move: `(from: string, to: string) -> ()`

$hspace{5pt}$Move a regular file or directory `from` a path `to` a new path.
$hspace{5pt}$
$hspace{5pt}$TODO: streamline fs.move and fs.copy with Entry:move_to and Entry:copy_to.
$hspace{5pt}$
$hspace{5pt}$## Errors
$hspace{5pt}$- if `from` or `to` are not valid utf-8 encoded paths
$hspace{5pt}$- `from` does not exist on the filesystem

fs.copy: `(source: string, destination: string) -> ()`

$hspace{5pt}$Copy a regular file or directory from `source` to `destination`.
$hspace{5pt}$
$hspace{5pt}$TODO: streamline fs.move and fs.copy with Entry:move_to and Entry:copy_to.

fs.find: `(path: string, options: { follow_symlinks: boolean?, error_if_permission_denied: boolean? }?) -> FindResult`

<details>

<summary> See the docs </summary

$hspace{5pt}$Check the filesystem for `path`, returning a `fs.FindResult` that's useful for finding `fs.FileEntry` or `fs.DirectoryEntry` to work with.
$hspace{5pt}$
$hspace{5pt}$This is a multifunctional api, which is usually used to find and unwrap `fs.Entry`-like tables, but is also used for general "finding stuff on the filesystem" usecases.
$hspace{5pt}$
$hspace{5pt}$Note that `fs.find` and `fs.Entry`-related apis are **not TOCTOU (Time Of Check To Time of Use) safe**; use the `try` apis (`fs.file.try_*` and `fs.dir.try_*`) instead for security or time critical applications.
$hspace{5pt}$
$hspace{5pt}$## Usage
$hspace{5pt}$Look for a `FileEntry` at `path`:
$hspace{5pt}$```luau
$hspace{5pt}$local file_content: string? = nil
$hspace{5pt}$local file = fs.find("./myfile.txt"):try_file()
$hspace{5pt}$if file then
$hspace{5pt}$    file_content = file:read()
$hspace{5pt}$end
$hspace{5pt}$```
$hspace{5pt}$Check if `path` is a file:
$hspace{5pt}$```luau
$hspace{5pt}$if fs.find("./mypath").type == "File" then
$hspace{5pt}$    -- code
$hspace{5pt}$end
$hspace{5pt}$```
$hspace{5pt}$Check if we have access to `path`
$hspace{5pt}$```luau
$hspace{5pt}$local result = fs.find(maybeaccesspath, { error_if_permission_denied = false })
$hspace{5pt}$if result.type ~= "PermissionDenied" then
$hspace{5pt}$    -- code
$hspace{5pt}$end
$hspace{5pt}$```
$hspace{5pt}$
$hspace{5pt}$## Errors
$hspace{5pt}$- if `path` is not a valid utf-8 encoded path
$hspace{5pt}$- if user does not have permission to access `path` and options.error_if_permission_denied is unspecified or set `true`

</details>

fs.entries: `(path: string) -> { [string]: Entry }`

<details>

<summary> See the docs </summary

$hspace{5pt}$Returns a table mapping the paths of the directory at `path` with their `fs.Entry`s.
$hspace{5pt}$
$hspace{5pt}$## Usage
$hspace{5pt}$```luau
$hspace{5pt}$for path, entry in fs.entries("./src") do
$hspace{5pt}$    if entry.type == "File" then
$hspace{5pt}$        print(`{entry.name} is a file`)
$hspace{5pt}$    elseif entry.type == "Directory" then
$hspace{5pt}$        print(`{entry.name} is a directory`)
$hspace{5pt}$    end
$hspace{5pt}$end
$hspace{5pt}$```

</details>

fs.file: `filelib.FileLib`

$hspace{5pt}$A sublib for handling operations with files and `fs.FileEntry`s.
$hspace{5pt}$
$hspace{5pt}$Contains (relatively) TOCTOU-safe apis such as `fs.file.try_read`, etc.
$hspace{5pt}$
$hspace{5pt}$This library can be called as a function as a convenience alternative for `fs.find(f):try_file()`.

fs.dir: `dirlib.DirLib`

$hspace{5pt}$A sublib for handling operations with directories and `fs.DirectoryEntry`s.
$hspace{5pt}$
$hspace{5pt}$This library can be called as a function as a convenience alternative to `fs.find(d):try_dir()`

fs.path: `pathlib.PathLib`

$hspace{5pt}$A sublib for handling file path operations with strings in an ergonomic and cross-platform-compliant manner.
$hspace{5pt}$
$hspace{5pt}$Commonly used `fs.path` functions include: `fs.path.join` for combining paths and `fs.path.cwd` and `fs.path.home`.

fs.tree: `() -> TreeBuilder`

$hspace{5pt}$Returns a `TreeBuilder` for use with `fs.writetree`, `DirectoryEntry:add_tree`, and `TreeBuilder:with_tree` apis.

`export type` PathIs

`export type` DirectoryTree

`export type` DirectoryBuilder

`export type` TreeBuilder

`export type` FindResult

`export type` Entry

`export type` FileEntry

`export type` DirectoryEntry

`export type` WatchOptions

WatchOptions.recursive: `boolean?`

WatchOptions.timeout_ms: `number?`

`export type` WatchEventCategory

<details>

<summary> See the docs </summary

$hspace{5pt}$Top level categories to filter events by.
$hspace{5pt}$
$hspace{5pt}$Some usage notes:
$hspace{5pt}$- You should probably ignore `"Access"` unless you need to know which files are currently open.
$hspace{5pt}$- Don't rely on solely `"Create"` or `"Modify::Data"` to check if a file was created/modified, use `WatchEventInfo.is_write` instead.
$hspace{5pt}$- `"None"` indicates a timeout was reached; use it to early exit or `break` without blocking the VM.

</details>

WatchEventCategory.| "Modify: `:Data"`

WatchEventCategory.| "Modify: `:Metadata"`

WatchEventCategory.| "Modify: `:Other"`

`export type` WatchEventInfo

WatchEventInfo.paths: `{ string }`

WatchEventInfo.kind: `WatchKind`

WatchEventInfo.is_write: `boolean`

$hspace{5pt}$ if the event is *most likely* a write event (`Create::File` or `Modify::Data` or `Close::Write`)

`export type` WatchKind

<details>

<summary> See the docs </summary

$hspace{5pt}$ Represents the specific Event.WatchKind from notify.
$hspace{5pt}$
$hspace{5pt}$ Note that relying on these is inherently unreliable as notify tends to combine related events.
$hspace{5pt}$ Especially if they're received within a short interval of each other.
$hspace{5pt}$
$hspace{5pt}$ Note that the `Kind::Any` options tend to be generated in place of `Kind::File` or
$hspace{5pt}$ `Kind::Directory` on Windows!
$hspace{5pt}$
$hspace{5pt}$ `None::Timeout` is fired if no events have been seen when `WatchOptions.timeout_ms` elapses
$hspace{5pt}$ for an iteration of `fs.watch`. This allows you to break early without indefinitely blocking the Luau VM.

</details>

WatchKind.| "Open: `:Execute"`

WatchKind.| "Open: `:Read"`

WatchKind.| "Open: `:Write"`

WatchKind.| "Open: `:Other"`

WatchKind.| "Close: `:Execute"`

WatchKind.| "Close: `:Read"`

WatchKind.| "Close: `:Write"`

WatchKind.| "Close: `:Other"`

WatchKind.| "Close: `:Any"`

WatchKind.| "Open: `:Any"`

WatchKind.| "Access: `:Any"`

WatchKind.| "Access: `:Other"`

WatchKind.| "Create: `:File"`

WatchKind.| "Create: `:Directory" -- sent on macos and unixlike`

WatchKind.| "Create: `:Other"`

WatchKind.| "Create: `:Any"`

WatchKind.| "Rename: `:Any"`

WatchKind.| "Rename: `:From"`

WatchKind.| "Rename: `:To"`

WatchKind.| "Rename: `:Both"`

WatchKind.| "Rename: `:Other"`

WatchKind.| "Modify: `:Data" -- sent on unixlike`

WatchKind.| "Modify: `:Data::Content"`

WatchKind.| "Modify: `:Data::Size"`

WatchKind.| "Modify: `:Data::Other" -- sent on windows`

WatchKind.| "Modify: `:Metadata::AccessTime"`

WatchKind.| "Modify: `:Metadata::WriteTime"`

WatchKind.| "Modify: `:Metadata::Ownership"`

WatchKind.| "Modify: `:Metadata::Permissions"`

WatchKind.| "Modify: `:Metadata::Extended"`

WatchKind.| "Modify: `:Metadata::Other"`

WatchKind.| "Modify: `:Metadata::Any"`

WatchKind.| "Modify: `:Any"`

WatchKind.| "Modify: `:Other"`

WatchKind.| "Remove: `:File" -- sent on unixlike`

WatchKind.| "Remove: `:Directory" -- sent on unixlike`

WatchKind.| "Remove: `:Other"`

WatchKind.| "Remove: `:Any" -- sent on Windows`

WatchKind.| "None: `:Timeout"`
