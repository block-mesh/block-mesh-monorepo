use crate::domain::option_uuid::OptionUuid;
use crate::domain::user::User;
use crate::domain::user::UserRole;
use bcrypt::{hash, DEFAULT_COST};
use block_mesh_common::constants::{BLOCKMESH_SERVER_UUID_ENVAR, BLOCK_MESH_SUPPORT_EMAIL};
use chrono::Utc;
use secret::Secret;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "Create Server User", skip(transaction), ret, err)]
pub(crate) async fn create_server_user(
    transaction: &mut Transaction<'_, Postgres>,
) -> anyhow::Result<User> {
    let now = Utc::now();
    let id = Uuid::parse_str(std::env::var(BLOCKMESH_SERVER_UUID_ENVAR).unwrap().as_str()).unwrap();
    let email = BLOCK_MESH_SUPPORT_EMAIL;
    let random = Uuid::new_v4().to_string();
    let password = hash(random, DEFAULT_COST)?;
    let user = sqlx::query_as!(
        User,
        r#"INSERT INTO users (id, created_at, wallet_address, email, password)
           VALUES ($1, $2, $3, $4, $5)
           ON CONFLICT (id)
           DO UPDATE set created_at = $2
           RETURNING
            id,
            created_at,
            password as "password: Secret<String>",
            email,
            wallet_address,
            role as "role: UserRole",
            invited_by as "invited_by: OptionUuid",
            verified_email
        "#,
        id,
        now,
        None::<String>,
        email,
        password
    )
    .fetch_one(&mut **transaction)
    .await?;
    Ok(user)
}
