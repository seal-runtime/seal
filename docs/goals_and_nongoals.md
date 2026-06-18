# Goals and non-goals

*seal* is meant for general purpose programming and shell scripting, but doesn't try to be the lowest-level Luau runtime out there. Specifically, *seal* thinks that writing native FFI bindings in Luau is a mistake due to userdata overhead and the fact that Luau is a GCed language. Please just write C or Rust bindings instead.

## Goals

- Be a great cross-platform alternative to shell scripts, wrappers, and quick-and-dirty solutions in the terminal.
- Provide a simple, useful, and expressive API that fulfills many high-level usecases, and can be composed to solve the rest.
- Reliability. When you run into trouble, *seal* should tell you *exactly* what went wrong with a custom, handcrafted warning or error message.

## Non-goals/out of scope

- Fully featured standard library for all usecases: `seal` is primarily suited for high level scripting and light general purpose programming.
- We don't want to add every single hash algorithm, nor bind to every single part of Rust's standard library—providing too many options might end up confusing to the average user.
- Use [Zune](<https://github.com/Scythe-Technology/zune>) instead if you need lower level bindings.
- Async webservers. *seal* is not an async runtime and thus cannot scale up enough for webservers. If you want to write one (or a Discord bot) in Luau and need async, I highly recommend using Zune instead.
- Premature optimization. Although *seal* is pretty-to-very fast, it might not be the absolute fastest way to use Luau due to `mluau`'s slight safety overhead.
  - Still way faster than Python.
  - On the other hand, because `seal` doesn't have any tokio or async overhead, its standard library should be faster than `Lune`'s, and because of its parallelism model, true multithreaded programs in `seal` should be more stable than programs that rely on Lune's `task` library and IO.
