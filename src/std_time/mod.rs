use crate::prelude::*;
use mluau::prelude::*;
use std::time::Duration;

pub mod datetime;
pub mod duration;
pub mod timespan;

use timespan::TimeSpan;
use duration::TimeDuration;

/// Sleeps for the given amount of time using thread::sleep.
///
/// Precision is limited to the system's sleep granularity—typically around 1ms.
///
/// This function is intended for simple time-based control in Luau.
fn time_wait(_luau: &Lua, value: LuaValue) -> LuaValueResult {
    let function_name = "time.wait(duration: Duration | number)";
    let duration = match value {
        LuaValue::Number(f) => {
            if f.is_sign_negative() {
                return wrap_err!("{}: cannot reverse time (got negative duration: {})", function_name, f);
            }
            // convert to millis so partial/decimal values get included
            let millis = (f * 1000.0) as u64;
            Duration::from_millis(millis)
        },
        LuaValue::Integer(i) => {
            Duration::from_secs(int_to_u64(i, function_name, "duration")?)
        },
        LuaValue::UserData(ref ud) if let Some(ud) = TimeDuration::borrow(ud) => {
            let signed = ud.inner;
            if signed.is_negative() {
                return wrap_err!("{}: cannot reverse time (got negative duration: {})", function_name, signed);
            }
            signed.unsigned_abs()
        },
        other => {
            return wrap_err!("{}: expected duration to be a seal Duration or a number of seconds, got: {:?}", function_name, other);
        }
    };
    std::thread::sleep(duration);
    Ok(LuaValue::Boolean(true)) // return true so while time.wait(1) loops still work
}

/// Parses a Duration written in ISO 8601 (`"PT1H30M"`) or in words (`"1h 30m"`).
/// We count a day as 24 hours and a week as 7 days here (users will raise a bug report if they need anything)
/// more accurate than that. If a user tries passing months or years here e tell them to use
/// datetime.timespan instead because that handles months and years correctly.
fn time_parse(_luau: &Lua, mut multivalue: LuaMultiValue) -> WrappedResult<TimeDuration> {
    let function_name = "time.parse(source: string)";
    let span = timespan::parse_span(multivalue.pop_front(), function_name)?;
    if span.get_years() != 0 || span.get_months() != 0 {
        return wrap_err!("{}: '{}' has months or years (which have no fixed length); use datetime.timespan instead", function_name, span);
    }
    match span.to_duration(jiff::SpanRelativeTo::days_are_24_hours()) {
        Ok(duration) => Ok(TimeDuration::new(duration).into_wrapped()),
        Err(err) => wrap_err!("{}: '{}' cannot fit in a Duration: {}", function_name, span, err),
    }
}

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_function_and_signature("wait", time_wait, signatures::STD_TIME_WAIT)?
        .with_function_and_signature("parse", time_parse, signatures::STD_TIME_PARSE)?
        .with_value("datetime", datetime::create(luau)?)?

        .with_function_and_signature("years", |_luau: &Lua, value: LuaValue| -> WrappedResult<TimeDuration> {
            let function_name = "time.years(y: number)";
            let years = match value {
                LuaValue::Number(f) => f,
                LuaValue::Integer(i) => i as f64,
                other => {
                    return wrap_err!("{} expected number, got: {:?}", function_name, other);
                }
            };
            Ok(TimeDuration::years(years)?.into_wrapped())
        }, signatures::STD_TIME_YEARS)?


        .with_function_and_signature("months", |_luau: &Lua, value: LuaValue| -> WrappedResult<TimeDuration> {
            let function_name = "time.months(m: number)";
            let months = match value {
                LuaValue::Number(f) => f,
                LuaValue::Integer(i) => i as f64,
                other => {
                    return wrap_err!("{} expected number, got: {:?}", function_name, other);
                }
            };
            Ok(TimeDuration::months(months)?.into_wrapped())
        }, signatures::STD_TIME_MONTHS)?

        .with_function_and_signature("days", |_luau: &Lua, value: LuaValue| -> WrappedResult<TimeDuration> {
            let function_name = "time.days(d: number)";
            let days = match value {
                LuaValue::Number(f) => f,
                LuaValue::Integer(i) => i as f64,
                other => {
                    return wrap_err!("{} expected number, got: {:?}", function_name, other);
                }
            };
            Ok(TimeDuration::days(days)?.into_wrapped())
        }, signatures::STD_TIME_DAYS)?

        .with_function_and_signature("hours", |_luau: &Lua, value: LuaValue| -> WrappedResult<TimeDuration> {
            let function_name = "time.hours(h: number)";
            let hours = match value {
                LuaValue::Number(f) => f,
                LuaValue::Integer(i) => i as f64,
                other => {
                    return wrap_err!("{} expected number, got: {:?}", function_name, other);
                }
            };
            Ok(TimeDuration::hours(hours)?.into_wrapped())
        }, signatures::STD_TIME_HOURS)?

        .with_function_and_signature("minutes", |_luau: &Lua, value: LuaValue| -> WrappedResult<TimeDuration> {
            let function_name = "time.minutes(m: number)";
            let minutes = match value {
                LuaValue::Number(f) => f,
                LuaValue::Integer(i) => i as f64,
                other => {
                    return wrap_err!("{} expected number, got: {:?}", function_name, other);
                }
            };
            Ok(TimeDuration::minutes(minutes)?.into_wrapped())
        }, signatures::STD_TIME_MINUTES)?

        .with_function_and_signature("seconds", |_luau: &Lua, value: LuaValue| -> WrappedResult<TimeDuration> {
            let function_name = "time.seconds(s: number)";
            let seconds = match value {
                LuaValue::Number(f) => f,
                LuaValue::Integer(i) => i as f64,
                other => {
                    return wrap_err!("{} expected number, got: {:?}", function_name, other);
                }
            };
            Ok(TimeDuration::seconds(seconds)?.into_wrapped())
        }, signatures::STD_TIME_SECONDS)?

        .with_function_and_signature("milliseconds", |_luau: &Lua, value: LuaValue| -> WrappedResult<TimeDuration> {
            let function_name = "time.milliseconds(ms: number)";
            let ms = match value {
                LuaValue::Number(f) => f,
                LuaValue::Integer(i) => i as f64,
                other => {
                    return wrap_err!("{} expected number, got: {:?}", function_name, other);
                }
            };
            Ok(TimeDuration::milliseconds(ms)?.into_wrapped())
        }, signatures::STD_TIME_MILLISECONDS)?

        .with_function_and_signature("microseconds", |_luau: &Lua, value: LuaValue| -> WrappedResult<TimeDuration> {
            let function_name = "time.microseconds(us: number)";
            let us = match value {
                LuaValue::Number(f) => f,
                LuaValue::Integer(i) => i as f64,
                other => {
                    return wrap_err!("{} expected number, got: {:?}", function_name, other);
                }
            };
            Ok(TimeDuration::microseconds(us)?.into_wrapped())
        }, signatures::STD_TIME_MICROSECONDS)?

        .with_function_and_signature("nanoseconds", |_luau: &Lua, value: LuaValue| -> WrappedResult<TimeDuration> {
            let function_name = "time.nanoseconds(ns: number)";
            let ns = match value {
                LuaValue::Number(f) => f,
                LuaValue::Integer(i) => i as f64,
                other => {
                    return wrap_err!("{} expected number, got: {:?}", function_name, other);
                }
            };
            Ok(TimeDuration::nanoseconds(ns)?.into_wrapped())
        }, signatures::STD_TIME_NANOSECONDS)?

        .build_readonly()
}