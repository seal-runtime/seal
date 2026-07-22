//! Shared TLS trust configuration for @std/net's http requests (ureq) and websockets
//! (tungstenite). Both are configured from the same env vars so behavior is consistent
//! across both.
//!
//! - `SSL_CERT_FILE`: path to a PEM file containing one or more additional CA
//!   certificates to trust (e.g. a corporate MITM proxy or self-hosted CA).
//! - `SEAL_SYSTEM_CERTS`: whether to also trust the OS's certificate store. Defaults to
//!   true; set to "0"/"false"/"no" to trust *only* the certs from `SSL_CERT_FILE`
//!   (or, if that's unset too, only the bundled Mozilla root list).

use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, OnceLock};

use ureq::tls::{Certificate, PemItem, RootCerts};

fn warn(message: &str) {
    let mut stderr = std::io::stderr().lock();
    let _ = writeln!(stderr, "[WARN] {}", message);
}

struct TlsSettings {
    ca_cert_path: Option<PathBuf>,
    use_system_certs: bool,
}

impl TlsSettings {
    fn from_env() -> Self {
        let ca_cert_path = std::env::var_os("SSL_CERT_FILE").map(PathBuf::from);
        let use_system_certs = match std::env::var("SEAL_SYSTEM_CERTS") {
            Ok(v) => !matches!(v.trim().to_ascii_lowercase().as_str(), "0" | "false" | "no"),
            Err(_) => true,
        };
        Self { ca_cert_path, use_system_certs }
    }
}

fn settings() -> &'static TlsSettings {
    static SETTINGS: OnceLock<TlsSettings> = OnceLock::new();
    SETTINGS.get_or_init(TlsSettings::from_env)
}

/// Loads and parses the user-provided CA cert bundle (if any).
fn custom_ca_certs() -> &'static [Certificate<'static>] {
    static CUSTOM_CERTS: OnceLock<Vec<Certificate<'static>>> = OnceLock::new();
    CUSTOM_CERTS.get_or_init(|| {
        let Some(path) = &settings().ca_cert_path else {
            return Vec::new();
        };

        let bytes = match std::fs::read(path) {
            Ok(bytes) => bytes,
            Err(err) => {
                warn(&format!("SSL_CERT_FILE={} could not be read: {}", path.display(), err));
                return Vec::new();
            }
        };

        let certs: Vec<Certificate<'static>> = ureq::tls::parse_pem(&bytes)
            .filter_map(|item| match item {
                Ok(PemItem::Certificate(cert)) => Some(cert),
                Ok(_) => None,
                Err(err) => {
                    warn(&format!("error parsing SSL_CERT_FILE={}: {}", path.display(), err));
                    None
                }
            })
            .collect();

        if certs.is_empty() {
            warn(&format!("SSL_CERT_FILE={} contains no PEM-encoded certificates", path.display()));
        }

        certs
    })
}

fn native_certs_as_ureq_certs() -> Vec<Certificate<'static>> {
    let result = rustls_native_certs::load_native_certs();
    for err in &result.errors {
        warn(&format!("error loading a system root certificate: {}", err));
    }
    result
        .certs
        .into_iter()
        .map(|der| Certificate::from_der(der.as_ref()).to_owned())
        .collect()
}

/// The `ureq::tls::RootCerts` to use, combining `SSL_CERT_FILE` and
/// `SEAL_SYSTEM_CERTS` as configured.
pub fn ureq_root_certs() -> RootCerts {
    static ROOT_CERTS: OnceLock<RootCerts> = OnceLock::new();
    ROOT_CERTS
        .get_or_init(|| {
            let custom = custom_ca_certs();
            let use_system = settings().use_system_certs;

            match (custom.is_empty(), use_system) {
                (true, true) => RootCerts::PlatformVerifier,
                (true, false) => RootCerts::WebPki,
                (false, _) => {
                    let mut certs = custom.to_vec();
                    if use_system {
                        certs.extend(native_certs_as_ureq_certs());
                    }
                    RootCerts::new_with_certs(&certs)
                }
            }
        })
        .clone()
}

/// The rustls `ClientConfig` websockets (tungstenite) should use, sharing the same
/// trust settings as `ureq_root_certs`.
pub fn rustls_client_config() -> Arc<rustls::ClientConfig> {
    static CONFIG: OnceLock<Arc<rustls::ClientConfig>> = OnceLock::new();
    CONFIG
        .get_or_init(|| {
            let mut store = rustls::RootCertStore::empty();
            let use_system = settings().use_system_certs;
            let custom = custom_ca_certs();

            // Mozilla's bundled list is only a fallback for when we have nothing else to trust:
            // either the system store couldn't be read, or the user disabled system certs and
            // didn't provide their own CA either. If the user explicitly disabled system certs
            // and provided SSL_CERT_FILE, we should trust *only* that (matching ureq_root_certs),
            // not silently widen trust back out to the public CA list.
            if use_system {
                let result = rustls_native_certs::load_native_certs();
                for err in &result.errors {
                    warn(&format!("error loading a system root certificate: {}", err));
                }
                let (added, _ignored) = store.add_parsable_certificates(result.certs);
                if added == 0 && custom.is_empty() {
                    store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
                }
            } else if custom.is_empty() {
                store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
            }

            for cert in custom {
                let der = rustls_pki_types::CertificateDer::from(cert.der().to_vec());
                if let Err(err) = store.add(der) {
                    warn(&format!("unable to trust a certificate from SSL_CERT_FILE: {}", err));
                }
            }

            let provider = rustls::crypto::CryptoProvider::get_default()
                .cloned()
                .unwrap_or_else(|| Arc::new(rustls::crypto::ring::default_provider()));

            let config = rustls::ClientConfig::builder_with_provider(provider)
                .with_safe_default_protocol_versions()
                .expect("rustls default protocol versions are always valid")
                .with_root_certificates(store)
                .with_no_client_auth();

            Arc::new(config)
        })
        .clone()
}
