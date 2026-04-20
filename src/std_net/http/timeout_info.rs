use crate::prelude::*;
use mluau::prelude::*;

use std::time::Duration;
use crate::std_time::duration::TimeDuration;
use ureq::Timeout as UreqTimeout;

fn format_duration(d: Duration) -> String {
    let ms = d.as_millis();
    if ms >= 1000 {
        format!("{:.2}s", d.as_secs_f64())
    } else {
        format!("{}ms", ms)
    }
}

const TIMEOUT_TYPE: &str = "{ request: Duration?, response: Duration?, send_body: Duration?, receive_body: Duration? }";

enum MaybeDuration {
    Some(Duration),
    None,
    KeepGoing(LuaTable),
}

#[derive(Clone)]
pub enum TimeoutInfo {
    Global(Duration),
    Custom {
        send_request: Option<Duration>,
        send_response: Option<Duration>,
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

        let send_request = Self::get_field(
            &timeout_options,
            "send_request",
            "RequestOptions.timeout.send_request",
            function_name
        )?;

        let send_response = Self::get_field(
            &timeout_options, 
            "send_response",
            "RequestOptions.timeout.send_response",
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
            "RequestOptions.timeout.receive_body",
            function_name,
        )?;

        if send_request.is_none() 
            && send_response.is_none() 
            && send_body.is_none() 
            && receive_body.is_none()
        {
            return wrap_err!("{}: RequestOptions.timeout is a table but all the fields we expected are nil\nExpected fields: \n  {}\nGot: \n  {:#?}", function_name, TIMEOUT_TYPE, timeout_options);
        }

        Ok(Some(TimeoutInfo::Custom {
            send_request,
            send_response,
            send_body,
            receive_body,
        }))
    }

    pub fn describe_elapsed(&self, which: UreqTimeout) -> String {
        match self {
            TimeoutInfo::Global(dur) => {
                format!("global timeout of {} elapsed; consider increasing options.timeout", format_duration(*dur))
            },
            TimeoutInfo::Custom { send_request, send_response, send_body, receive_body } => {
                let (field, dur): (&str, Option<Duration>) = match which {
                    UreqTimeout::SendRequest => ("options.timeout.send_request", *send_request),
                    UreqTimeout::RecvResponse => ("options.timeout.send_response", *send_response),
                    UreqTimeout::SendBody => ("options.timeout.send_body", *send_body),
                    UreqTimeout::RecvBody => ("options.timeout.receive_body", *receive_body),
                    other => return format!("{:?} timeout elapsed", other),
                };
                match dur {
                    Some(d) => format!("{} timeout of {} elapsed; consider increasing {}", field, format_duration(d), field),
                    None => format!("{:?} timeout elapsed", which),
                }
            }
        }
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