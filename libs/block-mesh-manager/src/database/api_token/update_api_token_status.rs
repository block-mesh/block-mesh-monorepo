use crate::domain::api_token::ApiTokenStatus;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "update_api_token_status", skip(transaction), ret, err)]
pub(crate) async fn update_api_token_status(
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
