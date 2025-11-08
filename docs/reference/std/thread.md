<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# thread

`local thread = require("@std/thread")`

Run Luau code in parallel in a new VM and communicate between threads via message passing.

This allows for structured parallelism that you can use for both multiprocessing and as
a replacement for async/task library usage, although with a little more boilerplate in the latter usecase.

Threads can be spawned via `thread.spawn`, communicated with on the *regular* or *bytes* channels with the `send*` and `read*` apis, and joined back
to the parent thread with `thread.join`.

### Channels

Each thread come with 2 channels to communicate with its parent thread:

On the *regular* channel, messages can be either data tables (tables consisting of json-serializable objects)--seal automatically
serializes and deserializes data tables for simplicity and ergonomics--or strings.

On the *bytes* channel, data can be sent and received with `buffer`s without any serialization overhead.

Each channel has a queue; by default the *regular* channel's queue capacity is 12 messages and the *byte* channel 24,
although this is configurable with `thread.spawn`'s `ThreadSpawnOptions`. Reading a message will pop it from the queue.

## Usage

```luau
-- parent.luau
local thread = require("@std/thread")

local handle = thread.spawn {
    path = "./child.luau", -- note these paths are relative like luau requires and unlike std/fs paths
    data = { urls = urls }, -- you can optionally pass in startup data to use in the other thread
}

-- receive data from your thread using handle:read and handle:read_await
local data = handle:read_await()
while thread.sleep(20) do
    local data = handle:read()
    if typeof(data) == "table" then
        print(data)
    else
        break
    end
end
-- send data to your child thread using handle:send and handle:sendbytes
handle:send("hi")

-- don't forget to join your threads before your program exits!!
handle:join()

-- child.luau
if channel then -- channel is a global that exists in child threads and can be used to communicate with the parent thread
    local urls = (channel.data :: { urls: { string } }).urls
    channel:send("first")
    for _, url in urls do
        local result = callapi(url)
        channel:send(result)
    end
    channel:send("done")
end

```

---

### thread.spawn

```luau
thread.spawn: (spawn_options: ThreadSpawnOptions) -> ThreadHandle,
```

<details>

<summary> See the docs </summary

Spawns a new Rust Thread running Luau code in a new Luau VM.

## Usage

```luau
-- main.luau
local thread = require("@std/thread")

local urls = {
    "https://sealfinder.net/api/random",
    "https://example.com/endpoint",
}

local threadpool: { thread.ThreadHandle } = {}
for _, url in urls do
    local handle = thread.spawn {
        path = "./web_get.luau",
        data = { url = url },
    }
    table.insert(threadpool, handle)
end

while true do
    for index, handle in threadpool do
        local response = handle:read()
        if response then
            print(response)
            handle:join()
            table.remove(threadpool, index)
        end
    end
end

-- web_get.luau
if channel then -- make sure we're in a child thread
    local http = require("@std/net/http")
    local response = http.get {
        url = channel.data.url,
    }
    channel:send(response)
end
```

</details>

---

### thread.sleep

```luau
thread.sleep: (milliseconds: number) -> true,
```

Literally the same as `time.wait`, except in milliseconds.

---

### `export type` JsonSerializableTable

---

### `export type` ThreadHandle

---

### ThreadHandle.name

```luau
ThreadHandle.name: string,
```

 the name of your thread (defaults to a petname if not provided)

---

### ThreadHandle.join

```luau
ThreadHandle.join: (self: ThreadHandle) -> (),
```

Joins the child thread back to the main thread; don't forget to join your handles lest you want runaway threads!

Errors if the thread has already been joined or somehow disappeared.

---

### ThreadHandle.send

```luau
ThreadHandle.send: (self: ThreadHandle, data: JsonSerializableTable | string) -> (),
```

<details>

<summary> See the docs </summary

Serializes and sends data to the child thread on the regular channel. Data can either be a string or a JsonSerializableTable; table data is serialized to json for transport
and automatically deserialized when received by :read methods.

If the channel is full, blocks the current thread until the channel isn't full anymore.
If you want to not block the current thread, use `try_send` instead.

Errors if the channel has somehow become disconnected or provided data isn't json-serializable.

</details>

---

### ThreadHandle.try_send

```luau
ThreadHandle.try_send: (self: ThreadHandle, data: JsonSerializableTable | string) -> (boolean, "Sent" | "Disconnected" | "Full"),
```

<details>

<summary> See the docs </summary

Try to send data to the child thread on the regular channel with the same semantics as `ThreadHandle:send`,
except doesn't block if the channel is already full, and doesn't throw an error if the channel is disconnected.

Returns two values: success and result.

- `result == "Disconnected"` means that the channel was disconnected and either the Sender or Receiver no longer exists.
This is usually caused by trying to send a message to a thread that's already been joined or exited.
- `result == "Full"` means that the channel's queue is full and no more new messages can be sent until the other side starts reading from the queue.

</details>

---

### ThreadHandle.sendbytes

```luau
ThreadHandle.sendbytes: (self: ThreadHandle, data: buffer) -> (),
```

Sends a buffer on the bytes channel, blocking the current thread if the channel is full.

Errors if the channel has somehow become disconnected.

---

### ThreadHandle.try_sendbytes

```luau
ThreadHandle.try_sendbytes: (self: ThreadHandle, data: buffer) -> (boolean, "Sent" | "Disconnected" | "Full"),
```

<details>

<summary> See the docs </summary

Try to send data on the bytes channel with the same semantics as `ThreadHandle:sendbytes`,
except doesn't block if the channel is already full, and doesn't throw an error if the channel is disconnected.

Returns two values: success and result.

- `result == "Disconnected"` means that the channel was disconnected and either the Sender or Receiver no longer exists.
This is usually caused by trying to send a message to a thread that's already been joined or exited.
- `result == "Full"` means that the channel's queue is full and no more new messages can be sent until the other side starts reading from the queue.

</details>

---

### ThreadHandle.read

```luau
ThreadHandle.read: (self: ThreadHandle) -> JsonSerializableTable? | string?,
```

Read a message from the regular channel without blocking the current thread.

Errors if the channel has somehow become disconnected.

---

### ThreadHandle.read_await

```luau
ThreadHandle.read_await: (self: ThreadHandle) -> JsonSerializableTable | string,
```

Read a message from the regular channel, blocking until the next message is available.

Errors if the channel has somehow become disconnected.

---

### ThreadHandle.readbytes

```luau
ThreadHandle.readbytes: (self: ThreadHandle) -> buffer?,
```

Read a message from the bytes channel without blocking the current thread.

Errors if the channel has somehow become disconnected.

---

### ThreadHandle.readbytes_await

```luau
ThreadHandle.readbytes_await: (self: ThreadHandle) -> buffer,
```

Read a message from the bytes channel, blocking until the next message is available.

Errors if the channel has somehow become disconnected.

---

### `export type` ThreadSpawnOptions

---

### ThreadSpawnOptions.name

```luau
ThreadSpawnOptions.name: string?,
```

 Name your thread to quickly identify which one it is; if not provided a default alliterative petname will be provided instead.

---

### ThreadSpawnOptions.path

```luau
ThreadSpawnOptions.path: string?,
```

 Path to your source file you want to run in the separate thread, relative to the current file (not cwd).

---

### ThreadSpawnOptions.src

```luau
ThreadSpawnOptions.src: string?,
```

 Source code to evaluate; recommend passing a path instead.

---

### ThreadSpawnOptions.data

```luau
ThreadSpawnOptions.data: JsonSerializableTable?,
```

 Optional data you want to provide to your thread at startup; accessible with `channel.data` in the child thread.

---

### ThreadSpawnOptions.capacity.regular

```luau
ThreadSpawnOptions.capacity.regular: number?,
```

 Override the queue capacity of your thread's regular and bytes channels.
 default is 12

---

### ThreadSpawnOptions.capacity.bytes

```luau
ThreadSpawnOptions.capacity.bytes: number?,
```

 default is 24

---
