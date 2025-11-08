<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# time.datetime

`local datetime = require("@std/time/datetime")`

 Constructs a `DateTime` from right now (based on system time) in your local timezone.
 Constructs a `DateTime` from a Unix Timestamp and an `IanaTimezone`.

- `timezone` defaults to `"UTC"` if not specified
- `nanos` defaults to `0` if not specified

---

### `export type` CommonFormatKeys

```luau

```

---

### CommonFormatKeys.source

```luau
`function` CommonFormatKeys.sourcedatetime.parse: (source: string, format: string | CommonFormatKeys, timezone: IanaTimezone) -> DateTime
```

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

---

### CommonFormatKeys.years

```luau
`function` CommonFormatKeys.yearsdatetime.years: (years: number, relative_to: DateTime?) -> TimeSpan
```

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

---

### CommonFormatKeys.months

```luau
`function` CommonFormatKeys.monthsdatetime.months: (months: number, relative_to: DateTime?) -> TimeSpan
```

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

---

### CommonFormatKeys.days

```luau
`function` CommonFormatKeys.daysdatetime.days: (days: number) -> TimeSpan
```

 Constructs a `TimeSpan` from days. Assumes every day is 24 hours.

---

### CommonFormatKeys.hours

```luau
`function` CommonFormatKeys.hoursdatetime.hours: (hours: number) -> TimeSpan
```

 Constructs a `TimeSpan` from hours.

---

### CommonFormatKeys.minutes

```luau
`function` CommonFormatKeys.minutesdatetime.minutes: (minutes: number) -> TimeSpan
```

 Constructs a `TimeSpan` from minutes.

---

### CommonFormatKeys.seconds

```luau
`function` CommonFormatKeys.secondsdatetime.seconds: (seconds: number) -> TimeSpan
```

 Constructs a `TimeSpan` from seconds.

---

### CommonFormatKeys.milliseconds

```luau
`function` CommonFormatKeys.millisecondsdatetime.milliseconds: (milliseconds: number) -> TimeSpan
```

 Constructs a `TimeSpan` from milliseconds.

---

### `export type` IanaTimezone

```luau

```

---

### IanaTimezone

```luau
| "AUTO" -- timezone info already included in input data
```

---

### IanaTimezone

```luau
| "Africa/Abidjan"
```

---

### IanaTimezone

```luau
| "Africa/Accra"
```

---

### IanaTimezone

```luau
| "Africa/Addis_Ababa"
```

---

### IanaTimezone

```luau
| "Africa/Algiers"
```

---

### IanaTimezone

```luau
| "Africa/Asmara"
```

---

### IanaTimezone

```luau
| "Africa/Asmera"
```

---

### IanaTimezone

```luau
| "Africa/Bamako"
```

---

### IanaTimezone

```luau
| "Africa/Bangui"
```

---

### IanaTimezone

```luau
| "Africa/Banjul"
```

---

### IanaTimezone

```luau
| "Africa/Bissau"
```

---

### IanaTimezone

```luau
| "Africa/Blantyre"
```

---

### IanaTimezone

```luau
| "Africa/Brazzaville"
```

---

### IanaTimezone

```luau
| "Africa/Bujumbura"
```

---

### IanaTimezone

```luau
| "Africa/Cairo"
```

---

### IanaTimezone

```luau
| "Africa/Casablanca"
```

---

### IanaTimezone

```luau
| "Africa/Ceuta"
```

---

### IanaTimezone

```luau
| "Africa/Conakry"
```

---

### IanaTimezone

```luau
| "Africa/Dakar"
```

---

### IanaTimezone

```luau
| "Africa/Dar_es_Salaam"
```

---

### IanaTimezone

```luau
| "Africa/Djibouti"
```

---

### IanaTimezone

```luau
| "Africa/Douala"
```

---

### IanaTimezone

```luau
| "Africa/El_Aaiun"
```

---

### IanaTimezone

```luau
| "Africa/Freetown"
```

---

### IanaTimezone

```luau
| "Africa/Gaborone"
```

---

### IanaTimezone

```luau
| "Africa/Harare"
```

---

### IanaTimezone

```luau
| "Africa/Johannesburg"
```

---

### IanaTimezone

```luau
| "Africa/Juba"
```

---

### IanaTimezone

```luau
| "Africa/Kampala"
```

---

### IanaTimezone

```luau
| "Africa/Khartoum"
```

---

### IanaTimezone

```luau
| "Africa/Kigali"
```

---

### IanaTimezone

```luau
| "Africa/Kinshasa"
```

---

### IanaTimezone

```luau
| "Africa/Lagos"
```

---

### IanaTimezone

```luau
| "Africa/Libreville"
```

---

### IanaTimezone

```luau
| "Africa/Lome"
```

---

### IanaTimezone

```luau
| "Africa/Luanda"
```

---

### IanaTimezone

```luau
| "Africa/Lubumbashi"
```

---

### IanaTimezone

```luau
| "Africa/Lusaka"
```

---

### IanaTimezone

```luau
| "Africa/Malabo"
```

---

### IanaTimezone

```luau
| "Africa/Maputo"
```

---

### IanaTimezone

```luau
| "Africa/Maseru"
```

---

### IanaTimezone

```luau
| "Africa/Mbabane"
```

---

### IanaTimezone

```luau
| "Africa/Mogadishu"
```

---

### IanaTimezone

```luau
| "Africa/Monrovia"
```

---

### IanaTimezone

```luau
| "Africa/Nairobi"
```

---

### IanaTimezone

```luau
| "Africa/Ndjamena"
```

---

### IanaTimezone

```luau
| "Africa/Niamey"
```

---

### IanaTimezone

```luau
| "Africa/Nouakchott"
```

---

### IanaTimezone

```luau
| "Africa/Ouagadougou"
```

---

### IanaTimezone

```luau
| "Africa/Porto-Novo"
```

---

### IanaTimezone

```luau
| "Africa/Sao_Tome"
```

---

### IanaTimezone

```luau
| "Africa/Timbuktu"
```

---

### IanaTimezone

```luau
| "Africa/Tripoli"
```

---

### IanaTimezone

```luau
| "Africa/Tunis"
```

---

### IanaTimezone

```luau
| "Africa/Windhoek"
```

---

### IanaTimezone

```luau
| "America/Adak"
```

---

### IanaTimezone

```luau
| "America/Anchorage"
```

---

### IanaTimezone

```luau
| "America/Anguilla"
```

---

### IanaTimezone

```luau
| "America/Antigua"
```

---

### IanaTimezone

```luau
| "America/Araguaina"
```

---

### IanaTimezone

```luau
| "America/Argentina/Buenos_Aires"
```

---

### IanaTimezone

```luau
| "America/Argentina/Catamarca"
```

---

### IanaTimezone

```luau
| "America/Argentina/ComodRivadavia"
```

---

### IanaTimezone

```luau
| "America/Argentina/Cordoba"
```

---

### IanaTimezone

```luau
| "America/Argentina/Jujuy"
```

---

### IanaTimezone

```luau
| "America/Argentina/La_Rioja"
```

---

### IanaTimezone

```luau
| "America/Argentina/Mendoza"
```

---

### IanaTimezone

```luau
| "America/Argentina/Rio_Gallegos"
```

---

### IanaTimezone

```luau
| "America/Argentina/Salta"
```

---

### IanaTimezone

```luau
| "America/Argentina/San_Juan"
```

---

### IanaTimezone

```luau
| "America/Argentina/San_Luis"
```

---

### IanaTimezone

```luau
| "America/Argentina/Tucuman"
```

---

### IanaTimezone

```luau
| "America/Argentina/Ushuaia"
```

---

### IanaTimezone

```luau
| "America/Aruba"
```

---

### IanaTimezone

```luau
| "America/Asuncion"
```

---

### IanaTimezone

```luau
| "America/Atikokan"
```

---

### IanaTimezone

```luau
| "America/Atka"
```

---

### IanaTimezone

```luau
| "America/Bahia"
```

---

### IanaTimezone

```luau
| "America/Bahia_Banderas"
```

---

### IanaTimezone

```luau
| "America/Barbados"
```

---

### IanaTimezone

```luau
| "America/Belem"
```

---

### IanaTimezone

```luau
| "America/Belize"
```

---

### IanaTimezone

```luau
| "America/Blanc-Sablon"
```

---

### IanaTimezone

```luau
| "America/Boa_Vista"
```

---

### IanaTimezone

```luau
| "America/Bogota"
```

---

### IanaTimezone

```luau
| "America/Boise"
```

---

### IanaTimezone

```luau
| "America/Buenos_Aires"
```

---

### IanaTimezone

```luau
| "America/Cambridge_Bay"
```

---

### IanaTimezone

```luau
| "America/Campo_Grande"
```

---

### IanaTimezone

```luau
| "America/Cancun"
```

---

### IanaTimezone

```luau
| "America/Caracas"
```

---

### IanaTimezone

```luau
| "America/Catamarca"
```

---

### IanaTimezone

```luau
| "America/Cayenne"
```

---

### IanaTimezone

```luau
| "America/Cayman"
```

---

### IanaTimezone

```luau
| "America/Chicago"
```

---

### IanaTimezone

```luau
| "America/Chihuahua"
```

---

### IanaTimezone

```luau
| "America/Ciudad_Juarez"
```

---

### IanaTimezone

```luau
| "America/Coral_Harbour"
```

---

### IanaTimezone

```luau
| "America/Cordoba"
```

---

### IanaTimezone

```luau
| "America/Costa_Rica"
```

---

### IanaTimezone

```luau
| "America/Creston"
```

---

### IanaTimezone

```luau
| "America/Cuiaba"
```

---

### IanaTimezone

```luau
| "America/Curacao"
```

---

### IanaTimezone

```luau
| "America/Danmarkshavn"
```

---

### IanaTimezone

```luau
| "America/Dawson"
```

---

### IanaTimezone

```luau
| "America/Dawson_Creek"
```

---

### IanaTimezone

```luau
| "America/Denver"
```

---

### IanaTimezone

```luau
| "America/Detroit"
```

---

### IanaTimezone

```luau
| "America/Dominica"
```

---

### IanaTimezone

```luau
| "America/Edmonton"
```

---

### IanaTimezone

```luau
| "America/Eirunepe"
```

---

### IanaTimezone

```luau
| "America/El_Salvador"
```

---

### IanaTimezone

```luau
| "America/Ensenada"
```

---

### IanaTimezone

```luau
| "America/Fort_Nelson"
```

---

### IanaTimezone

```luau
| "America/Fort_Wayne"
```

---

### IanaTimezone

```luau
| "America/Fortaleza"
```

---

### IanaTimezone

```luau
| "America/Glace_Bay"
```

---

### IanaTimezone

```luau
| "America/Godthab"
```

---

### IanaTimezone

```luau
| "America/Goose_Bay"
```

---

### IanaTimezone

```luau
| "America/Grand_Turk"
```

---

### IanaTimezone

```luau
| "America/Grenada"
```

---

### IanaTimezone

```luau
| "America/Guadeloupe"
```

---

### IanaTimezone

```luau
| "America/Guatemala"
```

---

### IanaTimezone

```luau
| "America/Guayaquil"
```

---

### IanaTimezone

```luau
| "America/Guyana"
```

---

### IanaTimezone

```luau
| "America/Halifax"
```

---

### IanaTimezone

```luau
| "America/Havana"
```

---

### IanaTimezone

```luau
| "America/Hermosillo"
```

---

### IanaTimezone

```luau
| "America/Indiana/Indianapolis"
```

---

### IanaTimezone

```luau
| "America/Indiana/Knox"
```

---

### IanaTimezone

```luau
| "America/Indiana/Marengo"
```

---

### IanaTimezone

```luau
| "America/Indiana/Petersburg"
```

---

### IanaTimezone

```luau
| "America/Indiana/Tell_City"
```

---

### IanaTimezone

```luau
| "America/Indiana/Vevay"
```

---

### IanaTimezone

```luau
| "America/Indiana/Vincennes"
```

---

### IanaTimezone

```luau
| "America/Indiana/Winamac"
```

---

### IanaTimezone

```luau
| "America/Indianapolis"
```

---

### IanaTimezone

```luau
| "America/Inuvik"
```

---

### IanaTimezone

```luau
| "America/Iqaluit"
```

---

### IanaTimezone

```luau
| "America/Jamaica"
```

---

### IanaTimezone

```luau
| "America/Jujuy"
```

---

### IanaTimezone

```luau
| "America/Juneau"
```

---

### IanaTimezone

```luau
| "America/Kentucky/Louisville"
```

---

### IanaTimezone

```luau
| "America/Kentucky/Monticello"
```

---

### IanaTimezone

```luau
| "America/Knox_IN"
```

---

### IanaTimezone

```luau
| "America/Kralendijk"
```

---

### IanaTimezone

```luau
| "America/La_Paz"
```

---

### IanaTimezone

```luau
| "America/Lima"
```

---

### IanaTimezone

```luau
| "America/Los_Angeles"
```

---

### IanaTimezone

```luau
| "America/Louisville"
```

---

### IanaTimezone

```luau
| "America/Lower_Princes"
```

---

### IanaTimezone

```luau
| "America/Maceio"
```

---

### IanaTimezone

```luau
| "America/Managua"
```

---

### IanaTimezone

```luau
| "America/Manaus"
```

---

### IanaTimezone

```luau
| "America/Marigot"
```

---

### IanaTimezone

```luau
| "America/Martinique"
```

---

### IanaTimezone

```luau
| "America/Matamoros"
```

---

### IanaTimezone

```luau
| "America/Mazatlan"
```

---

### IanaTimezone

```luau
| "America/Mendoza"
```

---

### IanaTimezone

```luau
| "America/Menominee"
```

---

### IanaTimezone

```luau
| "America/Merida"
```

---

### IanaTimezone

```luau
| "America/Metlakatla"
```

---

### IanaTimezone

```luau
| "America/Mexico_City"
```

---

### IanaTimezone

```luau
| "America/Miquelon"
```

---

### IanaTimezone

```luau
| "America/Moncton"
```

---

### IanaTimezone

```luau
| "America/Monterrey"
```

---

### IanaTimezone

```luau
| "America/Montevideo"
```

---

### IanaTimezone

```luau
| "America/Montreal"
```

---

### IanaTimezone

```luau
| "America/Montserrat"
```

---

### IanaTimezone

```luau
| "America/Nassau"
```

---

### IanaTimezone

```luau
| "America/New_York"
```

---

### IanaTimezone

```luau
| "America/Nipigon"
```

---

### IanaTimezone

```luau
| "America/Nome"
```

---

### IanaTimezone

```luau
| "America/Noronha"
```

---

### IanaTimezone

```luau
| "America/North_Dakota/Beulah"
```

---

### IanaTimezone

```luau
| "America/North_Dakota/Center"
```

---

### IanaTimezone

```luau
| "America/North_Dakota/New_Salem"
```

---

### IanaTimezone

```luau
| "America/Nuuk"
```

---

### IanaTimezone

```luau
| "America/Ojinaga"
```

---

### IanaTimezone

```luau
| "America/Panama"
```

---

### IanaTimezone

```luau
| "America/Pangnirtung"
```

---

### IanaTimezone

```luau
| "America/Paramaribo"
```

---

### IanaTimezone

```luau
| "America/Phoenix"
```

---

### IanaTimezone

```luau
| "America/Port-au-Prince"
```

---

### IanaTimezone

```luau
| "America/Port_of_Spain"
```

---

### IanaTimezone

```luau
| "America/Porto_Acre"
```

---

### IanaTimezone

```luau
| "America/Porto_Velho"
```

---

### IanaTimezone

```luau
| "America/Puerto_Rico"
```

---

### IanaTimezone

```luau
| "America/Punta_Arenas"
```

---

### IanaTimezone

```luau
| "America/Rainy_River"
```

---

### IanaTimezone

```luau
| "America/Rankin_Inlet"
```

---

### IanaTimezone

```luau
| "America/Recife"
```

---

### IanaTimezone

```luau
| "America/Regina"
```

---

### IanaTimezone

```luau
| "America/Resolute"
```

---

### IanaTimezone

```luau
| "America/Rio_Branco"
```

---

### IanaTimezone

```luau
| "America/Rosario"
```

---

### IanaTimezone

```luau
| "America/Santa_Isabel"
```

---

### IanaTimezone

```luau
| "America/Santarem"
```

---

### IanaTimezone

```luau
| "America/Santiago"
```

---

### IanaTimezone

```luau
| "America/Santo_Domingo"
```

---

### IanaTimezone

```luau
| "America/Sao_Paulo"
```

---

### IanaTimezone

```luau
| "America/Scoresbysund"
```

---

### IanaTimezone

```luau
| "America/Shiprock"
```

---

### IanaTimezone

```luau
| "America/Sitka"
```

---

### IanaTimezone

```luau
| "America/St_Barthelemy"
```

---

### IanaTimezone

```luau
| "America/St_Johns"
```

---

### IanaTimezone

```luau
| "America/St_Kitts"
```

---

### IanaTimezone

```luau
| "America/St_Lucia"
```

---

### IanaTimezone

```luau
| "America/St_Thomas"
```

---

### IanaTimezone

```luau
| "America/St_Vincent"
```

---

### IanaTimezone

```luau
| "America/Swift_Current"
```

---

### IanaTimezone

```luau
| "America/Tegucigalpa"
```

---

### IanaTimezone

```luau
| "America/Thule"
```

---

### IanaTimezone

```luau
| "America/Thunder_Bay"
```

---

### IanaTimezone

```luau
| "America/Tijuana"
```

---

### IanaTimezone

```luau
| "America/Toronto"
```

---

### IanaTimezone

```luau
| "America/Tortola"
```

---

### IanaTimezone

```luau
| "America/Vancouver"
```

---

### IanaTimezone

```luau
| "America/Virgin"
```

---

### IanaTimezone

```luau
| "America/Whitehorse"
```

---

### IanaTimezone

```luau
| "America/Winnipeg"
```

---

### IanaTimezone

```luau
| "America/Yakutat"
```

---

### IanaTimezone

```luau
| "America/Yellowknife"
```

---

### IanaTimezone

```luau
| "Antarctica/Casey"
```

---

### IanaTimezone

```luau
| "Antarctica/Davis"
```

---

### IanaTimezone

```luau
| "Antarctica/DumontDUrville"
```

---

### IanaTimezone

```luau
| "Antarctica/Macquarie"
```

---

### IanaTimezone

```luau
| "Antarctica/Mawson"
```

---

### IanaTimezone

```luau
| "Antarctica/McMurdo"
```

---

### IanaTimezone

```luau
| "Antarctica/Palmer"
```

---

### IanaTimezone

```luau
| "Antarctica/Rothera"
```

---

### IanaTimezone

```luau
| "Antarctica/South_Pole"
```

---

### IanaTimezone

```luau
| "Antarctica/Syowa"
```

---

### IanaTimezone

```luau
| "Antarctica/Troll"
```

---

### IanaTimezone

```luau
| "Antarctica/Vostok"
```

---

### IanaTimezone

```luau
| "Arctic/Longyearbyen"
```

---

### IanaTimezone

```luau
| "Asia/Aden"
```

---

### IanaTimezone

```luau
| "Asia/Almaty"
```

---

### IanaTimezone

```luau
| "Asia/Amman"
```

---

### IanaTimezone

```luau
| "Asia/Anadyr"
```

---

### IanaTimezone

```luau
| "Asia/Aqtau"
```

---

### IanaTimezone

```luau
| "Asia/Aqtobe"
```

---

### IanaTimezone

```luau
| "Asia/Ashgabat"
```

---

### IanaTimezone

```luau
| "Asia/Ashkhabad"
```

---

### IanaTimezone

```luau
| "Asia/Atyrau"
```

---

### IanaTimezone

```luau
| "Asia/Baghdad"
```

---

### IanaTimezone

```luau
| "Asia/Bahrain"
```

---

### IanaTimezone

```luau
| "Asia/Baku"
```

---

### IanaTimezone

```luau
| "Asia/Bangkok"
```

---

### IanaTimezone

```luau
| "Asia/Barnaul"
```

---

### IanaTimezone

```luau
| "Asia/Beirut"
```

---

### IanaTimezone

```luau
| "Asia/Bishkek"
```

---

### IanaTimezone

```luau
| "Asia/Brunei"
```

---

### IanaTimezone

```luau
| "Asia/Calcutta"
```

---

### IanaTimezone

```luau
| "Asia/Chita"
```

---

### IanaTimezone

```luau
| "Asia/Choibalsan"
```

---

### IanaTimezone

```luau
| "Asia/Chongqing"
```

---

### IanaTimezone

```luau
| "Asia/Chungking"
```

---

### IanaTimezone

```luau
| "Asia/Colombo"
```

---

### IanaTimezone

```luau
| "Asia/Dacca"
```

---

### IanaTimezone

```luau
| "Asia/Damascus"
```

---

### IanaTimezone

```luau
| "Asia/Dhaka"
```

---

### IanaTimezone

```luau
| "Asia/Dili"
```

---

### IanaTimezone

```luau
| "Asia/Dubai"
```

---

### IanaTimezone

```luau
| "Asia/Dushanbe"
```

---

### IanaTimezone

```luau
| "Asia/Famagusta"
```

---

### IanaTimezone

```luau
| "Asia/Gaza"
```

---

### IanaTimezone

```luau
| "Asia/Harbin"
```

---

### IanaTimezone

```luau
| "Asia/Hebron"
```

---

### IanaTimezone

```luau
| "Asia/Ho_Chi_Minh"
```

---

### IanaTimezone

```luau
| "Asia/Hong_Kong"
```

---

### IanaTimezone

```luau
| "Asia/Hovd"
```

---

### IanaTimezone

```luau
| "Asia/Irkutsk"
```

---

### IanaTimezone

```luau
| "Asia/Istanbul"
```

---

### IanaTimezone

```luau
| "Asia/Jakarta"
```

---

### IanaTimezone

```luau
| "Asia/Jayapura"
```

---

### IanaTimezone

```luau
| "Asia/Jerusalem"
```

---

### IanaTimezone

```luau
| "Asia/Kabul"
```

---

### IanaTimezone

```luau
| "Asia/Kamchatka"
```

---

### IanaTimezone

```luau
| "Asia/Karachi"
```

---

### IanaTimezone

```luau
| "Asia/Kashgar"
```

---

### IanaTimezone

```luau
| "Asia/Kathmandu"
```

---

### IanaTimezone

```luau
| "Asia/Katmandu"
```

---

### IanaTimezone

```luau
| "Asia/Khandyga"
```

---

### IanaTimezone

```luau
| "Asia/Kolkata"
```

---

### IanaTimezone

```luau
| "Asia/Krasnoyarsk"
```

---

### IanaTimezone

```luau
| "Asia/Kuala_Lumpur"
```

---

### IanaTimezone

```luau
| "Asia/Kuching"
```

---

### IanaTimezone

```luau
| "Asia/Kuwait"
```

---

### IanaTimezone

```luau
| "Asia/Macao"
```

---

### IanaTimezone

```luau
| "Asia/Macau"
```

---

### IanaTimezone

```luau
| "Asia/Magadan"
```

---

### IanaTimezone

```luau
| "Asia/Makassar"
```

---

### IanaTimezone

```luau
| "Asia/Manila"
```

---

### IanaTimezone

```luau
| "Asia/Muscat"
```

---

### IanaTimezone

```luau
| "Asia/Nicosia"
```

---

### IanaTimezone

```luau
| "Asia/Novokuznetsk"
```

---

### IanaTimezone

```luau
| "Asia/Novosibirsk"
```

---

### IanaTimezone

```luau
| "Asia/Omsk"
```

---

### IanaTimezone

```luau
| "Asia/Oral"
```

---

### IanaTimezone

```luau
| "Asia/Phnom_Penh"
```

---

### IanaTimezone

```luau
| "Asia/Pontianak"
```

---

### IanaTimezone

```luau
| "Asia/Pyongyang"
```

---

### IanaTimezone

```luau
| "Asia/Qatar"
```

---

### IanaTimezone

```luau
| "Asia/Qostanay"
```

---

### IanaTimezone

```luau
| "Asia/Qyzylorda"
```

---

### IanaTimezone

```luau
| "Asia/Rangoon"
```

---

### IanaTimezone

```luau
| "Asia/Riyadh"
```

---

### IanaTimezone

```luau
| "Asia/Saigon"
```

---

### IanaTimezone

```luau
| "Asia/Sakhalin"
```

---

### IanaTimezone

```luau
| "Asia/Samarkand"
```

---

### IanaTimezone

```luau
| "Asia/Seoul"
```

---

### IanaTimezone

```luau
| "Asia/Shanghai"
```

---

### IanaTimezone

```luau
| "Asia/Singapore"
```

---

### IanaTimezone

```luau
| "Asia/Srednekolymsk"
```

---

### IanaTimezone

```luau
| "Asia/Taipei"
```

---

### IanaTimezone

```luau
| "Asia/Tashkent"
```

---

### IanaTimezone

```luau
| "Asia/Tbilisi"
```

---

### IanaTimezone

```luau
| "Asia/Tehran"
```

---

### IanaTimezone

```luau
| "Asia/Tel_Aviv"
```

---

### IanaTimezone

```luau
| "Asia/Thimbu"
```

---

### IanaTimezone

```luau
| "Asia/Thimphu"
```

---

### IanaTimezone

```luau
| "Asia/Tokyo"
```

---

### IanaTimezone

```luau
| "Asia/Tomsk"
```

---

### IanaTimezone

```luau
| "Asia/Ujung_Pandang"
```

---

### IanaTimezone

```luau
| "Asia/Ulaanbaatar"
```

---

### IanaTimezone

```luau
| "Asia/Ulan_Bator"
```

---

### IanaTimezone

```luau
| "Asia/Urumqi"
```

---

### IanaTimezone

```luau
| "Asia/Ust-Nera"
```

---

### IanaTimezone

```luau
| "Asia/Vientiane"
```

---

### IanaTimezone

```luau
| "Asia/Vladivostok"
```

---

### IanaTimezone

```luau
| "Asia/Yakutsk"
```

---

### IanaTimezone

```luau
| "Asia/Yangon"
```

---

### IanaTimezone

```luau
| "Asia/Yekaterinburg"
```

---

### IanaTimezone

```luau
| "Asia/Yerevan"
```

---

### IanaTimezone

```luau
| "Atlantic/Azores"
```

---

### IanaTimezone

```luau
| "Atlantic/Bermuda"
```

---

### IanaTimezone

```luau
| "Atlantic/Canary"
```

---

### IanaTimezone

```luau
| "Atlantic/Cape_Verde"
```

---

### IanaTimezone

```luau
| "Atlantic/Faeroe"
```

---

### IanaTimezone

```luau
| "Atlantic/Faroe"
```

---

### IanaTimezone

```luau
| "Atlantic/Jan_Mayen"
```

---

### IanaTimezone

```luau
| "Atlantic/Madeira"
```

---

### IanaTimezone

```luau
| "Atlantic/Reykjavik"
```

---

### IanaTimezone

```luau
| "Atlantic/South_Georgia"
```

---

### IanaTimezone

```luau
| "Atlantic/St_Helena"
```

---

### IanaTimezone

```luau
| "Atlantic/Stanley"
```

---

### IanaTimezone

```luau
| "Australia/ACT"
```

---

### IanaTimezone

```luau
| "Australia/Adelaide"
```

---

### IanaTimezone

```luau
| "Australia/Brisbane"
```

---

### IanaTimezone

```luau
| "Australia/Broken_Hill"
```

---

### IanaTimezone

```luau
| "Australia/Canberra"
```

---

### IanaTimezone

```luau
| "Australia/Currie"
```

---

### IanaTimezone

```luau
| "Australia/Darwin"
```

---

### IanaTimezone

```luau
| "Australia/Eucla"
```

---

### IanaTimezone

```luau
| "Australia/Hobart"
```

---

### IanaTimezone

```luau
| "Australia/LHI"
```

---

### IanaTimezone

```luau
| "Australia/Lindeman"
```

---

### IanaTimezone

```luau
| "Australia/Lord_Howe"
```

---

### IanaTimezone

```luau
| "Australia/Melbourne"
```

---

### IanaTimezone

```luau
| "Australia/NSW"
```

---

### IanaTimezone

```luau
| "Australia/North"
```

---

### IanaTimezone

```luau
| "Australia/Perth"
```

---

### IanaTimezone

```luau
| "Australia/Queensland"
```

---

### IanaTimezone

```luau
| "Australia/South"
```

---

### IanaTimezone

```luau
| "Australia/Sydney"
```

---

### IanaTimezone

```luau
| "Australia/Tasmania"
```

---

### IanaTimezone

```luau
| "Australia/Victoria"
```

---

### IanaTimezone

```luau
| "Australia/West"
```

---

### IanaTimezone

```luau
| "Australia/Yancowinna"
```

---

### IanaTimezone

```luau
| "Brazil/Acre"
```

---

### IanaTimezone

```luau
| "Brazil/DeNoronha"
```

---

### IanaTimezone

```luau
| "Brazil/East"
```

---

### IanaTimezone

```luau
| "Brazil/West"
```

---

### IanaTimezone

```luau
| "CET"
```

---

### IanaTimezone

```luau
| "CST6CDT"
```

---

### IanaTimezone

```luau
| "Canada/Atlantic"
```

---

### IanaTimezone

```luau
| "Canada/Central"
```

---

### IanaTimezone

```luau
| "Canada/Eastern"
```

---

### IanaTimezone

```luau
| "Canada/Mountain"
```

---

### IanaTimezone

```luau
| "Canada/Newfoundland"
```

---

### IanaTimezone

```luau
| "Canada/Pacific"
```

---

### IanaTimezone

```luau
| "Canada/Saskatchewan"
```

---

### IanaTimezone

```luau
| "Canada/Yukon"
```

---

### IanaTimezone

```luau
| "Chile/Continental"
```

---

### IanaTimezone

```luau
| "Chile/EasterIsland"
```

---

### IanaTimezone

```luau
| "Cuba"
```

---

### IanaTimezone

```luau
| "EET"
```

---

### IanaTimezone

```luau
| "EST"
```

---

### IanaTimezone

```luau
| "EST5EDT"
```

---

### IanaTimezone

```luau
| "Egypt"
```

---

### IanaTimezone

```luau
| "Eire"
```

---

### IanaTimezone

```luau
| "Etc/GMT"
```

---

### IanaTimezone

```luau
| "Etc/GMT+0"
```

---

### IanaTimezone

```luau
| "Etc/GMT+1"
```

---

### IanaTimezone

```luau
| "Etc/GMT+10"
```

---

### IanaTimezone

```luau
| "Etc/GMT+11"
```

---

### IanaTimezone

```luau
| "Etc/GMT+12"
```

---

### IanaTimezone

```luau
| "Etc/GMT+2"
```

---

### IanaTimezone

```luau
| "Etc/GMT+3"
```

---

### IanaTimezone

```luau
| "Etc/GMT+4"
```

---

### IanaTimezone

```luau
| "Etc/GMT+5"
```

---

### IanaTimezone

```luau
| "Etc/GMT+6"
```

---

### IanaTimezone

```luau
| "Etc/GMT+7"
```

---

### IanaTimezone

```luau
| "Etc/GMT+8"
```

---

### IanaTimezone

```luau
| "Etc/GMT+9"
```

---

### IanaTimezone

```luau
| "Etc/GMT-0"
```

---

### IanaTimezone

```luau
| "Etc/GMT-1"
```

---

### IanaTimezone

```luau
| "Etc/GMT-10"
```

---

### IanaTimezone

```luau
| "Etc/GMT-11"
```

---

### IanaTimezone

```luau
| "Etc/GMT-12"
```

---

### IanaTimezone

```luau
| "Etc/GMT-13"
```

---

### IanaTimezone

```luau
| "Etc/GMT-14"
```

---

### IanaTimezone

```luau
| "Etc/GMT-2"
```

---

### IanaTimezone

```luau
| "Etc/GMT-3"
```

---

### IanaTimezone

```luau
| "Etc/GMT-4"
```

---

### IanaTimezone

```luau
| "Etc/GMT-5"
```

---

### IanaTimezone

```luau
| "Etc/GMT-6"
```

---

### IanaTimezone

```luau
| "Etc/GMT-7"
```

---

### IanaTimezone

```luau
| "Etc/GMT-8"
```

---

### IanaTimezone

```luau
| "Etc/GMT-9"
```

---

### IanaTimezone

```luau
| "Etc/GMT0"
```

---

### IanaTimezone

```luau
| "Etc/Greenwich"
```

---

### IanaTimezone

```luau
| "Etc/UCT"
```

---

### IanaTimezone

```luau
| "Etc/UTC"
```

---

### IanaTimezone

```luau
| "Etc/Universal"
```

---

### IanaTimezone

```luau
| "Etc/Zulu"
```

---

### IanaTimezone

```luau
| "Europe/Amsterdam"
```

---

### IanaTimezone

```luau
| "Europe/Andorra"
```

---

### IanaTimezone

```luau
| "Europe/Astrakhan"
```

---

### IanaTimezone

```luau
| "Europe/Athens"
```

---

### IanaTimezone

```luau
| "Europe/Belfast"
```

---

### IanaTimezone

```luau
| "Europe/Belgrade"
```

---

### IanaTimezone

```luau
| "Europe/Berlin"
```

---

### IanaTimezone

```luau
| "Europe/Bratislava"
```

---

### IanaTimezone

```luau
| "Europe/Brussels"
```

---

### IanaTimezone

```luau
| "Europe/Bucharest"
```

---

### IanaTimezone

```luau
| "Europe/Budapest"
```

---

### IanaTimezone

```luau
| "Europe/Busingen"
```

---

### IanaTimezone

```luau
| "Europe/Chisinau"
```

---

### IanaTimezone

```luau
| "Europe/Copenhagen"
```

---

### IanaTimezone

```luau
| "Europe/Dublin"
```

---

### IanaTimezone

```luau
| "Europe/Gibraltar"
```

---

### IanaTimezone

```luau
| "Europe/Guernsey"
```

---

### IanaTimezone

```luau
| "Europe/Helsinki"
```

---

### IanaTimezone

```luau
| "Europe/Isle_of_Man"
```

---

### IanaTimezone

```luau
| "Europe/Istanbul"
```

---

### IanaTimezone

```luau
| "Europe/Jersey"
```

---

### IanaTimezone

```luau
| "Europe/Kaliningrad"
```

---

### IanaTimezone

```luau
| "Europe/Kiev"
```

---

### IanaTimezone

```luau
| "Europe/Kirov"
```

---

### IanaTimezone

```luau
| "Europe/Kyiv"
```

---

### IanaTimezone

```luau
| "Europe/Lisbon"
```

---

### IanaTimezone

```luau
| "Europe/Ljubljana"
```

---

### IanaTimezone

```luau
| "Europe/London"
```

---

### IanaTimezone

```luau
| "Europe/Luxembourg"
```

---

### IanaTimezone

```luau
| "Europe/Madrid"
```

---

### IanaTimezone

```luau
| "Europe/Malta"
```

---

### IanaTimezone

```luau
| "Europe/Mariehamn"
```

---

### IanaTimezone

```luau
| "Europe/Minsk"
```

---

### IanaTimezone

```luau
| "Europe/Monaco"
```

---

### IanaTimezone

```luau
| "Europe/Moscow"
```

---

### IanaTimezone

```luau
| "Europe/Nicosia"
```

---

### IanaTimezone

```luau
| "Europe/Oslo"
```

---

### IanaTimezone

```luau
| "Europe/Paris"
```

---

### IanaTimezone

```luau
| "Europe/Podgorica"
```

---

### IanaTimezone

```luau
| "Europe/Prague"
```

---

### IanaTimezone

```luau
| "Europe/Riga"
```

---

### IanaTimezone

```luau
| "Europe/Rome"
```

---

### IanaTimezone

```luau
| "Europe/Samara"
```

---

### IanaTimezone

```luau
| "Europe/San_Marino"
```

---

### IanaTimezone

```luau
| "Europe/Sarajevo"
```

---

### IanaTimezone

```luau
| "Europe/Saratov"
```

---

### IanaTimezone

```luau
| "Europe/Simferopol"
```

---

### IanaTimezone

```luau
| "Europe/Skopje"
```

---

### IanaTimezone

```luau
| "Europe/Sofia"
```

---

### IanaTimezone

```luau
| "Europe/Stockholm"
```

---

### IanaTimezone

```luau
| "Europe/Tallinn"
```

---

### IanaTimezone

```luau
| "Europe/Tirane"
```

---

### IanaTimezone

```luau
| "Europe/Tiraspol"
```

---

### IanaTimezone

```luau
| "Europe/Ulyanovsk"
```

---

### IanaTimezone

```luau
| "Europe/Uzhgorod"
```

---

### IanaTimezone

```luau
| "Europe/Vaduz"
```

---

### IanaTimezone

```luau
| "Europe/Vatican"
```

---

### IanaTimezone

```luau
| "Europe/Vienna"
```

---

### IanaTimezone

```luau
| "Europe/Vilnius"
```

---

### IanaTimezone

```luau
| "Europe/Volgograd"
```

---

### IanaTimezone

```luau
| "Europe/Warsaw"
```

---

### IanaTimezone

```luau
| "Europe/Zagreb"
```

---

### IanaTimezone

```luau
| "Europe/Zaporozhye"
```

---

### IanaTimezone

```luau
| "Europe/Zurich"
```

---

### IanaTimezone

```luau
| "GB"
```

---

### IanaTimezone

```luau
| "GB-Eire"
```

---

### IanaTimezone

```luau
| "GMT"
```

---

### IanaTimezone

```luau
| "GMT+0"
```

---

### IanaTimezone

```luau
| "GMT-0"
```

---

### IanaTimezone

```luau
| "GMT0"
```

---

### IanaTimezone

```luau
| "Greenwich"
```

---

### IanaTimezone

```luau
| "HST"
```

---

### IanaTimezone

```luau
| "Hongkong"
```

---

### IanaTimezone

```luau
| "Iceland"
```

---

### IanaTimezone

```luau
| "Indian/Antananarivo"
```

---

### IanaTimezone

```luau
| "Indian/Chagos"
```

---

### IanaTimezone

```luau
| "Indian/Christmas"
```

---

### IanaTimezone

```luau
| "Indian/Cocos"
```

---

### IanaTimezone

```luau
| "Indian/Comoro"
```

---

### IanaTimezone

```luau
| "Indian/Kerguelen"
```

---

### IanaTimezone

```luau
| "Indian/Mahe"
```

---

### IanaTimezone

```luau
| "Indian/Maldives"
```

---

### IanaTimezone

```luau
| "Indian/Mauritius"
```

---

### IanaTimezone

```luau
| "Indian/Mayotte"
```

---

### IanaTimezone

```luau
| "Indian/Reunion"
```

---

### IanaTimezone

```luau
| "Iran"
```

---

### IanaTimezone

```luau
| "Israel"
```

---

### IanaTimezone

```luau
| "Jamaica"
```

---

### IanaTimezone

```luau
| "Japan"
```

---

### IanaTimezone

```luau
| "Kwajalein"
```

---

### IanaTimezone

```luau
| "Libya"
```

---

### IanaTimezone

```luau
| "MET"
```

---

### IanaTimezone

```luau
| "MST"
```

---

### IanaTimezone

```luau
| "MST7MDT"
```

---

### IanaTimezone

```luau
| "Mexico/BajaNorte"
```

---

### IanaTimezone

```luau
| "Mexico/BajaSur"
```

---

### IanaTimezone

```luau
| "Mexico/General"
```

---

### IanaTimezone

```luau
| "NZ"
```

---

### IanaTimezone

```luau
| "NZ-CHAT"
```

---

### IanaTimezone

```luau
| "Navajo"
```

---

### IanaTimezone

```luau
| "PRC"
```

---

### IanaTimezone

```luau
| "PST8PDT"
```

---

### IanaTimezone

```luau
| "Pacific/Apia"
```

---

### IanaTimezone

```luau
| "Pacific/Auckland"
```

---

### IanaTimezone

```luau
| "Pacific/Bougainville"
```

---

### IanaTimezone

```luau
| "Pacific/Chatham"
```

---

### IanaTimezone

```luau
| "Pacific/Chuuk"
```

---

### IanaTimezone

```luau
| "Pacific/Easter"
```

---

### IanaTimezone

```luau
| "Pacific/Efate"
```

---

### IanaTimezone

```luau
| "Pacific/Enderbury"
```

---

### IanaTimezone

```luau
| "Pacific/Fakaofo"
```

---

### IanaTimezone

```luau
| "Pacific/Fiji"
```

---

### IanaTimezone

```luau
| "Pacific/Funafuti"
```

---

### IanaTimezone

```luau
| "Pacific/Galapagos"
```

---

### IanaTimezone

```luau
| "Pacific/Gambier"
```

---

### IanaTimezone

```luau
| "Pacific/Guadalcanal"
```

---

### IanaTimezone

```luau
| "Pacific/Guam"
```

---

### IanaTimezone

```luau
| "Pacific/Honolulu"
```

---

### IanaTimezone

```luau
| "Pacific/Johnston"
```

---

### IanaTimezone

```luau
| "Pacific/Kanton"
```

---

### IanaTimezone

```luau
| "Pacific/Kiritimati"
```

---

### IanaTimezone

```luau
| "Pacific/Kosrae"
```

---

### IanaTimezone

```luau
| "Pacific/Kwajalein"
```

---

### IanaTimezone

```luau
| "Pacific/Majuro"
```

---

### IanaTimezone

```luau
| "Pacific/Marquesas"
```

---

### IanaTimezone

```luau
| "Pacific/Midway"
```

---

### IanaTimezone

```luau
| "Pacific/Nauru"
```

---

### IanaTimezone

```luau
| "Pacific/Niue"
```

---

### IanaTimezone

```luau
| "Pacific/Norfolk"
```

---

### IanaTimezone

```luau
| "Pacific/Noumea"
```

---

### IanaTimezone

```luau
| "Pacific/Pago_Pago"
```

---

### IanaTimezone

```luau
| "Pacific/Palau"
```

---

### IanaTimezone

```luau
| "Pacific/Pitcairn"
```

---

### IanaTimezone

```luau
| "Pacific/Pohnpei"
```

---

### IanaTimezone

```luau
| "Pacific/Ponape"
```

---

### IanaTimezone

```luau
| "Pacific/Port_Moresby"
```

---

### IanaTimezone

```luau
| "Pacific/Rarotonga"
```

---

### IanaTimezone

```luau
| "Pacific/Saipan"
```

---

### IanaTimezone

```luau
| "Pacific/Samoa"
```

---

### IanaTimezone

```luau
| "Pacific/Tahiti"
```

---

### IanaTimezone

```luau
| "Pacific/Tarawa"
```

---

### IanaTimezone

```luau
| "Pacific/Tongatapu"
```

---

### IanaTimezone

```luau
| "Pacific/Truk"
```

---

### IanaTimezone

```luau
| "Pacific/Wake"
```

---

### IanaTimezone

```luau
| "Pacific/Wallis"
```

---

### IanaTimezone

```luau
| "Pacific/Yap"
```

---

### IanaTimezone

```luau
| "Poland"
```

---

### IanaTimezone

```luau
| "Portugal"
```

---

### IanaTimezone

```luau
| "ROC"
```

---

### IanaTimezone

```luau
| "ROK"
```

---

### IanaTimezone

```luau
| "Singapore"
```

---

### IanaTimezone

```luau
| "Turkey"
```

---

### IanaTimezone

```luau
| "UCT"
```

---

### IanaTimezone

```luau
| "US/Alaska"
```

---

### IanaTimezone

```luau
| "US/Aleutian"
```

---

### IanaTimezone

```luau
| "US/Arizona"
```

---

### IanaTimezone

```luau
| "US/Central"
```

---

### IanaTimezone

```luau
| "US/East-Indiana"
```

---

### IanaTimezone

```luau
| "US/Eastern"
```

---

### IanaTimezone

```luau
| "US/Hawaii"
```

---

### IanaTimezone

```luau
| "US/Indiana-Starke"
```

---

### IanaTimezone

```luau
| "US/Michigan"
```

---

### IanaTimezone

```luau
| "US/Mountain"
```

---

### IanaTimezone

```luau
| "US/Pacific"
```

---

### IanaTimezone

```luau
| "US/Samoa"
```

---

### IanaTimezone

```luau
| "UTC"
```

---

### IanaTimezone

```luau
| "Universal"
```

---

### IanaTimezone

```luau
| "W-SU"
```

---

### IanaTimezone

```luau
| "WET"
```

---

### IanaTimezone

```luau
| "Zulu"
```

---
