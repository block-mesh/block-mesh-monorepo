use crate::domain::api_token::ApiTokenStatus;
use chrono::Utc;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "Create API Token", skip(transaction), ret, err)]
pub(crate) async fn create_api_token(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
) -> anyhow::Result<Uuid> {
    let now = Utc::now();
    let id = Uuid::new_v4();
    let token = Uuid::new_v4();
    sqlx::query!(
        r#"INSERT INTO api_tokens (id, created_at, token, status) VALUES ($1, $2, $3, $4)"#,
        id,
        now,
        token,
        ApiTokenStatus::Active.to_string(),
    )
    .execute(&mut **transaction)
    .await?;
    Ok(id)
}
