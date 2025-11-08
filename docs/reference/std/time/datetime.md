<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# time.datetime

`local datetime = require("@std/time/datetime")`

 Constructs a `DateTime` from right now (based on system time) in your local timezone.
 Constructs a `DateTime` from a Unix Timestamp and an `IanaTimezone`.

- `timezone` defaults to `"UTC"` if not specified
- `nanos` defaults to `0` if not specified

---

---

<h3>
```luau
CommonFormatKeys.parse: (source: string, format: string | CommonFormatKeys, timezone: IanaTimezone): DateTime
```
</h3>

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

<h3>
```luau
CommonFormatKeys.years: (years: number, relative_to: DateTime?): TimeSpan
```
</h3>

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

<h3>
```luau
CommonFormatKeys.months: (months: number, relative_to: DateTime?): TimeSpan
```
</h3>

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

<h3>
```luau
CommonFormatKeys.days: (days: number): TimeSpan
```
</h3>

 Constructs a `TimeSpan` from days. Assumes every day is 24 hours.

---

<h3>
```luau
CommonFormatKeys.hours: (hours: number): TimeSpan
```
</h3>

 Constructs a `TimeSpan` from hours.

---

<h3>
```luau
CommonFormatKeys.minutes: (minutes: number): TimeSpan
```
</h3>

 Constructs a `TimeSpan` from minutes.

---

<h3>
```luau
CommonFormatKeys.seconds: (seconds: number): TimeSpan
```
</h3>

 Constructs a `TimeSpan` from seconds.

---

<h3>
```luau
CommonFormatKeys.milliseconds: (milliseconds: number): TimeSpan
```
</h3>

 Constructs a `TimeSpan` from milliseconds.

---

---

<h3>
```luau
| "AUTO" -- timezone info already included in input data
```
</h3>

---

<h3>
```luau
| "Africa/Abidjan"
```
</h3>

---

<h3>
```luau
| "Africa/Accra"
```
</h3>

---

<h3>
```luau
| "Africa/Addis_Ababa"
```
</h3>

---

<h3>
```luau
| "Africa/Algiers"
```
</h3>

---

<h3>
```luau
| "Africa/Asmara"
```
</h3>

---

<h3>
```luau
| "Africa/Asmera"
```
</h3>

---

<h3>
```luau
| "Africa/Bamako"
```
</h3>

---

<h3>
```luau
| "Africa/Bangui"
```
</h3>

---

<h3>
```luau
| "Africa/Banjul"
```
</h3>

---

<h3>
```luau
| "Africa/Bissau"
```
</h3>

---

<h3>
```luau
| "Africa/Blantyre"
```
</h3>

---

<h3>
```luau
| "Africa/Brazzaville"
```
</h3>

---

<h3>
```luau
| "Africa/Bujumbura"
```
</h3>

---

<h3>
```luau
| "Africa/Cairo"
```
</h3>

---

<h3>
```luau
| "Africa/Casablanca"
```
</h3>

---

<h3>
```luau
| "Africa/Ceuta"
```
</h3>

---

<h3>
```luau
| "Africa/Conakry"
```
</h3>

---

<h3>
```luau
| "Africa/Dakar"
```
</h3>

---

<h3>
```luau
| "Africa/Dar_es_Salaam"
```
</h3>

---

<h3>
```luau
| "Africa/Djibouti"
```
</h3>

---

<h3>
```luau
| "Africa/Douala"
```
</h3>

---

<h3>
```luau
| "Africa/El_Aaiun"
```
</h3>

---

<h3>
```luau
| "Africa/Freetown"
```
</h3>

---

<h3>
```luau
| "Africa/Gaborone"
```
</h3>

---

<h3>
```luau
| "Africa/Harare"
```
</h3>

---

<h3>
```luau
| "Africa/Johannesburg"
```
</h3>

---

<h3>
```luau
| "Africa/Juba"
```
</h3>

---

<h3>
```luau
| "Africa/Kampala"
```
</h3>

---

<h3>
```luau
| "Africa/Khartoum"
```
</h3>

---

<h3>
```luau
| "Africa/Kigali"
```
</h3>

---

<h3>
```luau
| "Africa/Kinshasa"
```
</h3>

---

<h3>
```luau
| "Africa/Lagos"
```
</h3>

---

<h3>
```luau
| "Africa/Libreville"
```
</h3>

---

<h3>
```luau
| "Africa/Lome"
```
</h3>

---

<h3>
```luau
| "Africa/Luanda"
```
</h3>

---

<h3>
```luau
| "Africa/Lubumbashi"
```
</h3>

---

<h3>
```luau
| "Africa/Lusaka"
```
</h3>

---

<h3>
```luau
| "Africa/Malabo"
```
</h3>

---

<h3>
```luau
| "Africa/Maputo"
```
</h3>

---

<h3>
```luau
| "Africa/Maseru"
```
</h3>

---

<h3>
```luau
| "Africa/Mbabane"
```
</h3>

---

<h3>
```luau
| "Africa/Mogadishu"
```
</h3>

---

<h3>
```luau
| "Africa/Monrovia"
```
</h3>

---

<h3>
```luau
| "Africa/Nairobi"
```
</h3>

---

<h3>
```luau
| "Africa/Ndjamena"
```
</h3>

---

<h3>
```luau
| "Africa/Niamey"
```
</h3>

---

<h3>
```luau
| "Africa/Nouakchott"
```
</h3>

---

<h3>
```luau
| "Africa/Ouagadougou"
```
</h3>

---

<h3>
```luau
| "Africa/Porto-Novo"
```
</h3>

---

<h3>
```luau
| "Africa/Sao_Tome"
```
</h3>

---

<h3>
```luau
| "Africa/Timbuktu"
```
</h3>

---

<h3>
```luau
| "Africa/Tripoli"
```
</h3>

---

<h3>
```luau
| "Africa/Tunis"
```
</h3>

---

<h3>
```luau
| "Africa/Windhoek"
```
</h3>

---

<h3>
```luau
| "America/Adak"
```
</h3>

---

<h3>
```luau
| "America/Anchorage"
```
</h3>

---

<h3>
```luau
| "America/Anguilla"
```
</h3>

---

<h3>
```luau
| "America/Antigua"
```
</h3>

---

<h3>
```luau
| "America/Araguaina"
```
</h3>

---

<h3>
```luau
| "America/Argentina/Buenos_Aires"
```
</h3>

---

<h3>
```luau
| "America/Argentina/Catamarca"
```
</h3>

---

<h3>
```luau
| "America/Argentina/ComodRivadavia"
```
</h3>

---

<h3>
```luau
| "America/Argentina/Cordoba"
```
</h3>

---

<h3>
```luau
| "America/Argentina/Jujuy"
```
</h3>

---

<h3>
```luau
| "America/Argentina/La_Rioja"
```
</h3>

---

<h3>
```luau
| "America/Argentina/Mendoza"
```
</h3>

---

<h3>
```luau
| "America/Argentina/Rio_Gallegos"
```
</h3>

---

<h3>
```luau
| "America/Argentina/Salta"
```
</h3>

---

<h3>
```luau
| "America/Argentina/San_Juan"
```
</h3>

---

<h3>
```luau
| "America/Argentina/San_Luis"
```
</h3>

---

<h3>
```luau
| "America/Argentina/Tucuman"
```
</h3>

---

<h3>
```luau
| "America/Argentina/Ushuaia"
```
</h3>

---

<h3>
```luau
| "America/Aruba"
```
</h3>

---

<h3>
```luau
| "America/Asuncion"
```
</h3>

---

<h3>
```luau
| "America/Atikokan"
```
</h3>

---

<h3>
```luau
| "America/Atka"
```
</h3>

---

<h3>
```luau
| "America/Bahia"
```
</h3>

---

<h3>
```luau
| "America/Bahia_Banderas"
```
</h3>

---

<h3>
```luau
| "America/Barbados"
```
</h3>

---

<h3>
```luau
| "America/Belem"
```
</h3>

---

<h3>
```luau
| "America/Belize"
```
</h3>

---

<h3>
```luau
| "America/Blanc-Sablon"
```
</h3>

---

<h3>
```luau
| "America/Boa_Vista"
```
</h3>

---

<h3>
```luau
| "America/Bogota"
```
</h3>

---

<h3>
```luau
| "America/Boise"
```
</h3>

---

<h3>
```luau
| "America/Buenos_Aires"
```
</h3>

---

<h3>
```luau
| "America/Cambridge_Bay"
```
</h3>

---

<h3>
```luau
| "America/Campo_Grande"
```
</h3>

---

<h3>
```luau
| "America/Cancun"
```
</h3>

---

<h3>
```luau
| "America/Caracas"
```
</h3>

---

<h3>
```luau
| "America/Catamarca"
```
</h3>

---

<h3>
```luau
| "America/Cayenne"
```
</h3>

---

<h3>
```luau
| "America/Cayman"
```
</h3>

---

<h3>
```luau
| "America/Chicago"
```
</h3>

---

<h3>
```luau
| "America/Chihuahua"
```
</h3>

---

<h3>
```luau
| "America/Ciudad_Juarez"
```
</h3>

---

<h3>
```luau
| "America/Coral_Harbour"
```
</h3>

---

<h3>
```luau
| "America/Cordoba"
```
</h3>

---

<h3>
```luau
| "America/Costa_Rica"
```
</h3>

---

<h3>
```luau
| "America/Creston"
```
</h3>

---

<h3>
```luau
| "America/Cuiaba"
```
</h3>

---

<h3>
```luau
| "America/Curacao"
```
</h3>

---

<h3>
```luau
| "America/Danmarkshavn"
```
</h3>

---

<h3>
```luau
| "America/Dawson"
```
</h3>

---

<h3>
```luau
| "America/Dawson_Creek"
```
</h3>

---

<h3>
```luau
| "America/Denver"
```
</h3>

---

<h3>
```luau
| "America/Detroit"
```
</h3>

---

<h3>
```luau
| "America/Dominica"
```
</h3>

---

<h3>
```luau
| "America/Edmonton"
```
</h3>

---

<h3>
```luau
| "America/Eirunepe"
```
</h3>

---

<h3>
```luau
| "America/El_Salvador"
```
</h3>

---

<h3>
```luau
| "America/Ensenada"
```
</h3>

---

<h3>
```luau
| "America/Fort_Nelson"
```
</h3>

---

<h3>
```luau
| "America/Fort_Wayne"
```
</h3>

---

<h3>
```luau
| "America/Fortaleza"
```
</h3>

---

<h3>
```luau
| "America/Glace_Bay"
```
</h3>

---

<h3>
```luau
| "America/Godthab"
```
</h3>

---

<h3>
```luau
| "America/Goose_Bay"
```
</h3>

---

<h3>
```luau
| "America/Grand_Turk"
```
</h3>

---

<h3>
```luau
| "America/Grenada"
```
</h3>

---

<h3>
```luau
| "America/Guadeloupe"
```
</h3>

---

<h3>
```luau
| "America/Guatemala"
```
</h3>

---

<h3>
```luau
| "America/Guayaquil"
```
</h3>

---

<h3>
```luau
| "America/Guyana"
```
</h3>

---

<h3>
```luau
| "America/Halifax"
```
</h3>

---

<h3>
```luau
| "America/Havana"
```
</h3>

---

<h3>
```luau
| "America/Hermosillo"
```
</h3>

---

<h3>
```luau
| "America/Indiana/Indianapolis"
```
</h3>

---

<h3>
```luau
| "America/Indiana/Knox"
```
</h3>

---

<h3>
```luau
| "America/Indiana/Marengo"
```
</h3>

---

<h3>
```luau
| "America/Indiana/Petersburg"
```
</h3>

---

<h3>
```luau
| "America/Indiana/Tell_City"
```
</h3>

---

<h3>
```luau
| "America/Indiana/Vevay"
```
</h3>

---

<h3>
```luau
| "America/Indiana/Vincennes"
```
</h3>

---

<h3>
```luau
| "America/Indiana/Winamac"
```
</h3>

---

<h3>
```luau
| "America/Indianapolis"
```
</h3>

---

<h3>
```luau
| "America/Inuvik"
```
</h3>

---

<h3>
```luau
| "America/Iqaluit"
```
</h3>

---

<h3>
```luau
| "America/Jamaica"
```
</h3>

---

<h3>
```luau
| "America/Jujuy"
```
</h3>

---

<h3>
```luau
| "America/Juneau"
```
</h3>

---

<h3>
```luau
| "America/Kentucky/Louisville"
```
</h3>

---

<h3>
```luau
| "America/Kentucky/Monticello"
```
</h3>

---

<h3>
```luau
| "America/Knox_IN"
```
</h3>

---

<h3>
```luau
| "America/Kralendijk"
```
</h3>

---

<h3>
```luau
| "America/La_Paz"
```
</h3>

---

<h3>
```luau
| "America/Lima"
```
</h3>

---

<h3>
```luau
| "America/Los_Angeles"
```
</h3>

---

<h3>
```luau
| "America/Louisville"
```
</h3>

---

<h3>
```luau
| "America/Lower_Princes"
```
</h3>

---

<h3>
```luau
| "America/Maceio"
```
</h3>

---

<h3>
```luau
| "America/Managua"
```
</h3>

---

<h3>
```luau
| "America/Manaus"
```
</h3>

---

<h3>
```luau
| "America/Marigot"
```
</h3>

---

<h3>
```luau
| "America/Martinique"
```
</h3>

---

<h3>
```luau
| "America/Matamoros"
```
</h3>

---

<h3>
```luau
| "America/Mazatlan"
```
</h3>

---

<h3>
```luau
| "America/Mendoza"
```
</h3>

---

<h3>
```luau
| "America/Menominee"
```
</h3>

---

<h3>
```luau
| "America/Merida"
```
</h3>

---

<h3>
```luau
| "America/Metlakatla"
```
</h3>

---

<h3>
```luau
| "America/Mexico_City"
```
</h3>

---

<h3>
```luau
| "America/Miquelon"
```
</h3>

---

<h3>
```luau
| "America/Moncton"
```
</h3>

---

<h3>
```luau
| "America/Monterrey"
```
</h3>

---

<h3>
```luau
| "America/Montevideo"
```
</h3>

---

<h3>
```luau
| "America/Montreal"
```
</h3>

---

<h3>
```luau
| "America/Montserrat"
```
</h3>

---

<h3>
```luau
| "America/Nassau"
```
</h3>

---

<h3>
```luau
| "America/New_York"
```
</h3>

---

<h3>
```luau
| "America/Nipigon"
```
</h3>

---

<h3>
```luau
| "America/Nome"
```
</h3>

---

<h3>
```luau
| "America/Noronha"
```
</h3>

---

<h3>
```luau
| "America/North_Dakota/Beulah"
```
</h3>

---

<h3>
```luau
| "America/North_Dakota/Center"
```
</h3>

---

<h3>
```luau
| "America/North_Dakota/New_Salem"
```
</h3>

---

<h3>
```luau
| "America/Nuuk"
```
</h3>

---

<h3>
```luau
| "America/Ojinaga"
```
</h3>

---

<h3>
```luau
| "America/Panama"
```
</h3>

---

<h3>
```luau
| "America/Pangnirtung"
```
</h3>

---

<h3>
```luau
| "America/Paramaribo"
```
</h3>

---

<h3>
```luau
| "America/Phoenix"
```
</h3>

---

<h3>
```luau
| "America/Port-au-Prince"
```
</h3>

---

<h3>
```luau
| "America/Port_of_Spain"
```
</h3>

---

<h3>
```luau
| "America/Porto_Acre"
```
</h3>

---

<h3>
```luau
| "America/Porto_Velho"
```
</h3>

---

<h3>
```luau
| "America/Puerto_Rico"
```
</h3>

---

<h3>
```luau
| "America/Punta_Arenas"
```
</h3>

---

<h3>
```luau
| "America/Rainy_River"
```
</h3>

---

<h3>
```luau
| "America/Rankin_Inlet"
```
</h3>

---

<h3>
```luau
| "America/Recife"
```
</h3>

---

<h3>
```luau
| "America/Regina"
```
</h3>

---

<h3>
```luau
| "America/Resolute"
```
</h3>

---

<h3>
```luau
| "America/Rio_Branco"
```
</h3>

---

<h3>
```luau
| "America/Rosario"
```
</h3>

---

<h3>
```luau
| "America/Santa_Isabel"
```
</h3>

---

<h3>
```luau
| "America/Santarem"
```
</h3>

---

<h3>
```luau
| "America/Santiago"
```
</h3>

---

<h3>
```luau
| "America/Santo_Domingo"
```
</h3>

---

<h3>
```luau
| "America/Sao_Paulo"
```
</h3>

---

<h3>
```luau
| "America/Scoresbysund"
```
</h3>

---

<h3>
```luau
| "America/Shiprock"
```
</h3>

---

<h3>
```luau
| "America/Sitka"
```
</h3>

---

<h3>
```luau
| "America/St_Barthelemy"
```
</h3>

---

<h3>
```luau
| "America/St_Johns"
```
</h3>

---

<h3>
```luau
| "America/St_Kitts"
```
</h3>

---

<h3>
```luau
| "America/St_Lucia"
```
</h3>

---

<h3>
```luau
| "America/St_Thomas"
```
</h3>

---

<h3>
```luau
| "America/St_Vincent"
```
</h3>

---

<h3>
```luau
| "America/Swift_Current"
```
</h3>

---

<h3>
```luau
| "America/Tegucigalpa"
```
</h3>

---

<h3>
```luau
| "America/Thule"
```
</h3>

---

<h3>
```luau
| "America/Thunder_Bay"
```
</h3>

---

<h3>
```luau
| "America/Tijuana"
```
</h3>

---

<h3>
```luau
| "America/Toronto"
```
</h3>

---

<h3>
```luau
| "America/Tortola"
```
</h3>

---

<h3>
```luau
| "America/Vancouver"
```
</h3>

---

<h3>
```luau
| "America/Virgin"
```
</h3>

---

<h3>
```luau
| "America/Whitehorse"
```
</h3>

---

<h3>
```luau
| "America/Winnipeg"
```
</h3>

---

<h3>
```luau
| "America/Yakutat"
```
</h3>

---

<h3>
```luau
| "America/Yellowknife"
```
</h3>

---

<h3>
```luau
| "Antarctica/Casey"
```
</h3>

---

<h3>
```luau
| "Antarctica/Davis"
```
</h3>

---

<h3>
```luau
| "Antarctica/DumontDUrville"
```
</h3>

---

<h3>
```luau
| "Antarctica/Macquarie"
```
</h3>

---

<h3>
```luau
| "Antarctica/Mawson"
```
</h3>

---

<h3>
```luau
| "Antarctica/McMurdo"
```
</h3>

---

<h3>
```luau
| "Antarctica/Palmer"
```
</h3>

---

<h3>
```luau
| "Antarctica/Rothera"
```
</h3>

---

<h3>
```luau
| "Antarctica/South_Pole"
```
</h3>

---

<h3>
```luau
| "Antarctica/Syowa"
```
</h3>

---

<h3>
```luau
| "Antarctica/Troll"
```
</h3>

---

<h3>
```luau
| "Antarctica/Vostok"
```
</h3>

---

<h3>
```luau
| "Arctic/Longyearbyen"
```
</h3>

---

<h3>
```luau
| "Asia/Aden"
```
</h3>

---

<h3>
```luau
| "Asia/Almaty"
```
</h3>

---

<h3>
```luau
| "Asia/Amman"
```
</h3>

---

<h3>
```luau
| "Asia/Anadyr"
```
</h3>

---

<h3>
```luau
| "Asia/Aqtau"
```
</h3>

---

<h3>
```luau
| "Asia/Aqtobe"
```
</h3>

---

<h3>
```luau
| "Asia/Ashgabat"
```
</h3>

---

<h3>
```luau
| "Asia/Ashkhabad"
```
</h3>

---

<h3>
```luau
| "Asia/Atyrau"
```
</h3>

---

<h3>
```luau
| "Asia/Baghdad"
```
</h3>

---

<h3>
```luau
| "Asia/Bahrain"
```
</h3>

---

<h3>
```luau
| "Asia/Baku"
```
</h3>

---

<h3>
```luau
| "Asia/Bangkok"
```
</h3>

---

<h3>
```luau
| "Asia/Barnaul"
```
</h3>

---

<h3>
```luau
| "Asia/Beirut"
```
</h3>

---

<h3>
```luau
| "Asia/Bishkek"
```
</h3>

---

<h3>
```luau
| "Asia/Brunei"
```
</h3>

---

<h3>
```luau
| "Asia/Calcutta"
```
</h3>

---

<h3>
```luau
| "Asia/Chita"
```
</h3>

---

<h3>
```luau
| "Asia/Choibalsan"
```
</h3>

---

<h3>
```luau
| "Asia/Chongqing"
```
</h3>

---

<h3>
```luau
| "Asia/Chungking"
```
</h3>

---

<h3>
```luau
| "Asia/Colombo"
```
</h3>

---

<h3>
```luau
| "Asia/Dacca"
```
</h3>

---

<h3>
```luau
| "Asia/Damascus"
```
</h3>

---

<h3>
```luau
| "Asia/Dhaka"
```
</h3>

---

<h3>
```luau
| "Asia/Dili"
```
</h3>

---

<h3>
```luau
| "Asia/Dubai"
```
</h3>

---

<h3>
```luau
| "Asia/Dushanbe"
```
</h3>

---

<h3>
```luau
| "Asia/Famagusta"
```
</h3>

---

<h3>
```luau
| "Asia/Gaza"
```
</h3>

---

<h3>
```luau
| "Asia/Harbin"
```
</h3>

---

<h3>
```luau
| "Asia/Hebron"
```
</h3>

---

<h3>
```luau
| "Asia/Ho_Chi_Minh"
```
</h3>

---

<h3>
```luau
| "Asia/Hong_Kong"
```
</h3>

---

<h3>
```luau
| "Asia/Hovd"
```
</h3>

---

<h3>
```luau
| "Asia/Irkutsk"
```
</h3>

---

<h3>
```luau
| "Asia/Istanbul"
```
</h3>

---

<h3>
```luau
| "Asia/Jakarta"
```
</h3>

---

<h3>
```luau
| "Asia/Jayapura"
```
</h3>

---

<h3>
```luau
| "Asia/Jerusalem"
```
</h3>

---

<h3>
```luau
| "Asia/Kabul"
```
</h3>

---

<h3>
```luau
| "Asia/Kamchatka"
```
</h3>

---

<h3>
```luau
| "Asia/Karachi"
```
</h3>

---

<h3>
```luau
| "Asia/Kashgar"
```
</h3>

---

<h3>
```luau
| "Asia/Kathmandu"
```
</h3>

---

<h3>
```luau
| "Asia/Katmandu"
```
</h3>

---

<h3>
```luau
| "Asia/Khandyga"
```
</h3>

---

<h3>
```luau
| "Asia/Kolkata"
```
</h3>

---

<h3>
```luau
| "Asia/Krasnoyarsk"
```
</h3>

---

<h3>
```luau
| "Asia/Kuala_Lumpur"
```
</h3>

---

<h3>
```luau
| "Asia/Kuching"
```
</h3>

---

<h3>
```luau
| "Asia/Kuwait"
```
</h3>

---

<h3>
```luau
| "Asia/Macao"
```
</h3>

---

<h3>
```luau
| "Asia/Macau"
```
</h3>

---

<h3>
```luau
| "Asia/Magadan"
```
</h3>

---

<h3>
```luau
| "Asia/Makassar"
```
</h3>

---

<h3>
```luau
| "Asia/Manila"
```
</h3>

---

<h3>
```luau
| "Asia/Muscat"
```
</h3>

---

<h3>
```luau
| "Asia/Nicosia"
```
</h3>

---

<h3>
```luau
| "Asia/Novokuznetsk"
```
</h3>

---

<h3>
```luau
| "Asia/Novosibirsk"
```
</h3>

---

<h3>
```luau
| "Asia/Omsk"
```
</h3>

---

<h3>
```luau
| "Asia/Oral"
```
</h3>

---

<h3>
```luau
| "Asia/Phnom_Penh"
```
</h3>

---

<h3>
```luau
| "Asia/Pontianak"
```
</h3>

---

<h3>
```luau
| "Asia/Pyongyang"
```
</h3>

---

<h3>
```luau
| "Asia/Qatar"
```
</h3>

---

<h3>
```luau
| "Asia/Qostanay"
```
</h3>

---

<h3>
```luau
| "Asia/Qyzylorda"
```
</h3>

---

<h3>
```luau
| "Asia/Rangoon"
```
</h3>

---

<h3>
```luau
| "Asia/Riyadh"
```
</h3>

---

<h3>
```luau
| "Asia/Saigon"
```
</h3>

---

<h3>
```luau
| "Asia/Sakhalin"
```
</h3>

---

<h3>
```luau
| "Asia/Samarkand"
```
</h3>

---

<h3>
```luau
| "Asia/Seoul"
```
</h3>

---

<h3>
```luau
| "Asia/Shanghai"
```
</h3>

---

<h3>
```luau
| "Asia/Singapore"
```
</h3>

---

<h3>
```luau
| "Asia/Srednekolymsk"
```
</h3>

---

<h3>
```luau
| "Asia/Taipei"
```
</h3>

---

<h3>
```luau
| "Asia/Tashkent"
```
</h3>

---

<h3>
```luau
| "Asia/Tbilisi"
```
</h3>

---

<h3>
```luau
| "Asia/Tehran"
```
</h3>

---

<h3>
```luau
| "Asia/Tel_Aviv"
```
</h3>

---

<h3>
```luau
| "Asia/Thimbu"
```
</h3>

---

<h3>
```luau
| "Asia/Thimphu"
```
</h3>

---

<h3>
```luau
| "Asia/Tokyo"
```
</h3>

---

<h3>
```luau
| "Asia/Tomsk"
```
</h3>

---

<h3>
```luau
| "Asia/Ujung_Pandang"
```
</h3>

---

<h3>
```luau
| "Asia/Ulaanbaatar"
```
</h3>

---

<h3>
```luau
| "Asia/Ulan_Bator"
```
</h3>

---

<h3>
```luau
| "Asia/Urumqi"
```
</h3>

---

<h3>
```luau
| "Asia/Ust-Nera"
```
</h3>

---

<h3>
```luau
| "Asia/Vientiane"
```
</h3>

---

<h3>
```luau
| "Asia/Vladivostok"
```
</h3>

---

<h3>
```luau
| "Asia/Yakutsk"
```
</h3>

---

<h3>
```luau
| "Asia/Yangon"
```
</h3>

---

<h3>
```luau
| "Asia/Yekaterinburg"
```
</h3>

---

<h3>
```luau
| "Asia/Yerevan"
```
</h3>

---

<h3>
```luau
| "Atlantic/Azores"
```
</h3>

---

<h3>
```luau
| "Atlantic/Bermuda"
```
</h3>

---

<h3>
```luau
| "Atlantic/Canary"
```
</h3>

---

<h3>
```luau
| "Atlantic/Cape_Verde"
```
</h3>

---

<h3>
```luau
| "Atlantic/Faeroe"
```
</h3>

---

<h3>
```luau
| "Atlantic/Faroe"
```
</h3>

---

<h3>
```luau
| "Atlantic/Jan_Mayen"
```
</h3>

---

<h3>
```luau
| "Atlantic/Madeira"
```
</h3>

---

<h3>
```luau
| "Atlantic/Reykjavik"
```
</h3>

---

<h3>
```luau
| "Atlantic/South_Georgia"
```
</h3>

---

<h3>
```luau
| "Atlantic/St_Helena"
```
</h3>

---

<h3>
```luau
| "Atlantic/Stanley"
```
</h3>

---

<h3>
```luau
| "Australia/ACT"
```
</h3>

---

<h3>
```luau
| "Australia/Adelaide"
```
</h3>

---

<h3>
```luau
| "Australia/Brisbane"
```
</h3>

---

<h3>
```luau
| "Australia/Broken_Hill"
```
</h3>

---

<h3>
```luau
| "Australia/Canberra"
```
</h3>

---

<h3>
```luau
| "Australia/Currie"
```
</h3>

---

<h3>
```luau
| "Australia/Darwin"
```
</h3>

---

<h3>
```luau
| "Australia/Eucla"
```
</h3>

---

<h3>
```luau
| "Australia/Hobart"
```
</h3>

---

<h3>
```luau
| "Australia/LHI"
```
</h3>

---

<h3>
```luau
| "Australia/Lindeman"
```
</h3>

---

<h3>
```luau
| "Australia/Lord_Howe"
```
</h3>

---

<h3>
```luau
| "Australia/Melbourne"
```
</h3>

---

<h3>
```luau
| "Australia/NSW"
```
</h3>

---

<h3>
```luau
| "Australia/North"
```
</h3>

---

<h3>
```luau
| "Australia/Perth"
```
</h3>

---

<h3>
```luau
| "Australia/Queensland"
```
</h3>

---

<h3>
```luau
| "Australia/South"
```
</h3>

---

<h3>
```luau
| "Australia/Sydney"
```
</h3>

---

<h3>
```luau
| "Australia/Tasmania"
```
</h3>

---

<h3>
```luau
| "Australia/Victoria"
```
</h3>

---

<h3>
```luau
| "Australia/West"
```
</h3>

---

<h3>
```luau
| "Australia/Yancowinna"
```
</h3>

---

<h3>
```luau
| "Brazil/Acre"
```
</h3>

---

<h3>
```luau
| "Brazil/DeNoronha"
```
</h3>

---

<h3>
```luau
| "Brazil/East"
```
</h3>

---

<h3>
```luau
| "Brazil/West"
```
</h3>

---

<h3>
```luau
| "CET"
```
</h3>

---

<h3>
```luau
| "CST6CDT"
```
</h3>

---

<h3>
```luau
| "Canada/Atlantic"
```
</h3>

---

<h3>
```luau
| "Canada/Central"
```
</h3>

---

<h3>
```luau
| "Canada/Eastern"
```
</h3>

---

<h3>
```luau
| "Canada/Mountain"
```
</h3>

---

<h3>
```luau
| "Canada/Newfoundland"
```
</h3>

---

<h3>
```luau
| "Canada/Pacific"
```
</h3>

---

<h3>
```luau
| "Canada/Saskatchewan"
```
</h3>

---

<h3>
```luau
| "Canada/Yukon"
```
</h3>

---

<h3>
```luau
| "Chile/Continental"
```
</h3>

---

<h3>
```luau
| "Chile/EasterIsland"
```
</h3>

---

<h3>
```luau
| "Cuba"
```
</h3>

---

<h3>
```luau
| "EET"
```
</h3>

---

<h3>
```luau
| "EST"
```
</h3>

---

<h3>
```luau
| "EST5EDT"
```
</h3>

---

<h3>
```luau
| "Egypt"
```
</h3>

---

<h3>
```luau
| "Eire"
```
</h3>

---

<h3>
```luau
| "Etc/GMT"
```
</h3>

---

<h3>
```luau
| "Etc/GMT+0"
```
</h3>

---

<h3>
```luau
| "Etc/GMT+1"
```
</h3>

---

<h3>
```luau
| "Etc/GMT+10"
```
</h3>

---

<h3>
```luau
| "Etc/GMT+11"
```
</h3>

---

<h3>
```luau
| "Etc/GMT+12"
```
</h3>

---

<h3>
```luau
| "Etc/GMT+2"
```
</h3>

---

<h3>
```luau
| "Etc/GMT+3"
```
</h3>

---

<h3>
```luau
| "Etc/GMT+4"
```
</h3>

---

<h3>
```luau
| "Etc/GMT+5"
```
</h3>

---

<h3>
```luau
| "Etc/GMT+6"
```
</h3>

---

<h3>
```luau
| "Etc/GMT+7"
```
</h3>

---

<h3>
```luau
| "Etc/GMT+8"
```
</h3>

---

<h3>
```luau
| "Etc/GMT+9"
```
</h3>

---

<h3>
```luau
| "Etc/GMT-0"
```
</h3>

---

<h3>
```luau
| "Etc/GMT-1"
```
</h3>

---

<h3>
```luau
| "Etc/GMT-10"
```
</h3>

---

<h3>
```luau
| "Etc/GMT-11"
```
</h3>

---

<h3>
```luau
| "Etc/GMT-12"
```
</h3>

---

<h3>
```luau
| "Etc/GMT-13"
```
</h3>

---

<h3>
```luau
| "Etc/GMT-14"
```
</h3>

---

<h3>
```luau
| "Etc/GMT-2"
```
</h3>

---

<h3>
```luau
| "Etc/GMT-3"
```
</h3>

---

<h3>
```luau
| "Etc/GMT-4"
```
</h3>

---

<h3>
```luau
| "Etc/GMT-5"
```
</h3>

---

<h3>
```luau
| "Etc/GMT-6"
```
</h3>

---

<h3>
```luau
| "Etc/GMT-7"
```
</h3>

---

<h3>
```luau
| "Etc/GMT-8"
```
</h3>

---

<h3>
```luau
| "Etc/GMT-9"
```
</h3>

---

<h3>
```luau
| "Etc/GMT0"
```
</h3>

---

<h3>
```luau
| "Etc/Greenwich"
```
</h3>

---

<h3>
```luau
| "Etc/UCT"
```
</h3>

---

<h3>
```luau
| "Etc/UTC"
```
</h3>

---

<h3>
```luau
| "Etc/Universal"
```
</h3>

---

<h3>
```luau
| "Etc/Zulu"
```
</h3>

---

<h3>
```luau
| "Europe/Amsterdam"
```
</h3>

---

<h3>
```luau
| "Europe/Andorra"
```
</h3>

---

<h3>
```luau
| "Europe/Astrakhan"
```
</h3>

---

<h3>
```luau
| "Europe/Athens"
```
</h3>

---

<h3>
```luau
| "Europe/Belfast"
```
</h3>

---

<h3>
```luau
| "Europe/Belgrade"
```
</h3>

---

<h3>
```luau
| "Europe/Berlin"
```
</h3>

---

<h3>
```luau
| "Europe/Bratislava"
```
</h3>

---

<h3>
```luau
| "Europe/Brussels"
```
</h3>

---

<h3>
```luau
| "Europe/Bucharest"
```
</h3>

---

<h3>
```luau
| "Europe/Budapest"
```
</h3>

---

<h3>
```luau
| "Europe/Busingen"
```
</h3>

---

<h3>
```luau
| "Europe/Chisinau"
```
</h3>

---

<h3>
```luau
| "Europe/Copenhagen"
```
</h3>

---

<h3>
```luau
| "Europe/Dublin"
```
</h3>

---

<h3>
```luau
| "Europe/Gibraltar"
```
</h3>

---

<h3>
```luau
| "Europe/Guernsey"
```
</h3>

---

<h3>
```luau
| "Europe/Helsinki"
```
</h3>

---

<h3>
```luau
| "Europe/Isle_of_Man"
```
</h3>

---

<h3>
```luau
| "Europe/Istanbul"
```
</h3>

---

<h3>
```luau
| "Europe/Jersey"
```
</h3>

---

<h3>
```luau
| "Europe/Kaliningrad"
```
</h3>

---

<h3>
```luau
| "Europe/Kiev"
```
</h3>

---

<h3>
```luau
| "Europe/Kirov"
```
</h3>

---

<h3>
```luau
| "Europe/Kyiv"
```
</h3>

---

<h3>
```luau
| "Europe/Lisbon"
```
</h3>

---

<h3>
```luau
| "Europe/Ljubljana"
```
</h3>

---

<h3>
```luau
| "Europe/London"
```
</h3>

---

<h3>
```luau
| "Europe/Luxembourg"
```
</h3>

---

<h3>
```luau
| "Europe/Madrid"
```
</h3>

---

<h3>
```luau
| "Europe/Malta"
```
</h3>

---

<h3>
```luau
| "Europe/Mariehamn"
```
</h3>

---

<h3>
```luau
| "Europe/Minsk"
```
</h3>

---

<h3>
```luau
| "Europe/Monaco"
```
</h3>

---

<h3>
```luau
| "Europe/Moscow"
```
</h3>

---

<h3>
```luau
| "Europe/Nicosia"
```
</h3>

---

<h3>
```luau
| "Europe/Oslo"
```
</h3>

---

<h3>
```luau
| "Europe/Paris"
```
</h3>

---

<h3>
```luau
| "Europe/Podgorica"
```
</h3>

---

<h3>
```luau
| "Europe/Prague"
```
</h3>

---

<h3>
```luau
| "Europe/Riga"
```
</h3>

---

<h3>
```luau
| "Europe/Rome"
```
</h3>

---

<h3>
```luau
| "Europe/Samara"
```
</h3>

---

<h3>
```luau
| "Europe/San_Marino"
```
</h3>

---

<h3>
```luau
| "Europe/Sarajevo"
```
</h3>

---

<h3>
```luau
| "Europe/Saratov"
```
</h3>

---

<h3>
```luau
| "Europe/Simferopol"
```
</h3>

---

<h3>
```luau
| "Europe/Skopje"
```
</h3>

---

<h3>
```luau
| "Europe/Sofia"
```
</h3>

---

<h3>
```luau
| "Europe/Stockholm"
```
</h3>

---

<h3>
```luau
| "Europe/Tallinn"
```
</h3>

---

<h3>
```luau
| "Europe/Tirane"
```
</h3>

---

<h3>
```luau
| "Europe/Tiraspol"
```
</h3>

---

<h3>
```luau
| "Europe/Ulyanovsk"
```
</h3>

---

<h3>
```luau
| "Europe/Uzhgorod"
```
</h3>

---

<h3>
```luau
| "Europe/Vaduz"
```
</h3>

---

<h3>
```luau
| "Europe/Vatican"
```
</h3>

---

<h3>
```luau
| "Europe/Vienna"
```
</h3>

---

<h3>
```luau
| "Europe/Vilnius"
```
</h3>

---

<h3>
```luau
| "Europe/Volgograd"
```
</h3>

---

<h3>
```luau
| "Europe/Warsaw"
```
</h3>

---

<h3>
```luau
| "Europe/Zagreb"
```
</h3>

---

<h3>
```luau
| "Europe/Zaporozhye"
```
</h3>

---

<h3>
```luau
| "Europe/Zurich"
```
</h3>

---

<h3>
```luau
| "GB"
```
</h3>

---

<h3>
```luau
| "GB-Eire"
```
</h3>

---

<h3>
```luau
| "GMT"
```
</h3>

---

<h3>
```luau
| "GMT+0"
```
</h3>

---

<h3>
```luau
| "GMT-0"
```
</h3>

---

<h3>
```luau
| "GMT0"
```
</h3>

---

<h3>
```luau
| "Greenwich"
```
</h3>

---

<h3>
```luau
| "HST"
```
</h3>

---

<h3>
```luau
| "Hongkong"
```
</h3>

---

<h3>
```luau
| "Iceland"
```
</h3>

---

<h3>
```luau
| "Indian/Antananarivo"
```
</h3>

---

<h3>
```luau
| "Indian/Chagos"
```
</h3>

---

<h3>
```luau
| "Indian/Christmas"
```
</h3>

---

<h3>
```luau
| "Indian/Cocos"
```
</h3>

---

<h3>
```luau
| "Indian/Comoro"
```
</h3>

---

<h3>
```luau
| "Indian/Kerguelen"
```
</h3>

---

<h3>
```luau
| "Indian/Mahe"
```
</h3>

---

<h3>
```luau
| "Indian/Maldives"
```
</h3>

---

<h3>
```luau
| "Indian/Mauritius"
```
</h3>

---

<h3>
```luau
| "Indian/Mayotte"
```
</h3>

---

<h3>
```luau
| "Indian/Reunion"
```
</h3>

---

<h3>
```luau
| "Iran"
```
</h3>

---

<h3>
```luau
| "Israel"
```
</h3>

---

<h3>
```luau
| "Jamaica"
```
</h3>

---

<h3>
```luau
| "Japan"
```
</h3>

---

<h3>
```luau
| "Kwajalein"
```
</h3>

---

<h3>
```luau
| "Libya"
```
</h3>

---

<h3>
```luau
| "MET"
```
</h3>

---

<h3>
```luau
| "MST"
```
</h3>

---

<h3>
```luau
| "MST7MDT"
```
</h3>

---

<h3>
```luau
| "Mexico/BajaNorte"
```
</h3>

---

<h3>
```luau
| "Mexico/BajaSur"
```
</h3>

---

<h3>
```luau
| "Mexico/General"
```
</h3>

---

<h3>
```luau
| "NZ"
```
</h3>

---

<h3>
```luau
| "NZ-CHAT"
```
</h3>

---

<h3>
```luau
| "Navajo"
```
</h3>

---

<h3>
```luau
| "PRC"
```
</h3>

---

<h3>
```luau
| "PST8PDT"
```
</h3>

---

<h3>
```luau
| "Pacific/Apia"
```
</h3>

---

<h3>
```luau
| "Pacific/Auckland"
```
</h3>

---

<h3>
```luau
| "Pacific/Bougainville"
```
</h3>

---

<h3>
```luau
| "Pacific/Chatham"
```
</h3>

---

<h3>
```luau
| "Pacific/Chuuk"
```
</h3>

---

<h3>
```luau
| "Pacific/Easter"
```
</h3>

---

<h3>
```luau
| "Pacific/Efate"
```
</h3>

---

<h3>
```luau
| "Pacific/Enderbury"
```
</h3>

---

<h3>
```luau
| "Pacific/Fakaofo"
```
</h3>

---

<h3>
```luau
| "Pacific/Fiji"
```
</h3>

---

<h3>
```luau
| "Pacific/Funafuti"
```
</h3>

---

<h3>
```luau
| "Pacific/Galapagos"
```
</h3>

---

<h3>
```luau
| "Pacific/Gambier"
```
</h3>

---

<h3>
```luau
| "Pacific/Guadalcanal"
```
</h3>

---

<h3>
```luau
| "Pacific/Guam"
```
</h3>

---

<h3>
```luau
| "Pacific/Honolulu"
```
</h3>

---

<h3>
```luau
| "Pacific/Johnston"
```
</h3>

---

<h3>
```luau
| "Pacific/Kanton"
```
</h3>

---

<h3>
```luau
| "Pacific/Kiritimati"
```
</h3>

---

<h3>
```luau
| "Pacific/Kosrae"
```
</h3>

---

<h3>
```luau
| "Pacific/Kwajalein"
```
</h3>

---

<h3>
```luau
| "Pacific/Majuro"
```
</h3>

---

<h3>
```luau
| "Pacific/Marquesas"
```
</h3>

---

<h3>
```luau
| "Pacific/Midway"
```
</h3>

---

<h3>
```luau
| "Pacific/Nauru"
```
</h3>

---

<h3>
```luau
| "Pacific/Niue"
```
</h3>

---

<h3>
```luau
| "Pacific/Norfolk"
```
</h3>

---

<h3>
```luau
| "Pacific/Noumea"
```
</h3>

---

<h3>
```luau
| "Pacific/Pago_Pago"
```
</h3>

---

<h3>
```luau
| "Pacific/Palau"
```
</h3>

---

<h3>
```luau
| "Pacific/Pitcairn"
```
</h3>

---

<h3>
```luau
| "Pacific/Pohnpei"
```
</h3>

---

<h3>
```luau
| "Pacific/Ponape"
```
</h3>

---

<h3>
```luau
| "Pacific/Port_Moresby"
```
</h3>

---

<h3>
```luau
| "Pacific/Rarotonga"
```
</h3>

---

<h3>
```luau
| "Pacific/Saipan"
```
</h3>

---

<h3>
```luau
| "Pacific/Samoa"
```
</h3>

---

<h3>
```luau
| "Pacific/Tahiti"
```
</h3>

---

<h3>
```luau
| "Pacific/Tarawa"
```
</h3>

---

<h3>
```luau
| "Pacific/Tongatapu"
```
</h3>

---

<h3>
```luau
| "Pacific/Truk"
```
</h3>

---

<h3>
```luau
| "Pacific/Wake"
```
</h3>

---

<h3>
```luau
| "Pacific/Wallis"
```
</h3>

---

<h3>
```luau
| "Pacific/Yap"
```
</h3>

---

<h3>
```luau
| "Poland"
```
</h3>

---

<h3>
```luau
| "Portugal"
```
</h3>

---

<h3>
```luau
| "ROC"
```
</h3>

---

<h3>
```luau
| "ROK"
```
</h3>

---

<h3>
```luau
| "Singapore"
```
</h3>

---

<h3>
```luau
| "Turkey"
```
</h3>

---

<h3>
```luau
| "UCT"
```
</h3>

---

<h3>
```luau
| "US/Alaska"
```
</h3>

---

<h3>
```luau
| "US/Aleutian"
```
</h3>

---

<h3>
```luau
| "US/Arizona"
```
</h3>

---

<h3>
```luau
| "US/Central"
```
</h3>

---

<h3>
```luau
| "US/East-Indiana"
```
</h3>

---

<h3>
```luau
| "US/Eastern"
```
</h3>

---

<h3>
```luau
| "US/Hawaii"
```
</h3>

---

<h3>
```luau
| "US/Indiana-Starke"
```
</h3>

---

<h3>
```luau
| "US/Michigan"
```
</h3>

---

<h3>
```luau
| "US/Mountain"
```
</h3>

---

<h3>
```luau
| "US/Pacific"
```
</h3>

---

<h3>
```luau
| "US/Samoa"
```
</h3>

---

<h3>
```luau
| "UTC"
```
</h3>

---

<h3>
```luau
| "Universal"
```
</h3>

---

<h3>
```luau
| "W-SU"
```
</h3>

---

<h3>
```luau
| "WET"
```
</h3>

---

<h3>
```luau
| "Zulu"
```
</h3>

---
