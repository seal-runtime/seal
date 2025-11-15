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

### Read and write files/directories

[@std/fs](/docs/reference/std/fs/init.md)

```luau
local fs = require("@std/fs")
```

<details>
<summary>Examples</summary>

#### Read files

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

#### Create files and directories

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

#### List all files and iterate through a directory

List all files in a directory, recursively:

```luau
local files = fs.listdir("./src", true)
```

#### Remove files and directories

```luau
fs.removefile(fs.path.join(script:parent(), "seally.txt"))
fs.removetree("./src/old")
```

</details>

## Cookbook

Some fun things you could write with *seal*

- Automatically launch a program when you modify anything in a specific folder.
- Automatically back up everything in a folder every few hours to your server.
- GUI automation with programs like `kdotool` or AutoHotKey. Why write unreadable AHK when you can write Luau to write and run the AHK for you?
- Custom touchpad gestures on Linux (.. I've done this before; need to port it to *seal*)
- A background script that reminds you to drink water every hour.

