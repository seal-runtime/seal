use std::{borrow::Cow, cell::RefCell, time::SystemTime};

use crate::prelude::*;
use crate::std_time::duration::TimeDuration;
use crate::userdata::{WrappedUserDataFields, WrappedUserDataMethods};
use mluau::prelude::*;
use jiff::Zoned;
use jiff::fmt::temporal::{Pieces, PiecesOffset};

#[derive(Clone)]
pub struct DateTime {
    inner: Zoned,
}

use super::TimeSpan;

impl DateTime {
    pub fn from(zoned: Zoned) -> Self {
        Self {
            inner: zoned,
        }
    }
    pub fn from_unix_timestamp(timestamp: jiff::Timestamp, timezone: jiff::tz::TimeZone) -> Self {
        Self {
            inner: Zoned::new(timestamp, timezone)
        }
    }
    pub fn from_system_time(system_time: SystemTime, function_name: &'static str) -> LuaResult<Self> {
        let timestamp = match jiff::Timestamp::try_from(system_time) {
            Ok(stamp) => stamp,
            Err(err) => {
                return wrap_err!("{} cannot convert SystemTime to jiff::Timestamp due to err: {}", function_name, err);
            }
        };
        let timezone = jiff::tz::TimeZone::system();
        Ok(Self::from_unix_timestamp(timestamp, timezone))
    }
    fn zoned_from_pieces(
        pieces: &Pieces<'_>,
        source: &str,
        timezone: Option<&jiff::tz::TimeZone>,
        function_name: &'static str
    ) -> LuaResult<Zoned> {
        let time = pieces.time()
            .unwrap_or(jiff::civil::Time::midnight());
        let civil = pieces.date().to_datetime(time);

        let zone = match (pieces.offset(), timezone) {
            (Some(PiecesOffset::Zulu), _) => jiff::tz::TimeZone::UTC,
            (Some(offset), _) => jiff::tz::TimeZone::fixed(offset.to_numeric_offset()),
            (None, Some(timezone)) => timezone.clone(),
            (None, None) => {
                return wrap_err!("{}: '{}' has no offset or time zone (seal doesn't assume UTC)", function_name, source);
            }
        };

        match civil.to_zoned(zone) {
            Ok(zoned) => Ok(zoned),
            Err(err) => wrap_err!("{}: can't place '{}' in its time zone: {}", function_name, source, err),
        }
    }
    /// Parses source as an ISO 8601 date or a datetime from the following formats:
    /// - RFC 3339
    /// - RFC 9557
    /// - any of the bove with a space instead of a T
    /// - plain date at midnight
    /// 
    /// If a `timezone` already has a zone string and user also passes `timezone` then we convert it from the
    /// previous timezone to the one user specified (so we don't treat it incorrectly).
    pub fn parse_iso(source: &str, timezone: Option<jiff::tz::TimeZone>, function_name: &'static str) -> LuaResult<Self> {
        let Ok(pieces) = Pieces::parse(source) else {
            return wrap_err!("{}: '{}' isn't a valid ISO 8601 date nor one of the datetime formats we automatically parse; try checking the input or passing 'format' explicitly", function_name, source);
        };

        // jiff resolves a `[Zone]` annotation and checks the dt offset against it
        let zoned = if pieces.time_zone_annotation().is_some() {
            match source.parse::<Zoned>() {
                Ok(zoned) => zoned,
                Err(err) => {
                    return wrap_err!("{}: can't parse '{}' as a date-time with a time zone: {}", function_name, source, err);
                }
            }
        } else {
            Self::zoned_from_pieces(&pieces, source, timezone.as_ref(), function_name)?
        };

        match timezone {
            Some(timezone) => Ok(Self::from(zoned.with_time_zone(timezone))),
            None => Ok(Self::from(zoned)),
        }
    }
    pub fn now() -> Self {
        Self {
            inner: Zoned::now(),
        }
    }
    fn get_format_string(format_string: &str) -> &str {
        match format_string {
            "ISO_8601" => "%Y-%m-%d %H:%M",
            "RFC_2822" => "%a, %d %b %Y %H:%M:%S %z",
            "RFC_3339" => "%Y-%m-%dT%H:%M:%S%:z",
            "SHORT_DATE" => "%Y-%m-%d",
            "SHORT_TIME" => "%H:%M",
            "FULL_DATE_TIME" => "%A, %B %d, %Y %H:%M:%S",
            "LOGGING_12_HR" => "%a %b %e %I:%M:%S %p %Z %Y",
            "LOGGING_24_HR" => "%a %b %e %H:%M:%S %Z %Y",
            "MM/DD/YYYY" => "%m/%d/%Y",
            "MM/DD/YYYY HH:MM (AM/PM)" => "%m/%d/%Y %I:%M %p",
            "MM/DD/YY" => "%m/%d/%y",
            "HH:MM (AM/PM)" => "%I:%M %p",
            "AMERICAN_FULL_DATE_TIME" => "%A, %B %d, %Y %I:%M:%S %p",
            other => other,
        }
    }

    pub(crate) fn parse_tz(iana_timezone: &str) -> &str {
        // Legacy timezones should be modernized as some systems/vendors don't 
        // package legacy tzdata
        
        // Auto-generated from tzdata.zi (version 2026c, IANA "Link" entries).
        // Maps every deprecated/legacy IANA timezone alias (US/Central, Canada/*,
        // three-letter posix zones, single-word country names, etc.) to its
        // canonical zone name. Needed because jiff resolves timezone strings against
        // the OS's /usr/share/zoneinfo, and some distros (e.g. Fedora/RHEL since
        // tzdata 2023c) ship these legacy "backward" aliases in an optional
        // tzdata-legacy package that isn't installed by default -- so "US/Central"
        // fails to resolve there even though "America/Chicago" always works.
        //
        // Regenerate with:
        //   grep "^L" /usr/share/zoneinfo/tzdata.zi | sort -k3 | \
        //     awk '{printf "        \"%s\" => \"%s\",\n", $3, $2}'

        match iana_timezone {
            "Africa/Accra" => "Africa/Abidjan",
            "Africa/Addis_Ababa" => "Africa/Nairobi",
            "Africa/Asmara" => "Africa/Nairobi",
            "Africa/Asmera" => "Africa/Nairobi",
            "Africa/Bamako" => "Africa/Abidjan",
            "Africa/Bangui" => "Africa/Lagos",
            "Africa/Banjul" => "Africa/Abidjan",
            "Africa/Blantyre" => "Africa/Maputo",
            "Africa/Brazzaville" => "Africa/Lagos",
            "Africa/Bujumbura" => "Africa/Maputo",
            "Africa/Conakry" => "Africa/Abidjan",
            "Africa/Dakar" => "Africa/Abidjan",
            "Africa/Dar_es_Salaam" => "Africa/Nairobi",
            "Africa/Djibouti" => "Africa/Nairobi",
            "Africa/Douala" => "Africa/Lagos",
            "Africa/Freetown" => "Africa/Abidjan",
            "Africa/Gaborone" => "Africa/Maputo",
            "Africa/Harare" => "Africa/Maputo",
            "Africa/Kampala" => "Africa/Nairobi",
            "Africa/Kigali" => "Africa/Maputo",
            "Africa/Kinshasa" => "Africa/Lagos",
            "Africa/Libreville" => "Africa/Lagos",
            "Africa/Lome" => "Africa/Abidjan",
            "Africa/Luanda" => "Africa/Lagos",
            "Africa/Lubumbashi" => "Africa/Maputo",
            "Africa/Lusaka" => "Africa/Maputo",
            "Africa/Malabo" => "Africa/Lagos",
            "Africa/Maseru" => "Africa/Johannesburg",
            "Africa/Mbabane" => "Africa/Johannesburg",
            "Africa/Mogadishu" => "Africa/Nairobi",
            "Africa/Niamey" => "Africa/Lagos",
            "Africa/Nouakchott" => "Africa/Abidjan",
            "Africa/Ouagadougou" => "Africa/Abidjan",
            "Africa/Porto-Novo" => "Africa/Lagos",
            "Africa/Timbuktu" => "Africa/Abidjan",
            "America/Anguilla" => "America/Puerto_Rico",
            "America/Antigua" => "America/Puerto_Rico",
            "America/Argentina/ComodRivadavia" => "America/Argentina/Catamarca",
            "America/Aruba" => "America/Puerto_Rico",
            "America/Atikokan" => "America/Panama",
            "America/Atka" => "America/Adak",
            "America/Blanc-Sablon" => "America/Puerto_Rico",
            "America/Buenos_Aires" => "America/Argentina/Buenos_Aires",
            "America/Catamarca" => "America/Argentina/Catamarca",
            "America/Cayman" => "America/Panama",
            "America/Coral_Harbour" => "America/Panama",
            "America/Cordoba" => "America/Argentina/Cordoba",
            "America/Creston" => "America/Phoenix",
            "America/Curacao" => "America/Puerto_Rico",
            "America/Dominica" => "America/Puerto_Rico",
            "America/Ensenada" => "America/Tijuana",
            "America/Fort_Wayne" => "America/Indiana/Indianapolis",
            "America/Godthab" => "America/Nuuk",
            "America/Grenada" => "America/Puerto_Rico",
            "America/Guadeloupe" => "America/Puerto_Rico",
            "America/Indianapolis" => "America/Indiana/Indianapolis",
            "America/Jujuy" => "America/Argentina/Jujuy",
            "America/Knox_IN" => "America/Indiana/Knox",
            "America/Kralendijk" => "America/Puerto_Rico",
            "America/Louisville" => "America/Kentucky/Louisville",
            "America/Lower_Princes" => "America/Puerto_Rico",
            "America/Marigot" => "America/Puerto_Rico",
            "America/Mendoza" => "America/Argentina/Mendoza",
            "America/Montreal" => "America/Toronto",
            "America/Montserrat" => "America/Puerto_Rico",
            "America/Nassau" => "America/Toronto",
            "America/Nipigon" => "America/Toronto",
            "America/Pangnirtung" => "America/Iqaluit",
            "America/Porto_Acre" => "America/Rio_Branco",
            "America/Port_of_Spain" => "America/Puerto_Rico",
            "America/Rainy_River" => "America/Winnipeg",
            "America/Rosario" => "America/Argentina/Cordoba",
            "America/Santa_Isabel" => "America/Tijuana",
            "America/Shiprock" => "America/Denver",
            "America/St_Barthelemy" => "America/Puerto_Rico",
            "America/St_Kitts" => "America/Puerto_Rico",
            "America/St_Lucia" => "America/Puerto_Rico",
            "America/St_Thomas" => "America/Puerto_Rico",
            "America/St_Vincent" => "America/Puerto_Rico",
            "America/Thunder_Bay" => "America/Toronto",
            "America/Tortola" => "America/Puerto_Rico",
            "America/Virgin" => "America/Puerto_Rico",
            "America/Yellowknife" => "America/Edmonton",
            "Antarctica/DumontDUrville" => "Pacific/Port_Moresby",
            "Antarctica/McMurdo" => "Pacific/Auckland",
            "Antarctica/South_Pole" => "Pacific/Auckland",
            "Antarctica/Syowa" => "Asia/Riyadh",
            "Arctic/Longyearbyen" => "Europe/Berlin",
            "Asia/Aden" => "Asia/Riyadh",
            "Asia/Ashkhabad" => "Asia/Ashgabat",
            "Asia/Bahrain" => "Asia/Qatar",
            "Asia/Brunei" => "Asia/Kuching",
            "Asia/Calcutta" => "Asia/Kolkata",
            "Asia/Choibalsan" => "Asia/Ulaanbaatar",
            "Asia/Chongqing" => "Asia/Shanghai",
            "Asia/Chungking" => "Asia/Shanghai",
            "Asia/Dacca" => "Asia/Dhaka",
            "Asia/Harbin" => "Asia/Shanghai",
            "Asia/Istanbul" => "Europe/Istanbul",
            "Asia/Kashgar" => "Asia/Urumqi",
            "Asia/Katmandu" => "Asia/Kathmandu",
            "Asia/Kuala_Lumpur" => "Asia/Singapore",
            "Asia/Kuwait" => "Asia/Riyadh",
            "Asia/Macao" => "Asia/Macau",
            "Asia/Muscat" => "Asia/Dubai",
            "Asia/Phnom_Penh" => "Asia/Bangkok",
            "Asia/Rangoon" => "Asia/Yangon",
            "Asia/Saigon" => "Asia/Ho_Chi_Minh",
            "Asia/Tel_Aviv" => "Asia/Jerusalem",
            "Asia/Thimbu" => "Asia/Thimphu",
            "Asia/Ujung_Pandang" => "Asia/Makassar",
            "Asia/Ulan_Bator" => "Asia/Ulaanbaatar",
            "Asia/Vientiane" => "Asia/Bangkok",
            "Atlantic/Faeroe" => "Atlantic/Faroe",
            "Atlantic/Jan_Mayen" => "Europe/Berlin",
            "Atlantic/Reykjavik" => "Africa/Abidjan",
            "Atlantic/St_Helena" => "Africa/Abidjan",
            "Australia/ACT" => "Australia/Sydney",
            "Australia/Canberra" => "Australia/Sydney",
            "Australia/Currie" => "Australia/Hobart",
            "Australia/LHI" => "Australia/Lord_Howe",
            "Australia/North" => "Australia/Darwin",
            "Australia/NSW" => "Australia/Sydney",
            "Australia/Queensland" => "Australia/Brisbane",
            "Australia/South" => "Australia/Adelaide",
            "Australia/Tasmania" => "Australia/Hobart",
            "Australia/Victoria" => "Australia/Melbourne",
            "Australia/West" => "Australia/Perth",
            "Australia/Yancowinna" => "Australia/Broken_Hill",
            "Brazil/Acre" => "America/Rio_Branco",
            "Brazil/DeNoronha" => "America/Noronha",
            "Brazil/East" => "America/Sao_Paulo",
            "Brazil/West" => "America/Manaus",
            "Canada/Atlantic" => "America/Halifax",
            "Canada/Central" => "America/Winnipeg",
            "Canada/Eastern" => "America/Toronto",
            "Canada/Mountain" => "America/Edmonton",
            "Canada/Newfoundland" => "America/St_Johns",
            "Canada/Pacific" => "America/Vancouver",
            "Canada/Saskatchewan" => "America/Regina",
            "Canada/Yukon" => "America/Whitehorse",
            "CET" => "Europe/Brussels",
            "Chile/Continental" => "America/Santiago",
            "Chile/EasterIsland" => "Pacific/Easter",
            "CST6CDT" => "America/Chicago",
            "Cuba" => "America/Havana",
            "EET" => "Europe/Athens",
            "Egypt" => "Africa/Cairo",
            "Eire" => "Europe/Dublin",
            "EST" => "America/Panama",
            "EST5EDT" => "America/New_York",
            "Etc/GMT+0" => "Etc/GMT",
            "Etc/GMT-0" => "Etc/GMT",
            "Etc/GMT0" => "Etc/GMT",
            "Etc/Greenwich" => "Etc/GMT",
            "Etc/UCT" => "Etc/UTC",
            "Etc/Universal" => "Etc/UTC",
            "Etc/Zulu" => "Etc/UTC",
            "Europe/Amsterdam" => "Europe/Brussels",
            "Europe/Belfast" => "Europe/London",
            "Europe/Bratislava" => "Europe/Prague",
            "Europe/Busingen" => "Europe/Zurich",
            "Europe/Copenhagen" => "Europe/Berlin",
            "Europe/Guernsey" => "Europe/London",
            "Europe/Isle_of_Man" => "Europe/London",
            "Europe/Jersey" => "Europe/London",
            "Europe/Kiev" => "Europe/Kyiv",
            "Europe/Ljubljana" => "Europe/Belgrade",
            "Europe/Luxembourg" => "Europe/Brussels",
            "Europe/Mariehamn" => "Europe/Helsinki",
            "Europe/Monaco" => "Europe/Paris",
            "Europe/Nicosia" => "Asia/Nicosia",
            "Europe/Oslo" => "Europe/Berlin",
            "Europe/Podgorica" => "Europe/Belgrade",
            "Europe/San_Marino" => "Europe/Rome",
            "Europe/Sarajevo" => "Europe/Belgrade",
            "Europe/Skopje" => "Europe/Belgrade",
            "Europe/Stockholm" => "Europe/Berlin",
            "Europe/Tiraspol" => "Europe/Chisinau",
            "Europe/Uzhgorod" => "Europe/Kyiv",
            "Europe/Vaduz" => "Europe/Zurich",
            "Europe/Vatican" => "Europe/Rome",
            "Europe/Zagreb" => "Europe/Belgrade",
            "Europe/Zaporozhye" => "Europe/Kyiv",
            "GB" => "Europe/London",
            "GB-Eire" => "Europe/London",
            "GMT" => "Etc/GMT",
            "GMT+0" => "Etc/GMT",
            "GMT-0" => "Etc/GMT",
            "GMT0" => "Etc/GMT",
            "Greenwich" => "Etc/GMT",
            "Hongkong" => "Asia/Hong_Kong",
            "HST" => "Pacific/Honolulu",
            "Iceland" => "Africa/Abidjan",
            "Indian/Antananarivo" => "Africa/Nairobi",
            "Indian/Christmas" => "Asia/Bangkok",
            "Indian/Cocos" => "Asia/Yangon",
            "Indian/Comoro" => "Africa/Nairobi",
            "Indian/Kerguelen" => "Indian/Maldives",
            "Indian/Mahe" => "Asia/Dubai",
            "Indian/Mayotte" => "Africa/Nairobi",
            "Indian/Reunion" => "Asia/Dubai",
            "Iran" => "Asia/Tehran",
            "Israel" => "Asia/Jerusalem",
            "Jamaica" => "America/Jamaica",
            "Japan" => "Asia/Tokyo",
            "Kwajalein" => "Pacific/Kwajalein",
            "Libya" => "Africa/Tripoli",
            "MET" => "Europe/Brussels",
            "Mexico/BajaNorte" => "America/Tijuana",
            "Mexico/BajaSur" => "America/Mazatlan",
            "Mexico/General" => "America/Mexico_City",
            "MST" => "America/Phoenix",
            "MST7MDT" => "America/Denver",
            "Navajo" => "America/Denver",
            "NZ" => "Pacific/Auckland",
            "NZ-CHAT" => "Pacific/Chatham",
            "Pacific/Chuuk" => "Pacific/Port_Moresby",
            "Pacific/Enderbury" => "Pacific/Kanton",
            "Pacific/Funafuti" => "Pacific/Tarawa",
            "Pacific/Johnston" => "Pacific/Honolulu",
            "Pacific/Majuro" => "Pacific/Tarawa",
            "Pacific/Midway" => "Pacific/Pago_Pago",
            "Pacific/Pohnpei" => "Pacific/Guadalcanal",
            "Pacific/Ponape" => "Pacific/Guadalcanal",
            "Pacific/Saipan" => "Pacific/Guam",
            "Pacific/Samoa" => "Pacific/Pago_Pago",
            "Pacific/Truk" => "Pacific/Port_Moresby",
            "Pacific/Wake" => "Pacific/Tarawa",
            "Pacific/Wallis" => "Pacific/Tarawa",
            "Pacific/Yap" => "Pacific/Port_Moresby",
            "Poland" => "Europe/Warsaw",
            "Portugal" => "Europe/Lisbon",
            "PRC" => "Asia/Shanghai",
            "PST8PDT" => "America/Los_Angeles",
            "ROC" => "Asia/Taipei",
            "ROK" => "Asia/Seoul",
            "Singapore" => "Asia/Singapore",
            "Turkey" => "Europe/Istanbul",
            "UCT" => "Etc/UTC",
            "Universal" => "Etc/UTC",
            "US/Alaska" => "America/Anchorage",
            "US/Aleutian" => "America/Adak",
            "US/Arizona" => "America/Phoenix",
            "US/Central" => "America/Chicago",
            "US/Eastern" => "America/New_York",
            "US/East-Indiana" => "America/Indiana/Indianapolis",
            "US/Hawaii" => "Pacific/Honolulu",
            "US/Indiana-Starke" => "America/Indiana/Knox",
            "US/Michigan" => "America/Detroit",
            "US/Mountain" => "America/Denver",
            "US/Pacific" => "America/Los_Angeles",
            "US/Samoa" => "Pacific/Pago_Pago",
            "UTC" => "Etc/UTC",
            "WET" => "Europe/Lisbon",
            "W-SU" => "Europe/Moscow",
            "Zulu" => "Etc/UTC",
            other => other,
        }
    }

    pub fn parse(source: &mut String, format_string: &str, iana_timezone: &str, function_name: &'static str) -> LuaResult<Self> {
        let iana_timezone = Self::parse_tz(iana_timezone);

        let mut format_string = Self::get_format_string(format_string).to_string();

        // all Zoned DateTimes must have a %Q specifier, so we expose it as the third param
        // if user explicitly specifies "AUTO" as their timezone that means format string already
        // contains %Q or z/%z; jiff will throw an error if users use "AUTO" and don't include tz info
        if iana_timezone != "AUTO" && !format_string.contains("%Q") {
            format_string.push_str(" %Q");
            source.push(' ');
            source.push_str(iana_timezone);
        }

        match Zoned::strptime(&format_string, &source) {
            Ok(zoned) => Ok(Self { inner: zoned }),
            Err(err) => wrap_err!(
                "{}: failed to parse source '{}' with format '{}' due to err: {}",
                function_name,
                source,
                format_string,
                err
            ),
        }
    }
    pub fn format(&self, format_string: &str, function_name: &'static str) -> LuaResult<String> {
        let format_string = Self::get_format_string(format_string);
        match jiff::fmt::strtime::format(format_string, &self.inner) {
            Ok(s) => Ok(s),
            Err(err) => {
                wrap_err!("{} unable to format DateTime because {}", function_name, err)
            }
        }
    }
    pub fn get_userdata(self, luau: &Lua) -> LuaValueResult {
        ok_userdata(self, luau)
    }
    pub fn _to_system_time(&self) -> SystemTime {
        SystemTime::from(self.inner.timestamp())
    }
    pub fn date(&self) -> jiff::civil::Date {
        self.inner.date()
    }
}

impl Borrowable for DateTime {
    fn display_name() -> &'static str {
        "DateTime"
    }
}

impl WrappedUserData for DateTime {
    fn type_name<'a>() -> Cow<'a, str> {
        Cow::Borrowed("DateTime")
    }
    fn add_fields<F: WrappedUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("year", |_: &Lua, this| Ok(this.inner.date().year()));
        fields.add_field_method_get("month", |_: &Lua, this| Ok(this.inner.date().month()));
        fields.add_field_method_get("day", |_: &Lua, this| Ok(this.inner.date().day()));
        fields.add_field_method_get("hour", |_: &Lua, this| Ok(this.inner.time().hour()));
        fields.add_field_method_get("minute", |_: &Lua, this| Ok(this.inner.time().minute()));
        fields.add_field_method_get("second", |_: &Lua, this| Ok(this.inner.time().second()));
        fields.add_field_method_get("millisecond", |_: &Lua, this| Ok(this.inner.time().millisecond()));
        fields.add_field_method_get("weekday", |_: &Lua, this| {
            Ok(this.inner.strftime("%A").to_string()) // like 'Monday'
        });
        fields.add_field_method_get("unix_timestamp", |_: &Lua, this| {
            Ok(this.inner.timestamp().as_second())
        });
        fields.add_field_method_get("timezone", |luau: &Lua, this| {
            let timezone = match this.inner.time_zone().iana_name() {
                Some(tz) => tz.to_string(),
                None => String::default(),
            };
            ok_string(timezone, luau)
        });
        fields.add_field_method_get("iso", |_: &Lua, this| {
            Ok(this.inner.to_string())
        });
    }

    fn add_methods<M: WrappedUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, | luau: &Lua, this, _: LuaValue| -> LuaValueResult {
            ok_string(format!("DateTime<{}>", this.inner.strftime("%Y-%m-%d %H:%M:%S (%I:%M %p) [%Z]")), luau)
        });
        methods.add_method("display", | luau: &Lua, this, _value: ()| {
            // :display() shows format like 2025-08-18 18:20:44 (6:20 PM) [CDT]
            ok_string(format!("{}", this.inner.strftime("%Y-%m-%d %H:%M:%S (%I:%M %p) [%Z]")), luau)
        });
        methods.add_method("__dp", | luau: &Lua, this, _value: ()| {
            // shows the entire roundtrippable DateTime<2025-08-18T20:23:29.85205845-05:00[America/Chicago]>
            ok_string(format!("DateTime<{:?}>", this.inner), luau)
        });

        methods.add_method("format", |luau: &Lua, this, value: LuaValue| -> LuaValueResult {
            let function_name = "DateTime:format(format: string)";
            let format_string = match value {
                LuaValue::String(s) => s.to_string_lossy(),
                other => {
                    return wrap_err!("{} expected format to be a string, got: {:?}", function_name, other);
                }
            };
            ok_string(this.format(&format_string, function_name)?, luau)
        });

        methods.add_method("in_timezone", |_luau: &Lua, this, value: LuaValue| -> WrappedResult<DateTime> {
            let function_name = "DateTime:in_timezone(timezone: IanaTimezone)";
            let timezone = match value {
                LuaValue::String(s) => s.to_string_lossy(),
                other => {
                    return wrap_err!("{} expected timezone to be a string (one of the IanaTimezones), got: {:?}", function_name, other);
                }
            };
            let new_dt = match this.inner.in_tz(Self::parse_tz(&timezone)) {
                Ok(zoned) => zoned,
                Err(err) => {
                    return wrap_err!(
                        "{}: unable to convert DateTime to timezone '{}'; is it a valid IanaTimezone? err: {}",
                        function_name, timezone, err
                    )
                }
            };
            Ok(DateTime::from(new_dt).into_wrapped())
        });


        // literally the same as "timespan"
        methods.add_method("to", |_luau: &Lua, this, other: LuaValue| -> WrappedResult<TimeSpan> {
            let function_name = "DateTime:to(other: DateTime)";
            match other {
                LuaValue::UserData(ud) => {
                    if let Some(other_dt) = DateTime::borrow(&ud) {
                        let span = match this.inner.until(&other_dt.inner) {
                            Ok(span) => span,
                            Err(err) => {
                                return wrap_err!("{} unable to compute timespan due to err: {}", function_name, err);
                            }
                        };
                        Ok(TimeSpan::new(span).into_wrapped())
                    } else {
                        wrap_err!("{} expected other to be a DateTime, got: {:?}", function_name, ud.type_name()?)
                    }
                },
                other => {
                    wrap_err!("{} expected other to be a DateTime, got: {:?}", function_name, other)
                }
            }
        });

        methods.add_method("timespan", |_luau: &Lua, this, other: LuaValue| -> WrappedResult<TimeSpan> {
            let function_name = "DateTime:timespan(other: DateTime)";
            match other {
                LuaValue::UserData(ud) => {
                    if let Some(other_dt) = DateTime::borrow(&ud) {
                        let span = match this.inner.until(&other_dt.inner) {
                            Ok(span) => span,
                            Err(err) => {
                                return wrap_err!("{} unable to compute timespan due to err: {}", function_name, err);
                            }
                        };
                        Ok(TimeSpan::new(span).into_wrapped())
                    } else {
                        wrap_err!("{} expected other to be a DateTime, got: {:?}", function_name, ud.type_name()?)
                    }
                },
                other => {
                    wrap_err!("{} expected other to be a DateTime, got: {:?}", function_name, other)
                }
            }
        });

        methods.add_method("since", |_luau: &Lua, this, other: LuaValue| -> WrappedResult<TimeSpan> {
            let function_name = "DateTime:since(other: DateTime)";
            match other {
                LuaValue::UserData(ud) => {
                    if let Some(other_dt) = DateTime::borrow(&ud) {
                        let span = match this.inner.since(&other_dt.inner) {
                            Ok(span) => span,
                            Err(err) => {
                                return wrap_err!("{} unable to compute timespan due to err: {}", function_name, err);
                            }
                        };
                        Ok(TimeSpan::new(span).into_wrapped())
                    } else {
                        wrap_err!("{} expected other to be a DateTime, got: {:?}", function_name, ud.type_name()?)
                    }
                },
                other => {
                    wrap_err!("{} expected other to be a DateTime, got: {:?}", function_name, other)
                }
            }
        });

        // DateTime + TimeSpan -> DateTime
        methods.add_meta_method(LuaMetaMethod::Add, |_luau: &Lua, this, other: LuaValue| -> WrappedResult<DateTime> {
            let function_name = "DateTime.__add(self, other: TimeSpan)";
            match other {
                LuaValue::UserData(ud) => {
                    if let Some(other_timespan) = TimeSpan::borrow(&ud) {
                        let new_dt = &this.inner + other_timespan.inner;
                        Ok(DateTime::from(new_dt).into_wrapped())
                    } else if DateTime::borrow(&ud).is_some() {
                        wrap_err!("{}: adding DateTime + DateTime makes no sense and is ambiguous, did you mean to add a TimeSpan?", function_name)
                    } else if TimeDuration::borrow(&ud).is_some() {
                        wrap_err!("{}: unfortunately we can't add DateTime + Duration directly, you need to use a TimeSpan (time.datetime.days(n))", function_name)
                    } else {
                        wrap_err!("{}: other must be a TimeSpan", function_name)
                    }
                },
                other => {
                    wrap_err!("{} expected other to be a TimeSpan, got: {:?}", function_name, other)
                }
            }
        });

        // DateTime - TimeSpan -> DateTime;
        methods.add_meta_method(LuaMetaMethod::Sub, |_luau: &Lua, this, other: LuaValue| -> WrappedResult<DateTime> {
            let function_name = "DateTime.__sub(self, other: TimeSpan)";
            match other {
                LuaValue::UserData(ud) => {
                    if let Some(other_timespan) = TimeSpan::borrow(&ud) {
                        let new_dt = &this.inner - other_timespan.inner;
                        Ok(DateTime::from(new_dt).into_wrapped())
                    } else if DateTime::borrow(&ud).is_some() {
                        wrap_err!("{}: DateTime - DateTime is ambiguous; if you need a TimeSpan between two DateTimes, use DateTime:since(dt: DateTime)", function_name)
                    } else if TimeDuration::borrow(&ud).is_some() {
                        wrap_err!("{}: unfortunately we can't add DateTime + Duration directly, you need to use a TimeSpan (time.datetime.days(n))", function_name)
                    } else {
                        wrap_err!("{}: other must be a TimeSpan", function_name)
                    }
                },
                other => {
                    wrap_err!("{} expected other to be a TimeSpan, got: {:?}", function_name, other)
                }
            }
        });

        methods.add_meta_method(LuaMetaMethod::Eq, |_luau: &Lua, this, other: LuaValue| -> LuaValueResult {
            let function_name = "DateTime.__eq(self, other: DateTime)";
            match other {
                LuaValue::UserData(ud) => {
                    if let Some(other_dt) = DateTime::borrow(&ud) {
                        Ok(LuaValue::Boolean(this.inner == other_dt.inner))
                    } else if TimeSpan::borrow(&ud).is_some() {
                        wrap_err!("{}: DateTime == TimeSpan makes no sense to me lol", function_name)
                    } else {
                        wrap_err!("{}: other must be a DateTime", function_name)
                    }
                },
                other => {
                    wrap_err!("{} expected other to be a DateTime, got: {:?}", function_name, other)
                }
            }
        });

        methods.add_meta_method(LuaMetaMethod::Lt, |_luau: &Lua, this, other: LuaValue| -> LuaValueResult {
            let function_name = "DateTime.__lt(self, other: DateTime)";
            match other {
                LuaValue::UserData(ud) => {
                    if let Some(other_dt) = DateTime::borrow(&ud) {
                        Ok(LuaValue::Boolean(this.inner < other_dt.inner))
                    } else if TimeSpan::borrow(&ud).is_some() {
                        wrap_err!("{}: DateTime < TimeSpan makes no sense to me lol", function_name)
                    } else {
                        wrap_err!("{}: other must be a DateTime", function_name)
                    }
                },
                other => {
                    wrap_err!("{} expected other to be a DateTime, got: {:?}", function_name, other)
                }
            }
        });

        methods.add_meta_method(LuaMetaMethod::Le, |_luau: &Lua, this, other: LuaValue| -> LuaValueResult {
            let function_name = "DateTime.__le(self, other: DateTime)";
            match other {
                LuaValue::UserData(ud) => {
                    if let Some(other_dt) = DateTime::borrow(&ud) {
                        Ok(LuaValue::Boolean(this.inner <= other_dt.inner))
                    } else if TimeSpan::borrow(&ud).is_some() {
                        wrap_err!("{}: DateTime <= TimeSpan makes no sense to me lol", function_name)
                    } else {
                        wrap_err!("{}: other must be a DateTime", function_name)
                    }
                },
                other => {
                    wrap_err!("{} expected other to be a DateTime, got: {:?}", function_name, other)
                }
            }
        });

        // Mon, 18 Aug 2025 20:54:00 -0500
        methods.add_method("rfc_2822", |luau: &Lua, this, _value: ()| {
            ok_string(this.inner.strftime("%a, %d %b %Y %H:%M:%S %z").to_string(), luau)
        });

        // 2025-08-18T20:54:00-05:00
        methods.add_method("rfc_3339", |luau: &Lua, this, _value: ()| {
            ok_string(this.inner.strftime("%Y-%m-%dT%H:%M:%S%:z").to_string(), luau)
        });

        // 2025-08-18T20:54:00.000-05:00
        methods.add_method("rfc_3339_opts", |luau: &Lua, this, _value: ()| {
            ok_string(this.inner.strftime("%Y-%m-%dT%H:%M:%S%.3f%:z").to_string(), luau)
        });

        // 2025-08-18T20:54:00-05:00
        methods.add_method("iso_8601", |luau: &Lua, this, _value: ()| {
            ok_string(this.inner.strftime("%Y-%m-%dT%H:%M:%S%:z").to_string(), luau)
        });

        // Mon, 18 Aug 2025 20:54:00 GMT
        methods.add_method("http_date", |luau: &Lua, this, _value: ()| {
            ok_string(this.inner.strftime("%a, %d %b %Y %H:%M:%S GMT").to_string(), luau)
        });

    }
}


fn datetime_now(_luau: &Lua, _: ()) -> WrappedResult<DateTime> {
    Ok(DateTime::now().into_wrapped())
}

/// A time zone that `datetime.parse` looked up by name
struct NamedTimeZone {
    name: String,
    timezone: jiff::tz::TimeZone,
}

thread_local! {
    /// The last time zone `datetime.parse` looked up. Dates parsed in bulk usually share one zone, and reusing it
    /// instead of asking jiff's database again made a measurable difference in pika's benchmark.
    static LAST_TIME_ZONE: RefCell<Option<NamedTimeZone>> = const { RefCell::new(None) };
}

/// `TimeZone::get` that reuses the previous call's zone when `name` matches
fn cached_time_zone(name: &str) -> Result<jiff::tz::TimeZone, jiff::Error> {
    // jiff needs a time zone database to resolve the "Etc/UTC" that parse_tz turns "UTC" into
    if name == "Etc/UTC" {
        return Ok(jiff::tz::TimeZone::UTC);
    }
    LAST_TIME_ZONE.with_borrow_mut(|last| {
        if let Some(cached) = last
            && cached.name == name
        {
            return Ok(cached.timezone.clone());
        }
        let timezone = jiff::tz::TimeZone::get(name)?;
        *last = Some(NamedTimeZone {
            name: name.to_owned(),
            timezone: timezone.clone(),
        });
        Ok(timezone)
    })
}

fn datetime_parse(_luau: &Lua, mut multivalue: LuaMultiValue) -> WrappedResult<DateTime> {
    let function_name = "datetime.parse(source: string, format: (string | CommonFormats)?, timezone: IanaTimezone?)";
    let source_string = match multivalue.pop_front() {
        Some(LuaValue::String(s)) => s,
        Some(LuaNil) | None => {
            return wrap_err!("{} expected source to be a datetime-formattable string, but was incorrectly called with zero arguments or nil", function_name);
        },
        Some(other) => {
            return wrap_err!("{} expected source to be a datetime-formattable string, got: {:?}", function_name, other);
        }
    };

    fn pop_lua_string(multivalue: &mut LuaMultiValue) -> Result<Option<LuaString>, LuaValue> {
        match multivalue.pop_front() {
            Some(LuaValue::String(s)) => Ok(Some(s)),
            Some(LuaNil) | None => Ok(None),
            Some(other) => {
                Err(other)
            }
        }
    }
    let format = match pop_lua_string(&mut multivalue) {
        Ok(s) => s,
        Err(other) => {
            return wrap_err!("{}: Expected format to be a valid strtime format string, got: {:?}", function_name, other);
        }
    };
    let tz = match pop_lua_string(&mut multivalue) {
        Ok(s) => s,
        Err(other) => {
            return wrap_err!("{} expected the timezone to be one of the 500+ IANA timezones, \"AUTO\" or nil, got: {:?}", function_name, other);
        }
    };

    let source = match source_string.to_str() {
        Ok(source) => source,
        Err(_) => {
            return wrap_err!("{}: source string was unexpectedly invalid utf-8", function_name);
        }
    };
    let format_string = match &format {
        Some(s) => match s.to_str() {
            Ok(format_string) => Some(format_string),
            Err(_) => {
                return wrap_err!("{}: format was unexpectedly invalid utf-8", function_name);
            }
        },
        None => None,
    };
    let iana_timezone = match &tz {
        Some(s) => match s.to_str() {
            Ok(name) if DateTime::parse_tz(&name) == "AUTO" => None,
            Ok(name) => Some(name),
            Err(_) => {
                return wrap_err!("{}: timezone was unexpectedly invalid utf-8", function_name);
            }
        },
        None => None,
    };

    // treat source as ISO 8601 when format not explicitly provided
    let Some(format_string) = format_string else {
        let Some(name) = iana_timezone else {
            return Ok(DateTime::parse_iso(&source, None, function_name)?.into_wrapped());
        };
        let timezone = match cached_time_zone(DateTime::parse_tz(&name)) {
            Ok(timezone) => timezone,
            Err(err) => {
                return wrap_err!("{} expected timezone to be one of the 500+ IANA timezones, got '{}': {}", function_name, name, err);
            }
        };
        return Ok(DateTime::parse_iso(&source, Some(timezone), function_name)?.into_wrapped());
    };

    fn has_specifier(format: &str) -> bool {
        ["%z", "%:z", "%::z", "%:::z", "%Q", "%:Q"].iter().any(|specifier| format.contains(specifier))
    }

    // DateTime::parse takes in &mut to append %Q if needed
    let mut source = source.to_owned();
    let Some(iana_timezone) = iana_timezone else {
        let format = DateTime::get_format_string(&format_string);
        if has_specifier(format) {
            return Ok(DateTime::parse(&mut source, &format_string, "AUTO", function_name)?.into_wrapped());
        } else {
            return wrap_err!(
                "{}: the format '{}' has no time zone (%z or %Q) and no timezone was passed, please pass one explicitly (seal doesn't assume UTC); got {}",
                function_name, format, source
            );
        }
    };
    Ok(DateTime::parse(&mut source, &format_string, &iana_timezone, function_name)?.into_wrapped())
}

fn datetime_timespan(_luau: &Lua, mut multivalue: LuaMultiValue) -> WrappedResult<TimeSpan> {
    let function_name = "datetime.timespan(source: string, relative_to: DateTime?)";
    let span = crate::std_time::timespan::parse_span(multivalue.pop_front(), function_name)?;
    let relative_to = match multivalue.pop_front() {
        Some(LuaValue::UserData(ud)) => {
            if let Some(dt) = DateTime::borrow(&ud) {
                Some(dt.0.clone())
            } else {
                let type_name = ud.type_name()?.unwrap_or_default();
                return wrap_err!("{} expected relative_to to be a DateTime or nil/unspecified, got a userdata of type: {}", function_name, type_name);
            }
        },
        Some(LuaNil) | None => None,
        Some(other) => {
            return wrap_err!("{} expected relative_to to be DateTime or nil/unspecified, got: {:?}", function_name, other);
        }
    };
    match relative_to {
        Some(relative_to) => Ok(TimeSpan::relative_to(span, relative_to).into_wrapped()),
        None => Ok(TimeSpan::new(span).into_wrapped()),
    }
}

fn datetime_from(_luau: &Lua, mut multivalue: LuaMultiValue) -> WrappedResult<DateTime> {
    let function_name = "datetime.from(timestamp: number, timezone: string?, nanos: number?)";
    let timestamp = match multivalue.pop_front() {
        Some(LuaValue::Integer(i)) => int_to_i64(i),
        Some(LuaNil) | None => {
            return wrap_err!("{} expected timestamp to be an integer number, got nothing or nil", function_name);
        },
        Some(other) => {
            return wrap_err!("{} expected timestamp to be an integer number, got: {:?}", function_name, other);
        }
    };
    let timezone = match multivalue.pop_front() {
        Some(LuaValue::String(tz)) => tz.to_string_lossy(),
        Some(LuaNil) | None => "UTC".to_string(),
        Some(other) => {
            return wrap_err!("{} expected timezone to be an IanaTimezone string, got: {:?}", function_name, other);
        }
    };
    let nanos = match multivalue.pop_front() {
        Some(LuaValue::Integer(n)) => n,
        Some(LuaNil) | None => 0,
        Some(other) => {
            return wrap_err!("{} expected nanos to be an integer number or nil/unspecified, got: {:?}", function_name, other);
        }
    };
    let stampy = match jiff::Timestamp::new(timestamp, nanos as i32) {
        Ok(stamp) => stamp,
        Err(err) => {
            return wrap_err!("{} unable to generate timestamp from input due to err: {}", function_name, err);
        }
    };
    let jiff_timezone = match jiff::tz::TimeZone::get(&timezone) {
        Ok(tz) => tz,
        Err(err) => {
            return wrap_err!("{} unable to create TimeZone (is it valid?): {}", function_name, err);
        }
    };
    Ok(DateTime::from_unix_timestamp(stampy, jiff_timezone).into_wrapped())
}

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_function_and_signature("now", datetime_now, signatures::STD_TIME_DATETIME_NOW)?
        .with_function_and_signature("parse", datetime_parse, signatures::STD_TIME_DATETIME_PARSE)?
        .with_function_and_signature("timespan", datetime_timespan, signatures::STD_TIME_DATETIME_TIMESPAN)?
        .with_function_and_signature("from", datetime_from, signatures::STD_TIME_DATETIME_FROM)?
        .with_value("common_formats", TableBuilder::create(luau)?
            .with_value("ISO_8601", "%Y-%m-%d %H:%M")?
            .with_value("RFC_2822", "%a, %d %b %Y %H:%M:%S %z")?
            .with_value("RFC_3339", "%Y-%m-%dT%H:%M:%S%:z")?
            .with_value("SHORT_DATE", "%Y-%m-%d")?
            .with_value("SHORT_TIME", "%H:%M")?
            .with_value("FULL_DATE_TIME", "%A, %B %d, %Y %H:%M:%S")?
            .with_value("LOGGING_12_HR", "%a %b %e %I:%M:%S %p %Z %Y")?
            .with_value("LOGGING_24_HR", "%a %b %e %H:%M:%S %Z %Y")?
            // Common American formats
            .with_value("MM/DD/YYYY", "%m/%d/%Y")?
            .with_value("MM/DD/YYYY HH:MM (AM/PM)", "%m/%d/%Y %I:%M %p")?
            .with_value("MM/DD/YY", "%m/%d/%y")?
            .with_value("HH:MM (AM/PM)", "%I:%M %p")?
            .with_value("AMERICAN_FULL_DATE_TIME", "%A, %B %d, %Y %I:%M:%S %p")?
            .build_readonly()?
        )?
        .with_function_and_signature("years", | _luau: &Lua, mut multivalue: LuaMultiValue | -> WrappedResult<TimeSpan> {
            let function_name = "datetime.years(years: number)";
            let months = match multivalue.pop_front() {
                Some(LuaValue::Number(f)) => f as i64,
                Some(LuaValue::Integer(i)) => int_to_i64(i),
                other => {
                    return wrap_err!("{} expected years to be an integer number, got: {:?}", function_name, other);
                }
            };
            let relative_to = match multivalue.pop_front() {
                Some(LuaValue::UserData(ud)) => {
                    if let Some(dt) = DateTime::borrow(&ud) {
                        Some(dt.0.clone())
                    } else {
                        let type_name = ud.type_name()?.unwrap_or_default();
                        return wrap_err!("{} expected relative_to to be a DateTime or nil/unspecified, got a userdata of type: {}", function_name, type_name);
                    }
                },
                Some(LuaNil) | None => None,
                Some(other) => {
                    return wrap_err!("{} expected relative_to to be DateTime or nil/unspecified, got: {:?}", function_name, other);
                }
            };
            Ok(TimeSpan::years(months, relative_to).into_wrapped())
        }, signatures::STD_TIME_DATETIME_YEARS)?
        .with_function_and_signature("months", | _luau: &Lua, mut multivalue: LuaMultiValue | -> WrappedResult<TimeSpan> {
            let function_name = "time.months(months: number)";
            let months = match multivalue.pop_front() {
                Some(LuaValue::Number(f)) => f as i64,
                Some(LuaValue::Integer(i)) => int_to_i64(i),
                other => {
                    return wrap_err!("{} expected days to be an integer number, got: {:?}", function_name, other);
                }
            };
            let relative_to = match multivalue.pop_front() {
                Some(LuaValue::UserData(ud)) => {
                    if let Some(dt) = DateTime::borrow(&ud) {
                        Some(dt.0.clone())
                    } else {
                        let type_name = ud.type_name()?.unwrap_or_default();
                        return wrap_err!("{} expected relative_to to be a DateTime or nil/unspecified, got a userdata of type: {}", function_name, type_name);
                    }
                },
                Some(LuaNil) | None => None,
                Some(other) => {
                    return wrap_err!("{} expected relative_to to be DateTime or nil/unspecified, got: {:?}", function_name, other);
                }
            };
            Ok(TimeSpan::months(months, relative_to).into_wrapped())
        }, signatures::STD_TIME_DATETIME_MONTHS)?
        .with_function_and_signature("days", | _luau: &Lua, value: LuaValue | -> WrappedResult<TimeSpan> {
            let function_name = "time.days(d: number)";
            let days = match value {
                LuaValue::Number(f) => f as i64,
                LuaValue::Integer(i) => int_to_i64(i),
                other => {
                    return wrap_err!("{} expected days to be an integer number, got: {:?}", function_name, other);
                }
            };
            Ok(TimeSpan::days(days).into_wrapped())
        }, signatures::STD_TIME_DATETIME_DAYS)?
        .with_function_and_signature("hours", | _luau: &Lua, value: LuaValue | -> WrappedResult<TimeSpan> {
            let function_name = "time.hours(hours: number)";
            let days = match value {
                LuaValue::Number(f) => f as i64,
                LuaValue::Integer(i) => int_to_i64(i),
                other => {
                    return wrap_err!("{} expected hours to be an integer number, got: {:?}", function_name, other);
                }
            };
            Ok(TimeSpan::hours(days).into_wrapped())
        }, signatures::STD_TIME_DATETIME_HOURS)?
        .with_function_and_signature("minutes", | _luau: &Lua, value: LuaValue | -> WrappedResult<TimeSpan> {
            let function_name = "time.minutes(minutes: number)";
            let days = match value {
                LuaValue::Number(f) => f as i64,
                LuaValue::Integer(i) => int_to_i64(i),
                other => {
                    return wrap_err!("{} expected minutes to be an integer number, got: {:?}", function_name, other);
                }
            };
            Ok(TimeSpan::minutes(days).into_wrapped())
        }, signatures::STD_TIME_DATETIME_MINUTES)?
        .with_function_and_signature("seconds", | _luau: &Lua, value: LuaValue | -> WrappedResult<TimeSpan> {
            let function_name = "time.seconds(seconds: number)";
            let days = match value {
                LuaValue::Number(f) => f as i64,
                LuaValue::Integer(i) => int_to_i64(i),
                other => {
                    return wrap_err!("{} expected seconds to be an integer number, got: {:?}", function_name, other);
                }
            };
            Ok(TimeSpan::seconds(days).into_wrapped())
        }, signatures::STD_TIME_DATETIME_SECONDS)?
        .build_readonly()
}