use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use sqlx::Postgres;
use sqlx::{PgPool, Transaction};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct Flag {
    pub id: Uuid,
    pub name: String,
    pub value: serde_json::Value,
    pub created_at: Option<DateTime<Utc>>,
}

pub async fn get_flag(
    transaction: &mut Transaction<'_, Postgres>,
    name: &str,
) -> anyhow::Result<Flag> {
    let flag = sqlx::query_as!(
        Flag,
        r#"
        SELECT id, name, value, created_at
        FROM flags
        WHERE name = $1
        LIMIT 1
        "#,
        name
    )
    .fetch_one(&mut **transaction)
    .await?;
    Ok(flag)
}

pub async fn get_flags(transaction: &mut Transaction<'_, Postgres>) -> anyhow::Result<Vec<Flag>> {
    let flags = sqlx::query_as!(
        Flag,
        r#"
        SELECT id, name, value, created_at
        FROM flags
        "#,
    )
    .fetch_all(&mut **transaction)
    .await?;
    Ok(flags)
}

pub async fn create_flag(pool: &PgPool, name: &str, value: Value) -> anyhow::Result<()> {
    let id = Uuid::new_v4();
    let now = Utc::now();
    sqlx::query!(
        r#"
        INSERT INTO flags
        (id, name, value, created_at)
        VALUES
        ($1, $2, $3, $4)
        "#,
        id,
        name,
        value,
        now
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn pre_populate_db(pool: &PgPool) -> anyhow::Result<()> {
    create_flag(
        pool,
        "enrich_ip_and_cleanup_in_background",
        Value::Bool(false),
    )
    .await?;
    create_flag(
        pool,
        "polling_interval",
        Value::Number(Number::from(300_000)),
    )
    .await?;
    create_flag(
        pool,
        "report_uptime_daily_stats_via_channel",
        Value::Bool(true),
    )
    .await?;

    create_flag(pool, "send_cleanup_to_rayon", Value::Bool(true)).await?;
    create_flag(pool, "send_to_worker", Value::Bool(true)).await?;
    create_flag(pool, "submit_bandwidth_run_background", Value::Bool(false)).await?;
    create_flag(pool, "submit_bandwidth_via_channel", Value::Bool(true)).await?;
    create_flag(pool, "touch_users_ip", Value::Bool(true)).await?;
    create_flag(pool, "tx_analytics_agg", Value::Bool(true)).await?;
    create_flag(pool, "use_websocket", Value::Bool(true)).await?;
    create_flag(
        pool,
        "use_websocket_percent",
        Value::Number(Number::from(50)),
    )
    .await?;
    Ok(())
}
