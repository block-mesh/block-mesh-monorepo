use block_mesh_common::interfaces::server_api::FeedElement;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct DataSink {
    pub id: Uuid,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub raw: String,
    pub value: serde_json::Value,
    pub origin: String,
    pub origin_id: String,
    pub user_name: String,
    pub link: String,
}

impl DataSink {
    pub async fn create_data_sink(
        transaction: &mut Transaction<'_, Postgres>,
        user_id: &Uuid,
        data: FeedElement,
    ) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO data_sinks
            (user_id, origin, origin_id, user_name, link, raw)
            VALUES
            ($1, $2, $3, $4, $5, $6)
            "#,
            user_id,
            data.origin,
            format!("{}_{}", data.origin, data.id),
            data.user_name,
            data.link,
            data.raw
        )
        .execute(&mut **transaction)
        .await?;
        Ok(())
    }
}
