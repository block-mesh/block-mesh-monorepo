use serde::{Deserialize, Serialize};
use std::env;
use std::fmt::{Display, Formatter};
use std::sync::OnceLock;

pub const DEV_ENV: [&str; 3] = ["dev", "development", "local"];
pub const BLOCKMESH_TWITTER_USER_ID: u64 = 1766124448778784768;
pub const BLOCKMESH_FOUNDER_TWITTER_USER_ID: u64 = 1778711300127821824;
pub const BLOCKMESH_PG_NOTIFY_WORKER: &str = "pgchannel";
pub const BLOCKMESH_PG_NOTIFY_API: &str = "pgchannel_api";
pub const BLOCKMESH_VPS: &str = "https://vps.blockmesh.xyz";

pub const BLOCKMESH_WS_REDIS_COUNT_KEY: &str = "BLOCKMESH_WS_REDIS_COUNT_KEY";

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
    "https://imagedelivery.net/3RKw_J_fJQ_4KpJP3_YgXA/d68dc689-b8ad-492b-ffc4-8f1478685800/public";

pub static BLOCK_MESH_LANDING_PAGE_IMAGE: &str =
    "https://imagedelivery.net/3RKw_J_fJQ_4KpJP3_YgXA/dfda0dd2-a321-4c75-cbbe-5521b2355f00/public";
pub static BLOCK_MESH_IP_WORKER: &str = "https://cloudflare-worker-ip-data.blockmesh.xyz/";
pub static BLOCK_MESH_PROGRAM_ID: &str = "FRkQxATWhWqkj3SPZmbBCtkVM4fChd6VYLbEGhgCuHHJ";
// pub static BLOCK_MESH_LOGGER: &str = "https://cloudflare-worker-logs-queue.blockmesh.xyz";
pub static BLOCK_MESH_LOGGER: &str = "https://cloudflare-worker-logger-proxy.blockmesh.xyz";

pub static BLOCK_MESH_CHROME_EXTENSION_LINK: &str =
    "https://chromewebstore.google.com/detail/blockmesh-network/obfhoiefijlolgdmphcekifedagnkfjp";

pub static BLOCK_MESH_APP_SERVER: &str = "https://app.blockmesh.xyz";
pub static BLOCK_MESH_API_SERVER: &str = "https://api.blockmesh.xyz";

pub static BLOCK_MESH_GITHUB: &str = "https://github.com/block-mesh/block-mesh-monorepo";

pub static BLOCK_MESH_TWITTER: &str = "https://twitter.com/blockmesh_xyz";

pub static BLOCK_MESH_GITBOOK: &str = "https://gitbook.blockmesh.xyz/";

pub static BLOCKMESH_LOG_ENV: &str = "BLOCKMESH_LOG_ENV";

pub static BLOCKMESH_HOME_DIR_ENVAR: &str = "BLOCKMESH_HOME_DIR";
pub static BLOCKMESH_DISABLE_GUI_ENVAR: &str = "BLOCKMESH_DISABLE_GUI";

pub static CONFIG_FILENAME: &str = "blockmesh.json";

pub static BLOCKMESH_SERVER_UUID_ENVAR: &str = "BLOCKMESH_SERVER_UUID";

pub static BLOCKMESH_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
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
    Worker,
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
            DeviceType::Worker => write!(f, "worker"),
        }
    }
}

impl From<&str> for DeviceType {
    fn from(value: &str) -> Self {
        match value {
            "extension" => DeviceType::Extension,
            "desktop" => DeviceType::Desktop,
            "mobile" => DeviceType::Mobile,
            "tablet" => DeviceType::Tablet,
            "unknown" => DeviceType::Unknown,
            "test-proxy-master" => DeviceType::TestProxyMaster,
            "test-proxy-endpoint" => DeviceType::TestProxyEndpoint,
            "app-server" => DeviceType::AppServer,
            "cli" => DeviceType::Cli,
            "worker" => DeviceType::Worker,
            _ => DeviceType::Unknown,
        }
    }
}

impl From<String> for DeviceType {
    fn from(value: String) -> Self {
        DeviceType::from(value.as_str())
    }
}
