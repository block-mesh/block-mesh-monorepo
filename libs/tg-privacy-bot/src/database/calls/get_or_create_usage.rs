use crate::database::models::usage::Usage;
use sqlx::{Postgres, Transaction};
use std::env;
use time::OffsetDateTime;
use uuid::Uuid;

pub async fn get_or_create_usage(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
) -> anyhow::Result<Usage> {
    let usage_limit = env::var("USAGE_LIMIT")
        .unwrap_or("10".to_string())
        .parse::<i64>()?;
    let id = Uuid::new_v4();
    let now = OffsetDateTime::now_utc();
    let day = now.date();
    let usage = sqlx::query_as!(
        Usage,
        r#"
        INSERT INTO usages
        (id, user_id, usage_limit, usage, created_at, updated_at, day)
        VALUES
        ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (user_id, day) DO UPDATE SET updated_at = $6
        RETURNING id, user_id, usage_limit, usage, created_at, updated_at, day
        "#,
        id,
        user_id,
        usage_limit,
        0i64,
        now,
        now.clone(),
        day
    )
    .fetch_one(&mut **transaction)
    .await?;
    Ok(usage)
}
