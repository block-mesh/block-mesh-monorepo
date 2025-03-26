use block_mesh_manager_database_domain::domain::user::User;
use block_mesh_manager_database_domain::domain::user::UserRole;
use database_utils::utils::option_uuid::OptionUuid;
use secret::Secret;
use sqlx::{Postgres, Transaction};

#[tracing::instrument(name = "get_user_opt_by_wallet", skip_all)]
pub async fn get_user_opt_by_wallet(
    transaction: &mut Transaction<'_, Postgres>,
    wallet: &str,
) -> anyhow::Result<Option<User>> {
    Ok(sqlx::query_as!(
        User,
        r#"SELECT
        id,
        created_at,
        password as "password: Secret<String>",
        email,
        wallet_address,
        role as "role: UserRole",
        invited_by as "invited_by: OptionUuid",
        verified_email
        FROM users WHERE wallet_address = $1 LIMIT 1"#,
        wallet
    )
    .fetch_optional(&mut **transaction)
    .await?)
}
