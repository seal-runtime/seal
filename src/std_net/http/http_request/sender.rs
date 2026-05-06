use crate::prelude::*;
use mluau::prelude::*;

use ureq::http::{HeaderValue, Method};
use ureq::http::header::CONTENT_TYPE;

use super::super::{
    TimeoutInfo,
    ResponseWithBody
};
use super::RequestBody;
use super::UreqError;

/// Configuring the config builder applies to both WithBody and WithoutBody;
/// and there's no syntactical difference between the branches
/// so I replaced the duplicate logic with this macro.
macro_rules! configure_config_builder {
    ($builder:expr, $timeout:expr, $max_redirects:expr) => {
        {
            let mut configuring = $builder
                .config()
                .http_status_as_error(false);

            if let Some(max_redirects) = $max_redirects {
                configuring = configuring.max_redirects(max_redirects);
            }
    
            if let Some(timeout) = $timeout {
                match timeout {
                    TimeoutInfo::Global(duration) => {
                        configuring = configuring.timeout_global(Some(duration));
                    },
                    TimeoutInfo::Custom { 
                        send_request, 
                        receive_response,
                        send_body,
                        receive_body,
                    } => {
                        configuring = configuring
                            .timeout_send_request(send_request)
                            .timeout_send_body(send_body)
                            .timeout_recv_response(receive_response)
                            .timeout_recv_body(receive_body);
                    }
                }
            }
    
            configuring.build()
        }
    }
}

/// Applying headers/params has the exact same logic in both WithBody and WithoutBody
/// branches, made it a macro to reduce duplication here as well.
/// 
/// Note that this macro mutates `builder`.
macro_rules! apply_headers_and_params {
    ($headers:expr, $params:expr, $builder:expr) => {
        if let Some(headers) = $headers {
            for (key, value) in headers {
                $builder = $builder.header(key, value);
            }
        }

        if let Some(params) = $params {
            $builder = $builder.query_pairs(params);
        }
    }
}

pub enum Sender {
    WithoutBody(ureq::RequestBuilder<ureq::typestate::WithoutBody>),
    WithBody(ureq::RequestBuilder<ureq::typestate::WithBody>),
}
impl Sender {
    pub(super) fn from_http_method(
        m: Method, 
        uri: String, 
        function_name: &'static str
    ) -> LuaResult<Self> {
        let builder = match m {
            // should be without body
            Method::GET => Self::WithoutBody(ureq::get(uri)),
            Method::TRACE => Self::WithoutBody(ureq::trace(uri)),
            Method::DELETE => Self::WithoutBody(ureq::delete(uri)),
            Method::CONNECT => Self::WithoutBody(ureq::connect(uri)),
            Method::HEAD => Self::WithoutBody(ureq::head(uri)),
            Method::OPTIONS => Self::WithoutBody(ureq::options(uri)),

            // should have body
            Method::POST => Self::WithBody(ureq::post(uri)),
            Method::PATCH => Self::WithBody(ureq::patch(uri)),
            Method::PUT => Self::WithBody(ureq::put(uri)),

            // idk what to do with these
            other => {
                return wrap_err!("{}: new or extension HTTP method {} not supported", function_name, other);
            }
        };

        Ok(builder)
    }

    pub(super) fn configure(
        self, 
        timeout: Option<TimeoutInfo>,
        max_redirects: Option<u32>
    ) -> Self {
        match self {
            Self::WithBody(builder) => {
                let builder = configure_config_builder!(builder, timeout, max_redirects);
                Self::WithBody(builder)
            },
            Self::WithoutBody(builder) => {
                let builder = configure_config_builder!(builder, timeout, max_redirects);
                Self::WithoutBody(builder)
            }
        }
    }

    pub(super) fn send(
        self,
        headers: Option<Vec<(String, String)>>,
        params: Option<Vec<(String, String)>>,
        body: Option<RequestBody>,
    ) -> UreqResponseResult {
        match self {
            Sender::WithoutBody(mut builder) => {
                apply_headers_and_params!(headers, params, builder);

                if let Some(body) = body {
                    let builder = builder.force_send_body();
                    builder.send_with_body_context(body)
                } else {
                    builder.call()
                }
            },
            Sender::WithBody(mut builder) => {
                apply_headers_and_params!(headers, params, builder);

                if let Some(body) = body {
                    builder.send_with_body_context(body)
                } else {
                    builder.send_empty()
                }
            }
        }
    }
}

type UreqResponseResult = Result<ResponseWithBody, UreqError>;
type RequestBuilderWithBody = ureq::RequestBuilder<ureq::typestate::WithBody>;

trait SendWithContext {
    fn send_with_body_context(self, body: RequestBody) -> UreqResponseResult;
}
impl SendWithContext for RequestBuilderWithBody {
    fn send_with_body_context(mut self, body: RequestBody) -> UreqResponseResult {
        match body {
            RequestBody::Text(body) => {
                self.send(body)
            },
            RequestBody::Json(body) => {
                if let Some(heads) = self.headers_mut() 
                    && !heads.contains_key(CONTENT_TYPE) 
                {
                    heads.append(
                        CONTENT_TYPE, 
                        HeaderValue::from_static("application/json; charset=utf-8")
                    );
                }

                self.send(body)
            },
            RequestBody::Bytes(bytes) => {
                self.send(bytes)
            }
        }
    }
}