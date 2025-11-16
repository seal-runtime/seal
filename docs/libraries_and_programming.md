<!-- markdownlint-disable MD033 -->

# Libraries and Programming

If you're new to Luau, check out seal's [Luau Book](/docs/luau-book/index.md) and Luau's [official documentation](https://luau.org).

Although seal provides some builtin globals (such as `p`, `pp`, `channel` (in a child thread), and `script`), most features are in the standard library. You can import standard libraries like so:

```luau
local fs = require("@std/fs")
local colors = require("@std/colors") -- (the most important one tbh)

-- some libs are nested:
local http = require("@std/net/http")
local prompt = require("@std/io/prompt")
```

With Luau Language Server, you should be able to see documentation, usage examples, and type definitions for each library when you hover over them in your editor.

For convenience, all of *seal*'s type definitions and documentation are located in your project's `./.seal/typedefs` directory, and for script codebases, in your home directory's `~/.seal/typedefs` folder.

If you want to see documentation on the web, check out the [references](/docs/reference/) folder in this repository.

## Programming

You can use *seal* to write programs that interact with your filesystem, get and send things on the internet, connect other programs together (glue scripts), do multiple things at the same time, and more. In the future, you'll be able to write cross-platform GUI apps directly in *seal*!

## Read and write files/directories

[@std/fs](/docs/reference/std/fs/init.md)

```luau
local fs = require("@std/fs")
```

<details>
<summary>Examples</summary>

### Read files

```luau
local content = fs.readfile("./myfile.txt")
print(content)
```

Without erroring if the file doesn't exist or isn't found:

```luau
local content, result = fs.file.try_read("./myfile.txt")
if content then
    print(content)
elseif result == "NotFound" then
    print("file not found")
elseif result == "PermissionDenied" then
    print("permission denied")
end
```

### Create files and directories

Write files to a path:

```luau
local seally_path = fs.path.join(script:parent(), "seally.txt")
fs.writefile(seally_path, "did you know seals can bark?")
```

Write a new directory, creating any directories needed, without erroring if one doesn't exist:

```luau
fs.makedir("./src/elements", {
    create_missing = true,
    error_if_exists = false,
})
```

Write a new directory tree, with content:

```luau
fs.writetree("./tests", fs.tree()
    :with_file("run_tests.luau", run_tests_src)
    :with_tree("cases", fs.tree()
        :with_file("case1", cases[1])
    )
)
```

### List all files and iterate through a directory

List all files in a directory, recursively:

```luau
local files = fs.listdir("./src", true)
```

Loop over paths in a directory (not recursively):

```luau
for _, path in fs.listdir("./src") do
    if fs.is(path) == "File" then
        print(fs.readfile(path))
    end
end
```

### Remove files and directories

Files, erroring if the file doesn't exist:

```luau
fs.removefile(fs.path.join(script:parent(), "seally.txt"))
```

Without erroring if the file doesn't exist/permission denied:

```luau
local removed = fs.file.try_remove("./idk/some_file.txt")
```

Directory trees (empty or not):

```luau
fs.removetree("./src/old")
```

Without erroring:

```luau
local removed, result, partial = fs.dir.try_remove("./src/old")
if result == "NotFound" or result == "PermissionDenied" then
    error(print(`can't remove dir cause {result}`))
end
if partial then
    error(`uh oh, directory partially removed? {partial}`)
end
```

### Path library

[@std/fs/path](/docs/reference/std/fs/path.md)

Contains useful functions for getting the user's home directory, cwd, project directories, normalizing and canonicalizing paths (handling Windows edge cases for you!), and of course, joining paths together in a cross-platform compatible way.

### Full examples using `fs` with other libraries

Removing files older than *n* weeks (fs and datetime):

[Example](/examples/older_than_a_week.luau)

Uploading a file whenever it gets added to a folder (file watching):

[Example](/examples/upload_files_in_folder.luau)

</details>

## Getting and sending stuff on the internet

[@std/net/http](/docs/reference/std/net/http/init.md)

```luau
local http = require("@std/net/http")
```

<details>
<summary>Examples</summary>

### Get stuff off the internet (HTTP GET)

```luau
local response = http.get {
    url = "https://jsonplaceholder.typicode.com/posts",
    params = {
        userId = "1",
    }
}:unwrap_json()
```

### Sending stuff to the internet (HTTP POST)

Tables passed to `http.post` automatically get treated as JSON!

```luau
local post_response = http.post {
    url = "https://mycatlist.me/api/add_cat/post",
    headers = {
        Authorization = `Bearer {TOKEN}`,
    },
    body = {
        name = "Taz",
        age = 12,
    }, -- pass a table? seal serializes it for you (and sets Content-Type: application/json)!
}
if not post_response.ok then
    print(`uh oh, got: {post_response.status_code}`)
end
```

</details>

## Running other programs and shell commands ~~(ffi at home)~~

[@std/process](/docs/reference/std/process.md)

Library for running other programs as child processes and exiting the current program (`process.exit`).

<details>
<summary>Examples</summary>

Run a quick shell command with your default shell:

```luau
local output = process.shell("seal ./cats.luau"):unwrap()
local files = process.shell("ls -l"):unwrap()
```

Run a program directly as a child process (waits 'til it completes)

```luau
local result = process.run {
    program = "seal",
    args = { "./cats.luau" },
}:unwrap()
```

Spawn a long-running child process and see what it's outputting in realtime:

```luau
-- listen to all user input on linux
local child = process.spawn {
    program = "libinput debug-events --show-keycodes",
    shell = "sh",
}

-- ensure seal's running in sudo (administrator mode) so we can see keycodes
local err = child.stderr:read(128, 0.25)
if err and err:match("Permission denied") then
    error("must run as sudo")
end

for line in child.stdout:lines() do
    print(line)
end
```

Spawn a child process in parallel, allowing you to run multiple programs at the same time:

```luau
for _, path in fs.listdir("./input") do
    process.spawn {
        program = "fixer",
        args = { path },
    }
end
```

</details>

## Prompt user input

[@std/io/prompt](/docs/reference/std/io/prompt.md)

Higher-level terminal prompt and validation functions.

[@std/io/input](/docs/reference/std/io/input.md)

Lower-level access to the terminal, including setting rawmode and listening for terminal events.

<details>
<summary>Examples</summary>

### Confirm an action

```luau
local prompt = require("@std/io/prompt")

if prompt.confirm("are roses red") then
    print("violets are blue")
end
```

### Ask a question

```luau
local prompt = require("@std/io/prompt")

local response = prompt.text("What's your name?")
print(`Hello {response}!`)
```

</details>

## Concurrency and Parallelism

*seal* is not an async runtime, and doesn't bind to `tokio` or similar for performance and simplicity, but it still supports concurrency and parallelism!

### Concurrency

Although *seal* doesn't have a `task` library to make coroutine scheduling more ergonomic, you can absolutely use Luau's `coroutine` library for concurrency. Just keep in mind that standard library functions, including `fs.readfile`, `time.wait`, and `http.get` can completely block the VM. This means you can use coroutines to interleave operations, but you can't use them as an alternative to `thread.spawn` in terms of making an inherently blocking operation non-blocking.

### Parallelism

[@std/thread](/docs/reference/std/thread.md)

*seal* provides access to Real Rust Threads with a relatively simple API. Each Rust thread spawns its own Luau VM, which allows you to execute Luau code in parallel.

To send messages between threads, use the `:send` and `:read` methods located on `ThreadHandle` (for parent threads) and `channel` (child threads) respectively. The regular `:send`/`:read` methods seamlessly serialize and transmit data tables for you between VMs!

For better performance, use the `bytes` APIs to exchange `buffer`s without serialization overhead.

<details>
<summary>Example</summary>

```luau
-- parent.luau
local thread = require("@std/thread")

local handle = thread.spawn {
    path = "./child.luau",
    data = {
        url = "https://example.net",
    }
}
 -- do something else
local res = handle:read_await()

handle:join() -- don't forget to join your handles!
```

Child threads have a global `channel` exposed, which you can use to send data to the main thread:

```luau
-- child.luau
local http = require("@std/net/http")
if channel then
    local data = channel.data :: { url: string }
    local response = http.get(data.url):unwrap_json()
    channel:send(response)
end
```

</details>

## Cookbook

Some fun things you could write with *seal*

- Automatically launch a program when you modify anything in a specific folder.
- Automatically back up everything in a folder every few hours to your server.
- GUI automation with programs like `kdotool` or AutoHotKey. Why write unreadable AHK when you can write Luau to write and run the AHK for you?
- Custom touchpad gestures on Linux (.. I've done this before; need to port it to *seal*)
- A background script that reminds you to drink water every hour.
