<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD024 -->

# time.datetime

`local datetime = require("@std/time/datetime")`

---

RFC_2822 = "%a, %d %b %Y %H: `%M:%S %z" :: "%a, %d %b %Y %H:%M:%S %z"`

---

RFC_3339 = "%Y-%m-%dT%H: `%M:%S%:z" :: "%Y-%m-%dT%H:%M:%S%:z"`

---

SHORT_TIME = "%H: `%M" :: "%H:%M"`

---

FULL_DATE_TIME = "%A, %B %d, %Y %H: `%M:%S" :: "%A, %B %d, %Y %H:%M:%S"`

---

LOGGING_24_HR = "%a %b %e %H: `%M:%S %Z %Y" :: "%a %b %e %H:%M:%S %Z %Y"`

---

LOGGING_12_HR = "%a %b %e %I: `%M:%S %p %Z %Y" :: "%a %b %e %I:%M:%S %p %Z %Y"`

---

["MM/DD/YYYY HH: `MM : (AM/PM)"] = "%m/%d/%Y %I:%M %p" :: "%m/%d/%Y %I:%M %p"`

---

["HH: `MM : (AM/PM)"] = "%I:%M %p" :: "%I:%M %p"`

---

AMERICAN_FULL_DATE_TIME = "%A, %B %d, %Y %I: `%M:%S %p" :: "%A, %B %d, %Y %I:%M:%S %p"`

---

datetime.from: `(timestamp: number, timezone: IanaTimezone?, nanos: number?)`

 Constructs a `DateTime` from right now (based on system time) in your local timezone.
 Constructs a `DateTime` from a Unix Timestamp and an `IanaTimezone`.

- `timezone` defaults to `"UTC"` if not specified
- `nanos` defaults to `0` if not specified

---

`export type` CommonFormatKeys

---

datetime.parse: `(source: string, format: string | CommonFormatKeys, timezone: IanaTimezone) -> DateTime`

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

datetime.years: `(years: number, relative_to: DateTime?) -> TimeSpan`

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

datetime.months: `(months: number, relative_to: DateTime?) -> TimeSpan`

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

datetime.days: `(days: number) -> TimeSpan`

 Constructs a `TimeSpan` from days. Assumes every day is 24 hours.

---

datetime.hours: `(hours: number) -> TimeSpan`

 Constructs a `TimeSpan` from hours.

---

datetime.minutes: `(minutes: number) -> TimeSpan`

 Constructs a `TimeSpan` from minutes.

---

datetime.seconds: `(seconds: number) -> TimeSpan`

 Constructs a `TimeSpan` from seconds.

---

datetime.milliseconds: `(milliseconds: number) -> TimeSpan`

 Constructs a `TimeSpan` from milliseconds.

---

`export type` IanaTimezone

---

IanaTimezone: `| "AUTO" -- timezone info already included in input data`

---

IanaTimezone: `| "Africa/Abidjan"`

---

IanaTimezone: `| "Africa/Accra"`

---

IanaTimezone: `| "Africa/Addis_Ababa"`

---

IanaTimezone: `| "Africa/Algiers"`

---

IanaTimezone: `| "Africa/Asmara"`

---

IanaTimezone: `| "Africa/Asmera"`

---

IanaTimezone: `| "Africa/Bamako"`

---

IanaTimezone: `| "Africa/Bangui"`

---

IanaTimezone: `| "Africa/Banjul"`

---

IanaTimezone: `| "Africa/Bissau"`

---

IanaTimezone: `| "Africa/Blantyre"`

---

IanaTimezone: `| "Africa/Brazzaville"`

---

IanaTimezone: `| "Africa/Bujumbura"`

---

IanaTimezone: `| "Africa/Cairo"`

---

IanaTimezone: `| "Africa/Casablanca"`

---

IanaTimezone: `| "Africa/Ceuta"`

---

IanaTimezone: `| "Africa/Conakry"`

---

IanaTimezone: `| "Africa/Dakar"`

---

IanaTimezone: `| "Africa/Dar_es_Salaam"`

---

IanaTimezone: `| "Africa/Djibouti"`

---

IanaTimezone: `| "Africa/Douala"`

---

IanaTimezone: `| "Africa/El_Aaiun"`

---

IanaTimezone: `| "Africa/Freetown"`

---

IanaTimezone: `| "Africa/Gaborone"`

---

IanaTimezone: `| "Africa/Harare"`

---

IanaTimezone: `| "Africa/Johannesburg"`

---

IanaTimezone: `| "Africa/Juba"`

---

IanaTimezone: `| "Africa/Kampala"`

---

IanaTimezone: `| "Africa/Khartoum"`

---

IanaTimezone: `| "Africa/Kigali"`

---

IanaTimezone: `| "Africa/Kinshasa"`

---

IanaTimezone: `| "Africa/Lagos"`

---

IanaTimezone: `| "Africa/Libreville"`

---

IanaTimezone: `| "Africa/Lome"`

---

IanaTimezone: `| "Africa/Luanda"`

---

IanaTimezone: `| "Africa/Lubumbashi"`

---

IanaTimezone: `| "Africa/Lusaka"`

---

IanaTimezone: `| "Africa/Malabo"`

---

IanaTimezone: `| "Africa/Maputo"`

---

IanaTimezone: `| "Africa/Maseru"`

---

IanaTimezone: `| "Africa/Mbabane"`

---

IanaTimezone: `| "Africa/Mogadishu"`

---

IanaTimezone: `| "Africa/Monrovia"`

---

IanaTimezone: `| "Africa/Nairobi"`

---

IanaTimezone: `| "Africa/Ndjamena"`

---

IanaTimezone: `| "Africa/Niamey"`

---

IanaTimezone: `| "Africa/Nouakchott"`

---

IanaTimezone: `| "Africa/Ouagadougou"`

---

IanaTimezone: `| "Africa/Porto-Novo"`

---

IanaTimezone: `| "Africa/Sao_Tome"`

---

IanaTimezone: `| "Africa/Timbuktu"`

---

IanaTimezone: `| "Africa/Tripoli"`

---

IanaTimezone: `| "Africa/Tunis"`

---

IanaTimezone: `| "Africa/Windhoek"`

---

IanaTimezone: `| "America/Adak"`

---

IanaTimezone: `| "America/Anchorage"`

---

IanaTimezone: `| "America/Anguilla"`

---

IanaTimezone: `| "America/Antigua"`

---

IanaTimezone: `| "America/Araguaina"`

---

IanaTimezone: `| "America/Argentina/Buenos_Aires"`

---

IanaTimezone: `| "America/Argentina/Catamarca"`

---

IanaTimezone: `| "America/Argentina/ComodRivadavia"`

---

IanaTimezone: `| "America/Argentina/Cordoba"`

---

IanaTimezone: `| "America/Argentina/Jujuy"`

---

IanaTimezone: `| "America/Argentina/La_Rioja"`

---

IanaTimezone: `| "America/Argentina/Mendoza"`

---

IanaTimezone: `| "America/Argentina/Rio_Gallegos"`

---

IanaTimezone: `| "America/Argentina/Salta"`

---

IanaTimezone: `| "America/Argentina/San_Juan"`

---

IanaTimezone: `| "America/Argentina/San_Luis"`

---

IanaTimezone: `| "America/Argentina/Tucuman"`

---

IanaTimezone: `| "America/Argentina/Ushuaia"`

---

IanaTimezone: `| "America/Aruba"`

---

IanaTimezone: `| "America/Asuncion"`

---

IanaTimezone: `| "America/Atikokan"`

---

IanaTimezone: `| "America/Atka"`

---

IanaTimezone: `| "America/Bahia"`

---

IanaTimezone: `| "America/Bahia_Banderas"`

---

IanaTimezone: `| "America/Barbados"`

---

IanaTimezone: `| "America/Belem"`

---

IanaTimezone: `| "America/Belize"`

---

IanaTimezone: `| "America/Blanc-Sablon"`

---

IanaTimezone: `| "America/Boa_Vista"`

---

IanaTimezone: `| "America/Bogota"`

---

IanaTimezone: `| "America/Boise"`

---

IanaTimezone: `| "America/Buenos_Aires"`

---

IanaTimezone: `| "America/Cambridge_Bay"`

---

IanaTimezone: `| "America/Campo_Grande"`

---

IanaTimezone: `| "America/Cancun"`

---

IanaTimezone: `| "America/Caracas"`

---

IanaTimezone: `| "America/Catamarca"`

---

IanaTimezone: `| "America/Cayenne"`

---

IanaTimezone: `| "America/Cayman"`

---

IanaTimezone: `| "America/Chicago"`

---

IanaTimezone: `| "America/Chihuahua"`

---

IanaTimezone: `| "America/Ciudad_Juarez"`

---

IanaTimezone: `| "America/Coral_Harbour"`

---

IanaTimezone: `| "America/Cordoba"`

---

IanaTimezone: `| "America/Costa_Rica"`

---

IanaTimezone: `| "America/Creston"`

---

IanaTimezone: `| "America/Cuiaba"`

---

IanaTimezone: `| "America/Curacao"`

---

IanaTimezone: `| "America/Danmarkshavn"`

---

IanaTimezone: `| "America/Dawson"`

---

IanaTimezone: `| "America/Dawson_Creek"`

---

IanaTimezone: `| "America/Denver"`

---

IanaTimezone: `| "America/Detroit"`

---

IanaTimezone: `| "America/Dominica"`

---

IanaTimezone: `| "America/Edmonton"`

---

IanaTimezone: `| "America/Eirunepe"`

---

IanaTimezone: `| "America/El_Salvador"`

---

IanaTimezone: `| "America/Ensenada"`

---

IanaTimezone: `| "America/Fort_Nelson"`

---

IanaTimezone: `| "America/Fort_Wayne"`

---

IanaTimezone: `| "America/Fortaleza"`

---

IanaTimezone: `| "America/Glace_Bay"`

---

IanaTimezone: `| "America/Godthab"`

---

IanaTimezone: `| "America/Goose_Bay"`

---

IanaTimezone: `| "America/Grand_Turk"`

---

IanaTimezone: `| "America/Grenada"`

---

IanaTimezone: `| "America/Guadeloupe"`

---

IanaTimezone: `| "America/Guatemala"`

---

IanaTimezone: `| "America/Guayaquil"`

---

IanaTimezone: `| "America/Guyana"`

---

IanaTimezone: `| "America/Halifax"`

---

IanaTimezone: `| "America/Havana"`

---

IanaTimezone: `| "America/Hermosillo"`

---

IanaTimezone: `| "America/Indiana/Indianapolis"`

---

IanaTimezone: `| "America/Indiana/Knox"`

---

IanaTimezone: `| "America/Indiana/Marengo"`

---

IanaTimezone: `| "America/Indiana/Petersburg"`

---

IanaTimezone: `| "America/Indiana/Tell_City"`

---

IanaTimezone: `| "America/Indiana/Vevay"`

---

IanaTimezone: `| "America/Indiana/Vincennes"`

---

IanaTimezone: `| "America/Indiana/Winamac"`

---

IanaTimezone: `| "America/Indianapolis"`

---

IanaTimezone: `| "America/Inuvik"`

---

IanaTimezone: `| "America/Iqaluit"`

---

IanaTimezone: `| "America/Jamaica"`

---

IanaTimezone: `| "America/Jujuy"`

---

IanaTimezone: `| "America/Juneau"`

---

IanaTimezone: `| "America/Kentucky/Louisville"`

---

IanaTimezone: `| "America/Kentucky/Monticello"`

---

IanaTimezone: `| "America/Knox_IN"`

---

IanaTimezone: `| "America/Kralendijk"`

---

IanaTimezone: `| "America/La_Paz"`

---

IanaTimezone: `| "America/Lima"`

---

IanaTimezone: `| "America/Los_Angeles"`

---

IanaTimezone: `| "America/Louisville"`

---

IanaTimezone: `| "America/Lower_Princes"`

---

IanaTimezone: `| "America/Maceio"`

---

IanaTimezone: `| "America/Managua"`

---

IanaTimezone: `| "America/Manaus"`

---

IanaTimezone: `| "America/Marigot"`

---

IanaTimezone: `| "America/Martinique"`

---

IanaTimezone: `| "America/Matamoros"`

---

IanaTimezone: `| "America/Mazatlan"`

---

IanaTimezone: `| "America/Mendoza"`

---

IanaTimezone: `| "America/Menominee"`

---

IanaTimezone: `| "America/Merida"`

---

IanaTimezone: `| "America/Metlakatla"`

---

IanaTimezone: `| "America/Mexico_City"`

---

IanaTimezone: `| "America/Miquelon"`

---

IanaTimezone: `| "America/Moncton"`

---

IanaTimezone: `| "America/Monterrey"`

---

IanaTimezone: `| "America/Montevideo"`

---

IanaTimezone: `| "America/Montreal"`

---

IanaTimezone: `| "America/Montserrat"`

---

IanaTimezone: `| "America/Nassau"`

---

IanaTimezone: `| "America/New_York"`

---

IanaTimezone: `| "America/Nipigon"`

---

IanaTimezone: `| "America/Nome"`

---

IanaTimezone: `| "America/Noronha"`

---

IanaTimezone: `| "America/North_Dakota/Beulah"`

---

IanaTimezone: `| "America/North_Dakota/Center"`

---

IanaTimezone: `| "America/North_Dakota/New_Salem"`

---

IanaTimezone: `| "America/Nuuk"`

---

IanaTimezone: `| "America/Ojinaga"`

---

IanaTimezone: `| "America/Panama"`

---

IanaTimezone: `| "America/Pangnirtung"`

---

IanaTimezone: `| "America/Paramaribo"`

---

IanaTimezone: `| "America/Phoenix"`

---

IanaTimezone: `| "America/Port-au-Prince"`

---

IanaTimezone: `| "America/Port_of_Spain"`

---

IanaTimezone: `| "America/Porto_Acre"`

---

IanaTimezone: `| "America/Porto_Velho"`

---

IanaTimezone: `| "America/Puerto_Rico"`

---

IanaTimezone: `| "America/Punta_Arenas"`

---

IanaTimezone: `| "America/Rainy_River"`

---

IanaTimezone: `| "America/Rankin_Inlet"`

---

IanaTimezone: `| "America/Recife"`

---

IanaTimezone: `| "America/Regina"`

---

IanaTimezone: `| "America/Resolute"`

---

IanaTimezone: `| "America/Rio_Branco"`

---

IanaTimezone: `| "America/Rosario"`

---

IanaTimezone: `| "America/Santa_Isabel"`

---

IanaTimezone: `| "America/Santarem"`

---

IanaTimezone: `| "America/Santiago"`

---

IanaTimezone: `| "America/Santo_Domingo"`

---

IanaTimezone: `| "America/Sao_Paulo"`

---

IanaTimezone: `| "America/Scoresbysund"`

---

IanaTimezone: `| "America/Shiprock"`

---

IanaTimezone: `| "America/Sitka"`

---

IanaTimezone: `| "America/St_Barthelemy"`

---

IanaTimezone: `| "America/St_Johns"`

---

IanaTimezone: `| "America/St_Kitts"`

---

IanaTimezone: `| "America/St_Lucia"`

---

IanaTimezone: `| "America/St_Thomas"`

---

IanaTimezone: `| "America/St_Vincent"`

---

IanaTimezone: `| "America/Swift_Current"`

---

IanaTimezone: `| "America/Tegucigalpa"`

---

IanaTimezone: `| "America/Thule"`

---

IanaTimezone: `| "America/Thunder_Bay"`

---

IanaTimezone: `| "America/Tijuana"`

---

IanaTimezone: `| "America/Toronto"`

---

IanaTimezone: `| "America/Tortola"`

---

IanaTimezone: `| "America/Vancouver"`

---

IanaTimezone: `| "America/Virgin"`

---

IanaTimezone: `| "America/Whitehorse"`

---

IanaTimezone: `| "America/Winnipeg"`

---

IanaTimezone: `| "America/Yakutat"`

---

IanaTimezone: `| "America/Yellowknife"`

---

IanaTimezone: `| "Antarctica/Casey"`

---

IanaTimezone: `| "Antarctica/Davis"`

---

IanaTimezone: `| "Antarctica/DumontDUrville"`

---

IanaTimezone: `| "Antarctica/Macquarie"`

---

IanaTimezone: `| "Antarctica/Mawson"`

---

IanaTimezone: `| "Antarctica/McMurdo"`

---

IanaTimezone: `| "Antarctica/Palmer"`

---

IanaTimezone: `| "Antarctica/Rothera"`

---

IanaTimezone: `| "Antarctica/South_Pole"`

---

IanaTimezone: `| "Antarctica/Syowa"`

---

IanaTimezone: `| "Antarctica/Troll"`

---

IanaTimezone: `| "Antarctica/Vostok"`

---

IanaTimezone: `| "Arctic/Longyearbyen"`

---

IanaTimezone: `| "Asia/Aden"`

---

IanaTimezone: `| "Asia/Almaty"`

---

IanaTimezone: `| "Asia/Amman"`

---

IanaTimezone: `| "Asia/Anadyr"`

---

IanaTimezone: `| "Asia/Aqtau"`

---

IanaTimezone: `| "Asia/Aqtobe"`

---

IanaTimezone: `| "Asia/Ashgabat"`

---

IanaTimezone: `| "Asia/Ashkhabad"`

---

IanaTimezone: `| "Asia/Atyrau"`

---

IanaTimezone: `| "Asia/Baghdad"`

---

IanaTimezone: `| "Asia/Bahrain"`

---

IanaTimezone: `| "Asia/Baku"`

---

IanaTimezone: `| "Asia/Bangkok"`

---

IanaTimezone: `| "Asia/Barnaul"`

---

IanaTimezone: `| "Asia/Beirut"`

---

IanaTimezone: `| "Asia/Bishkek"`

---

IanaTimezone: `| "Asia/Brunei"`

---

IanaTimezone: `| "Asia/Calcutta"`

---

IanaTimezone: `| "Asia/Chita"`

---

IanaTimezone: `| "Asia/Choibalsan"`

---

IanaTimezone: `| "Asia/Chongqing"`

---

IanaTimezone: `| "Asia/Chungking"`

---

IanaTimezone: `| "Asia/Colombo"`

---

IanaTimezone: `| "Asia/Dacca"`

---

IanaTimezone: `| "Asia/Damascus"`

---

IanaTimezone: `| "Asia/Dhaka"`

---

IanaTimezone: `| "Asia/Dili"`

---

IanaTimezone: `| "Asia/Dubai"`

---

IanaTimezone: `| "Asia/Dushanbe"`

---

IanaTimezone: `| "Asia/Famagusta"`

---

IanaTimezone: `| "Asia/Gaza"`

---

IanaTimezone: `| "Asia/Harbin"`

---

IanaTimezone: `| "Asia/Hebron"`

---

IanaTimezone: `| "Asia/Ho_Chi_Minh"`

---

IanaTimezone: `| "Asia/Hong_Kong"`

---

IanaTimezone: `| "Asia/Hovd"`

---

IanaTimezone: `| "Asia/Irkutsk"`

---

IanaTimezone: `| "Asia/Istanbul"`

---

IanaTimezone: `| "Asia/Jakarta"`

---

IanaTimezone: `| "Asia/Jayapura"`

---

IanaTimezone: `| "Asia/Jerusalem"`

---

IanaTimezone: `| "Asia/Kabul"`

---

IanaTimezone: `| "Asia/Kamchatka"`

---

IanaTimezone: `| "Asia/Karachi"`

---

IanaTimezone: `| "Asia/Kashgar"`

---

IanaTimezone: `| "Asia/Kathmandu"`

---

IanaTimezone: `| "Asia/Katmandu"`

---

IanaTimezone: `| "Asia/Khandyga"`

---

IanaTimezone: `| "Asia/Kolkata"`

---

IanaTimezone: `| "Asia/Krasnoyarsk"`

---

IanaTimezone: `| "Asia/Kuala_Lumpur"`

---

IanaTimezone: `| "Asia/Kuching"`

---

IanaTimezone: `| "Asia/Kuwait"`

---

IanaTimezone: `| "Asia/Macao"`

---

IanaTimezone: `| "Asia/Macau"`

---

IanaTimezone: `| "Asia/Magadan"`

---

IanaTimezone: `| "Asia/Makassar"`

---

IanaTimezone: `| "Asia/Manila"`

---

IanaTimezone: `| "Asia/Muscat"`

---

IanaTimezone: `| "Asia/Nicosia"`

---

IanaTimezone: `| "Asia/Novokuznetsk"`

---

IanaTimezone: `| "Asia/Novosibirsk"`

---

IanaTimezone: `| "Asia/Omsk"`

---

IanaTimezone: `| "Asia/Oral"`

---

IanaTimezone: `| "Asia/Phnom_Penh"`

---

IanaTimezone: `| "Asia/Pontianak"`

---

IanaTimezone: `| "Asia/Pyongyang"`

---

IanaTimezone: `| "Asia/Qatar"`

---

IanaTimezone: `| "Asia/Qostanay"`

---

IanaTimezone: `| "Asia/Qyzylorda"`

---

IanaTimezone: `| "Asia/Rangoon"`

---

IanaTimezone: `| "Asia/Riyadh"`

---

IanaTimezone: `| "Asia/Saigon"`

---

IanaTimezone: `| "Asia/Sakhalin"`

---

IanaTimezone: `| "Asia/Samarkand"`

---

IanaTimezone: `| "Asia/Seoul"`

---

IanaTimezone: `| "Asia/Shanghai"`

---

IanaTimezone: `| "Asia/Singapore"`

---

IanaTimezone: `| "Asia/Srednekolymsk"`

---

IanaTimezone: `| "Asia/Taipei"`

---

IanaTimezone: `| "Asia/Tashkent"`

---

IanaTimezone: `| "Asia/Tbilisi"`

---

IanaTimezone: `| "Asia/Tehran"`

---

IanaTimezone: `| "Asia/Tel_Aviv"`

---

IanaTimezone: `| "Asia/Thimbu"`

---

IanaTimezone: `| "Asia/Thimphu"`

---

IanaTimezone: `| "Asia/Tokyo"`

---

IanaTimezone: `| "Asia/Tomsk"`

---

IanaTimezone: `| "Asia/Ujung_Pandang"`

---

IanaTimezone: `| "Asia/Ulaanbaatar"`

---

IanaTimezone: `| "Asia/Ulan_Bator"`

---

IanaTimezone: `| "Asia/Urumqi"`

---

IanaTimezone: `| "Asia/Ust-Nera"`

---

IanaTimezone: `| "Asia/Vientiane"`

---

IanaTimezone: `| "Asia/Vladivostok"`

---

IanaTimezone: `| "Asia/Yakutsk"`

---

IanaTimezone: `| "Asia/Yangon"`

---

IanaTimezone: `| "Asia/Yekaterinburg"`

---

IanaTimezone: `| "Asia/Yerevan"`

---

IanaTimezone: `| "Atlantic/Azores"`

---

IanaTimezone: `| "Atlantic/Bermuda"`

---

IanaTimezone: `| "Atlantic/Canary"`

---

IanaTimezone: `| "Atlantic/Cape_Verde"`

---

IanaTimezone: `| "Atlantic/Faeroe"`

---

IanaTimezone: `| "Atlantic/Faroe"`

---

IanaTimezone: `| "Atlantic/Jan_Mayen"`

---

IanaTimezone: `| "Atlantic/Madeira"`

---

IanaTimezone: `| "Atlantic/Reykjavik"`

---

IanaTimezone: `| "Atlantic/South_Georgia"`

---

IanaTimezone: `| "Atlantic/St_Helena"`

---

IanaTimezone: `| "Atlantic/Stanley"`

---

IanaTimezone: `| "Australia/ACT"`

---

IanaTimezone: `| "Australia/Adelaide"`

---

IanaTimezone: `| "Australia/Brisbane"`

---

IanaTimezone: `| "Australia/Broken_Hill"`

---

IanaTimezone: `| "Australia/Canberra"`

---

IanaTimezone: `| "Australia/Currie"`

---

IanaTimezone: `| "Australia/Darwin"`

---

IanaTimezone: `| "Australia/Eucla"`

---

IanaTimezone: `| "Australia/Hobart"`

---

IanaTimezone: `| "Australia/LHI"`

---

IanaTimezone: `| "Australia/Lindeman"`

---

IanaTimezone: `| "Australia/Lord_Howe"`

---

IanaTimezone: `| "Australia/Melbourne"`

---

IanaTimezone: `| "Australia/NSW"`

---

IanaTimezone: `| "Australia/North"`

---

IanaTimezone: `| "Australia/Perth"`

---

IanaTimezone: `| "Australia/Queensland"`

---

IanaTimezone: `| "Australia/South"`

---

IanaTimezone: `| "Australia/Sydney"`

---

IanaTimezone: `| "Australia/Tasmania"`

---

IanaTimezone: `| "Australia/Victoria"`

---

IanaTimezone: `| "Australia/West"`

---

IanaTimezone: `| "Australia/Yancowinna"`

---

IanaTimezone: `| "Brazil/Acre"`

---

IanaTimezone: `| "Brazil/DeNoronha"`

---

IanaTimezone: `| "Brazil/East"`

---

IanaTimezone: `| "Brazil/West"`

---

IanaTimezone: `| "CET"`

---

IanaTimezone: `| "CST6CDT"`

---

IanaTimezone: `| "Canada/Atlantic"`

---

IanaTimezone: `| "Canada/Central"`

---

IanaTimezone: `| "Canada/Eastern"`

---

IanaTimezone: `| "Canada/Mountain"`

---

IanaTimezone: `| "Canada/Newfoundland"`

---

IanaTimezone: `| "Canada/Pacific"`

---

IanaTimezone: `| "Canada/Saskatchewan"`

---

IanaTimezone: `| "Canada/Yukon"`

---

IanaTimezone: `| "Chile/Continental"`

---

IanaTimezone: `| "Chile/EasterIsland"`

---

IanaTimezone: `| "Cuba"`

---

IanaTimezone: `| "EET"`

---

IanaTimezone: `| "EST"`

---

IanaTimezone: `| "EST5EDT"`

---

IanaTimezone: `| "Egypt"`

---

IanaTimezone: `| "Eire"`

---

IanaTimezone: `| "Etc/GMT"`

---

IanaTimezone: `| "Etc/GMT+0"`

---

IanaTimezone: `| "Etc/GMT+1"`

---

IanaTimezone: `| "Etc/GMT+10"`

---

IanaTimezone: `| "Etc/GMT+11"`

---

IanaTimezone: `| "Etc/GMT+12"`

---

IanaTimezone: `| "Etc/GMT+2"`

---

IanaTimezone: `| "Etc/GMT+3"`

---

IanaTimezone: `| "Etc/GMT+4"`

---

IanaTimezone: `| "Etc/GMT+5"`

---

IanaTimezone: `| "Etc/GMT+6"`

---

IanaTimezone: `| "Etc/GMT+7"`

---

IanaTimezone: `| "Etc/GMT+8"`

---

IanaTimezone: `| "Etc/GMT+9"`

---

IanaTimezone: `| "Etc/GMT-0"`

---

IanaTimezone: `| "Etc/GMT-1"`

---

IanaTimezone: `| "Etc/GMT-10"`

---

IanaTimezone: `| "Etc/GMT-11"`

---

IanaTimezone: `| "Etc/GMT-12"`

---

IanaTimezone: `| "Etc/GMT-13"`

---

IanaTimezone: `| "Etc/GMT-14"`

---

IanaTimezone: `| "Etc/GMT-2"`

---

IanaTimezone: `| "Etc/GMT-3"`

---

IanaTimezone: `| "Etc/GMT-4"`

---

IanaTimezone: `| "Etc/GMT-5"`

---

IanaTimezone: `| "Etc/GMT-6"`

---

IanaTimezone: `| "Etc/GMT-7"`

---

IanaTimezone: `| "Etc/GMT-8"`

---

IanaTimezone: `| "Etc/GMT-9"`

---

IanaTimezone: `| "Etc/GMT0"`

---

IanaTimezone: `| "Etc/Greenwich"`

---

IanaTimezone: `| "Etc/UCT"`

---

IanaTimezone: `| "Etc/UTC"`

---

IanaTimezone: `| "Etc/Universal"`

---

IanaTimezone: `| "Etc/Zulu"`

---

IanaTimezone: `| "Europe/Amsterdam"`

---

IanaTimezone: `| "Europe/Andorra"`

---

IanaTimezone: `| "Europe/Astrakhan"`

---

IanaTimezone: `| "Europe/Athens"`

---

IanaTimezone: `| "Europe/Belfast"`

---

IanaTimezone: `| "Europe/Belgrade"`

---

IanaTimezone: `| "Europe/Berlin"`

---

IanaTimezone: `| "Europe/Bratislava"`

---

IanaTimezone: `| "Europe/Brussels"`

---

IanaTimezone: `| "Europe/Bucharest"`

---

IanaTimezone: `| "Europe/Budapest"`

---

IanaTimezone: `| "Europe/Busingen"`

---

IanaTimezone: `| "Europe/Chisinau"`

---

IanaTimezone: `| "Europe/Copenhagen"`

---

IanaTimezone: `| "Europe/Dublin"`

---

IanaTimezone: `| "Europe/Gibraltar"`

---

IanaTimezone: `| "Europe/Guernsey"`

---

IanaTimezone: `| "Europe/Helsinki"`

---

IanaTimezone: `| "Europe/Isle_of_Man"`

---

IanaTimezone: `| "Europe/Istanbul"`

---

IanaTimezone: `| "Europe/Jersey"`

---

IanaTimezone: `| "Europe/Kaliningrad"`

---

IanaTimezone: `| "Europe/Kiev"`

---

IanaTimezone: `| "Europe/Kirov"`

---

IanaTimezone: `| "Europe/Kyiv"`

---

IanaTimezone: `| "Europe/Lisbon"`

---

IanaTimezone: `| "Europe/Ljubljana"`

---

IanaTimezone: `| "Europe/London"`

---

IanaTimezone: `| "Europe/Luxembourg"`

---

IanaTimezone: `| "Europe/Madrid"`

---

IanaTimezone: `| "Europe/Malta"`

---

IanaTimezone: `| "Europe/Mariehamn"`

---

IanaTimezone: `| "Europe/Minsk"`

---

IanaTimezone: `| "Europe/Monaco"`

---

IanaTimezone: `| "Europe/Moscow"`

---

IanaTimezone: `| "Europe/Nicosia"`

---

IanaTimezone: `| "Europe/Oslo"`

---

IanaTimezone: `| "Europe/Paris"`

---

IanaTimezone: `| "Europe/Podgorica"`

---

IanaTimezone: `| "Europe/Prague"`

---

IanaTimezone: `| "Europe/Riga"`

---

IanaTimezone: `| "Europe/Rome"`

---

IanaTimezone: `| "Europe/Samara"`

---

IanaTimezone: `| "Europe/San_Marino"`

---

IanaTimezone: `| "Europe/Sarajevo"`

---

IanaTimezone: `| "Europe/Saratov"`

---

IanaTimezone: `| "Europe/Simferopol"`

---

IanaTimezone: `| "Europe/Skopje"`

---

IanaTimezone: `| "Europe/Sofia"`

---

IanaTimezone: `| "Europe/Stockholm"`

---

IanaTimezone: `| "Europe/Tallinn"`

---

IanaTimezone: `| "Europe/Tirane"`

---

IanaTimezone: `| "Europe/Tiraspol"`

---

IanaTimezone: `| "Europe/Ulyanovsk"`

---

IanaTimezone: `| "Europe/Uzhgorod"`

---

IanaTimezone: `| "Europe/Vaduz"`

---

IanaTimezone: `| "Europe/Vatican"`

---

IanaTimezone: `| "Europe/Vienna"`

---

IanaTimezone: `| "Europe/Vilnius"`

---

IanaTimezone: `| "Europe/Volgograd"`

---

IanaTimezone: `| "Europe/Warsaw"`

---

IanaTimezone: `| "Europe/Zagreb"`

---

IanaTimezone: `| "Europe/Zaporozhye"`

---

IanaTimezone: `| "Europe/Zurich"`

---

IanaTimezone: `| "GB"`

---

IanaTimezone: `| "GB-Eire"`

---

IanaTimezone: `| "GMT"`

---

IanaTimezone: `| "GMT+0"`

---

IanaTimezone: `| "GMT-0"`

---

IanaTimezone: `| "GMT0"`

---

IanaTimezone: `| "Greenwich"`

---

IanaTimezone: `| "HST"`

---

IanaTimezone: `| "Hongkong"`

---

IanaTimezone: `| "Iceland"`

---

IanaTimezone: `| "Indian/Antananarivo"`

---

IanaTimezone: `| "Indian/Chagos"`

---

IanaTimezone: `| "Indian/Christmas"`

---

IanaTimezone: `| "Indian/Cocos"`

---

IanaTimezone: `| "Indian/Comoro"`

---

IanaTimezone: `| "Indian/Kerguelen"`

---

IanaTimezone: `| "Indian/Mahe"`

---

IanaTimezone: `| "Indian/Maldives"`

---

IanaTimezone: `| "Indian/Mauritius"`

---

IanaTimezone: `| "Indian/Mayotte"`

---

IanaTimezone: `| "Indian/Reunion"`

---

IanaTimezone: `| "Iran"`

---

IanaTimezone: `| "Israel"`

---

IanaTimezone: `| "Jamaica"`

---

IanaTimezone: `| "Japan"`

---

IanaTimezone: `| "Kwajalein"`

---

IanaTimezone: `| "Libya"`

---

IanaTimezone: `| "MET"`

---

IanaTimezone: `| "MST"`

---

IanaTimezone: `| "MST7MDT"`

---

IanaTimezone: `| "Mexico/BajaNorte"`

---

IanaTimezone: `| "Mexico/BajaSur"`

---

IanaTimezone: `| "Mexico/General"`

---

IanaTimezone: `| "NZ"`

---

IanaTimezone: `| "NZ-CHAT"`

---

IanaTimezone: `| "Navajo"`

---

IanaTimezone: `| "PRC"`

---

IanaTimezone: `| "PST8PDT"`

---

IanaTimezone: `| "Pacific/Apia"`

---

IanaTimezone: `| "Pacific/Auckland"`

---

IanaTimezone: `| "Pacific/Bougainville"`

---

IanaTimezone: `| "Pacific/Chatham"`

---

IanaTimezone: `| "Pacific/Chuuk"`

---

IanaTimezone: `| "Pacific/Easter"`

---

IanaTimezone: `| "Pacific/Efate"`

---

IanaTimezone: `| "Pacific/Enderbury"`

---

IanaTimezone: `| "Pacific/Fakaofo"`

---

IanaTimezone: `| "Pacific/Fiji"`

---

IanaTimezone: `| "Pacific/Funafuti"`

---

IanaTimezone: `| "Pacific/Galapagos"`

---

IanaTimezone: `| "Pacific/Gambier"`

---

IanaTimezone: `| "Pacific/Guadalcanal"`

---

IanaTimezone: `| "Pacific/Guam"`

---

IanaTimezone: `| "Pacific/Honolulu"`

---

IanaTimezone: `| "Pacific/Johnston"`

---

IanaTimezone: `| "Pacific/Kanton"`

---

IanaTimezone: `| "Pacific/Kiritimati"`

---

IanaTimezone: `| "Pacific/Kosrae"`

---

IanaTimezone: `| "Pacific/Kwajalein"`

---

IanaTimezone: `| "Pacific/Majuro"`

---

IanaTimezone: `| "Pacific/Marquesas"`

---

IanaTimezone: `| "Pacific/Midway"`

---

IanaTimezone: `| "Pacific/Nauru"`

---

IanaTimezone: `| "Pacific/Niue"`

---

IanaTimezone: `| "Pacific/Norfolk"`

---

IanaTimezone: `| "Pacific/Noumea"`

---

IanaTimezone: `| "Pacific/Pago_Pago"`

---

IanaTimezone: `| "Pacific/Palau"`

---

IanaTimezone: `| "Pacific/Pitcairn"`

---

IanaTimezone: `| "Pacific/Pohnpei"`

---

IanaTimezone: `| "Pacific/Ponape"`

---

IanaTimezone: `| "Pacific/Port_Moresby"`

---

IanaTimezone: `| "Pacific/Rarotonga"`

---

IanaTimezone: `| "Pacific/Saipan"`

---

IanaTimezone: `| "Pacific/Samoa"`

---

IanaTimezone: `| "Pacific/Tahiti"`

---

IanaTimezone: `| "Pacific/Tarawa"`

---

IanaTimezone: `| "Pacific/Tongatapu"`

---

IanaTimezone: `| "Pacific/Truk"`

---

IanaTimezone: `| "Pacific/Wake"`

---

IanaTimezone: `| "Pacific/Wallis"`

---

IanaTimezone: `| "Pacific/Yap"`

---

IanaTimezone: `| "Poland"`

---

IanaTimezone: `| "Portugal"`

---

IanaTimezone: `| "ROC"`

---

IanaTimezone: `| "ROK"`

---

IanaTimezone: `| "Singapore"`

---

IanaTimezone: `| "Turkey"`

---

IanaTimezone: `| "UCT"`

---

IanaTimezone: `| "US/Alaska"`

---

IanaTimezone: `| "US/Aleutian"`

---

IanaTimezone: `| "US/Arizona"`

---

IanaTimezone: `| "US/Central"`

---

IanaTimezone: `| "US/East-Indiana"`

---

IanaTimezone: `| "US/Eastern"`

---

IanaTimezone: `| "US/Hawaii"`

---

IanaTimezone: `| "US/Indiana-Starke"`

---

IanaTimezone: `| "US/Michigan"`

---

IanaTimezone: `| "US/Mountain"`

---

IanaTimezone: `| "US/Pacific"`

---

IanaTimezone: `| "US/Samoa"`

---

IanaTimezone: `| "UTC"`

---

IanaTimezone: `| "Universal"`

---

IanaTimezone: `| "W-SU"`

---

IanaTimezone: `| "WET"`

---

IanaTimezone: `| "Zulu"`

---
