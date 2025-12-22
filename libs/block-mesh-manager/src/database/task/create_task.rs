use block_mesh_manager_database_domain::domain::task::TaskMethod;
use block_mesh_manager_database_domain::domain::task::TaskStatus;
use sqlx::types::JsonValue;
use sqlx::{Postgres, Transaction};
use time::OffsetDateTime;
use uuid::Uuid;

pub async fn create_task(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
    url: &str,
    method: &TaskMethod,
    headers: Option<JsonValue>,
    body: Option<JsonValue>,
) -> anyhow::Result<Uuid> {
    let now = OffsetDateTime::now_utc();
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
        TaskStatus::Pending.to_string(),
        user_id
    )
    .execute(&mut **transaction)
    .await?;
    Ok(id)
}
