<!-- markdownlint-disable MD033 -->

# Time

`type time = {`

[=[
A time library with `time.wait` and functions for creating `Duration` objects.

`wait: (seconds: number) -> true`

`datetime: typeof(require("@self/datetime"))`

`years: (y: number) -> Duration`

`months: (months: number) -> Duration`

`days: (d: number) -> Duration`

`hours: (h: number) -> Duration`

`minutes: (m: number) -> Duration`

`seconds: (s: number) -> Duration`

`milliseconds: (ms: number) -> Duration`

`microseconds: (us: number) -> Duration`

`nanoseconds: (n: number) -> Duration`
