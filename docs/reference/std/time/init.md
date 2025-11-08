<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# time

`local time = require("@std/time")`

A time library with `time.wait` and functions for creating `Duration` objects.

---

time.wait: `(seconds: number) -> true`

 Blocks the current VM for approximately `seconds`, accurate to millisecond-ish precision.
 Implemented with Rust's `thread::sleep`.

---

time.datetime: `typeof(require("@self/datetime"))`

 `DateTime` and `TimeSpan` libraries.

---

time.years: `(y: number) -> Duration`

 Constructs a `Duration` representing `y` years. This assumes years are `365` days long.

---

time.months: `(months: number) -> Duration`

 Constructs a `Duration` representing `months` months. This assumes months are 365 / 12 days long.
 <br> For accurate months, use `TimeSpan`s instead.

---

time.days: `(d: number) -> Duration`

 Constructs a `Duration` representing `d` days.

---

time.hours: `(h: number) -> Duration`

 Constructs a `Duration` representing `h` hours.

---

time.minutes: `(m: number) -> Duration`

 Constructs a `Duration` representing `m` minutes.

---

time.seconds: `(s: number) -> Duration`

 Constructs a `Duration` representing `s` seconds.

---

time.milliseconds: `(ms: number) -> Duration`

 Constructs a `Duration` representing `ms` milliseconds.

---

time.microseconds: `(us: number) -> Duration`

 Constructs a `Duration` representing `us` microseconds.

---

time.nanoseconds: `(n: number) -> Duration`

 Constructs a `Duration` representing `n` nanoseconds. This is accurate only up to ~285 years (expressed in nanoseconds).

---
