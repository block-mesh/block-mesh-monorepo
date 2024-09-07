use bcrypt::{hash, DEFAULT_COST};
use block_mesh_common::constants::{BLOCKMESH_SERVER_UUID_ENVAR, BLOCK_MESH_SUPPORT_EMAIL};
use chrono::Utc;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(
    name = "Create Server User",
    skip(transaction),
    ret,
    err,
    level = "trace"
)]
pub(crate) async fn create_server_user(
    transaction: &mut Transaction<'_, Postgres>,
) -> anyhow::Result<Uuid> {
    let now = Utc::now();
    let id = Uuid::parse_str(std::env::var(BLOCKMESH_SERVER_UUID_ENVAR).unwrap().as_str()).unwrap();
    let email = BLOCK_MESH_SUPPORT_EMAIL;
    let random = Uuid::new_v4().to_string();
    let password = hash(random, DEFAULT_COST)?;
    sqlx::query!(
        r#"
        WITH
            extant AS (
                SELECT id FROM users WHERE id = $1
            ),
            inserted AS (
                INSERT INTO users (id, created_at, wallet_address, email, password, invited_by, verified_email, role)
                SELECT $1, $2, $3, $4, $5, null , true, 'User'
                WHERE NOT EXISTS (SELECT FROM extant)
                RETURNING id
            )
        SELECT id FROM inserted
        UNION ALL
        SELECT id FROM extant
        "#,
        id,
        now,
        None::<String>,
        email,
        password
    )
        .fetch_one(&mut **transaction)
        .await?;
    Ok(id)
}
