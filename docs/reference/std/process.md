<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# process

`local process = require("@std/process")`

$\hspace{5pt}$ Library for running child processes.
$\hspace{5pt}$
$\hspace{5pt}$ ## Usage
$\hspace{5pt}$
$\hspace{5pt}$ ```luau
$\hspace{5pt}$ local process = require("@std/process")
$\hspace{5pt}$
$\hspace{5pt}$ -- run a simple program with args
$\hspace{5pt}$ local result = process.run {
$\hspace{5pt}$     program = "markdownlint-cli2",
$\hspace{5pt}$     args = { "--fix", "myfile.md" },
$\hspace{5pt}$ }
$\hspace{5pt}$
$\hspace{5pt}$ -- a shell command
$\hspace{5pt}$ local files = process.shell("ls -l"):unwrap()
$\hspace{5pt}$
$\hspace{5pt}$ -- a long running child process
$\hspace{5pt}$ local child = process.spawn({
$\hspace{5pt}$     program = "someutil --watch",
$\hspace{5pt}$     shell = "sh",
$\hspace{5pt}$ })
$\hspace{5pt}$
$\hspace{5pt}$ for line in child.stdout:lines() do
$\hspace{5pt}$     local thing_changed = line:match("([%w]+) changed!")
$\hspace{5pt}$     print(`Change detected: {thing_changed}`)
$\hspace{5pt}$ end
$\hspace{5pt}$```

process.run: `(options: RunOptions) -> RunResult`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ Runs a program, yields until it completes, and returns its results.
$\hspace{5pt}$
$\hspace{5pt}$ Takes a RunOptions table:
$\hspace{5pt}$ ```luau
$\hspace{5pt}$ type RunOptions = {
$\hspace{5pt}$     program: string,
$\hspace{5pt}$     --- optional args you want to pass
$\hspace{5pt}$     args: { string }?,
$\hspace{5pt}$     --- the name or path of the shell, omit to run without shell
$\hspace{5pt}$     shell: string?
$\hspace{5pt}$     --- path to the the working directory you want your command to execute in
$\hspace{5pt}$     cwd: string?,
$\hspace{5pt}$ }
$\hspace{5pt}$```
$\hspace{5pt}$
$\hspace{5pt}$ ### Blocks
$\hspace{5pt}$
$\hspace{5pt}$ Until the process exits.
$\hspace{5pt}$
$\hspace{5pt}$ ### Usage
$\hspace{5pt}$ ```luau
$\hspace{5pt}$ local process = require("@std/process")
$\hspace{5pt}$ local result = process.run {
$\hspace{5pt}$     program = "lune",
$\hspace{5pt}$     args = {"run", somefile},
$\hspace{5pt}$ }
$\hspace{5pt}$ if result.ok then
$\hspace{5pt}$     print(result.stdout)
$\hspace{5pt}$ end
$\hspace{5pt}$```

</details>

process.shell: `(command: string) -> RunResult`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ Launches a shell command in a child process.
$\hspace{5pt}$
$\hspace{5pt}$ Uses the same shell you're using to run `seal` (so your aliases should available, except on Windows,
$\hspace{5pt}$ where it's a bit more complicated).
$\hspace{5pt}$
$\hspace{5pt}$ To find your current shell, `process.shell` checks your `SHELL` environment variable, and
$\hspace{5pt}$ if not found, defaults to `sh` on Unix-like systems and `powershell` (Windows PowerShell) on Windows.
$\hspace{5pt}$
$\hspace{5pt}$ On Windows, arguments aren't necessarily separated by whitespace like on Linux, and every program
$\hspace{5pt}$ might accept arguments in a slightly different way, so be careful and use `process.run` with `args`
$\hspace{5pt}$ when in doubt.
$\hspace{5pt}$
$\hspace{5pt}$ Note that spawning processes (even starting the `powershell` process) is slow on Windows,
$\hspace{5pt}$ so I recommend sticking to `process.run` with `args` unless you need shell behavior for your usecase.
$\hspace{5pt}$
$\hspace{5pt}$ ### Usage
$\hspace{5pt}$ ```luau
$\hspace{5pt}$ local process = require("@std/process")
$\hspace{5pt}$ local file_stuff: {string} = process.shell("ls -l"):unwrap()
$\hspace{5pt}$ print(file_stuff)
$\hspace{5pt}$```

</details>

process.spawn: `(options: SpawnOptions) -> ChildProcess`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ Spawns a long-running process in a non-blocking manner, returns a `ChildProcess` that contains handles to the spawned process' stdout, stderr, and stdin.
$\hspace{5pt}$
$\hspace{5pt}$ ## Usage
$\hspace{5pt}$ ```luau
$\hspace{5pt}$ local process = require("@std/process")
$\hspace{5pt}$ local child = process.spawn({
$\hspace{5pt}$     program = "someutil --watch",
$\hspace{5pt}$     shell = "sh",
$\hspace{5pt}$ })
$\hspace{5pt}$
$\hspace{5pt}$ for line in child.stdout:lines() do
$\hspace{5pt}$     local thing_changed = line:match("([%w]+) changed!")
$\hspace{5pt}$     print(`Change detected: {thing_changed}`)
$\hspace{5pt}$ end
$\hspace{5pt}$```

</details>

process.setexitcallback: `((number) -> ()) -> ()`

$\hspace{5pt}$ Doesn't work.

process.exit: `(code: number?) -> never`

$\hspace{5pt}$ Immediately terminate the current program with exit `code`.
$\hspace{5pt}$
$\hspace{5pt}$ Typically exit code 0 means success and exit code 1 means failure.

`export type` RunResult

RunResult.unwrap: `(self: RunResult) -> string`

$\hspace{5pt}$  Returns the `RunResult`'s `stdout` if it was successful, stripping trailing whitespace and newlines.
$\hspace{5pt}$  Errors if the RunResult was unsuccessful.

RunResult.unwrap_or: `(self: RunResult, default: string | (result: RunResult) -> string) -> string`

$\hspace{5pt}$ Returns the `RunResult`'s `stdout` if it was successful, otherwise returns a default value.
$\hspace{5pt}$
$\hspace{5pt}$ The `default` can be either a string or a function that takes in the `RunResult` and returns a string.
$\hspace{5pt}$ If you provide a `default` function, `:unwrap_or` will return what it returns.

`export type` RunResultOk

RunResultOk.ok: `true`

RunResultOk.out: `string`

$\hspace{5pt}$  cleaned standard output of the process, shouldn't have trailing newlines or whitespace

RunResultOk.stdout: `string`

$\hspace{5pt}$  the raw standard output (stdout) generated by the process; this includes trailing newlines and whitespace

RunResultOk.stderr: `string`

$\hspace{5pt}$  the raw standard error (stderr) generated by the process; this includes trailing newlines and whitespace

`export type` RunResultErr

RunResultErr.ok: `false`

RunResultErr.err: `string`

RunResultErr.stdout: `string`

RunResultErr.stderr: `string`

`export type` RunOptions

RunOptions.program: `string`

RunOptions.args: `{ string }?`

$\hspace{5pt}$  an optional list of arguments to pass into the program; on Windows, you should use this instead of trying to pass whitespace-separated arguments in `program`

RunOptions.shell: `string?`

$\hspace{5pt}$  specify a shell to run the program with; otherwise runs it as a bare process with no shell

RunOptions.cwd: `string?`

$\hspace{5pt}$  path to the the working directory you want your command to execute in, defaults to your shell's cwd

`export type` SpawnOptions

SpawnOptions.program: `string`

$\hspace{5pt}$  the program you want to run, must be available in $PATH or be an absolute path to an executable

SpawnOptions.args: `{ string }?`

$\hspace{5pt}$  an optional list of arguments to pass into the program; on Windows, you should use this instead of trying to pass whitespace-separated arguments in `program`

SpawnOptions.shell: `string?`

SpawnOptions.cwd: `string?`

$\hspace{5pt}$  path to the the working directory you want your command to execute in, defaults to your shell's cwd

SpawnOptions.stream.stdout_capacity: `number?`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ A `ChildProcessStream` captures incoming bytes from your `ChildProcess`' output streams (either stdout or stderr),
$\hspace{5pt}$ and caches them in its `inner` buffer. Each stream is spawned in a separate Rust thread to facilitate
$\hspace{5pt}$ consistently nonblocking, dependable reads, allowing most `ChildProcess.stream:read` methods to be fully nonblocking unless
$\hspace{5pt}$ specifically requested otherwise.
$\hspace{5pt}$
$\hspace{5pt}$ ## Options
$\hspace{5pt}$
$\hspace{5pt}$ ### Stream capacity
$\hspace{5pt}$
$\hspace{5pt}$ To prevent memory leaks (if you spawn a child process and never read from stdout or stderr), each stream's inner buffer capacity is capped,
$\hspace{5pt}$ and adjustable by setting `stdout_capacity` and `stderr_capacity`, respectively.
$\hspace{5pt}$
$\hspace{5pt}$ By default, `stdout` streams are capped to 2048 bytes and `stderr` streams to 1028.
$\hspace{5pt}$
$\hspace{5pt}$ When more bytes are read from the stream than can fit in the buffer, old bytes are drained (and lost!) so the buffer
$\hspace{5pt}$ can remain the same size (preventing memory leaks).
$\hspace{5pt}$
$\hspace{5pt}$ By increasing stream capacity, you allow more bytes to be read/consumable at the same time without losing data if you infrequently read from the buffer.
$\hspace{5pt}$
$\hspace{5pt}$ If you're reading from stdout in really tight loops, and your ChildProcess only spits out data in chunks of 512 bytes at a time, you can definitely
$\hspace{5pt}$ decrease your stream capacity to 512 bytes (or slightly larger just in case), to reduce allocations.
$\hspace{5pt}$
$\hspace{5pt}$ ## Truncation
$\hspace{5pt}$
$\hspace{5pt}$ By default, seal truncates bytes from the front of inner, causing old data to be lost first. Set `truncate == "back"` to override this behavior (and preserve old data at the expense of incoming data)
$\hspace{5pt}$  inner buffer capacity of `ChildProcess.stdout`, default 2048

</details>

SpawnOptions.stream.stderr_capacity: `number?`

$\hspace{5pt}$  inner buffer capacity of `ChildProcess.stderr`, default 1024

SpawnOptions.stream.stdout_truncate: `("front" | "back")?`

$\hspace{5pt}$  what side of stdout should be truncated when full? defaults to "front"

SpawnOptions.stream.stderr_truncate: `("front" | "back")?`

$\hspace{5pt}$  what side of stderr should be truncated when full? defaults to "front"

`export type` ChildProcessStream

$\hspace{5pt}$  Represents the stdout and stderr streams of a `ChildProcess`, both ran in parallel threads
$\hspace{5pt}$  and streamed for nonblocking behavior.

ChildProcessStream.read: `(self: ChildProcessStream, count: number?, timeout: number?) -> string?`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ Reads up to `count` bytes from the stream for up to `timeout` seconds, retrying while the stream remains empty.
$\hspace{5pt}$
$\hspace{5pt}$ - If `count` is unspecified or nil, reads the entire stream instead of stopping at `count` bytes.
$\hspace{5pt}$ - If `timeout` is unspecified or nil, keeps retrying *forever* while the stream is empty and the reader is still alive.
$\hspace{5pt}$ - If `timeout > 0`, keeps retrying for `timeout` seconds while the stream is empty and the reader is still alive.
$\hspace{5pt}$
$\hspace{5pt}$ ### Returns
$\hspace{5pt}$
$\hspace{5pt}$ Returns as soon as anything gets written to the stream, consuming and returning the available bytes without
$\hspace{5pt}$ any intermediate utf8 coercion/validation. Returns `nil` if `timeout` seconds elapse and the stream remains empty.
$\hspace{5pt}$
$\hspace{5pt}$ If you want to wait until a specific number of bytes are available, use `:read_exact` instead.
$\hspace{5pt}$
$\hspace{5pt}$ ### Blocks
$\hspace{5pt}$
$\hspace{5pt}$ Blocks the current VM until the stream's not empty, `timeout` seconds elapse, or the reader thread exits.
$\hspace{5pt}$
$\hspace{5pt}$ To prevent this function from blocking, pass a `timeout` of 0 seconds!
$\hspace{5pt}$
$\hspace{5pt}$ ## Usage
$\hspace{5pt}$
$\hspace{5pt}$ Keep reading until data appears (default behavior):
$\hspace{5pt}$ ```luau
$\hspace{5pt}$ local first_message = child.stdout:read() :: string
$\hspace{5pt}$ print(first_message)
$\hspace{5pt}$```
$\hspace{5pt}$ Read the first 256 bytes once data appears:
$\hspace{5pt}$ ```luau
$\hspace{5pt}$ local first_part = child.stdout:read(256) :: string
$\hspace{5pt}$```
$\hspace{5pt}$ Get everything currently in the stream without blocking:
$\hspace{5pt}$ ```luau
$\hspace{5pt}$ local current_data = child.stdout:read(nil, 0.0)
$\hspace{5pt}$```

</details>

ChildProcessStream.read_exact: `(self: ChildProcessStream, count: number, timeout: number?) -> string?`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ Reads exactly `count` bytes from the stream, retrying until `count` bytes are available or `timeout` seconds elapse.
$\hspace{5pt}$
$\hspace{5pt}$ Bytes are not consumed from the stream until exactly `count` bytes are available.
$\hspace{5pt}$
$\hspace{5pt}$ - If `timeout` is unspecified or nil, keeps retrying *forever* while the stream is empty and the reader is still alive.
$\hspace{5pt}$ - If `timeout > 0`, keeps retrying for `timeout` seconds while the stream is empty and the reader is still alive.
$\hspace{5pt}$
$\hspace{5pt}$ ### Returns
$\hspace{5pt}$
$\hspace{5pt}$ A string of exactly `count` length without any intermediate utf-8 coercion/validation, or `nil` if exactly `count` bytes couldn't be read from the stream.
$\hspace{5pt}$
$\hspace{5pt}$ ### Blocks
$\hspace{5pt}$
$\hspace{5pt}$ Blocks the current VM until `count` bytes are found, `timeout` seconds elapse, or the reader thread exits.
$\hspace{5pt}$
$\hspace{5pt}$ Pass a timeout of `0` seconds to make this function nonblocking.
$\hspace{5pt}$
$\hspace{5pt}$ ## Usage
$\hspace{5pt}$
$\hspace{5pt}$ Read exactly 512 bytes as soon as 512 bytes are available:
$\hspace{5pt}$
$\hspace{5pt}$ ```luau
$\hspace{5pt}$ local first_512 = child.stdout:read_exact(512)
$\hspace{5pt}$```
$\hspace{5pt}$
$\hspace{5pt}$ Read from both streams every 0.5 seconds, byte by byte, without otherwise blocking the VM:
$\hspace{5pt}$
$\hspace{5pt}$ ```luau
$\hspace{5pt}$ local stdout_chars: { string } = {}
$\hspace{5pt}$ local stderr_chars: { string } = {}
$\hspace{5pt}$ while time.wait(0.5) and child:alive() do
$\hspace{5pt}$     local stdout_char = child.stdout:read_exact(1, 0.0)
$\hspace{5pt}$     if stdout_char then
$\hspace{5pt}$         table.insert(stdout_chars, stdout_char)
$\hspace{5pt}$     end
$\hspace{5pt}$     local stderr_char = child.stderr:read_exact(1, 0.0)
$\hspace{5pt}$     if stderr_char then
$\hspace{5pt}$         table.insert(stderr_chars, stderr_char)
$\hspace{5pt}$     end
$\hspace{5pt}$ end
$\hspace{5pt}$```

</details>

ChildProcessStream.read_to: `(self: ChildProcessStream, term: string, inclusive: boolean?, timeout: number?, allow_partial: boolean?) -> string?`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ Keep reading from the stream until search `term` is encountered. This is especially useful if you're trying to read line-by-line,
$\hspace{5pt}$ or until a specific delimiter is encountered.
$\hspace{5pt}$
$\hspace{5pt}$ By default, blocks the current VM (Rust Thread) until `term` is found, and doesn't consume any bytes from the stream until `term` is found.
$\hspace{5pt}$
$\hspace{5pt}$ - If `inclusive == true`, includes `term` with the resulting string, otherwise strips it from the result.
$\hspace{5pt}$ - If `timeout` is specified, `allow_partial` is unspecified or false, and `term` isn't found before `timeout` seconds elapse, returns `nil` without consuming any bytes from the stream.
$\hspace{5pt}$ - If `timeout` is specified, `allow_partial` is true, and `term` isn't found before `timeout` seconds elapse, consumes and returns the entire contents of the stream.
$\hspace{5pt}$ - `allow_partial` doesn't do anything if `timeout` is unspecified.
$\hspace{5pt}$
$\hspace{5pt}$ ### Returns
$\hspace{5pt}$
$\hspace{5pt}$ The stream's contents without any intermediate utf8 conversion/validation.
$\hspace{5pt}$ Returns `nil` if `timeout` elapses and the search term hasn't been found.
$\hspace{5pt}$
$\hspace{5pt}$ ### Blocks
$\hspace{5pt}$
$\hspace{5pt}$ Blocks the current VM until `term` is found, `timeout` seconds elapse, or the reader thread exits.

</details>

ChildProcessStream.fill: `(self: ChildProcessStream, target: buffer, target_offset: number?, timeout: number?) -> number`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ Fill the `target` buffer with as many bytes as possible from the stream. Retries until the stream is nonempty or `timeout` seconds elapse.
$\hspace{5pt}$
$\hspace{5pt}$ - `target_offset` defaults to `0` if unspecified or nil.
$\hspace{5pt}$
$\hspace{5pt}$ ### Returns
$\hspace{5pt}$
$\hspace{5pt}$ The number of bytes successfully consumed from the stream and read into the `target` buffer.
$\hspace{5pt}$
$\hspace{5pt}$ This function returns as soon as anything is written to the stream; use `:fill_exact` instead to return as soon as a specific number
$\hspace{5pt}$ of bytes are available in the stream.
$\hspace{5pt}$
$\hspace{5pt}$ ### Errors
$\hspace{5pt}$
$\hspace{5pt}$ - If `target_offset` is greater than the buffer's length.
$\hspace{5pt}$
$\hspace{5pt}$ This function should not overfill the target buffer! A maximum of `buffer.len(target) - buffer_offset` bytes should be consumed.
$\hspace{5pt}$
$\hspace{5pt}$ ## Usage
$\hspace{5pt}$
$\hspace{5pt}$ ```luau
$\hspace{5pt}$ local buffy = buffer.create(1024)
$\hspace{5pt}$ local offset = 0
$\hspace{5pt}$ while child:alive() and offset < 1024 do
$\hspace{5pt}$     local count = child.stdout:fill(buffy, offset)
$\hspace{5pt}$     offset += count
$\hspace{5pt}$ end
$\hspace{5pt}$```

</details>

ChildProcessStream.fill_exact: `(self: ChildProcessStream, count: number, target: buffer, target_offset: number?, timeout: number?) -> boolean`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ Read exactly `count` bytes into the `target` buffer at `target_offset`, retrying until `count` bytes are available or `timeout` seconds elapse.
$\hspace{5pt}$
$\hspace{5pt}$ - `target_offset` defaults to `0` if unspecified or nil.
$\hspace{5pt}$
$\hspace{5pt}$ ### Returns
$\hspace{5pt}$
$\hspace{5pt}$ `true` if `count` bytes were successfully read and consumed from the stream, `false` otherwise.
$\hspace{5pt}$
$\hspace{5pt}$ ### Errors
$\hspace{5pt}$
$\hspace{5pt}$ - If `target_offset` + `count` > buffer length - 1; this usually means a logic bug. Remember to clamp your offsets!
$\hspace{5pt}$
$\hspace{5pt}$ ### Blocks
$\hspace{5pt}$
$\hspace{5pt}$ Blocks the current VM until `count` bytes are available in the stream or `timeout` seconds elapse.
$\hspace{5pt}$
$\hspace{5pt}$ Pass a `timeout` of `0` seconds to prevent this function from blocking!

</details>

ChildProcessStream.len: `(self: ChildProcessStream) -> number`

$\hspace{5pt}$  Returns the current length/size of the stream's inner buffer.

ChildProcessStream.capacity: `(self: ChildProcessStream) -> number`

$\hspace{5pt}$  Returns the maximum capacity of the stream's inner buffer.

ChildProcessStream.lines: `(self: ChildProcessStream, timeout: number?) -> (() -> string)`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ Iterate over the lines in the stream, blocking the current VM (Rust Thread) until all lines are read or the timeout has been reached.
$\hspace{5pt}$
$\hspace{5pt}$ If a `timeout` is specified, `:lines()` will stop iterating once a line hasn't been seen for `timeout` seconds, allowing you to
$\hspace{5pt}$ early-interrupt when new data hasn't been seen for a while.
$\hspace{5pt}$
$\hspace{5pt}$ Unlike `:iter`, this method cleans up `\r` prefixes and trailing `\n`s.
$\hspace{5pt}$
$\hspace{5pt}$ ## Usage
$\hspace{5pt}$
$\hspace{5pt}$ ### In a loop:
$\hspace{5pt}$
$\hspace{5pt}$ ```luau
$\hspace{5pt}$ local process = require("@std/process")
$\hspace{5pt}$ local child = process.spawn({
$\hspace{5pt}$     program = "someutil --watch",
$\hspace{5pt}$     shell = "sh",
$\hspace{5pt}$ })
$\hspace{5pt}$
$\hspace{5pt}$ for line in child.stdout:lines() do
$\hspace{5pt}$     local thing_changed = line:match("([%w]+) changed!")
$\hspace{5pt}$     print(`Change detected: {thing_changed}`)
$\hspace{5pt}$ end
$\hspace{5pt}$```
$\hspace{5pt}$
$\hspace{5pt}$ ### As iterator:
$\hspace{5pt}$
$\hspace{5pt}$ ```luau
$\hspace{5pt}$ local process = require("@std/process")
$\hspace{5pt}$ local child = process.spawn {
$\hspace{5pt}$     program = "somewatcher --watch",
$\hspace{5pt}$     shell = "sh",
$\hspace{5pt}$ }
$\hspace{5pt}$
$\hspace{5pt}$ local next_line = child.stdout:lines()
$\hspace{5pt}$ local first_line = next_line()
$\hspace{5pt}$ local second_line = next_line()
$\hspace{5pt}$```

</details>

ChildProcessStream.iter: `(self: ChildProcessStream, timeout: number?, write_delay_ms: number?) -> () -> string`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ Iterate over the stream with more granular options:
$\hspace{5pt}$
$\hspace{5pt}$ To prevent nonterminating iteration without an explicit `break`, you can provide a `timeout`, which stops iteration
$\hspace{5pt}$ when new data hasn't been seen for `timeout` seconds.
$\hspace{5pt}$
$\hspace{5pt}$ When iteration attempts to read from an empty stream, it waits `write_delay_ms` milliseconds (default 5) before trying again.
$\hspace{5pt}$ Increase this value if you see weird chunking behavior (you want to see more data each iteration),
$\hspace{5pt}$ or decrease this value if your child process outputs quickly and you want iteration to go faster.
$\hspace{5pt}$
$\hspace{5pt}$ This function does *not* strip preceding '\r's and trailing '\n's (unlike `:lines()` and generalized iteration).

</details>

ChildProcessStream.__iter: `(self: ChildProcessStream) -> () -> string`

$\hspace{5pt}$ Iterate over the lines of the `ChildProcessStream` with generalized iteration, blocking until `break` or the reader thread exits.
$\hspace{5pt}$
$\hspace{5pt}$ Basically equivalent to `ChildProcessStream:lines()` except with generalized iteration you can't specify a `timeout`.

`export type` ChildProcessStdin

ChildProcessStdin.write: `(self: ChildProcessStdin, data: string) -> error?`

$\hspace{5pt}$ Attempts to write to the child process' stdin; if an error occurs (usually a broken pipe), returns a seal `error` userdata.

ChildProcessStdin.close: `(self: ChildProcessStdin) -> ()`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ Explicitly closes the child process stdin; this signals EOF for some programs that read multiple lines from stdin.
$\hspace{5pt}$
$\hspace{5pt}$ Errors if it can't flush the child process' stdin before closing.
$\hspace{5pt}$
$\hspace{5pt}$ ## Usage
$\hspace{5pt}$
$\hspace{5pt}$ ```luau
$\hspace{5pt}$ local child = process.spawn {
$\hspace{5pt}$     program = "python3",
$\hspace{5pt}$     args = { "-" },
$\hspace{5pt}$ }
$\hspace{5pt}$ child.stdin:write(PYTHON_SRC)
$\hspace{5pt}$ child.stdin:close()
$\hspace{5pt}$```

</details>

`export type` ChildProcess

ChildProcess.id: `number`

ChildProcess.alive: `(self: ChildProcess) -> boolean`

ChildProcess.kill: `(self: ChildProcess) -> ()`

ChildProcess.stdout: `ChildProcessStream`

ChildProcess.stderr: `ChildProcessStream`

ChildProcess.stdin: `ChildProcessStdin`
