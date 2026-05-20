--[=[
    A time library with `time.wait` and functions for creating `Duration` objects.
]=]
export type time = {
    --[=[
        Blocks the current VM for at least `duration` (which may be either a `Duration` instance
        or a `number` in seconds).

        This function should be accurate to around 1ms precision on most platforms,
        and may be more accurate if your platform supports nanosecond precision.

        ## Platform specific behavior

        See the underlying Rust's `std::thread::sleep` for detailed info on platform specific behavior.

        ### Linux, macOS, and Unix-like
        - The system call may be invoked multiple times to ensure the system sleeps for `duration`
        - A zero `duration` will not result in a `nanosleep` syscall

        ### Windows
        - Always invokes the `Sleep` system call even when a zero `Duration` is provided.

        If you need a more accurate way to wait you might need to spin (while true do loop).
    ]=]
    wait: (duration: Duration | number) -> true,

    --- `DateTime` and `TimeSpan` libraries.
    datetime: typeof(require("@self/datetime")),

    --- Constructs a `Duration` representing `y` years. This assumes years are `365` days long.
    years: (y: number) -> Duration,

    --- Constructs a `Duration` representing `months` months. This assumes months are 365 / 12 days long.
    --- <br> For accurate months, use `TimeSpan`s instead.
    months: (months: number) -> Duration,

    --- Constructs a `Duration` representing `d` days.
    days: (d: number) -> Duration,

    --- Constructs a `Duration` representing `h` hours.
    hours: (h: number) -> Duration,

    --- Constructs a `Duration` representing `m` minutes.
    minutes: (m: number) -> Duration,

    --- Constructs a `Duration` representing `s` seconds.
    seconds: (s: number) -> Duration,

    --- Constructs a `Duration` representing `ms` milliseconds.
    milliseconds: (ms: number) -> Duration,

    --- Constructs a `Duration` representing `us` microseconds.
    microseconds: (us: number) -> Duration,

    --- Constructs a `Duration` representing `n` nanoseconds. This is accurate only up to ~285 years (expressed in nanoseconds).
    nanoseconds: (n: number) -> Duration,
}

return {} :: time