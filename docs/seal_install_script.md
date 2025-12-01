# Installing *seal* with *seal*

This is a basic tutorial on how to use *seal* by writing a *seal* script to install *seal* for you.

If you're on Windows, use `seal.exe`, otherwise use `seal`.

1. Open your Downloads folder in your File Explorer or equivalent file browser.
2. Find `seal(.exe)` or the folder you just downloaded that contains `seal(.exe)`.
3. Right click on it and select `Open in Terminal`.
4. Your terminal application should open in the folder containing `seal(.exe)`.
5. In a terminal, the terminal's current location is called the current working directory (cwd), and can be referenced by `.`.
6. To see all files/folders in your current location, run `ls`. You should see `seal(.exe)` listed.
7. To check if *seal* works, run `./seal(.exe) --help` in your terminal.
8. The `$PATH` variable lists all folders you can place programs to run them without having to type out an explicit filesystem path to reach them. We need to move `seal(.exe)` somewhere in your `$PATH`.
9. Let's write a *seal* script to do that for us.
10. To create a new *seal* script codebase and set up type definitions, in your terminal, run `./seal setup script` (or `.\seal.exe setup script` on Windows).
11. Run `code ./.seal/script.luau` to open the newly created script file in VS Code.
12. The `fs` (filesystem) library should already be imported in `script.luau`:

```luau
local fs = require("@std/fs")
-- ..snip
```

The `fs` library allows you to create and modify files, directories (folders), and watch your folders for changes (amongst other functions).

The `fs.path` table contains most filesystem path operations, including an easy way to find your home directory.

13. We want to copy `seal(.exe)` somewhere in your `$PATH`. A typical place to put `seal` is `~/.local/bin/seal(.exe)`. In a terminal, `~` means your home directory, typically `C:\Users\<USERNAME>\` on Windows or `/home/<USERNAME>` on Linux.
14. If `~/.local/bin` doesn't exist, we want to make it. To do this, add these lines:

```luau
-- ..snip

local home_path = fs.path.home()
local folder_path = fs.path.join(home_path, ".local", "bin")
if fs.is(folder_path) == "NotFound" then
    fs.makedir(folder_path)
    print("We made a new .local/bin")
end
```

15. To copy `seal(.exe)`, we use the `fs.copy` function:

```luau
-- ..snip

local env = require("@std/env") -- this library can access info about our script's running environment, 
-- including what operating system we're on
local seal_exe = if env.os == "Windows" then "seal.exe" else "seal"

local current_seal_path = fs.dir.cwd():join(seal_exe)
local new_seal_path = fs.path.join(folder_path, seal_exe)

fs.copy(current_seal_path, new_seal_path)
```

16. Now because we ran `seal setup script` earlier, we've made a few files/folders we might want to clean up. To remove them after we've copied the file, use `fs.removefile` and `fs.removetree`.

```luau
-- ..snip
fs.removetree("./.seal")
fs.removetree("./.vscode")
fs.file.try_remove("./.luaurc")
fs.file.try_remove("./.config.luau")

print("Done!")
```

17. Now save the file and run it with *seal* by executing the following command in your terminal: `./seal(.exe) ./.seal/script.luau`
18. If running the script created a new `~/.local/bin` folder (or that folder isn't added to your `$PATH` yet), you need to modify your shell's path to include `~/.local/bin`.
19. Adding `~/.local/bin` to your `$PATH`:
    1. On Windows, you need to make a new PowerShell configuration file.
       1. To do this, run `code $PROFILE` in your terminal to open a new one in VS Code.
       2. Add `$env:Path += ";C:\Users\<USERNAME>\.local\bin"` to this file, replacing `<USERNAME>` with your actual Windows user path.
    2. On Linux/macOS/Android, you need to locate your `$SHELL`'s configuration file (usually `~/.bashrc` or `~/.zshrc`) and ensure `~/.local/bin` (or wherever you placed *seal*) is in your `$PATH`.
       1. For example, the following line adds `~/.local/bin` and `/usr/local/bin` to your `$PATH`:
       2. `export PATH=$HOME/.local/bin:/usr/local/bin:$PATH`
    3. Save the file.
20. Reopen your terminal anywhere and run `seal --help` to ensure you can access *seal* no matter where your cwd is in your filesystem.
