use chrono::Utc;
use sqlx::types::JsonValue;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "Create Task", level = "trace", skip(transaction), ret, err)]
pub(crate) async fn create_task(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
    url: &str,
    method: &str,
    headers: Option<JsonValue>,
    body: Option<JsonValue>,
) -> anyhow::Result<Uuid> {
    let now = Utc::now();
    let id = Uuid::new_v4();
    sqlx::query!(
        r#"INSERT
           INTO tasks
           (id, created_at, url, method, headers, body, status, user_id)
           VALUES
           ($1, $2, $3, $4, $5, $6, $7, $8)"#,
        id,
        now,
        url,
        method.to_string(),
        headers,
        body,
        "Pending".to_string(),
        user_id
    )
    .execute(&mut **transaction)
    .await?;
    Ok(id)
}
