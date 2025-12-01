# *seal* docs

## Standard Library

The *seal* standard library exposes functionality from Rust that you probably need in a CLI program. Libraries in `@std` add standard general-purpose functionality to Luau like reading/writing files, getting things from the internet, running other programs, and more.

Look in the [std](/docs/reference/std) folder for the standard library reference.

If a library contains multiple sub-libraries (like `@std/fs`), documentation for the top-level library will be at `library/init.md`.

## Extra Library

The `@extra` library exposes optional functionality written in Luau that you may or may not need depending on your usecases. *seal* currently bundles `@extra` within itself, but later may split it out or expose ways to pick and choose the libraries you want.

Unlike `@std` libraries, `@extra` libraries can be modified by directly editing their `*.luau` files to change their functionality.

## Interop Library

The `@interop` library exposes functionality for looking into *seal*'s internals, interacting with compiled *seal* standalone executables, and more.

## Globals

In addition to the Luau standard libraries (like `table`, `os`, `buffer`, etc.), *seal* exposes the globals:

### `script`

Exposes useful information about the current source file, such as where it's located on the filesystem and some methods for accessing its parent directory and *seal* project.

```luau
declare script: {
    entry_path: string,
    path: (self: any) -> string,
    parent: (self: any) -> string,
    project: (self: any) -> string,
}
```

To check if the currently-running script was ran directly (like `if __name__ == "__main__"` in python), use:

```luau
if script:path() == script.entry_path then
    -- ran directly with seal ./my/file.luau or seal run or as a standalone binary
else
    -- was required by another file
end
```

Note that in standalone programs (compiled with `seal compile`), your `script:path() == script.entry_path` check will ***change behavior*** and will always execute.

### `channel`

In child threads, *seal* exposes a `channel` table which allows you to send messages back to the parent thread:

```luau
declare channel: {
    send: <D>(self: any, data: D | string) -> (),
    sendbytes: <D>(self: any, data: buffer) -> (),
    read: <D>(self: any) -> D?,
    read_await: <D>(self: any) -> D,
    readbytes: (self: any) -> buffer?,
    readbytes_await: (self: any) -> buffer,
    data: any?,
}?
```

### Printing globals

- `p`: simple print and returns
- `pp`: pretty print and returns
- `dp`: debug print with line number and source file, also returns; good for investigating strings with arbitrary bytes.
- `warn`: writes a yellow error message to stderr

### Other globals

- `_SEAL_VERSION`: *seal*'s version
- `ecall`: converts a function that can return an `error` into one that throws an `error` (opposite of `pcall`)

## Internal Library

The `@internal` alias is used to expose internal Rust functionality to parts of *seal* written in Luau, predominantly the *seal* standard library.
