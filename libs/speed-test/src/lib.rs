pub mod types;
pub mod utils;

pub use types::metadata::Metadata;
pub use utils::*;
pub const BASE_URL: &str = "https://speed.cloudflare.com";
pub const DOWNLOAD_URL: &str = "__down?bytes=";
pub const UPLOAD_URL: &str = "__up";
