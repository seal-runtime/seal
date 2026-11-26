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
            // tls and http_status_as_error are set on the agent by `new_agent`
            let mut configuring = $builder.config();

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

/// Creates a ureq agent that trusts the certs from `std_net::tls_config` and returns 4xx/5xx
/// statuses as responses instead of errors. Requests sent through the same agent (or its clones)
/// reuse each other's connections to the same host.
pub fn new_agent(max_idle_connections_per_host: Option<usize>) -> ureq::Agent {
    let mut config = ureq::Agent::config_builder()
        .http_status_as_error(false)
        .tls_config(
            ureq::tls::TlsConfig::builder()
                .provider(ureq::tls::TlsProvider::Rustls)
                // combines the OS trust store and/or a custom CA bundle depending on
                // SEAL_SYSTEM_CERTS/SSL_CERT_FILE; see std_net::tls_config
                .root_certs(crate::std_net::tls_config::ureq_root_certs())
                .build()
        );

    if let Some(per_host) = max_idle_connections_per_host {
        config = config
            .max_idle_connections_per_host(per_host)
            .max_idle_connections(per_host.max(10));
    }

    config.build().new_agent()
}

pub enum Sender {
    WithoutBody(ureq::RequestBuilder<ureq::typestate::WithoutBody>),
    WithBody(ureq::RequestBuilder<ureq::typestate::WithBody>),
}
impl Sender {
    /// Errors if we can't send requests with method `m`.
    pub(super) fn check_method(m: &Method, function_name: &'static str) -> LuaResult<()> {
        match *m {
            Method::GET | Method::TRACE | Method::DELETE | Method::CONNECT | Method::HEAD | Method::OPTIONS
            | Method::POST | Method::PATCH | Method::PUT => Ok(()),
            // idk what to do with these
            ref other => {
                wrap_err!("{}: new or extension HTTP method {} not supported", function_name, other)
            }
        }
    }

    /// `m` must have passed `Sender::check_method`.
    pub(super) fn from_http_method(agent: &ureq::Agent, m: Method, uri: String) -> Self {
        match m {
            // should be without body
            Method::GET => Self::WithoutBody(agent.get(uri)),
            Method::TRACE => Self::WithoutBody(agent.trace(uri)),
            Method::DELETE => Self::WithoutBody(agent.delete(uri)),
            Method::CONNECT => Self::WithoutBody(agent.connect(uri)),
            Method::HEAD => Self::WithoutBody(agent.head(uri)),
            Method::OPTIONS => Self::WithoutBody(agent.options(uri)),

            // should have body
            Method::POST => Self::WithBody(agent.post(uri)),
            Method::PATCH => Self::WithBody(agent.patch(uri)),
            Method::PUT => Self::WithBody(agent.put(uri)),

            other => unreachable!("Sender::check_method rejects {}", other),
        }
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
    fn set_content_type_if_unset(&mut self, value: HeaderValue);
    fn send_with_body_context(self, body: RequestBody) -> UreqResponseResult;
}
impl SendWithContext for RequestBuilderWithBody {
    fn set_content_type_if_unset(&mut self, value: HeaderValue) {
        if let Some(heads) = self.headers_mut()
            && !heads.contains_key(CONTENT_TYPE)
        {
            heads.append(CONTENT_TYPE, value);
        }
    }

    fn send_with_body_context(mut self, body: RequestBody) -> UreqResponseResult {
        match body {
            RequestBody::Text(body) => {
                self.set_content_type_if_unset(HeaderValue::from_static("text/plain; charset=utf-8"));
                self.send(body)
            },
            RequestBody::Json(body) => {
                self.set_content_type_if_unset(HeaderValue::from_static("application/json; charset=utf-8"));
                self.send(body)
            },
            RequestBody::Bytes(bytes) => {
                self.set_content_type_if_unset(HeaderValue::from_static("application/octet-stream"));
                self.send(bytes)
            }
        }
    }
}
