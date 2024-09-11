use crate::database::invite_code::create_invite_code::create_invite_code;
use crate::database::nonce::create_nonce::create_nonce;
use crate::database::uptime_report::create_uptime_report::create_uptime_report;
use crate::domain::api_token::ApiTokenStatus;
use crate::domain::nonce::Nonce;
use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use secret::Secret;
use sqlx::{Postgres, Transaction};
use std::env;
use uuid::Uuid;

#[tracing::instrument(
    name = "Create Test User",
    skip(transaction),
    ret,
    err,
    level = "trace"
)]
pub async fn create_test_user(transaction: &mut Transaction<'_, Postgres>) -> anyhow::Result<()> {
    let app_environment = env::var("APP_ENVIRONMENT").unwrap_or("local".to_string());
    if app_environment != "local" {
        return Ok(());
    }
    let now = Utc::now();
    let id = Uuid::parse_str("5fdea056-1128-4659-a606-698acacc4cba").unwrap();
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
    sqlx::query!(
        r#"INSERT INTO api_tokens (id, created_at, token, status, user_id) VALUES ($1, $2, $3, $4, $5)"#,
        id,
        now,
        token,
        ApiTokenStatus::Active.to_string(),
        id
    )
        .execute(&mut **transaction)
        .await?;
    let nonce = Nonce::generate_nonce(16);
    let nonce_secret = Secret::from(nonce.clone());
    create_nonce(transaction, &id, &nonce_secret).await?;
    create_invite_code(transaction, id, Uuid::new_v4().to_string()).await?;
    create_uptime_report(transaction, &id, &None).await?;
    Ok(())
}
