use block_mesh_manager_database_domain::domain::api_token::ApiTokenStatus;
use chrono::Utc;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub(crate) async fn create_api_token(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
) -> anyhow::Result<Uuid> {
    let now = Utc::now();
    let id = Uuid::new_v4();
    let token = Uuid::new_v4();
    sqlx::query!(
        r#"INSERT INTO api_tokens (id, created_at, token, status, user_id) VALUES ($1, $2, $3, $4, $5)"#,
        id,
        now,
        token,
        ApiTokenStatus::Active.to_string(),
        user_id
    )
    .execute(&mut **transaction)
    .await?;
    Ok(id)
}
