use anyhow::anyhow;
use clickhouse::Client;

#[tracing::instrument(name = "migrate_clickhouse", skip_all, ret, err)]
pub async fn migrate_clickhouse(client: &Client) -> anyhow::Result<()> {
    client
        .query(
            r#"
CREATE TABLE IF NOT EXISTS data_sinks_clickhouse
(
    id UUID DEFAULT generateUUIDv4(),
    user_id UUID,
    origin String,
    origin_id String,
    user_name String,
    link String,
    created_at DateTime64(9) DEFAULT now(),
    updated_at DateTime64(9) DEFAULT now(),
    raw String
)
ENGINE = MergeTree
ORDER BY (user_name, origin_id) -- Optimize queries that filter by these
SETTINGS index_granularity = 8192
        "#,
        )
        .execute()
        .await
        .map_err(|e| anyhow!("Error {}", e))
}
