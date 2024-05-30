use std::fmt::Display;

#[derive(Debug)]
pub enum StorageValues {
    BlockMeshUrl,
    Email,
    ApiToken,
    DeviceId,
    Uptime,
}

impl Display for StorageValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            StorageValues::BlockMeshUrl => "blockmesh_url".to_string(),
            StorageValues::Email => "email".to_string(),
            StorageValues::ApiToken => "blockmesh_api_token".to_string(),
            StorageValues::DeviceId => "device_id".to_string(),
            StorageValues::Uptime => "uptime".to_string(),
        };
        write!(f, "{}", str)
    }
}
