use crate::state::WsAppState;
use redis::{AsyncCommands, RedisResult};
use std::sync::Arc;
use std::time::Duration;

#[tracing::instrument(name = "redis_loop", skip_all)]
pub async fn redis_loop(state: Arc<WsAppState>) -> Result<(), anyhow::Error> {
    loop {
        let mut redis = state.redis.clone();
        let _: RedisResult<()> = redis.expire(&state.redis_key(), 120).await;
        tokio::time::sleep(Duration::from_secs(100)).await;
    }
}
