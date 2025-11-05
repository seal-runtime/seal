<!-- markdownlint-disable MD033 -->

# Time

`type time = {`

> A time library with `time.wait` and functions for creating `Duration` objects.

`wait: (seconds: number) -> true`

> --- Blocks the current VM for approximately `seconds`, accurate to millisecond-ish precision.

> --- Implemented with Rust's `thread::sleep`.

`datetime: typeof(require("@self/datetime"))`

> --- `DateTime` and `TimeSpan` libraries.

`years: (y: number) -> Duration`

> --- Constructs a `Duration` representing `y` years. This assumes years are `365` days long.

`months: (months: number) -> Duration`

> --- Constructs a `Duration` representing `months` months. This assumes months are 365 / 12 days long.

> --- <br> For accurate months, use `TimeSpan`s instead.

`days: (d: number) -> Duration`

> --- Constructs a `Duration` representing `d` days.

`hours: (h: number) -> Duration`

> --- Constructs a `Duration` representing `h` hours.

`minutes: (m: number) -> Duration`

> --- Constructs a `Duration` representing `m` minutes.

`seconds: (s: number) -> Duration`

> --- Constructs a `Duration` representing `s` seconds.

`milliseconds: (ms: number) -> Duration`

> --- Constructs a `Duration` representing `ms` milliseconds.

`microseconds: (us: number) -> Duration`

> --- Constructs a `Duration` representing `us` microseconds.

`nanoseconds: (n: number) -> Duration`

> --- Constructs a `Duration` representing `n` nanoseconds. This is accurate only up to ~285 years (expressed in nanoseconds).
