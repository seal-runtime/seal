<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# time

`local time = require("@std/time")`

A time library with `time.wait` and functions for creating `Duration` objects.

---

<h3>
```luau
time.wait: (seconds: number) -> true,
```
</h3>

 Blocks the current VM for approximately `seconds`, accurate to millisecond-ish precision.
 Implemented with Rust's `thread::sleep`.

---

<h3>
```luau
time.datetime: typeof(require("@self/datetime")),
```
</h3>

 `DateTime` and `TimeSpan` libraries.

---

<h3>
```luau
time.years: (y: number) -> Duration,
```
</h3>

 Constructs a `Duration` representing `y` years. This assumes years are `365` days long.

---

<h3>
```luau
time.months: (months: number) -> Duration,
```
</h3>

 Constructs a `Duration` representing `months` months. This assumes months are 365 / 12 days long.
 <br> For accurate months, use `TimeSpan`s instead.

---

<h3>
```luau
time.days: (d: number) -> Duration,
```
</h3>

 Constructs a `Duration` representing `d` days.

---

<h3>
```luau
time.hours: (h: number) -> Duration,
```
</h3>

 Constructs a `Duration` representing `h` hours.

---

<h3>
```luau
time.minutes: (m: number) -> Duration,
```
</h3>

 Constructs a `Duration` representing `m` minutes.

---

<h3>
```luau
time.seconds: (s: number) -> Duration,
```
</h3>

 Constructs a `Duration` representing `s` seconds.

---

<h3>
```luau
time.milliseconds: (ms: number) -> Duration,
```
</h3>

 Constructs a `Duration` representing `ms` milliseconds.

---

<h3>
```luau
time.microseconds: (us: number) -> Duration,
```
</h3>

 Constructs a `Duration` representing `us` microseconds.

---

<h3>
```luau
time.nanoseconds: (n: number) -> Duration,
```
</h3>

 Constructs a `Duration` representing `n` nanoseconds. This is accurate only up to ~285 years (expressed in nanoseconds).

---
