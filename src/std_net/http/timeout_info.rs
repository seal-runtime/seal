use crate::prelude::*;
use mluau::prelude::*;

use std::time::Duration;
use crate::std_time::duration::TimeDuration;

const TIMEOUT_TYPE: &str = "{ request: Duration?, response: Duration?, send_body: Duration?, receive_body: Duration? }";

enum MaybeDuration {
    Some(Duration),
    None,
    KeepGoing(LuaTable),
}

pub enum TimeoutInfo {
    Global(Duration),
    Custom {
        request_timeout: Option<Duration>,
        response_timeout: Option<Duration>,
        send_body: Option<Duration>,
        receive_body: Option<Duration>,
    }
}
impl TimeoutInfo {
    /// timeout can be specified as a global timeout (timeout: Duration)
    /// or have custom request/response timeouts by passing in a table
    pub fn from_config(config: LuaTable, function_name: &'static str) -> LuaResult<Option<Self>> {
        let timeout_options = match Self::index_and_keep_going(
            &config, 
            "timeout",
            "RequestOptions.timeout",
            function_name,
        )? {
            MaybeDuration::KeepGoing(t) => t,
            MaybeDuration::Some(duration) => {
                return Ok(Some(Self::Global(duration)));
            },
            MaybeDuration::None => {
                return Ok(None);
            },
        };

        let request_timeout = Self::get_field(
            &timeout_options,
            "request",
            "RequestOptions.timeout.request",
            function_name
        )?;

        let response_timeout = Self::get_field(
            &timeout_options, 
            "response",
            "RequestOptions.timeout.response",
            function_name,
        )?;

        let send_body = Self::get_field(
            &timeout_options,
            "send_body",
            "RequestOptions.timeout.send_body",
            function_name,
        )?;

        let receive_body = Self::get_field(
            &timeout_options,
            "receive_body",
            "RequestOptions.timeout.receive",
            function_name,
        )?;

        if request_timeout.is_none() 
            && response_timeout.is_none() 
            && send_body.is_none() 
            && receive_body.is_none()
        {
            return wrap_err!("{}: RequestOptions.timeout is a table but all the fields we expected are nil\nExpected fields: \n  {}\nGot: \n  {:#?}", function_name, TIMEOUT_TYPE, timeout_options);
        }

        Ok(Some(TimeoutInfo::Custom { 
            request_timeout, 
            response_timeout,
            send_body,
            receive_body,
        }))
    }

    fn index_and_keep_going(
        t: &LuaTable,
        key: &'static str, 
        field_name: &'static str, 
        function_name: &'static str
    ) -> LuaResult<MaybeDuration> {
        Self::get_duration_from_table(t, key, field_name, function_name, true)
    }

    fn get_field(
        t: &LuaTable,
        key: &'static str,
        field_name: &'static str,
        function_name: &'static str,
    ) -> LuaResult<Option<Duration>> {
        match Self::get_duration_from_table(t, key, field_name, function_name, false)? {
            MaybeDuration::Some(s) => Ok(Some(s)),
            MaybeDuration::None => Ok(None),
            MaybeDuration::KeepGoing(_) => {
                unreachable!("KeepGoing cannot be triggered when keep_going == false")
            }
        }
    }

    fn get_duration_from_table(
        t: &LuaTable,
        key: &'static str,
        field_name: &'static str,
        function_name: &'static str,
        keep_going: bool,
    ) -> LuaResult<MaybeDuration> {
        Ok(match t.raw_get(key)? {
            LuaValue::UserData(ud) if let Ok(duration) = ud.borrow::<TimeDuration>() => {
                let timeout = (*duration).inner; // SignedDuration is clone on copy, we don't have to worry about dropping it here
                if !timeout.is_positive() {
                    return wrap_err!("{}: {}: Duration must be positive, got: {:#?}", function_name, field_name, timeout);
                }
                MaybeDuration::Some(timeout.unsigned_abs())
            },
            LuaValue::UserData(ud) => {
                let type_name = ud.type_name()?.unwrap_or("userdata (missing __type metafield)".to_string());
                return wrap_err!("{}: expected {} to be a Duration userdata, but got the wrong kind of userdata: {}", function_name, field_name, type_name);
            },
            LuaValue::Number(_) | LuaValue::Integer(_) => {
                return wrap_err!("{}: {}: should be a Duration (from @std/time), not a regular number", function_name, field_name);
            },
            LuaValue::Table(t) if keep_going => MaybeDuration::KeepGoing(t),
            LuaNil => MaybeDuration::None,
            other => {
                if keep_going {
                    return wrap_err!("{}: expected {} to be a Duration, {}, or nil, got: {:?}", function_name, field_name, TIMEOUT_TYPE, other);
                } else {
                    return wrap_err!("{}: expected {} to be a Duration or nil, got: {:?}", function_name, field_name, other);
                }
            },
        })
    }
}