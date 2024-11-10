use crate::database::invite_code::create_invite_code::create_invite_code;
use crate::database::nonce::create_nonce::create_nonce;
use crate::database::uptime_report::create_uptime_report::create_uptime_report;
use bcrypt::{hash, DEFAULT_COST};
use block_mesh_common::constants::{BLOCKMESH_SERVER_UUID_ENVAR, BLOCK_MESH_SUPPORT_EMAIL};
use block_mesh_manager_database_domain::domain::api_token::ApiTokenStatus;
use block_mesh_manager_database_domain::domain::nonce::Nonce;
use block_mesh_manager_database_domain::domain::prep_user::prep_user;
use chrono::Utc;
use secret::Secret;
use sqlx::PgPool;
use std::env;
use uuid::Uuid;

#[tracing::instrument(name = "create_test_user", skip_all, ret, err)]
pub async fn create_test_user(pool: &PgPool) -> anyhow::Result<()> {
    let app_environment = env::var("APP_ENVIRONMENT").unwrap_or("local".to_string());
    if app_environment != "local" {
        return Ok(());
    }
    let now = Utc::now();
    let user_id = Uuid::parse_str("5fdea056-1128-4659-a606-698acacc4cba").unwrap();
    let token = Uuid::parse_str("5fdea056-1128-4659-a606-698acacc4cba").unwrap();
    let email = "123@blockmesh.xyz";
    let password = hash("123", DEFAULT_COST)?;
    sqlx::query!(
        r#"
        WITH
            extant AS (
                SELECT id FROM users WHERE id = $1
            ),
            inserted AS (
                INSERT INTO users (id, created_at, wallet_address, email, password, invited_by, verified_email, role)
                SELECT $1, $2, $3, $4, $5, null , true, 'admin'
                WHERE NOT EXISTS (SELECT FROM extant)
                RETURNING id
            )
        SELECT id FROM inserted
        UNION ALL
        SELECT id FROM extant
        "#,
        user_id,
        now,
        None::<String>,
        email,
        password
    )
        .fetch_one(pool)
        .await?;
    sqlx::query!(
        r#"INSERT INTO api_tokens (id, created_at, token, status, user_id) VALUES ($1, $2, $3, $4, $5)"#,
        user_id,
        now,
        token,
        ApiTokenStatus::Active.to_string(),
        user_id
    )
        .execute(pool)
        .await?;
    let nonce = Nonce::generate_nonce(16);
    let nonce_secret = Secret::from(nonce.clone());
    let mut transaction = pool.begin().await?;
    create_nonce(&mut transaction, &user_id, &nonce_secret).await?;
    create_invite_code(&mut transaction, user_id, Uuid::new_v4().to_string()).await?;
    create_uptime_report(&mut transaction, &user_id, &None).await?;
    prep_user(&mut transaction, &user_id).await?;
    transaction.commit().await?;

    let now = Utc::now();
    let server_user_id =
        Uuid::parse_str(env::var(BLOCKMESH_SERVER_UUID_ENVAR).unwrap().as_str()).unwrap();
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
        server_user_id,
        now,
        None::<String>,
        email,
        password
    )
        .fetch_one(pool)
        .await?;
    Ok(())
}
