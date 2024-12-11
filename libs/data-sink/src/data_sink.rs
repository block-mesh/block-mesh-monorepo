use block_mesh_common::interfaces::server_api::FeedElement;
use chrono::{DateTime, Utc};
use clickhouse::sql::Identifier;
use clickhouse::Client;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};
use std::time::UNIX_EPOCH;
use uuid::Uuid;

pub const CLICKHOUSE_TABLE_NAME: &str = "data_sinks_clickhouse";
#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone, clickhouse::Row)]
pub struct DataSink {
    pub id: Uuid,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub raw: String,
    pub origin: String,
    pub origin_id: String,
    pub user_name: String,
    pub link: String,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone, clickhouse::Row)]
pub struct DataSinkClickHouse {
    #[serde(with = "clickhouse::serde::uuid")]
    pub id: Uuid,
    #[serde(with = "clickhouse::serde::uuid")]
    pub user_id: Uuid,
    pub created_at: u64,
    pub updated_at: u64,
    pub raw: String,
    pub origin: String,
    pub origin_id: String,
    pub user_name: String,
    pub link: String,
}

impl DataSink {
    #[allow(dead_code)]
    #[tracing::instrument(name = "create_data_sink", skip_all)]
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

    #[allow(dead_code)]
    #[tracing::instrument(name = "create_data_sink_clickhouse", skip_all)]
    pub async fn create_data_sink_clickhouse(
        client: &Client,
        user_id: &Uuid,
        data: FeedElement,
    ) -> anyhow::Result<()> {
        let mut insert = client.insert(CLICKHOUSE_TABLE_NAME)?;
        let now = Utc::now().timestamp_nanos_opt().unwrap_or(now_backup());
        let row = DataSinkClickHouse {
            id: Uuid::new_v4(),
            user_id: *user_id,
            raw: data.raw,
            origin: data.origin,
            origin_id: data.id,
            user_name: data.user_name,
            link: data.link,
            created_at: now as u64,
            updated_at: now as u64,
        };
        insert.write(&row).await?;
        insert.end().await?;
        Ok(())
    }

    #[tracing::instrument(name = "dup_exists_clickhouse", skip_all)]
    pub async fn dup_exists_clickhouse(
        client: &Client,
        origin: &str,
        origin_id: &str,
    ) -> anyhow::Result<bool> {
        let result = client
            // .query("select * from ?")
            .query("SELECT ?fields FROM ? WHERE origin = ? AND origin_id = ? LIMIT 1")
            .bind(Identifier(CLICKHOUSE_TABLE_NAME))
            .bind(origin)
            .bind(origin_id)
            .fetch_optional::<DataSinkClickHouse>()
            .await?;
        Ok(result.is_some())
    }
}

pub fn now_backup() -> i64 {
    UNIX_EPOCH
        .elapsed()
        .expect("invalid system time")
        .as_nanos() as i64
}
