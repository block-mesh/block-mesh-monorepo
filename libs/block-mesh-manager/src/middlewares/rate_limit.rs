use crate::utils::cache_envar::get_envar;
use redis::aio::MultiplexedConnection;
use redis::{AsyncCommands, RedisResult};
use uuid::Uuid;

pub fn next_allowed_key(suffix: &str) -> String {
    format!("next-allowed-{}", suffix)
}

#[tracing::instrument(name = "get_next_allowed_request", skip_all)]
pub async fn get_next_allowed_request(
    con: &mut MultiplexedConnection,
    user_id: &Uuid,
    ip: &str,
) -> anyhow::Result<bool> {
    let r: RedisResult<String> = con.get(next_allowed_key(&user_id.to_string())).await;
    match r {
        Ok(_) => return Ok(true),
        Err(e) => {
            tracing::error!("create_next_allowed_request e => {e}");
        }
    };
    let r: RedisResult<String> = con.get(next_allowed_key(ip)).await;
    match r {
        Ok(_) => return Ok(true),
        Err(e) => {
            tracing::error!("create_next_allowed_request e => {e}");
        }
    };
    Ok(false)
}

#[tracing::instrument(name = "create_next_allowed_request", skip_all)]
pub async fn create_next_allowed_request(
    con: &mut MultiplexedConnection,
    user_id: &Uuid,
    ip: &str,
    expiry: u64,
) {
    let _: RedisResult<()> = con
        .set_ex(next_allowed_key(&user_id.to_string()), true, expiry)
        .await;
    let _: RedisResult<()> = con.set_ex(next_allowed_key(ip), true, expiry).await;
}

#[tracing::instrument(name = "filter_request", skip_all)]
pub async fn filter_request(
    con: &mut MultiplexedConnection,
    user_id: &Uuid,
    ip: &str,
) -> anyhow::Result<bool> {
    let expiry = get_envar("FILTER_REQUEST_EXPIRY_SECONDS")
        .await
        .parse()
        .unwrap_or(3u64);
    let exists = get_next_allowed_request(con, user_id, ip).await?;
    if exists {
        Ok(false)
    } else {
        create_next_allowed_request(con, user_id, ip, expiry).await;
        Ok(true)
    }
}
