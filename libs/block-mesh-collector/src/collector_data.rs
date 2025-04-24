use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct CollectorData {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub source: String,
    pub data: Value,
}

impl CollectorData {
    pub async fn create_new_collector_data(
        transaction: &mut Transaction<'_, Postgres>,
        source: &str,
        data: &Value,
    ) -> anyhow::Result<()> {
        let now = Utc::now();
        let id = Uuid::new_v4();
        sqlx::query!(
            r#"
            INSERT INTO collector_datas
            (id, created_at, source, data)
            VALUES ($1, $2, $3, $4)
        "#,
            id,
            now,
            source,
            data
        )
        .execute(&mut **transaction)
        .await?;
        Ok(())
    }
}
