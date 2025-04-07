use super::RamaState;
use rama::{
    Context,
    error::{ErrorContext, OpaqueError},
    http::Request,
    net::fingerprint::{Ja3, Ja4, Ja4H},
    tls::types::{
        SecureTransport,
        client::{ClientHello, ClientHelloExtension},
    },
    ua::profile::Http2Settings,
};
use serde::Serialize;
use std::{str::FromStr, sync::Arc};

#[derive(Debug, Clone, Default, Serialize)]
#[allow(dead_code)]
pub enum FetchMode {
    Cors,
    #[default]
    Navigate,
    NoCors,
    SameOrigin,
    Websocket,
}

impl std::fmt::Display for FetchMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cors => write!(f, "cors"),
            Self::Navigate => write!(f, "navigate"),
            Self::NoCors => write!(f, "no-cors"),
            Self::SameOrigin => write!(f, "same-origin"),
            Self::Websocket => write!(f, "websocket"),
        }
    }
}

impl FromStr for FetchMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cors" => Ok(Self::Cors),
            "navigate" => Ok(Self::Navigate),
            "no-cors" => Ok(Self::NoCors),
            "same-origin" => Ok(Self::SameOrigin),
            "websocket" => Ok(Self::Websocket),
            _ => Err(s.to_owned()),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[allow(dead_code)]
pub enum ResourceType {
    #[default]
    Document,
    Xhr,
    Form,
}

impl std::fmt::Display for ResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Document => write!(f, "document"),
            Self::Xhr => write!(f, "xhr"),
            Self::Form => write!(f, "form"),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[allow(dead_code)]
pub enum Initiator {
    #[default]
    Navigator,
    Fetch,
    XMLHttpRequest,
    Form,
}

impl std::fmt::Display for Initiator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Navigator => write!(f, "navigator"),
            Self::Fetch => write!(f, "fetch"),
            Self::XMLHttpRequest => write!(f, "xmlhttprequest"),
            Self::Form => write!(f, "form"),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct DataSource {
    pub name: String,
    pub version: String,
}

impl Default for DataSource {
    fn default() -> Self {
        Self {
            name: rama::utils::info::NAME.to_owned(),
            version: rama::utils::info::VERSION.to_owned(),
        }
    }
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct UserAgentInfo {
    pub user_agent: String,
    pub kind: Option<String>,
    pub version: Option<usize>,
    pub platform: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RequestInfo {
    pub version: String,
    pub scheme: String,
    pub authority: String,
    pub method: String,
    pub fetch_mode: FetchMode,
    pub resource_type: ResourceType,
    pub initiator: Initiator,
    pub path: String,
    pub uri: String,
    pub peer_addr: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Ja4HInfo {
    pub hash: String,
    pub human_str: String,
}

pub fn get_ja4h_info<B>(req: &Request<B>) -> Option<Ja4HInfo> {
    Ja4H::compute(req)
        .inspect_err(|err| tracing::error!(?err, "ja4h compute failure"))
        .ok()
        .map(|ja4h| Ja4HInfo {
            hash: format!("{ja4h}"),
            human_str: format!("{ja4h:?}"),
        })
}

#[derive(Debug, Clone, Serialize)]
pub struct HttpInfo {
    pub headers: Vec<(String, String)>,
    pub h2_settings: Option<Http2Settings>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TlsDisplayInfo {
    pub ja4: Ja4DisplayInfo,
    pub ja3: Ja3DisplayInfo,
    pub protocol_version: String,
    pub cipher_suites: Vec<String>,
    pub compression_algorithms: Vec<String>,
    pub extensions: Vec<TlsDisplayInfoExtension>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Ja4DisplayInfo {
    pub full: String,
    pub hash: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Ja3DisplayInfo {
    pub full: String,
    pub hash: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TlsDisplayInfoExtension {
    pub id: String,
    pub data: Option<TlsDisplayInfoExtensionData>,
}

#[derive(Debug, Clone, Serialize)]
pub enum TlsDisplayInfoExtensionData {
    Single(String),
    Multi(Vec<String>),
}

pub async fn get_tls_display_info_and_store(
    ctx: &Context<Arc<RamaState>>,
) -> Result<Option<TlsDisplayInfo>, OpaqueError> {
    let hello: &ClientHello = match ctx
        .get::<SecureTransport>()
        .and_then(|st| st.client_hello())
    {
        Some(hello) => hello,
        None => return Ok(None),
    };

    let ja4 = Ja4::compute(ctx.extensions()).context("ja4 compute")?;
    let ja3 = Ja3::compute(ctx.extensions()).context("ja3 compute")?;

    Ok(Some(TlsDisplayInfo {
        ja4: Ja4DisplayInfo {
            full: format!("{ja4:?}"),
            hash: format!("{ja4}"),
        },
        ja3: Ja3DisplayInfo {
            full: format!("{ja3}"),
            hash: format!("{ja3:x}"),
        },
        protocol_version: hello.protocol_version().to_string(),
        cipher_suites: hello
            .cipher_suites()
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>(),
        compression_algorithms: hello
            .compression_algorithms()
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>(),
        extensions: hello
            .extensions()
            .iter()
            .map(|extension| match extension {
                ClientHelloExtension::ServerName(domain) => TlsDisplayInfoExtension {
                    id: extension.id().to_string(),
                    data: domain
                        .as_ref()
                        .map(|d| TlsDisplayInfoExtensionData::Single(d.to_string())),
                },
                ClientHelloExtension::SignatureAlgorithms(v) => TlsDisplayInfoExtension {
                    id: extension.id().to_string(),
                    data: Some(TlsDisplayInfoExtensionData::Multi(
                        v.iter().map(|s| s.to_string()).collect(),
                    )),
                },
                ClientHelloExtension::SupportedVersions(v) => TlsDisplayInfoExtension {
                    id: extension.id().to_string(),
                    data: Some(TlsDisplayInfoExtensionData::Multi(
                        v.iter().map(|s| s.to_string()).collect(),
                    )),
                },
                ClientHelloExtension::ApplicationLayerProtocolNegotiation(v) => {
                    TlsDisplayInfoExtension {
                        id: extension.id().to_string(),
                        data: Some(TlsDisplayInfoExtensionData::Multi(
                            v.iter().map(|s| s.to_string()).collect(),
                        )),
                    }
                }
                ClientHelloExtension::SupportedGroups(v) => TlsDisplayInfoExtension {
                    id: extension.id().to_string(),
                    data: Some(TlsDisplayInfoExtensionData::Multi(
                        v.iter().map(|s| s.to_string()).collect(),
                    )),
                },
                ClientHelloExtension::ECPointFormats(v) => TlsDisplayInfoExtension {
                    id: extension.id().to_string(),
                    data: Some(TlsDisplayInfoExtensionData::Multi(
                        v.iter().map(|s| s.to_string()).collect(),
                    )),
                },
                ClientHelloExtension::CertificateCompression(v) => TlsDisplayInfoExtension {
                    id: extension.id().to_string(),
                    data: Some(TlsDisplayInfoExtensionData::Multi(
                        v.iter().map(|s| s.to_string()).collect(),
                    )),
                },
                ClientHelloExtension::RecordSizeLimit(v) => TlsDisplayInfoExtension {
                    id: extension.id().to_string(),
                    data: Some(TlsDisplayInfoExtensionData::Single(v.to_string())),
                },
                ClientHelloExtension::Opaque { id, data } => TlsDisplayInfoExtension {
                    id: id.to_string(),
                    data: if data.is_empty() {
                        None
                    } else {
                        Some(TlsDisplayInfoExtensionData::Single(format!(
                            "0x{}",
                            hex::encode(data)
                        )))
                    },
                },
            })
            .collect::<Vec<_>>(),
    }))
}
