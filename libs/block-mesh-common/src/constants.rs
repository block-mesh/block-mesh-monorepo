use serde::{Deserialize, Serialize};
use std::env;
use std::fmt::{Display, Formatter};
use std::sync::OnceLock;

pub const DEV_ENV: [&str; 3] = ["dev", "development", "local"];

pub fn env_url() -> String {
    static APP_ENVIRONMENT: OnceLock<String> = OnceLock::new();
    let app_environment =
        APP_ENVIRONMENT.get_or_init(|| env::var("APP_ENVIRONMENT").unwrap_or_default());
    if DEV_ENV.contains(&app_environment.as_str()) {
        "http://localhost:8000".to_string()
    } else {
        "https://app.blockmesh.xyz".to_string()
    }
}
pub static BLOCK_MESH_SUPPORT_CHAT: &str = "https://blockmesh.xyz/support-chat";

pub static BLOCK_MESH_FEATURE_FLAGS: &str = "https://feature-flags.blockmesh.xyz";
pub static BLOCK_MESH_SUPPORT_EMAIL: &str = "support@blockmesh.xyz";
pub static BLOCK_MESH_LOGO: &str =
    "https://imagedelivery.net/3RKw_J_fJQ_4KpJP3_YgXA/e4f3cdc0-c2ba-442d-3e48-e2f31c0dc100/public";

pub static BLOCK_MESH_LANDING_PAGE_IMAGE: &str =
    "https://imagedelivery.net/3RKw_J_fJQ_4KpJP3_YgXA/dfda0dd2-a321-4c75-cbbe-5521b2355f00/public";
pub static BLOCK_MESH_IP_WORKER: &str = "https://cloudflare-worker-ip-data.blockmesh.xyz/";
pub static BLOCK_MESH_PROGRAM_ID: &str = "FRkQxATWhWqkj3SPZmbBCtkVM4fChd6VYLbEGhgCuHHJ";
// pub static BLOCK_MESH_LOGGER: &str = "https://cloudflare-worker-logs-queue.blockmesh.xyz";
pub static BLOCK_MESH_LOGGER: &str = "https://cloudflare-worker-logger-proxy.blockmesh.xyz";

pub static BLOCK_MESH_CHROME_EXTENSION_LINK: &str =
    "https://chromewebstore.google.com/detail/blockmesh-network/obfhoiefijlolgdmphcekifedagnkfjp";

pub static BLOCK_MESH_APP_SERVER: &str = "https://app.blockmesh.xyz";

pub static BLOCK_MESH_GITHUB: &str = "https://github.com/block-mesh/block-mesh-monorepo";

pub static BLOCK_MESH_TWITTER: &str = "https://twitter.com/blockmesh_xyz";

pub static BLOCK_MESH_GITBOOK: &str = "https://gitbook.blockmesh.xyz/";

pub static BLOCKMESH_LOG_ENV: &str = "BLOCKMESH_LOG_ENV";

pub static BLOCKMESH_HOME_DIR_ENVAR: &str = "BLOCKMESH_HOME_DIR";
pub static BLOCKMESH_DISABLE_GUI_ENVAR: &str = "BLOCKMESH_DISABLE_GUI";

pub static CONFIG_FILENAME: &str = "blockmesh.json";

pub static BLOCKMESH_SERVER_UUID_ENVAR: &str = "BLOCKMESH_SERVER_UUID";

pub static BLOCKMESH_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
pub enum DeviceType {
    Extension,
    Desktop,
    Mobile,
    Tablet,
    Unknown,
    TestProxyMaster,
    TestProxyEndpoint,
    AppServer,
    Cli,
}

impl Display for DeviceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceType::Extension => write!(f, "extension"),
            DeviceType::Desktop => write!(f, "desktop"),
            DeviceType::Mobile => write!(f, "mobile"),
            DeviceType::Tablet => write!(f, "tablet"),
            DeviceType::Unknown => write!(f, "unknown"),
            DeviceType::TestProxyMaster => write!(f, "test-proxy-master"),
            DeviceType::TestProxyEndpoint => write!(f, "test-proxy-endpoint"),
            DeviceType::AppServer => write!(f, "app-server"),
            DeviceType::Cli => write!(f, "cli"),
        }
    }
}
