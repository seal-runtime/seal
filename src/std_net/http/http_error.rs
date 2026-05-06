use crate::prelude::*;
use mluau::prelude::*;

use ureq::Error as UreqError;
use ureq::Timeout as UreqTimeout;
use super::timeout_info::TimeoutInfo;

/// Represents both HttpIoError and HttpTimeoutError tables
pub struct HttpError {
    pub kind: &'static str,
    pub reason: String,
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
        let reason = info
            .map(|t| t.describe_elapsed(which))
            .unwrap_or_else(|| format!("{:?} timeout elapsed", which));
        Self { kind: "Timeout", reason, timeout_phase: Some(phase) }
    }
    pub fn from_error(err: UreqError, timeout_info: Option<TimeoutInfo>) -> Self {
        match err {
            UreqError::Timeout(which) => {
                Self::from_timeout(which, timeout_info)
            },
            UreqError::BodyExceedsLimit(limit) => Self {
                kind: "BodyExceedsLimit",
                reason: format!("response body exceeds max_body_size limit of {} bytes", limit),
                timeout_phase: None,
            },
            UreqError::Io(err) => Self {
                kind: "NetworkError",
                reason: format!("network error: {}", err),
                timeout_phase: None,
            },
            UreqError::ConnectionFailed => Self {
                kind: "ConnectionFailed",
                reason: "connection failed".to_string(),
                timeout_phase: None,
            },
            UreqError::TooManyRedirects => Self {
                kind: "TooManyRedirects",
                reason: "too many redirects".to_string(),
                timeout_phase: None,
            },
            other => Self {
                kind: "Other",
                reason: format!("request error: {}", other),
                timeout_phase: None,
            },
        }
    }
    pub fn into_table(self, luau: &Lua) -> LuaResult<LuaTable> {
        let mut builder = TableBuilder::create(luau)?
            .with_value("ok", false)?
            .with_value("kind", self.kind)?
            .with_value("reason", self.reason)?;
        if let Some(phase) = self.timeout_phase {
            builder = builder.with_value("phase", phase)?;
        }
        builder.build_readonly()
    }
}
