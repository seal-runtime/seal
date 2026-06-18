<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# time.datetime

`local datetime = require("@std/time/datetime")`

A timezone-aware library for parsing `DateTime`s and doing `TimeSpan` arithmetic.

See `datetime.parse`.

---

### datetime.now

<h4>

```luau
function datetime.now() -> DateTime,
```

</h4>

 Constructs a `DateTime` from right now (based on system time) in your local timezone.

---

### datetime.from

<h4>

```luau
function datetime.from(timestamp: number, timezone: IanaTimezone, nanos: number?) -> DateTime,
```

</h4>

Constructs a `DateTime` from a Unix Timestamp and an `IanaTimezone`.

- `timezone` defaults to `"UTC"` if not specified
- `nanos` defaults to `0` if not specified

---

### datetime.parse

<h4>

```luau
function datetime.parse(source: string, format: string | CommonFormats, timezone: IanaTimezone) -> DateTime,
```

</h4>

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

### datetime.common_formats.ISO_8601

<h4>

```luau
ISO_8601: "%Y-%m-%d %H:%M",
```

</h4>

 Common DateTime format strings.

---

### datetime.common_formats.RFC_2822

<h4>

```luau
RFC_2822: "%a, %d %b %Y %H:%M:%S %z",
```

</h4>

---

### datetime.common_formats.RFC_3339

<h4>

```luau
RFC_3339: "%Y-%m-%dT%H:%M:%S%:z",
```

</h4>

---

### datetime.common_formats.SHORT_DATE

<h4>

```luau
SHORT_DATE: "%Y-%m-%d",
```

</h4>

---

### datetime.common_formats.SHORT_TIME

<h4>

```luau
SHORT_TIME: "%H:%M",
```

</h4>

---

### datetime.common_formats.FULL_DATE_TIME

<h4>

```luau
FULL_DATE_TIME: "%A, %B %d, %Y %H:%M:%S",
```

</h4>

---

### datetime.common_formats.LOGGING_24_HR

<h4>

```luau
LOGGING_24_HR: "%a %b %e %H:%M:%S %Z %Y",
```

</h4>

---

### datetime.common_formats.LOGGING_12_HR

<h4>

```luau
LOGGING_12_HR: "%a %b %e %I:%M:%S %p %Z %Y",
```

</h4>

---

### datetime.common_formats.AMERICAN_FULL_DATE_TIME

<h4>

```luau
AMERICAN_FULL_DATE_TIME: "%A, %B %d, %Y %I:%M:%S %p",
```

</h4>

---

### datetime.years

<h4>

```luau
function datetime.years(number, relative_to: DateTime?) -> TimeSpan,
```

</h4>

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

### datetime.months

<h4>

```luau
function datetime.months(number, relative_to: DateTime?) -> TimeSpan,
```

</h4>

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

### datetime.days

<h4>

```luau
function datetime.days(number) -> TimeSpan,
```

</h4>

 Constructs a `TimeSpan` from days. Assumes every day is 24 hours.

---

### datetime.hours

<h4>

```luau
function datetime.hours(number) -> TimeSpan,
```

</h4>

 Constructs a `TimeSpan` from hours.

---

### datetime.minutes

<h4>

```luau
function datetime.minutes(number) -> TimeSpan,
```

</h4>

 Constructs a `TimeSpan` from minutes.

---

### datetime.seconds

<h4>

```luau
function datetime.seconds(number) -> TimeSpan,
```

</h4>

 Constructs a `TimeSpan` from seconds.

---

### datetime.milliseconds

<h4>

```luau
function datetime.milliseconds(number) -> TimeSpan,
```

</h4>

 Constructs a `TimeSpan` from milliseconds.

---

## `export type` CommonFormats

<h4>

```luau
export type CommonFormats = index<datetime, "common_formats">
```

</h4>

---

## `export type` IanaTimezone

<h4>

```luau
export type IanaTimezone =
```

</h4>

---

```luau
| "AUTO" -- timezone info already included in input data
```

---

```luau
| "Africa/Abidjan"
```

---

```luau
| "Africa/Accra"
```

---

```luau
| "Africa/Addis_Ababa"
```

---

```luau
| "Africa/Algiers"
```

---

```luau
| "Africa/Asmara"
```

---

```luau
| "Africa/Asmera"
```

---

```luau
| "Africa/Bamako"
```

---

```luau
| "Africa/Bangui"
```

---

```luau
| "Africa/Banjul"
```

---

```luau
| "Africa/Bissau"
```

---

```luau
| "Africa/Blantyre"
```

---

```luau
| "Africa/Brazzaville"
```

---

```luau
| "Africa/Bujumbura"
```

---

```luau
| "Africa/Cairo"
```

---

```luau
| "Africa/Casablanca"
```

---

```luau
| "Africa/Ceuta"
```

---

```luau
| "Africa/Conakry"
```

---

```luau
| "Africa/Dakar"
```

---

```luau
| "Africa/Dar_es_Salaam"
```

---

```luau
| "Africa/Djibouti"
```

---

```luau
| "Africa/Douala"
```

---

```luau
| "Africa/El_Aaiun"
```

---

```luau
| "Africa/Freetown"
```

---

```luau
| "Africa/Gaborone"
```

---

```luau
| "Africa/Harare"
```

---

```luau
| "Africa/Johannesburg"
```

---

```luau
| "Africa/Juba"
```

---

```luau
| "Africa/Kampala"
```

---

```luau
| "Africa/Khartoum"
```

---

```luau
| "Africa/Kigali"
```

---

```luau
| "Africa/Kinshasa"
```

---

```luau
| "Africa/Lagos"
```

---

```luau
| "Africa/Libreville"
```

---

```luau
| "Africa/Lome"
```

---

```luau
| "Africa/Luanda"
```

---

```luau
| "Africa/Lubumbashi"
```

---

```luau
| "Africa/Lusaka"
```

---

```luau
| "Africa/Malabo"
```

---

```luau
| "Africa/Maputo"
```

---

```luau
| "Africa/Maseru"
```

---

```luau
| "Africa/Mbabane"
```

---

```luau
| "Africa/Mogadishu"
```

---

```luau
| "Africa/Monrovia"
```

---

```luau
| "Africa/Nairobi"
```

---

```luau
| "Africa/Ndjamena"
```

---

```luau
| "Africa/Niamey"
```

---

```luau
| "Africa/Nouakchott"
```

---

```luau
| "Africa/Ouagadougou"
```

---

```luau
| "Africa/Porto-Novo"
```

---

```luau
| "Africa/Sao_Tome"
```

---

```luau
| "Africa/Timbuktu"
```

---

```luau
| "Africa/Tripoli"
```

---

```luau
| "Africa/Tunis"
```

---

```luau
| "Africa/Windhoek"
```

---

```luau
| "America/Adak"
```

---

```luau
| "America/Anchorage"
```

---

```luau
| "America/Anguilla"
```

---

```luau
| "America/Antigua"
```

---

```luau
| "America/Araguaina"
```

---

```luau
| "America/Argentina/Buenos_Aires"
```

---

```luau
| "America/Argentina/Catamarca"
```

---

```luau
| "America/Argentina/ComodRivadavia"
```

---

```luau
| "America/Argentina/Cordoba"
```

---

```luau
| "America/Argentina/Jujuy"
```

---

```luau
| "America/Argentina/La_Rioja"
```

---

```luau
| "America/Argentina/Mendoza"
```

---

```luau
| "America/Argentina/Rio_Gallegos"
```

---

```luau
| "America/Argentina/Salta"
```

---

```luau
| "America/Argentina/San_Juan"
```

---

```luau
| "America/Argentina/San_Luis"
```

---

```luau
| "America/Argentina/Tucuman"
```

---

```luau
| "America/Argentina/Ushuaia"
```

---

```luau
| "America/Aruba"
```

---

```luau
| "America/Asuncion"
```

---

```luau
| "America/Atikokan"
```

---

```luau
| "America/Atka"
```

---

```luau
| "America/Bahia"
```

---

```luau
| "America/Bahia_Banderas"
```

---

```luau
| "America/Barbados"
```

---

```luau
| "America/Belem"
```

---

```luau
| "America/Belize"
```

---

```luau
| "America/Blanc-Sablon"
```

---

```luau
| "America/Boa_Vista"
```

---

```luau
| "America/Bogota"
```

---

```luau
| "America/Boise"
```

---

```luau
| "America/Buenos_Aires"
```

---

```luau
| "America/Cambridge_Bay"
```

---

```luau
| "America/Campo_Grande"
```

---

```luau
| "America/Cancun"
```

---

```luau
| "America/Caracas"
```

---

```luau
| "America/Catamarca"
```

---

```luau
| "America/Cayenne"
```

---

```luau
| "America/Cayman"
```

---

```luau
| "America/Chicago"
```

---

```luau
| "America/Chihuahua"
```

---

```luau
| "America/Ciudad_Juarez"
```

---

```luau
| "America/Coral_Harbour"
```

---

```luau
| "America/Cordoba"
```

---

```luau
| "America/Costa_Rica"
```

---

```luau
| "America/Creston"
```

---

```luau
| "America/Cuiaba"
```

---

```luau
| "America/Curacao"
```

---

```luau
| "America/Danmarkshavn"
```

---

```luau
| "America/Dawson"
```

---

```luau
| "America/Dawson_Creek"
```

---

```luau
| "America/Denver"
```

---

```luau
| "America/Detroit"
```

---

```luau
| "America/Dominica"
```

---

```luau
| "America/Edmonton"
```

---

```luau
| "America/Eirunepe"
```

---

```luau
| "America/El_Salvador"
```

---

```luau
| "America/Ensenada"
```

---

```luau
| "America/Fort_Nelson"
```

---

```luau
| "America/Fort_Wayne"
```

---

```luau
| "America/Fortaleza"
```

---

```luau
| "America/Glace_Bay"
```

---

```luau
| "America/Godthab"
```

---

```luau
| "America/Goose_Bay"
```

---

```luau
| "America/Grand_Turk"
```

---

```luau
| "America/Grenada"
```

---

```luau
| "America/Guadeloupe"
```

---

```luau
| "America/Guatemala"
```

---

```luau
| "America/Guayaquil"
```

---

```luau
| "America/Guyana"
```

---

```luau
| "America/Halifax"
```

---

```luau
| "America/Havana"
```

---

```luau
| "America/Hermosillo"
```

---

```luau
| "America/Indiana/Indianapolis"
```

---

```luau
| "America/Indiana/Knox"
```

---

```luau
| "America/Indiana/Marengo"
```

---

```luau
| "America/Indiana/Petersburg"
```

---

```luau
| "America/Indiana/Tell_City"
```

---

```luau
| "America/Indiana/Vevay"
```

---

```luau
| "America/Indiana/Vincennes"
```

---

```luau
| "America/Indiana/Winamac"
```

---

```luau
| "America/Indianapolis"
```

---

```luau
| "America/Inuvik"
```

---

```luau
| "America/Iqaluit"
```

---

```luau
| "America/Jamaica"
```

---

```luau
| "America/Jujuy"
```

---

```luau
| "America/Juneau"
```

---

```luau
| "America/Kentucky/Louisville"
```

---

```luau
| "America/Kentucky/Monticello"
```

---

```luau
| "America/Knox_IN"
```

---

```luau
| "America/Kralendijk"
```

---

```luau
| "America/La_Paz"
```

---

```luau
| "America/Lima"
```

---

```luau
| "America/Los_Angeles"
```

---

```luau
| "America/Louisville"
```

---

```luau
| "America/Lower_Princes"
```

---

```luau
| "America/Maceio"
```

---

```luau
| "America/Managua"
```

---

```luau
| "America/Manaus"
```

---

```luau
| "America/Marigot"
```

---

```luau
| "America/Martinique"
```

---

```luau
| "America/Matamoros"
```

---

```luau
| "America/Mazatlan"
```

---

```luau
| "America/Mendoza"
```

---

```luau
| "America/Menominee"
```

---

```luau
| "America/Merida"
```

---

```luau
| "America/Metlakatla"
```

---

```luau
| "America/Mexico_City"
```

---

```luau
| "America/Miquelon"
```

---

```luau
| "America/Moncton"
```

---

```luau
| "America/Monterrey"
```

---

```luau
| "America/Montevideo"
```

---

```luau
| "America/Montreal"
```

---

```luau
| "America/Montserrat"
```

---

```luau
| "America/Nassau"
```

---

```luau
| "America/New_York"
```

---

```luau
| "America/Nipigon"
```

---

```luau
| "America/Nome"
```

---

```luau
| "America/Noronha"
```

---

```luau
| "America/North_Dakota/Beulah"
```

---

```luau
| "America/North_Dakota/Center"
```

---

```luau
| "America/North_Dakota/New_Salem"
```

---

```luau
| "America/Nuuk"
```

---

```luau
| "America/Ojinaga"
```

---

```luau
| "America/Panama"
```

---

```luau
| "America/Pangnirtung"
```

---

```luau
| "America/Paramaribo"
```

---

```luau
| "America/Phoenix"
```

---

```luau
| "America/Port-au-Prince"
```

---

```luau
| "America/Port_of_Spain"
```

---

```luau
| "America/Porto_Acre"
```

---

```luau
| "America/Porto_Velho"
```

---

```luau
| "America/Puerto_Rico"
```

---

```luau
| "America/Punta_Arenas"
```

---

```luau
| "America/Rainy_River"
```

---

```luau
| "America/Rankin_Inlet"
```

---

```luau
| "America/Recife"
```

---

```luau
| "America/Regina"
```

---

```luau
| "America/Resolute"
```

---

```luau
| "America/Rio_Branco"
```

---

```luau
| "America/Rosario"
```

---

```luau
| "America/Santa_Isabel"
```

---

```luau
| "America/Santarem"
```

---

```luau
| "America/Santiago"
```

---

```luau
| "America/Santo_Domingo"
```

---

```luau
| "America/Sao_Paulo"
```

---

```luau
| "America/Scoresbysund"
```

---

```luau
| "America/Shiprock"
```

---

```luau
| "America/Sitka"
```

---

```luau
| "America/St_Barthelemy"
```

---

```luau
| "America/St_Johns"
```

---

```luau
| "America/St_Kitts"
```

---

```luau
| "America/St_Lucia"
```

---

```luau
| "America/St_Thomas"
```

---

```luau
| "America/St_Vincent"
```

---

```luau
| "America/Swift_Current"
```

---

```luau
| "America/Tegucigalpa"
```

---

```luau
| "America/Thule"
```

---

```luau
| "America/Thunder_Bay"
```

---

```luau
| "America/Tijuana"
```

---

```luau
| "America/Toronto"
```

---

```luau
| "America/Tortola"
```

---

```luau
| "America/Vancouver"
```

---

```luau
| "America/Virgin"
```

---

```luau
| "America/Whitehorse"
```

---

```luau
| "America/Winnipeg"
```

---

```luau
| "America/Yakutat"
```

---

```luau
| "America/Yellowknife"
```

---

```luau
| "Antarctica/Casey"
```

---

```luau
| "Antarctica/Davis"
```

---

```luau
| "Antarctica/DumontDUrville"
```

---

```luau
| "Antarctica/Macquarie"
```

---

```luau
| "Antarctica/Mawson"
```

---

```luau
| "Antarctica/McMurdo"
```

---

```luau
| "Antarctica/Palmer"
```

---

```luau
| "Antarctica/Rothera"
```

---

```luau
| "Antarctica/South_Pole"
```

---

```luau
| "Antarctica/Syowa"
```

---

```luau
| "Antarctica/Troll"
```

---

```luau
| "Antarctica/Vostok"
```

---

```luau
| "Arctic/Longyearbyen"
```

---

```luau
| "Asia/Aden"
```

---

```luau
| "Asia/Almaty"
```

---

```luau
| "Asia/Amman"
```

---

```luau
| "Asia/Anadyr"
```

---

```luau
| "Asia/Aqtau"
```

---

```luau
| "Asia/Aqtobe"
```

---

```luau
| "Asia/Ashgabat"
```

---

```luau
| "Asia/Ashkhabad"
```

---

```luau
| "Asia/Atyrau"
```

---

```luau
| "Asia/Baghdad"
```

---

```luau
| "Asia/Bahrain"
```

---

```luau
| "Asia/Baku"
```

---

```luau
| "Asia/Bangkok"
```

---

```luau
| "Asia/Barnaul"
```

---

```luau
| "Asia/Beirut"
```

---

```luau
| "Asia/Bishkek"
```

---

```luau
| "Asia/Brunei"
```

---

```luau
| "Asia/Calcutta"
```

---

```luau
| "Asia/Chita"
```

---

```luau
| "Asia/Choibalsan"
```

---

```luau
| "Asia/Chongqing"
```

---

```luau
| "Asia/Chungking"
```

---

```luau
| "Asia/Colombo"
```

---

```luau
| "Asia/Dacca"
```

---

```luau
| "Asia/Damascus"
```

---

```luau
| "Asia/Dhaka"
```

---

```luau
| "Asia/Dili"
```

---

```luau
| "Asia/Dubai"
```

---

```luau
| "Asia/Dushanbe"
```

---

```luau
| "Asia/Famagusta"
```

---

```luau
| "Asia/Gaza"
```

---

```luau
| "Asia/Harbin"
```

---

```luau
| "Asia/Hebron"
```

---

```luau
| "Asia/Ho_Chi_Minh"
```

---

```luau
| "Asia/Hong_Kong"
```

---

```luau
| "Asia/Hovd"
```

---

```luau
| "Asia/Irkutsk"
```

---

```luau
| "Asia/Istanbul"
```

---

```luau
| "Asia/Jakarta"
```

---

```luau
| "Asia/Jayapura"
```

---

```luau
| "Asia/Jerusalem"
```

---

```luau
| "Asia/Kabul"
```

---

```luau
| "Asia/Kamchatka"
```

---

```luau
| "Asia/Karachi"
```

---

```luau
| "Asia/Kashgar"
```

---

```luau
| "Asia/Kathmandu"
```

---

```luau
| "Asia/Katmandu"
```

---

```luau
| "Asia/Khandyga"
```

---

```luau
| "Asia/Kolkata"
```

---

```luau
| "Asia/Krasnoyarsk"
```

---

```luau
| "Asia/Kuala_Lumpur"
```

---

```luau
| "Asia/Kuching"
```

---

```luau
| "Asia/Kuwait"
```

---

```luau
| "Asia/Macao"
```

---

```luau
| "Asia/Macau"
```

---

```luau
| "Asia/Magadan"
```

---

```luau
| "Asia/Makassar"
```

---

```luau
| "Asia/Manila"
```

---

```luau
| "Asia/Muscat"
```

---

```luau
| "Asia/Nicosia"
```

---

```luau
| "Asia/Novokuznetsk"
```

---

```luau
| "Asia/Novosibirsk"
```

---

```luau
| "Asia/Omsk"
```

---

```luau
| "Asia/Oral"
```

---

```luau
| "Asia/Phnom_Penh"
```

---

```luau
| "Asia/Pontianak"
```

---

```luau
| "Asia/Pyongyang"
```

---

```luau
| "Asia/Qatar"
```

---

```luau
| "Asia/Qostanay"
```

---

```luau
| "Asia/Qyzylorda"
```

---

```luau
| "Asia/Rangoon"
```

---

```luau
| "Asia/Riyadh"
```

---

```luau
| "Asia/Saigon"
```

---

```luau
| "Asia/Sakhalin"
```

---

```luau
| "Asia/Samarkand"
```

---

```luau
| "Asia/Seoul"
```

---

```luau
| "Asia/Shanghai"
```

---

```luau
| "Asia/Singapore"
```

---

```luau
| "Asia/Srednekolymsk"
```

---

```luau
| "Asia/Taipei"
```

---

```luau
| "Asia/Tashkent"
```

---

```luau
| "Asia/Tbilisi"
```

---

```luau
| "Asia/Tehran"
```

---

```luau
| "Asia/Tel_Aviv"
```

---

```luau
| "Asia/Thimbu"
```

---

```luau
| "Asia/Thimphu"
```

---

```luau
| "Asia/Tokyo"
```

---

```luau
| "Asia/Tomsk"
```

---

```luau
| "Asia/Ujung_Pandang"
```

---

```luau
| "Asia/Ulaanbaatar"
```

---

```luau
| "Asia/Ulan_Bator"
```

---

```luau
| "Asia/Urumqi"
```

---

```luau
| "Asia/Ust-Nera"
```

---

```luau
| "Asia/Vientiane"
```

---

```luau
| "Asia/Vladivostok"
```

---

```luau
| "Asia/Yakutsk"
```

---

```luau
| "Asia/Yangon"
```

---

```luau
| "Asia/Yekaterinburg"
```

---

```luau
| "Asia/Yerevan"
```

---

```luau
| "Atlantic/Azores"
```

---

```luau
| "Atlantic/Bermuda"
```

---

```luau
| "Atlantic/Canary"
```

---

```luau
| "Atlantic/Cape_Verde"
```

---

```luau
| "Atlantic/Faeroe"
```

---

```luau
| "Atlantic/Faroe"
```

---

```luau
| "Atlantic/Jan_Mayen"
```

---

```luau
| "Atlantic/Madeira"
```

---

```luau
| "Atlantic/Reykjavik"
```

---

```luau
| "Atlantic/South_Georgia"
```

---

```luau
| "Atlantic/St_Helena"
```

---

```luau
| "Atlantic/Stanley"
```

---

```luau
| "Australia/ACT"
```

---

```luau
| "Australia/Adelaide"
```

---

```luau
| "Australia/Brisbane"
```

---

```luau
| "Australia/Broken_Hill"
```

---

```luau
| "Australia/Canberra"
```

---

```luau
| "Australia/Currie"
```

---

```luau
| "Australia/Darwin"
```

---

```luau
| "Australia/Eucla"
```

---

```luau
| "Australia/Hobart"
```

---

```luau
| "Australia/LHI"
```

---

```luau
| "Australia/Lindeman"
```

---

```luau
| "Australia/Lord_Howe"
```

---

```luau
| "Australia/Melbourne"
```

---

```luau
| "Australia/NSW"
```

---

```luau
| "Australia/North"
```

---

```luau
| "Australia/Perth"
```

---

```luau
| "Australia/Queensland"
```

---

```luau
| "Australia/South"
```

---

```luau
| "Australia/Sydney"
```

---

```luau
| "Australia/Tasmania"
```

---

```luau
| "Australia/Victoria"
```

---

```luau
| "Australia/West"
```

---

```luau
| "Australia/Yancowinna"
```

---

```luau
| "Brazil/Acre"
```

---

```luau
| "Brazil/DeNoronha"
```

---

```luau
| "Brazil/East"
```

---

```luau
| "Brazil/West"
```

---

```luau
| "CET"
```

---

```luau
| "CST6CDT"
```

---

```luau
| "Canada/Atlantic"
```

---

```luau
| "Canada/Central"
```

---

```luau
| "Canada/Eastern"
```

---

```luau
| "Canada/Mountain"
```

---

```luau
| "Canada/Newfoundland"
```

---

```luau
| "Canada/Pacific"
```

---

```luau
| "Canada/Saskatchewan"
```

---

```luau
| "Canada/Yukon"
```

---

```luau
| "Chile/Continental"
```

---

```luau
| "Chile/EasterIsland"
```

---

```luau
| "Cuba"
```

---

```luau
| "EET"
```

---

```luau
| "EST"
```

---

```luau
| "EST5EDT"
```

---

```luau
| "Egypt"
```

---

```luau
| "Eire"
```

---

```luau
| "Etc/GMT"
```

---

```luau
| "Etc/GMT+0"
```

---

```luau
| "Etc/GMT+1"
```

---

```luau
| "Etc/GMT+10"
```

---

```luau
| "Etc/GMT+11"
```

---

```luau
| "Etc/GMT+12"
```

---

```luau
| "Etc/GMT+2"
```

---

```luau
| "Etc/GMT+3"
```

---

```luau
| "Etc/GMT+4"
```

---

```luau
| "Etc/GMT+5"
```

---

```luau
| "Etc/GMT+6"
```

---

```luau
| "Etc/GMT+7"
```

---

```luau
| "Etc/GMT+8"
```

---

```luau
| "Etc/GMT+9"
```

---

```luau
| "Etc/GMT-0"
```

---

```luau
| "Etc/GMT-1"
```

---

```luau
| "Etc/GMT-10"
```

---

```luau
| "Etc/GMT-11"
```

---

```luau
| "Etc/GMT-12"
```

---

```luau
| "Etc/GMT-13"
```

---

```luau
| "Etc/GMT-14"
```

---

```luau
| "Etc/GMT-2"
```

---

```luau
| "Etc/GMT-3"
```

---

```luau
| "Etc/GMT-4"
```

---

```luau
| "Etc/GMT-5"
```

---

```luau
| "Etc/GMT-6"
```

---

```luau
| "Etc/GMT-7"
```

---

```luau
| "Etc/GMT-8"
```

---

```luau
| "Etc/GMT-9"
```

---

```luau
| "Etc/GMT0"
```

---

```luau
| "Etc/Greenwich"
```

---

```luau
| "Etc/UCT"
```

---

```luau
| "Etc/UTC"
```

---

```luau
| "Etc/Universal"
```

---

```luau
| "Etc/Zulu"
```

---

```luau
| "Europe/Amsterdam"
```

---

```luau
| "Europe/Andorra"
```

---

```luau
| "Europe/Astrakhan"
```

---

```luau
| "Europe/Athens"
```

---

```luau
| "Europe/Belfast"
```

---

```luau
| "Europe/Belgrade"
```

---

```luau
| "Europe/Berlin"
```

---

```luau
| "Europe/Bratislava"
```

---

```luau
| "Europe/Brussels"
```

---

```luau
| "Europe/Bucharest"
```

---

```luau
| "Europe/Budapest"
```

---

```luau
| "Europe/Busingen"
```

---

```luau
| "Europe/Chisinau"
```

---

```luau
| "Europe/Copenhagen"
```

---

```luau
| "Europe/Dublin"
```

---

```luau
| "Europe/Gibraltar"
```

---

```luau
| "Europe/Guernsey"
```

---

```luau
| "Europe/Helsinki"
```

---

```luau
| "Europe/Isle_of_Man"
```

---

```luau
| "Europe/Istanbul"
```

---

```luau
| "Europe/Jersey"
```

---

```luau
| "Europe/Kaliningrad"
```

---

```luau
| "Europe/Kiev"
```

---

```luau
| "Europe/Kirov"
```

---

```luau
| "Europe/Kyiv"
```

---

```luau
| "Europe/Lisbon"
```

---

```luau
| "Europe/Ljubljana"
```

---

```luau
| "Europe/London"
```

---

```luau
| "Europe/Luxembourg"
```

---

```luau
| "Europe/Madrid"
```

---

```luau
| "Europe/Malta"
```

---

```luau
| "Europe/Mariehamn"
```

---

```luau
| "Europe/Minsk"
```

---

```luau
| "Europe/Monaco"
```

---

```luau
| "Europe/Moscow"
```

---

```luau
| "Europe/Nicosia"
```

---

```luau
| "Europe/Oslo"
```

---

```luau
| "Europe/Paris"
```

---

```luau
| "Europe/Podgorica"
```

---

```luau
| "Europe/Prague"
```

---

```luau
| "Europe/Riga"
```

---

```luau
| "Europe/Rome"
```

---

```luau
| "Europe/Samara"
```

---

```luau
| "Europe/San_Marino"
```

---

```luau
| "Europe/Sarajevo"
```

---

```luau
| "Europe/Saratov"
```

---

```luau
| "Europe/Simferopol"
```

---

```luau
| "Europe/Skopje"
```

---

```luau
| "Europe/Sofia"
```

---

```luau
| "Europe/Stockholm"
```

---

```luau
| "Europe/Tallinn"
```

---

```luau
| "Europe/Tirane"
```

---

```luau
| "Europe/Tiraspol"
```

---

```luau
| "Europe/Ulyanovsk"
```

---

```luau
| "Europe/Uzhgorod"
```

---

```luau
| "Europe/Vaduz"
```

---

```luau
| "Europe/Vatican"
```

---

```luau
| "Europe/Vienna"
```

---

```luau
| "Europe/Vilnius"
```

---

```luau
| "Europe/Volgograd"
```

---

```luau
| "Europe/Warsaw"
```

---

```luau
| "Europe/Zagreb"
```

---

```luau
| "Europe/Zaporozhye"
```

---

```luau
| "Europe/Zurich"
```

---

```luau
| "GB"
```

---

```luau
| "GB-Eire"
```

---

```luau
| "GMT"
```

---

```luau
| "GMT+0"
```

---

```luau
| "GMT-0"
```

---

```luau
| "GMT0"
```

---

```luau
| "Greenwich"
```

---

```luau
| "HST"
```

---

```luau
| "Hongkong"
```

---

```luau
| "Iceland"
```

---

```luau
| "Indian/Antananarivo"
```

---

```luau
| "Indian/Chagos"
```

---

```luau
| "Indian/Christmas"
```

---

```luau
| "Indian/Cocos"
```

---

```luau
| "Indian/Comoro"
```

---

```luau
| "Indian/Kerguelen"
```

---

```luau
| "Indian/Mahe"
```

---

```luau
| "Indian/Maldives"
```

---

```luau
| "Indian/Mauritius"
```

---

```luau
| "Indian/Mayotte"
```

---

```luau
| "Indian/Reunion"
```

---

```luau
| "Iran"
```

---

```luau
| "Israel"
```

---

```luau
| "Jamaica"
```

---

```luau
| "Japan"
```

---

```luau
| "Kwajalein"
```

---

```luau
| "Libya"
```

---

```luau
| "MET"
```

---

```luau
| "MST"
```

---

```luau
| "MST7MDT"
```

---

```luau
| "Mexico/BajaNorte"
```

---

```luau
| "Mexico/BajaSur"
```

---

```luau
| "Mexico/General"
```

---

```luau
| "NZ"
```

---

```luau
| "NZ-CHAT"
```

---

```luau
| "Navajo"
```

---

```luau
| "PRC"
```

---

```luau
| "PST8PDT"
```

---

```luau
| "Pacific/Apia"
```

---

```luau
| "Pacific/Auckland"
```

---

```luau
| "Pacific/Bougainville"
```

---

```luau
| "Pacific/Chatham"
```

---

```luau
| "Pacific/Chuuk"
```

---

```luau
| "Pacific/Easter"
```

---

```luau
| "Pacific/Efate"
```

---

```luau
| "Pacific/Enderbury"
```

---

```luau
| "Pacific/Fakaofo"
```

---

```luau
| "Pacific/Fiji"
```

---

```luau
| "Pacific/Funafuti"
```

---

```luau
| "Pacific/Galapagos"
```

---

```luau
| "Pacific/Gambier"
```

---

```luau
| "Pacific/Guadalcanal"
```

---

```luau
| "Pacific/Guam"
```

---

```luau
| "Pacific/Honolulu"
```

---

```luau
| "Pacific/Johnston"
```

---

```luau
| "Pacific/Kanton"
```

---

```luau
| "Pacific/Kiritimati"
```

---

```luau
| "Pacific/Kosrae"
```

---

```luau
| "Pacific/Kwajalein"
```

---

```luau
| "Pacific/Majuro"
```

---

```luau
| "Pacific/Marquesas"
```

---

```luau
| "Pacific/Midway"
```

---

```luau
| "Pacific/Nauru"
```

---

```luau
| "Pacific/Niue"
```

---

```luau
| "Pacific/Norfolk"
```

---

```luau
| "Pacific/Noumea"
```

---

```luau
| "Pacific/Pago_Pago"
```

---

```luau
| "Pacific/Palau"
```

---

```luau
| "Pacific/Pitcairn"
```

---

```luau
| "Pacific/Pohnpei"
```

---

```luau
| "Pacific/Ponape"
```

---

```luau
| "Pacific/Port_Moresby"
```

---

```luau
| "Pacific/Rarotonga"
```

---

```luau
| "Pacific/Saipan"
```

---

```luau
| "Pacific/Samoa"
```

---

```luau
| "Pacific/Tahiti"
```

---

```luau
| "Pacific/Tarawa"
```

---

```luau
| "Pacific/Tongatapu"
```

---

```luau
| "Pacific/Truk"
```

---

```luau
| "Pacific/Wake"
```

---

```luau
| "Pacific/Wallis"
```

---

```luau
| "Pacific/Yap"
```

---

```luau
| "Poland"
```

---

```luau
| "Portugal"
```

---

```luau
| "ROC"
```

---

```luau
| "ROK"
```

---

```luau
| "Singapore"
```

---

```luau
| "Turkey"
```

---

```luau
| "UCT"
```

---

```luau
| "US/Alaska"
```

---

```luau
| "US/Aleutian"
```

---

```luau
| "US/Arizona"
```

---

```luau
| "US/Central"
```

---

```luau
| "US/East-Indiana"
```

---

```luau
| "US/Eastern"
```

---

```luau
| "US/Hawaii"
```

---

```luau
| "US/Indiana-Starke"
```

---

```luau
| "US/Michigan"
```

---

```luau
| "US/Mountain"
```

---

```luau
| "US/Pacific"
```

---

```luau
| "US/Samoa"
```

---

```luau
| "UTC"
```

---

```luau
| "Universal"
```

---

```luau
| "W-SU"
```

---

```luau
| "WET"
```

---

```luau
| "Zulu"
```

---

Autogenerated from [std/time/datetime.luau](/.seal/typedefs/std/time/datetime.luau).

*seal* is best experienced with inline, in-editor documentation. Please see the linked typedefs file if this documentation is confusing, too verbose, or inaccurate.
