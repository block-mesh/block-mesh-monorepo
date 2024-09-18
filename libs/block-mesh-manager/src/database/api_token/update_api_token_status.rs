use block_mesh_manager_database_domain::domain::api_token::ApiTokenStatus;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub async fn update_api_token_status(
    transaction: &mut Transaction<'_, Postgres>,
    id: Uuid,
    status: ApiTokenStatus,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE api_tokens SET status = $1 WHERE id = $2"#,
        status.to_string(),
        id
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
