use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

pub static BLOCK_MESH_IP_WORKER: &str = "https://cloudflare-worker-ip-data.blockmesh.xyz/";
pub static BLOCK_MESH_PROGRAM_ID: &str = "FRkQxATWhWqkj3SPZmbBCtkVM4fChd6VYLbEGhgCuHHJ";
// pub static BLOCK_MESH_LOGGER: &str = "https://cloudflare-worker-logs-queue.blockmesh.xyz";
pub static BLOCK_MESH_LOGGER: &str = "https://cloudflare-worker-logger-proxy.blockmesh.xyz";

pub static BLOCK_MESH_LOG_ENV: &str = "BLOCKMESH_LOG_ENV";

pub static BLOCKMESH_HOME_DIR_ENVAR: &str = "BLOCKMESH_HOME_DIR";
pub static BLOCKMESH_DISABLE_GUI_ENVAR: &str = "BLOCKMESH_DISABLE_GUI";

pub static CONFIG_FILENAME: &str = "blockmesh.json";

pub static BLOCKMESH_SERVER_UUID_ENVAR: &str = "BLOCKMESH_SERVER_UUID";

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum DeviceType {
    Extension,
    Desktop,
    Mobile,
    Tablet,
    Unknown,
    TestProxyMaster,
    TestProxyEndpoint,
    AppServer,
}

impl Display for DeviceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceType::Extension => write!(f, "extension"),
            DeviceType::Desktop => write!(f, "desktop"),
            DeviceType::Mobile => write!(f, "mobile"),
            DeviceType::Tablet => write!(f, "tablet"),
            DeviceType::Unknown => write!(f, "unknown"),
            DeviceType::TestProxyMaster => write!(f, "blabla-proxy-master"),
            DeviceType::TestProxyEndpoint => write!(f, "blabla-proxy-endpoint"),
            DeviceType::AppServer => write!(f, "app-server"),
        }
    }
}
