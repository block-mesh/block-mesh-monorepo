use block_mesh_common::interfaces::server_api::Referral;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "get_user_referrals", skip(transaction), ret, err)]
pub(crate) async fn get_user_referrals(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
) -> anyhow::Result<Vec<Referral>> {
    let users = sqlx::query_as!(
        Referral,
        r#"SELECT
        created_at, email, verified_email
        FROM USERS
        WHERE
        invited_by = $1
        "#,
        user_id
    )
    .fetch_all(&mut **transaction)
    .await?;
    Ok(users)
}
