<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# time

`local time = require("@std/time")`

$hspace{5pt}$A time library with `time.wait` and functions for creating `Duration` objects.

time.wait: `(seconds: number) -> true`

$hspace{5pt}$ Blocks the current VM for approximately `seconds`, accurate to millisecond-ish precision.
$hspace{5pt}$ Implemented with Rust's `thread::sleep`.

time.datetime: `typeof(require("@self/datetime"))`

$hspace{5pt}$ `DateTime` and `TimeSpan` libraries.

time.years: `(y: number) -> Duration`

$hspace{5pt}$ Constructs a `Duration` representing `y` years. This assumes years are `365` days long.

time.months: `(months: number) -> Duration`

$hspace{5pt}$ Constructs a `Duration` representing `months` months. This assumes months are 365 / 12 days long.
$hspace{5pt}$ <br> For accurate months, use `TimeSpan`s instead.

time.days: `(d: number) -> Duration`

$hspace{5pt}$ Constructs a `Duration` representing `d` days.

time.hours: `(h: number) -> Duration`

$hspace{5pt}$ Constructs a `Duration` representing `h` hours.

time.minutes: `(m: number) -> Duration`

$hspace{5pt}$ Constructs a `Duration` representing `m` minutes.

time.seconds: `(s: number) -> Duration`

$hspace{5pt}$ Constructs a `Duration` representing `s` seconds.

time.milliseconds: `(ms: number) -> Duration`

$hspace{5pt}$ Constructs a `Duration` representing `ms` milliseconds.

time.microseconds: `(us: number) -> Duration`

$hspace{5pt}$ Constructs a `Duration` representing `us` microseconds.

time.nanoseconds: `(n: number) -> Duration`

$hspace{5pt}$ Constructs a `Duration` representing `n` nanoseconds. This is accurate only up to ~285 years (expressed in nanoseconds).
