use moka::future::Cache;
use sqlx::{Postgres, Transaction};
use std::sync::LazyLock;
use std::time::Duration;
use time::OffsetDateTime;
use uuid::Uuid;

static TOUCH_CACHE: LazyLock<Cache<Uuid, ()>> = LazyLock::new(|| {
    Cache::builder()
        .max_capacity(50_000)
        .time_to_live(Duration::from_secs(60))
        .build()
});

pub async fn is_touch_cached(user_id: &Uuid) -> bool {
    TOUCH_CACHE.get(user_id).await.is_some()
}

#[tracing::instrument(name = "touch_user_aggregates", skip_all)]
pub async fn touch_user_aggregates(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
) -> anyhow::Result<()> {
    if TOUCH_CACHE.get(user_id).await.is_some() {
        return Ok(());
    }
    let now = OffsetDateTime::now_utc();
    sqlx::query!(
        r#"UPDATE aggregates SET updated_at = $1 WHERE user_id = $2"#,
        now,
        user_id,
    )
    .execute(&mut **transaction)
    .await?;
    TOUCH_CACHE.insert(*user_id, ()).await;
    Ok(())
}
