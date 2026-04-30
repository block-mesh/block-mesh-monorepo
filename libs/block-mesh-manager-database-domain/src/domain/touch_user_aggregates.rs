use crate::domain::bulk_get_or_create_aggregate_by_user_and_name::record_aggregate_liveness;
use moka::future::Cache;
use sqlx::{Postgres, Transaction};
use std::env;
use std::sync::LazyLock;
use std::time::Duration;
use uuid::Uuid;

const DEFAULT_TOUCH_CACHE_TTL_SECONDS: u64 = 60;

fn touch_cache_ttl_seconds() -> u64 {
    env::var("TOUCH_CACHE_TTL_SECONDS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .filter(|ttl| *ttl > 0)
        .unwrap_or(DEFAULT_TOUCH_CACHE_TTL_SECONDS)
}

static TOUCH_CACHE: LazyLock<Cache<Uuid, ()>> = LazyLock::new(|| {
    Cache::builder()
        .max_capacity(50_000)
        .time_to_live(Duration::from_secs(touch_cache_ttl_seconds()))
        .build()
});

pub async fn is_touch_cached(user_id: &Uuid) -> bool {
    TOUCH_CACHE.get(user_id).await.is_some()
}

#[tracing::instrument(name = "touch_user_aggregates", skip_all)]
pub async fn touch_user_aggregates(
    _transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
) -> anyhow::Result<()> {
    if TOUCH_CACHE.get(user_id).await.is_some() {
        return Ok(());
    }
    record_aggregate_liveness(user_id).await;
    TOUCH_CACHE.insert(*user_id, ()).await;
    Ok(())
}
