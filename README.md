# *seal*, make shell scripts readable

<!-- markdownlint-disable MD033 -->

<div align="center">
    <img src="assets/seal-smaller.png" width="240" alt="seal mascot reading reference book" /><br />
    <em>the cutest scripting runtime</em><br />
    <!-- Start-Precommit-Marker-2 --><img src="https://img.shields.io/badge/seal-0.0.8--rc.2-f0f8ff" alt="seal version" /><!-- End-Precommit-Marker-2 --> <!-- Start-Precommit-Marker-3 --><img src="https://img.shields.io/badge/Luau-0.725-4f99ba" alt="Luau version" /><!-- End-Precommit-Marker-3 --><br /><br />
    <a href="/docs/usage.md">Usage</a> | <a href="/docs/libraries_and_programming.md">Programming</a> | <a href="/docs/reference/">API Reference</a>
</div>
<br>
<!-- markdownlint-enable MD033 -->

<!-- *seal* is an all-in-one scripting tool that lets you write cross-platform scripts and applications in Luau with a greater utility than shell scripts and with less of a hassle than Python. -->

***seal*** is a cross-platform scripting tool that emphasizes correctness, performance, and fun.

It can surreptitiously replace your shell and single-use Python scripts with Luau scripts, making them more readable to everyone. Or you can use it to write a full application.

<!-- **Usecases:**

- cross-platform scripting tool with autocomplete
- faster Python replacement
- automation & task runner
- data viewing, manipulation, serialization
- write cute TUIs
- quickly deploy to major platforms
- data is pretty—just print it (colors included) -->

**Some features:**

- Faster than Python for general purpose scripting.
- *seal* projects are more scalable than shell scripts.
- Built in terminal manipulation for TUIs.
- Powerful multithreading.
- <!-- Start-Precommit-Marker-1 -->1136<!-- End-Precommit-Marker-1 --> handcrafted error messages.

## Install

Grab the [latest release](https://github.com/deviaze/seal/releases/latest) or check out [these install instructions](docs/install.md) for a detailed walkthrough.

To get started, you just need:

1. A text editor (VSCode, Zed, and nvim are supported by Luau Language Server).
2. [Luau Language Server](https://github.com/JohnnyMorganz/luau-lsp) installed in your editor.
3. The *seal* executable in your `$PATH`

## Usage

To start a new project with *seal*, make a new directory, run `seal setup project` inside it, and open it up with `code .`

- `seal ./filename.luau` runs a Luau file with *seal*.
- `seal run` runs the project at your current working directory.
- `seal compile` bundles and compiles the project at your current working directory into a standalone executable for your platform.

Check out the full [usage instructions](docs/usage.md) for more.

## Programming

Check out [the programming intro](docs/libraries_and_programming.md) to get started
or the [standard library reference](/docs/reference/) for all current features and APIs.
In supported editors, you can take advantage of modern tooling such as strict typechecking and autocomplete, inline documentation, automatic imports, etc.

Some quick examples:

- [HTTP - calling an API with API key](examples/basic_get.luau)
- [FS - file watching (upload files added to folder)](examples/upload_files_in_folder.luau)
- [FS - remove files older than a week](examples/older_than_a_week.luau)
- [TUI - two column option picker](examples/double_column_picker.luau)

## Roadmap

- More featureful `@extra` library.
- Custom and customizable `seal setup` scripts.
- Ecosystem of [external libraries](/docs/external_libraries.md) to expand *seal*'s functionality with native bindings.
- Cross-platform GUI and input automation libraries.
- Dedicated tooling integrations such as extensions and an MCP server.

## Community

[Join the Discord](https://discord.gg/3MJ37CFNWh) if you want to talk about *seal*, need help, or want to contribute!

## Reliability

*seal* wants to empower you to write correct code that doesn't explode at runtime. In most cases, runtime errors will be expressly documented, and/or returned as an `error` type to facilitate nonthrowing error handling with the typechecker.

If you encounter a bug, panic, or security vulnerability, please make an issue in this repo right away; you may attach a repro or send one privately to [dev@deviaze.com](mailto:dev@deviaze.com) or `@deviaze` on Discord.
