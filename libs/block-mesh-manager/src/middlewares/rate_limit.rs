use crate::middlewares::authentication::Backend;
use chrono::{DateTime, Duration, Utc};
use redis::aio::MultiplexedConnection;
use redis::{AsyncCommands, RedisResult};
use serde::{Deserialize, Serialize};
use std::cmp::max;
use std::env;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitUser {
    user_id: Uuid,
    ip: String,
    update_at: DateTime<Utc>,
}

impl RateLimitUser {
    pub fn new(user_id: &Uuid, ip: &str) -> Self {
        Self {
            user_id: *user_id,
            ip: ip.to_string(),
            update_at: Utc::now(),
        }
    }
}

pub fn get_key(key: &str) -> String {
    format!("rate-limit-{}", key)
}

pub fn user_ip_key(user_id: &Uuid, ip: &str) -> String {
    format!("{}-{}", user_id, ip)
}

#[tracing::instrument(name = "get_value_from_redis", skip_all)]
pub async fn get_value_from_redis(
    con: &mut MultiplexedConnection,
    key: &str,
    fallback: &RateLimitUser,
) -> anyhow::Result<RateLimitUser> {
    let redis_user: String = match con.get(get_key(key)).await {
        Ok(u) => u,
        Err(_) => return Ok(fallback.clone()),
    };
    let redis_user = match serde_json::from_str::<RateLimitUser>(&redis_user) {
        Ok(u) => u,
        Err(_) => return Ok(fallback.clone()),
    };
    Ok(redis_user)
}

#[tracing::instrument(name = "touch_redis_value", skip_all)]
pub async fn touch_redis_value(con: &mut MultiplexedConnection, user_id: &Uuid, ip: &str) {
    let redis_user = RateLimitUser::new(user_id, ip);
    if let Ok(redis_user) = serde_json::to_string(&redis_user) {
        let _: RedisResult<()> = con
            .set_ex(
                &get_key(&user_ip_key(user_id, ip)),
                redis_user.clone(),
                Backend::get_expire() as u64,
            )
            .await;
        let _: RedisResult<()> = con
            .set_ex(get_key(ip), redis_user, Backend::get_expire() as u64)
            .await;
    }
}

#[tracing::instrument(name = "filter_request", skip_all)]
pub async fn filter_request(
    con: &mut MultiplexedConnection,
    user_id: &Uuid,
    ip: &str,
) -> anyhow::Result<bool> {
    let now = Utc::now();
    let limit = env::var("FILTER_REQUEST")
        .unwrap_or("2500".to_string())
        .parse()
        .unwrap_or(2500);
    let diff = now - Duration::milliseconds(limit);
    let fallback = RateLimitUser::new(user_id, ip);
    let by_user: RateLimitUser =
        get_value_from_redis(con, &user_ip_key(user_id, ip), &fallback).await?;
    let by_ip: RateLimitUser = get_value_from_redis(con, ip, &fallback).await?;
    touch_redis_value(con, user_id, ip).await;
    Ok(max(by_user.update_at, by_ip.update_at) < diff)
}
