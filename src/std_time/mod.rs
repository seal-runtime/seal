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
        LuaValue::UserData(ud) if let Ok(ud) = ud.borrow::<TimeDuration>() => {
            let signed = (*ud).inner;
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

pub fn create(luau: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::create(luau)?
        .with_function_and_signature("wait", time_wait, signatures::STD_TIME_WAIT)?
        .with_value("datetime", datetime::create(luau)?)?

        .with_function_and_signature("years", |luau: &Lua, value: LuaValue| -> LuaValueResult {
            let function_name = "time.years(y: number)";
            let years = match value {
                LuaValue::Number(f) => f,
                LuaValue::Integer(i) => i as f64,
                other => {
                    return wrap_err!("{} expected number, got: {:?}", function_name, other);
                }
            };
            TimeDuration::years(years)?.get_userdata(luau)
        }, signatures::STD_TIME_YEARS)?


        .with_function_and_signature("months", |luau: &Lua, value: LuaValue| -> LuaValueResult {
            let function_name = "time.months(m: number)";
            let months = match value {
                LuaValue::Number(f) => f,
                LuaValue::Integer(i) => i as f64,
                other => {
                    return wrap_err!("{} expected number, got: {:?}", function_name, other);
                }
            };
            TimeDuration::months(months)?.get_userdata(luau)
        }, signatures::STD_TIME_MONTHS)?

        .with_function_and_signature("days", |luau: &Lua, value: LuaValue| -> LuaValueResult {
            let function_name = "time.days(d: number)";
            let days = match value {
                LuaValue::Number(f) => f,
                LuaValue::Integer(i) => i as f64,
                other => {
                    return wrap_err!("{} expected number, got: {:?}", function_name, other);
                }
            };
            TimeDuration::days(days)?.get_userdata(luau)
        }, signatures::STD_TIME_DAYS)?

        .with_function_and_signature("hours", |luau: &Lua, value: LuaValue| -> LuaValueResult {
            let function_name = "time.hours(h: number)";
            let hours = match value {
                LuaValue::Number(f) => f,
                LuaValue::Integer(i) => i as f64,
                other => {
                    return wrap_err!("{} expected number, got: {:?}", function_name, other);
                }
            };
            TimeDuration::hours(hours)?.get_userdata(luau)
        }, signatures::STD_TIME_HOURS)?

        .with_function_and_signature("minutes", |luau: &Lua, value: LuaValue| -> LuaValueResult {
            let function_name = "time.minutes(m: number)";
            let minutes = match value {
                LuaValue::Number(f) => f,
                LuaValue::Integer(i) => i as f64,
                other => {
                    return wrap_err!("{} expected number, got: {:?}", function_name, other);
                }
            };
            TimeDuration::minutes(minutes)?.get_userdata(luau)
        }, signatures::STD_TIME_MINUTES)?

        .with_function_and_signature("seconds", |luau: &Lua, value: LuaValue| -> LuaValueResult {
            let function_name = "time.seconds(s: number)";
            let seconds = match value {
                LuaValue::Number(f) => f,
                LuaValue::Integer(i) => i as f64,
                other => {
                    return wrap_err!("{} expected number, got: {:?}", function_name, other);
                }
            };
            TimeDuration::seconds(seconds)?.get_userdata(luau)
        }, signatures::STD_TIME_SECONDS)?

        .with_function_and_signature("milliseconds", |luau: &Lua, value: LuaValue| -> LuaValueResult {
            let function_name = "time.milliseconds(ms: number)";
            let ms = match value {
                LuaValue::Number(f) => f,
                LuaValue::Integer(i) => i as f64,
                other => {
                    return wrap_err!("{} expected number, got: {:?}", function_name, other);
                }
            };
            TimeDuration::milliseconds(ms)?.get_userdata(luau)
        }, signatures::STD_TIME_MILLISECONDS)?

        .with_function_and_signature("microseconds", |luau: &Lua, value: LuaValue| -> LuaValueResult {
            let function_name = "time.microseconds(us: number)";
            let us = match value {
                LuaValue::Number(f) => f,
                LuaValue::Integer(i) => i as f64,
                other => {
                    return wrap_err!("{} expected number, got: {:?}", function_name, other);
                }
            };
            TimeDuration::microseconds(us)?.get_userdata(luau)
        }, signatures::STD_TIME_MICROSECONDS)?

        .with_function_and_signature("nanoseconds", |luau: &Lua, value: LuaValue| -> LuaValueResult {
            let function_name = "time.nanoseconds(ns: number)";
            let ns = match value {
                LuaValue::Number(f) => f,
                LuaValue::Integer(i) => i as f64,
                other => {
                    return wrap_err!("{} expected number, got: {:?}", function_name, other);
                }
            };
            TimeDuration::nanoseconds(ns)?.get_userdata(luau)
        }, signatures::STD_TIME_NANOSECONDS)?

        .build_readonly()
}