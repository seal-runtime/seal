<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# time.datetime

`local datetime = require("@std/time/datetime")`

.RFC_2822 = "%a, %d %b %Y %H: `%M:%S %z" :: "%a, %d %b %Y %H:%M:%S %z"`

.RFC_3339 = "%Y-%m-%dT%H: `%M:%S%:z" :: "%Y-%m-%dT%H:%M:%S%:z"`

.SHORT_TIME = "%H: `%M" :: "%H:%M"`

.FULL_DATE_TIME = "%A, %B %d, %Y %H: `%M:%S" :: "%A, %B %d, %Y %H:%M:%S"`

.LOGGING_24_HR = "%a %b %e %H: `%M:%S %Z %Y" :: "%a %b %e %H:%M:%S %Z %Y"`

.LOGGING_12_HR = "%a %b %e %I: `%M:%S %p %Z %Y" :: "%a %b %e %I:%M:%S %p %Z %Y"`

.["MM/DD/YYYY HH: `MM (AM/PM)"] = "%m/%d/%Y %I:%M %p" :: "%m/%d/%Y %I:%M %p"`

.["HH: `MM (AM/PM)"] = "%I:%M %p" :: "%I:%M %p"`

.AMERICAN_FULL_DATE_TIME = "%A, %B %d, %Y %I: `%M:%S %p" :: "%A, %B %d, %Y %I:%M:%S %p"`

.function datetime.from(timestamp: `number, timezone: IanaTimezone?, nanos: number?)`

$hspace{5pt}$ Constructs a `DateTime` from right now (based on system time) in your local timezone.
$hspace{5pt}$ Constructs a `DateTime` from a Unix Timestamp and an `IanaTimezone`.
$hspace{5pt}$
$hspace{5pt}$ - `timezone` defaults to `"UTC"` if not specified
$hspace{5pt}$ - `nanos` defaults to `0` if not specified

`export type` CommonFormatKeys

CommonFormatKeys.function datetime.parse(source: `string, format: string | CommonFormatKeys, timezone: IanaTimezone): DateTime`

<details>

<summary> See the docs </summary

$hspace{5pt}$Constructs a `DateTime` from a strtime string or `common_format`.
$hspace{5pt}$
$hspace{5pt}$All seal `DateTime`s are timezone aware, which prevents annoying and complex bugs down the line.
$hspace{5pt}$
$hspace{5pt}$- Specify `"AUTO"` as your `timezone` if your `format` string is timezone-aware by default (RFC 3339, has `%Q` or `%z/z/%Z` specifiers, etc.)
$hspace{5pt}$- Specify `"UTC"` as your `timezone` if you want to treat `source` as UTC.
$hspace{5pt}$
$hspace{5pt}$## Usage
$hspace{5pt}$
$hspace{5pt}$```luau
$hspace{5pt}$    -- Parse a simple ISO 8601 timestamp that we're sure came from US/CST time.
$hspace{5pt}$    local dt = datetime.parse("2025-01-02 05:00", "ISO_8601", "US/Central")
$hspace{5pt}$    -- Parse an RFC 3339 timestamp that already has timezone offset info in it.
$hspace{5pt}$    local brazilian_dt = datetime.parse("2025-08-24T21:48:20-00:00", "RFC_3339", "AUTO")
$hspace{5pt}$```

</details>

CommonFormatKeys.function datetime.years(years: `number, relative_to: DateTime?): TimeSpan`

<details>

<summary> See the docs </summary

$hspace{5pt}$Constructs a `TimeSpan` from years. Months and years *must* be relative to a `DateTime` do any `TimeSpan` arithmetic!
$hspace{5pt}$
$hspace{5pt}$## how relatives work
$hspace{5pt}$```luau
$hspace{5pt}$local now = datetime.now()
$hspace{5pt}$
$hspace{5pt}$-- you don't need to specify relative here, because we know it's relative to `now`
$hspace{5pt}$local next_year = now + datetime.years(1)
$hspace{5pt}$
$hspace{5pt}$-- you don't need to specify relative here, because operands execute left to right
$hspace{5pt}$local next_year = now + datetime.years(1) + datetime.months(2)
$hspace{5pt}$
$hspace{5pt}$-- you NEED to specify relative here because we don't know what it's relative to
$hspace{5pt}$local span = datetime.years(1, datetime.now()) + datetime.seconds(10)
$hspace{5pt}$
$hspace{5pt}$-- only one of the operands needs to have a relative date (and it's inherited by the resulting `TimeSpan`)
$hspace{5pt}$local span2 = datetime.years(1) + datetime.months(1, datetime.now())
$hspace{5pt}$assert(span2.relative_to ~= nil, "should have relative DateTime")
$hspace{5pt}$```

</details>

CommonFormatKeys.function datetime.months(months: `number, relative_to: DateTime?): TimeSpan`

<details>

<summary> See the docs </summary

$hspace{5pt}$Constructs a `TimeSpan` from months. Months and years *must* be relative to a `DateTime` do any `TimeSpan` arithmetic!
$hspace{5pt}$
$hspace{5pt}$## how relatives work
$hspace{5pt}$```luau
$hspace{5pt}$local now = datetime.now()
$hspace{5pt}$
$hspace{5pt}$-- you don't need to specify relative here, because we know it's relative to `now`
$hspace{5pt}$local next_year = now + datetime.years(1)
$hspace{5pt}$
$hspace{5pt}$-- you don't need to specify relative here, because operands execute left to right
$hspace{5pt}$local next_year = now + datetime.years(1) + datetime.months(2)
$hspace{5pt}$
$hspace{5pt}$-- you NEED to specify relative here because we don't know what it's relative to
$hspace{5pt}$local span = datetime.years(1, datetime.now()) + datetime.seconds(10)
$hspace{5pt}$
$hspace{5pt}$-- only one of the operands needs to have a relative date (and it's inherited by the resulting `TimeSpan`)
$hspace{5pt}$local span2 = datetime.years(1) + datetime.months(1, datetime.now())
$hspace{5pt}$assert(span2.relative_to ~= nil, "should have relative DateTime")
$hspace{5pt}$```

</details>

CommonFormatKeys.function datetime.days(days: `number): TimeSpan`

$hspace{5pt}$ Constructs a `TimeSpan` from days. Assumes every day is 24 hours.

CommonFormatKeys.function datetime.hours(hours: `number): TimeSpan`

$hspace{5pt}$ Constructs a `TimeSpan` from hours.

CommonFormatKeys.function datetime.minutes(minutes: `number): TimeSpan`

$hspace{5pt}$ Constructs a `TimeSpan` from minutes.

CommonFormatKeys.function datetime.seconds(seconds: `number): TimeSpan`

$hspace{5pt}$ Constructs a `TimeSpan` from seconds.

CommonFormatKeys.function datetime.milliseconds(milliseconds: `number): TimeSpan`

$hspace{5pt}$ Constructs a `TimeSpan` from milliseconds.

`export type` IanaTimezone
