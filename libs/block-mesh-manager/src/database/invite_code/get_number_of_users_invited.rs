use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "get_number_of_users_invited", skip(transaction), ret, err)]
pub(crate) async fn get_number_of_users_invited(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
) -> anyhow::Result<i64> {
    let count: Option<i64> = sqlx::query_scalar!(
        r#"SELECT
        COALESCE(COUNT(*), 0) AS verified_invited_users
        FROM USERS
        WHERE
        invited_by = $1
        AND
        verified_email = true
        "#,
        user_id
    )
    .fetch_one(&mut **transaction)
    .await?;
    Ok(count.unwrap_or_default())
}
