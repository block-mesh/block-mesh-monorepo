use std::fmt::Display;

#[derive(Debug)]
pub enum StorageValues {
    BlockMeshUrl,
    Email,
    ApiToken,
    DeviceId,
    Uptime,
    InviteCode,
    DownloadSpeed,
    UploadSpeed,
}

impl Display for StorageValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            StorageValues::BlockMeshUrl => "blockmesh_url".to_string(),
            StorageValues::Email => "email".to_string(),
            StorageValues::ApiToken => "blockmesh_api_token".to_string(),
            StorageValues::DeviceId => "device_id".to_string(),
            StorageValues::Uptime => "uptime".to_string(),
            StorageValues::InviteCode => "invite_code".to_string(),
            StorageValues::DownloadSpeed => "download_speed".to_string(),
            StorageValues::UploadSpeed => "upload_speed".to_string(),
        };
        write!(f, "{}", str)
    }
}
