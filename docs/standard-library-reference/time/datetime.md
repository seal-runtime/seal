<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# Datetime

`function datetime.now(): DateTime`

 Constructs a `DateTime` from right now (based on system time) in your local timezone.

`function datetime.from(timestamp: number, timezone: IanaTimezone?, nanos: number?)`

 Constructs a `DateTime` from a Unix Timestamp and an `IanaTimezone`.

- `timezone` defaults to `"UTC"` if not specified
- `nanos` defaults to `0` if not specified

`function datetime.parse(source: string, format: string | CommonFormatKeys, timezone: IanaTimezone): DateTime`

<details>

<summary> See the docs </summary

Constructs a `DateTime` from a strtime string or `common_format`.

All seal `DateTime`s are timezone aware, which prevents annoying and complex bugs down the line.

- Specify `"AUTO"` as your `timezone` if your `format` string is timezone-aware by default (RFC 3339, has `%Q` or `%z/z/%Z` specifiers, etc.)
- Specify `"UTC"` as your `timezone` if you want to treat `source` as UTC.

## Usage

```luau
    -- Parse a simple ISO 8601 timestamp that we're sure came from US/CST time.
local dt = datetime.parse("2025-01-02 05:00", "ISO_8601", "US/Central")
-- Parse an RFC 3339 timestamp that already has timezone offset info in it.
local brazilian_dt = datetime.parse("2025-08-24T21:48:20-00:00", "RFC_3339", "AUTO")
```

</details>

`function datetime.years(years: number, relative_to: DateTime?): TimeSpan`

<details>

<summary> See the docs </summary

Constructs a `TimeSpan` from years. Months and years *must* be relative to a `DateTime` do any `TimeSpan` arithmetic!

## how relatives work

```luau
local now = datetime.now()

-- you don't need to specify relative here, because we know it's relative to `now`
local next_year = now + datetime.years(1)

-- you don't need to specify relative here, because operands execute left to right
local next_year = now + datetime.years(1) + datetime.months(2)

-- you NEED to specify relative here because we don't know what it's relative to
local span = datetime.years(1, datetime.now()) + datetime.seconds(10)

-- only one of the operands needs to have a relative date (and it's inherited by the resulting `TimeSpan`)
local span2 = datetime.years(1) + datetime.months(1, datetime.now())
assert(span2.relative_to ~= nil, "should have relative DateTime")
```

</details>

`function datetime.months(months: number, relative_to: DateTime?): TimeSpan`

<details>

<summary> See the docs </summary

Constructs a `TimeSpan` from months. Months and years *must* be relative to a `DateTime` do any `TimeSpan` arithmetic!

## how relatives work

```luau
local now = datetime.now()

-- you don't need to specify relative here, because we know it's relative to `now`
local next_year = now + datetime.years(1)

-- you don't need to specify relative here, because operands execute left to right
local next_year = now + datetime.years(1) + datetime.months(2)

-- you NEED to specify relative here because we don't know what it's relative to
local span = datetime.years(1, datetime.now()) + datetime.seconds(10)

-- only one of the operands needs to have a relative date (and it's inherited by the resulting `TimeSpan`)
local span2 = datetime.years(1) + datetime.months(1, datetime.now())
assert(span2.relative_to ~= nil, "should have relative DateTime")
```

</details>

`function datetime.days(days: number): TimeSpan`

 Constructs a `TimeSpan` from days. Assumes every day is 24 hours.

`function datetime.hours(hours: number): TimeSpan`

 Constructs a `TimeSpan` from hours.

`function datetime.minutes(minutes: number): TimeSpan`

 Constructs a `TimeSpan` from minutes.

`function datetime.seconds(seconds: number): TimeSpan`

 Constructs a `TimeSpan` from seconds.

`function datetime.milliseconds(milliseconds: number): TimeSpan`

 Constructs a `TimeSpan` from milliseconds.
