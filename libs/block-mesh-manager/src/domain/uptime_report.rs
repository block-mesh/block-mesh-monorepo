use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct UptimeReport {
    pub id: Uuid,
    pub nonce: Uuid,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub ip: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub country: Option<String>,
    pub country_code: Option<String>,
    pub timezone: Option<String>,
    pub isp: Option<String>,
}
