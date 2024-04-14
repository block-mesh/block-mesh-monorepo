use chrono::Utc;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "Create User", skip(transaction), ret, err)]
pub(crate) async fn create_user(
    transaction: &mut Transaction<'_, Postgres>,
    wallet_address: Option<String>,
    email: &str,
    password: &str,
) -> anyhow::Result<Uuid> {
    let now = Utc::now();
    let id = Uuid::new_v4();
    sqlx::query!(
        r#"INSERT INTO users (id, created_at, wallet_address, email, password) VALUES ($1, $2, $3, $4, $5)"#,
        id,
        now,
        wallet_address,
        email,
        password
    )
        .execute(&mut **transaction)
        .await?;
    Ok(id)
}
