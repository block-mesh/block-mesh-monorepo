use crate::database::users_ip::update_users_ip_bulk::update_users_ip_bulk;
use chrono::Utc;
use flume::Receiver;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashMap;
use std::env;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UsersIpMessage {
    pub id: Uuid,
    pub ip: String,
}

pub async fn users_ip_agg(pool: PgPool, rx: Receiver<UsersIpMessage>) -> Result<(), anyhow::Error> {
    let agg_size = env::var("USERS_IP_AGG_SIZE")
        .unwrap_or("300".to_string())
        .parse()
        .unwrap_or(300);
    let mut calls: HashMap<(Uuid, String), String> = HashMap::new();
    let mut count = 0;
    let mut prev = Utc::now();
    while let Ok(message) = rx.recv_async().await {
        calls.insert((message.id, message.ip.clone()), message.ip);
        count += 1;
        let now = Utc::now();
        let diff = now - prev;
        let run = diff.num_seconds() > 5 || count >= agg_size;
        prev = Utc::now();
        if run {
            let _ = users_ips_submit_to_db(&pool, &mut calls).await;
            count = 0;
            calls.clear();
        }
    }
    Ok(())
}

#[tracing::instrument(name = "users_ips_submit_to_db", skip(pool, calls), ret, err)]
pub async fn users_ips_submit_to_db(
    pool: &PgPool,
    calls: &mut HashMap<(Uuid, String), String>,
) -> anyhow::Result<()> {
    let mut transaction = pool.begin().await?;
    update_users_ip_bulk(&mut transaction, calls).await?;
    transaction.commit().await?;
    Ok(())
}
