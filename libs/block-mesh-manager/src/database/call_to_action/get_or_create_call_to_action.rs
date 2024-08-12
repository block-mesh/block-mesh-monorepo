use chrono::Utc;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::domain::call_to_action::CallToActionName;

#[tracing::instrument(
    name = "get_or_create_call_to_action",
    skip(transaction),
    level = "trace",
    ret,
    err
)]
pub(crate) async fn get_or_create_call_to_action(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
    name: CallToActionName,
    status: bool,
) -> anyhow::Result<()> {
    let now = Utc::now();
    let id = Uuid::new_v4();
    sqlx::query!(
        r#"
        WITH
            extant AS (
                SELECT id, user_id, name, created_at, status FROM call_to_actions WHERE user_id = $3 AND name = $4
            ),
            inserted AS (
                INSERT INTO call_to_actions (id , created_at, user_id, name, status)
                SELECT $1, $2, $3, $4,  $5
                WHERE NOT EXISTS (SELECT FROM extant)
                RETURNING id, user_id, name, created_at, status
            )
        SELECT id, user_id, name, created_at, status FROM inserted
        UNION ALL
        SELECT id, user_id, name, created_at, status FROM extant
        "#,
        id,
        now,
        user_id,
        name.to_string(),
        status
    )
        .fetch_one(&mut **transaction)
        .await?;
    Ok(())
}
