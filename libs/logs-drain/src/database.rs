use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};
use uuid::Uuid;
#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct LogEntry {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub raw: String,
}

#[tracing::instrument(name = "store_log_entry", skip_all)]
pub async fn store_log_entry(
    transaction: &mut Transaction<'_, Postgres>,
    raw: &str,
) -> anyhow::Result<()> {
    let _ = sqlx::query!(
        r#"
        INSERT
        INTO log_entries
        (raw)
        VALUES
        ($1)
        "#,
        raw
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
