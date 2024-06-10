use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct BandwidthReport {
    pub id: Uuid,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub download_speed: f64,
    pub upload_speed: f64,
    pub latency: f64,
    pub country: String,
    pub city: String,
    pub ip: String,
    pub asn: String,
    pub colo: String,
}
