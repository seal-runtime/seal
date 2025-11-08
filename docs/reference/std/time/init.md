<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# time

`local time = require("@std/time")`

A time library with `time.wait` and functions for creating `Duration` objects.

---

### time.wait

<h4>

```luau
function (seconds: number) -> true,
```

</h4>

 Blocks the current VM for approximately `seconds`, accurate to millisecond-ish precision.
 Implemented with Rust's `thread::sleep`.

---

### time.datetime

<h4>

```luau
datetime: typeof(require("@self/datetime")),
```

</h4>

 `DateTime` and `TimeSpan` libraries.

---

### time.years

<h4>

```luau
function (y: number) -> Duration,
```

</h4>

 Constructs a `Duration` representing `y` years. This assumes years are `365` days long.

---

### time.months

<h4>

```luau
function (months: number) -> Duration,
```

</h4>

 Constructs a `Duration` representing `months` months. This assumes months are 365 / 12 days long.
 <br> For accurate months, use `TimeSpan`s instead.

---

### time.days

<h4>

```luau
function (d: number) -> Duration,
```

</h4>

 Constructs a `Duration` representing `d` days.

---

### time.hours

<h4>

```luau
function (h: number) -> Duration,
```

</h4>

 Constructs a `Duration` representing `h` hours.

---

### time.minutes

<h4>

```luau
function (m: number) -> Duration,
```

</h4>

 Constructs a `Duration` representing `m` minutes.

---

### time.seconds

<h4>

```luau
function (s: number) -> Duration,
```

</h4>

 Constructs a `Duration` representing `s` seconds.

---

### time.milliseconds

<h4>

```luau
function (ms: number) -> Duration,
```

</h4>

 Constructs a `Duration` representing `ms` milliseconds.

---

### time.microseconds

<h4>

```luau
function (us: number) -> Duration,
```

</h4>

 Constructs a `Duration` representing `us` microseconds.

---

### time.nanoseconds

<h4>

```luau
function (n: number) -> Duration,
```

</h4>

 Constructs a `Duration` representing `n` nanoseconds. This is accurate only up to ~285 years (expressed in nanoseconds).

---
