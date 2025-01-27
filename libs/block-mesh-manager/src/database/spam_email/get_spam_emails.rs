use crate::domain::spam_email::SpamEmail;
use chrono::{DateTime, Utc};
use dash_with_expiry::hash_set_with_expiry::HashSetWithExpiry;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::{OnceCell, RwLock};

pub static RATE_LIMIT_EMAIL: OnceCell<Arc<RwLock<HashSetWithExpiry<String>>>> =
    OnceCell::const_new();

pub static SPAM_EMAIL_CACHE: OnceCell<Arc<RwLock<Vec<SpamEmail>>>> = OnceCell::const_new();

pub async fn get_from_email_rate_limit(key: &String) -> Option<String> {
    let cache = RATE_LIMIT_EMAIL
        .get_or_init(|| async { Arc::new(RwLock::new(HashSetWithExpiry::new())) })
        .await;
    cache.read().await.get(key).await
}

pub async fn update_email_rate_limit(email_or_ip: &str, date: Option<DateTime<Utc>>) {
    let cache = RATE_LIMIT_EMAIL
        .get_or_init(|| async { Arc::new(RwLock::new(HashSetWithExpiry::new())) })
        .await;
    cache
        .write()
        .await
        .insert(email_or_ip.to_string(), date)
        .await;
}

pub async fn get_spam_emails_cache() -> Vec<SpamEmail> {
    let cache = SPAM_EMAIL_CACHE
        .get_or_init(|| async { Arc::new(RwLock::new(Vec::new())) })
        .await;
    cache.read().await.clone()
}

pub async fn init_spam_emails_cache(pool: &PgPool) -> anyhow::Result<()> {
    let cache = SPAM_EMAIL_CACHE
        .get_or_init(|| async { Arc::new(RwLock::new(Vec::new())) })
        .await;
    let mut transaction = create_txn(pool).await?;
    let spam_emails = sqlx::query_as!(
        SpamEmail,
        r#"
        SELECT
        id,
        domain,
        created_at,
        updated_at
        FROM spam_emails
        "#,
    )
    .fetch_all(&mut *transaction)
    .await?;
    *cache.write().await = spam_emails.clone();
    commit_txn(transaction).await?;
    Ok(())
}
