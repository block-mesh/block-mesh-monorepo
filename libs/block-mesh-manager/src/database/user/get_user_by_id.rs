use block_mesh_manager_database_domain::domain::option_uuid::OptionUuid;
use block_mesh_manager_database_domain::domain::user::User;
use block_mesh_manager_database_domain::domain::user::UserRole;
use secret::Secret;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

pub async fn get_user_opt_by_id(
    transaction: &mut Transaction<'_, Postgres>,
    id: &Uuid,
) -> anyhow::Result<Option<User>> {
    Ok(sqlx::query_as!(
        User,
        r#"SELECT
        id,
        email,
        created_at,
        password as "password: Secret<String>",
        wallet_address,
        role as "role: UserRole",
        invited_by as "invited_by: OptionUuid",
        verified_email
        FROM users WHERE id = $1 LIMIT 1"#,
        id
    )
    .fetch_optional(&mut **transaction)
    .await?)
}

pub async fn get_user_opt_by_id_pool(pool: &PgPool, id: &Uuid) -> anyhow::Result<Option<User>> {
    Ok(sqlx::query_as!(
        User,
        r#"SELECT
        id,
        email,
        created_at,
        password as "password: Secret<String>",
        wallet_address,
        role as "role: UserRole",
        invited_by as "invited_by: OptionUuid",
        verified_email
        FROM users WHERE id = $1 LIMIT 1"#,
        id
    )
    .fetch_optional(pool)
    .await?)
}
