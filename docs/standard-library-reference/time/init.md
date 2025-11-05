<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# Time

A time library with `time.wait` and functions for creating `Duration` objects.

`function time.wait(seconds: number): true`

 Blocks the current VM for approximately `seconds`, accurate to millisecond-ish precision.
 Implemented with Rust's `thread::sleep`.

`datetime: typeof(require("@self/datetime"))`

 `DateTime` and `TimeSpan` libraries.

`function time.years(y: number): Duration`

 Constructs a `Duration` representing `y` years. This assumes years are `365` days long.

`function time.months(months: number): Duration`

 Constructs a `Duration` representing `months` months. This assumes months are 365 / 12 days long.
 <br> For accurate months, use `TimeSpan`s instead.

`function time.days(d: number): Duration`

 Constructs a `Duration` representing `d` days.

`function time.hours(h: number): Duration`

 Constructs a `Duration` representing `h` hours.

`function time.minutes(m: number): Duration`

 Constructs a `Duration` representing `m` minutes.

`function time.seconds(s: number): Duration`

 Constructs a `Duration` representing `s` seconds.

`function time.milliseconds(ms: number): Duration`

 Constructs a `Duration` representing `ms` milliseconds.

`function time.microseconds(us: number): Duration`

 Constructs a `Duration` representing `us` microseconds.

`function time.nanoseconds(n: number): Duration`

 Constructs a `Duration` representing `n` nanoseconds. This is accurate only up to ~285 years (expressed in nanoseconds).
