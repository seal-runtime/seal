use mluau::prelude::*;
use ureq::Error as UreqError;
use ureq::Timeout as UreqTimeout;
use super::timeout_info::TimeoutInfo;

pub struct HttpError {
    pub kind: &'static str,
    pub message: String,
    pub timeout_phase: Option<&'static str>,
}

impl HttpError {
    pub fn from_timeout(which: ureq::Timeout, info: Option<TimeoutInfo>) -> Self {
        let phase = match which {
            UreqTimeout::Global | UreqTimeout::PerCall => "Global",
            UreqTimeout::SendRequest => "SendRequest",
            UreqTimeout::RecvResponse => "ReceiveResponse",
            UreqTimeout::SendBody => "SendBody",
            UreqTimeout::RecvBody => "ReceiveBody",
            UreqTimeout::Connect => "Connect",
            UreqTimeout::Resolve => "Resolve",
            _ => "Other",
        };
        let message = info
            .map(|t| t.describe_elapsed(which))
            .unwrap_or_else(|| format!("{:?} timeout elapsed", which));
        Self { kind: "Timeout", message, timeout_phase: Some(phase) }
    }
    pub fn from_error(err: UreqError, timeout_info: Option<TimeoutInfo>) -> Self {
        match err {
            UreqError::Timeout(which) => {
                Self::from_timeout(which, timeout_info)
            },
            UreqError::BodyExceedsLimit(limit) => Self {
                kind: "BodyExceedsLimit",
                message: format!("response body exceeds max_body_size limit of {} bytes", limit),
                timeout_phase: None,
            },
            UreqError::Io(err) => Self {
                kind: "NetworkError",
                message: format!("network error: {}", err),
                timeout_phase: None,
            },
            UreqError::ConnectionFailed => Self {
                kind: "ConnectionFailed",
                message: "connection failed".to_string(),
                timeout_phase: None,
            },
            UreqError::TooManyRedirects => Self {
                kind: "TooManyRedirects",
                message: "too many redirects".to_string(),
                timeout_phase: None,
            },
            other => Self {
                kind: "Other",
                message: format!("request error: {}", other),
                timeout_phase: None,
            },
        }
    }
}

impl LuaUserData for HttpError {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_meta_field("__type", "HttpError");
        fields.add_field_method_get("kind", |_, this| Ok(this.kind));
        fields.add_field_method_get("message", |_, this| Ok(this.message.clone()));
        fields.add_field_method_get("timeout_phase", |_, this| Ok(this.timeout_phase));
    }
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |_, this, _: ()| {
            Ok(format!("HttpError: {}", this.message.clone()))
        });
    }
}
