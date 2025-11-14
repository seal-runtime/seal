# *seal*, the cutest scripting runtime

Use *seal* to write fun, maintainable, and easily-deployable programs in [Luau](https://luau.org), a simple, dependable, and extremely fast scripting language with good typechecking and tooling support.

## Goals

- Be a great cross-platform alternative to shell scripts, wrappers, and quick-and-dirty solutions in the terminal. And when your project grows into a real program, *seal* will grow with it too; it's fast enough.
- Provide a simple, useful, and expressive API that allows you to get right into your project and start writing code. You can prototype faster thanks to inline documentation, modern tooling, and type safety.
- Put you back in control. When you run into trouble, *seal* should tell you *exactly* what went wrong with a custom, handcrafted warning or error message.

## Install

See the [install instructions](docs/install.md) for a detailed walkthrough of getting *seal* on your system. Basically you just need a text editor (VSCode, Zed, or nvim), [Luau Language Server](https://github.com/JohnnyMorganz/luau-lsp) installed in your text editor, and the [latest release](<https://github.com/deviaze/seal/releases/latest>) of *seal* in your `$PATH`.

## Usage

To start a new project with *seal*, make a new directory, run `seal setup project` inside it, and open it up with `code .`

- `seal ./filename.luau` runs a Luau file with *seal*.
- `seal run` runs the project at your current working directory.
- `seal compile` bundles and compiles the project at your current working directory into a standalone executable.

Check out the full [usage instructions](docs/usage.md) for more.

## Programming and Standard Library

If you're new to Luau, check out *seal*'s [Luau Book](/docs/luau-book/index.md). For a few examples of using *seal* libraries, check out [the programming intro](/docs/libraries_and_programming.md).

See the [standard library reference](/docs/reference/std/) for all current features and APIs.

## Roadmap

- Integrated webview for cross-platform GUI applications.
- Plug-and-play FFI 'crates' system that allows loading Rust dependencies at runtime.
