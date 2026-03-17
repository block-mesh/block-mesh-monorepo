use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "get_extension_activated", skip_all)]
pub async fn get_extension_activated(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
) -> Result<bool, sqlx::Error> {
    sqlx::query_scalar!(
        r#"SELECT extension_activated FROM users WHERE id = $1"#,
        user_id
    )
    .fetch_one(&mut **transaction)
    .await
}
