use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct UptimeReport {
    pub id: Uuid,
    pub nonce: Uuid,
    pub user_id: Uuid,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    pub ip: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub country: Option<String>,
    pub timezone: Option<String>,
    pub isp: Option<String>,
}
