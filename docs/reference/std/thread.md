<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# thread

`local thread = require("@std/thread")`

$\hspace{5pt}$ Run Luau code in parallel in a new VM and communicate between threads via message passing.
$\hspace{5pt}$
$\hspace{5pt}$ This allows for structured parallelism that you can use for both multiprocessing and as
$\hspace{5pt}$ a replacement for async/task library usage, although with a little more boilerplate in the latter usecase.
$\hspace{5pt}$
$\hspace{5pt}$ Threads can be spawned via `thread.spawn`, communicated with on the *regular* or *bytes* channels with the `send*` and `read*` apis, and joined back
$\hspace{5pt}$ to the parent thread with `thread.join`.
$\hspace{5pt}$
$\hspace{5pt}$ ### Channels
$\hspace{5pt}$
$\hspace{5pt}$ Each thread come with 2 channels to communicate with its parent thread:
$\hspace{5pt}$
$\hspace{5pt}$ On the *regular* channel, messages can be either data tables (tables consisting of json-serializable objects)--seal automatically
$\hspace{5pt}$ serializes and deserializes data tables for simplicity and ergonomics--or strings.
$\hspace{5pt}$
$\hspace{5pt}$ On the *bytes* channel, data can be sent and received with `buffer`s without any serialization overhead.
$\hspace{5pt}$
$\hspace{5pt}$ Each channel has a queue; by default the *regular* channel's queue capacity is 12 messages and the *byte* channel 24,
$\hspace{5pt}$ although this is configurable with `thread.spawn`'s `ThreadSpawnOptions`. Reading a message will pop it from the queue.
$\hspace{5pt}$
$\hspace{5pt}$ ## Usage
$\hspace{5pt}$ ```luau
$\hspace{5pt}$     -- parent.luau
$\hspace{5pt}$     local thread = require("@std/thread")
$\hspace{5pt}$
$\hspace{5pt}$     local handle = thread.spawn {
$\hspace{5pt}$         path = "./child.luau", -- note these paths are relative like luau requires and unlike std/fs paths
$\hspace{5pt}$         data = { urls = urls }, -- you can optionally pass in startup data to use in the other thread
$\hspace{5pt}$     }
$\hspace{5pt}$
$\hspace{5pt}$     -- receive data from your thread using handle:read and handle:read_await
$\hspace{5pt}$     local data = handle:read_await()
$\hspace{5pt}$     while thread.sleep(20) do
$\hspace{5pt}$         local data = handle:read()
$\hspace{5pt}$         if typeof(data) == "table" then
$\hspace{5pt}$             print(data)
$\hspace{5pt}$         else
$\hspace{5pt}$             break
$\hspace{5pt}$         end
$\hspace{5pt}$     end
$\hspace{5pt}$     -- send data to your child thread using handle:send and handle:sendbytes
$\hspace{5pt}$     handle:send("hi")
$\hspace{5pt}$
$\hspace{5pt}$     -- don't forget to join your threads before your program exits!!
$\hspace{5pt}$     handle:join()
$\hspace{5pt}$
$\hspace{5pt}$     -- child.luau
$\hspace{5pt}$     if channel then -- channel is a global that exists in child threads and can be used to communicate with the parent thread
$\hspace{5pt}$         local urls = (channel.data :: { urls: { string } }).urls
$\hspace{5pt}$         channel:send("first")
$\hspace{5pt}$         for _, url in urls do
$\hspace{5pt}$             local result = callapi(url)
$\hspace{5pt}$             channel:send(result)
$\hspace{5pt}$         end
$\hspace{5pt}$         channel:send("done")
$\hspace{5pt}$     end
$\hspace{5pt}$
$\hspace{5pt}$```
$\hspace{5pt}$ Spawns a new Rust Thread running Luau code in a new Luau VM.
$\hspace{5pt}$
$\hspace{5pt}$ ## Usage
$\hspace{5pt}$ ```luau
$\hspace{5pt}$     -- main.luau
$\hspace{5pt}$     local thread = require("@std/thread")
$\hspace{5pt}$
$\hspace{5pt}$     local urls = {
$\hspace{5pt}$         "https://sealfinder.net/api/random",
$\hspace{5pt}$         "https://example.com/endpoint",
$\hspace{5pt}$     }
$\hspace{5pt}$
$\hspace{5pt}$     local threadpool: { thread.ThreadHandle } = {}
$\hspace{5pt}$     for _, url in urls do
$\hspace{5pt}$         local handle = thread.spawn {
$\hspace{5pt}$             path = "./web_get.luau",
$\hspace{5pt}$             data = { url = url },
$\hspace{5pt}$         }
$\hspace{5pt}$         table.insert(threadpool, handle)
$\hspace{5pt}$     end
$\hspace{5pt}$
$\hspace{5pt}$     while true do
$\hspace{5pt}$         for index, handle in threadpool do
$\hspace{5pt}$             local response = handle:read()
$\hspace{5pt}$             if response then
$\hspace{5pt}$                 print(response)
$\hspace{5pt}$                 handle:join()
$\hspace{5pt}$                 table.remove(threadpool, index)
$\hspace{5pt}$             end
$\hspace{5pt}$         end
$\hspace{5pt}$     end
$\hspace{5pt}$
$\hspace{5pt}$     -- web_get.luau
$\hspace{5pt}$     if channel then -- make sure we're in a child thread
$\hspace{5pt}$         local http = require("@std/net/http")
$\hspace{5pt}$         local response = http.get {
$\hspace{5pt}$             url = channel.data.url,
$\hspace{5pt}$         }
$\hspace{5pt}$         channel:send(response)
$\hspace{5pt}$     end
$\hspace{5pt}$```

.function thread.sleep(milliseconds: `number): true`

$\hspace{5pt}$ Literally the same as `time.wait`, except in milliseconds.

`export type` JsonSerializableTable

`export type` ThreadHandle

ThreadHandle.read name: `string`

$\hspace{5pt}$  the name of your thread (defaults to a petname if not provided)

ThreadHandle.join: `(self: ThreadHandle) -> ()`

$\hspace{5pt}$ Joins the child thread back to the main thread; don't forget to join your handles lest you want runaway threads!
$\hspace{5pt}$
$\hspace{5pt}$ Errors if the thread has already been joined or somehow disappeared.

ThreadHandle.send: `(self: ThreadHandle, data: JsonSerializableTable | string) -> ()`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ Serializes and sends data to the child thread on the regular channel. Data can either be a string or a JsonSerializableTable; table data is serialized to json for transport
$\hspace{5pt}$ and automatically deserialized when received by :read methods.
$\hspace{5pt}$
$\hspace{5pt}$ If the channel is full, blocks the current thread until the channel isn't full anymore.
$\hspace{5pt}$ If you want to not block the current thread, use `try_send` instead.
$\hspace{5pt}$
$\hspace{5pt}$ Errors if the channel has somehow become disconnected or provided data isn't json-serializable.

</details>

ThreadHandle.try_send: `(self: ThreadHandle, data: JsonSerializableTable | string) -> (boolean, "Sent" | "Disconnected" | "Full")`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ Try to send data to the child thread on the regular channel with the same semantics as `ThreadHandle:send`,
$\hspace{5pt}$ except doesn't block if the channel is already full, and doesn't throw an error if the channel is disconnected.
$\hspace{5pt}$
$\hspace{5pt}$ Returns two values: success and result.
$\hspace{5pt}$ - `result == "Disconnected"` means that the channel was disconnected and either the Sender or Receiver no longer exists.
$\hspace{5pt}$ This is usually caused by trying to send a message to a thread that's already been joined or exited.
$\hspace{5pt}$ - `result == "Full"` means that the channel's queue is full and no more new messages can be sent until the other side starts reading from the queue.

</details>

ThreadHandle.sendbytes: `(self: ThreadHandle, data: buffer) -> ()`

$\hspace{5pt}$ Sends a buffer on the bytes channel, blocking the current thread if the channel is full.
$\hspace{5pt}$
$\hspace{5pt}$ Errors if the channel has somehow become disconnected.

ThreadHandle.try_sendbytes: `(self: ThreadHandle, data: buffer) -> (boolean, "Sent" | "Disconnected" | "Full")`

<details>

<summary> See the docs </summary

$\hspace{5pt}$ Try to send data on the bytes channel with the same semantics as `ThreadHandle:sendbytes`,
$\hspace{5pt}$ except doesn't block if the channel is already full, and doesn't throw an error if the channel is disconnected.
$\hspace{5pt}$
$\hspace{5pt}$ Returns two values: success and result.
$\hspace{5pt}$ - `result == "Disconnected"` means that the channel was disconnected and either the Sender or Receiver no longer exists.
$\hspace{5pt}$ This is usually caused by trying to send a message to a thread that's already been joined or exited.
$\hspace{5pt}$ - `result == "Full"` means that the channel's queue is full and no more new messages can be sent until the other side starts reading from the queue.

</details>

ThreadHandle.read: `(self: ThreadHandle) -> JsonSerializableTable? | string?`

$\hspace{5pt}$ Read a message from the regular channel without blocking the current thread.
$\hspace{5pt}$
$\hspace{5pt}$ Errors if the channel has somehow become disconnected.

ThreadHandle.read_await: `(self: ThreadHandle) -> JsonSerializableTable | string`

$\hspace{5pt}$ Read a message from the regular channel, blocking until the next message is available.
$\hspace{5pt}$
$\hspace{5pt}$ Errors if the channel has somehow become disconnected.

ThreadHandle.readbytes: `(self: ThreadHandle) -> buffer?`

$\hspace{5pt}$ Read a message from the bytes channel without blocking the current thread.
$\hspace{5pt}$
$\hspace{5pt}$ Errors if the channel has somehow become disconnected.

ThreadHandle.readbytes_await: `(self: ThreadHandle) -> buffer`

$\hspace{5pt}$ Read a message from the bytes channel, blocking until the next message is available.
$\hspace{5pt}$
$\hspace{5pt}$ Errors if the channel has somehow become disconnected.

`export type` ThreadSpawnOptions

ThreadSpawnOptions.name: `string?`

$\hspace{5pt}$  Name your thread to quickly identify which one it is; if not provided a default alliterative petname will be provided instead.

ThreadSpawnOptions.path: `string?`

$\hspace{5pt}$  Path to your source file you want to run in the separate thread, relative to the current file (not cwd).

ThreadSpawnOptions.src: `string?`

$\hspace{5pt}$  Source code to evaluate; recommend passing a path instead.

ThreadSpawnOptions.data: `JsonSerializableTable?`

$\hspace{5pt}$  Optional data you want to provide to your thread at startup; accessible with `channel.data` in the child thread.

ThreadSpawnOptions.capacity.regular: `number?`

$\hspace{5pt}$  Override the queue capacity of your thread's regular and bytes channels.
$\hspace{5pt}$  default is 12

ThreadSpawnOptions.capacity.bytes: `number?`

$\hspace{5pt}$  default is 24
