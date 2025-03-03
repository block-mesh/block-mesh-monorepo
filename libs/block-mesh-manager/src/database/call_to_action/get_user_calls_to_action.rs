use crate::domain::call_to_action::CallToAction;
use chrono::{Duration, Utc};
use dash_with_expiry::hash_map_with_expiry::HashMapWithExpiry;
use sqlx::{query_as, Postgres, Transaction};
use std::sync::Arc;
use tokio::sync::{OnceCell, RwLock};
use uuid::Uuid;

#[allow(dead_code)]
struct Id {
    id: Uuid,
}

static CACHE: OnceCell<Arc<RwLock<HashMapWithExpiry<Uuid, Vec<CallToAction>>>>> =
    OnceCell::const_new();

pub async fn get_user_call_to_action(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
) -> anyhow::Result<Vec<CallToAction>> {
    let cache = CACHE
        .get_or_init(|| async { Arc::new(RwLock::new(HashMapWithExpiry::new())) })
        .await;
    if let Some(out) = cache.read().await.get(user_id).await {
        return Ok(out);
    }
    let calls_to_action = query_as!(
        CallToAction,
        r#"
        SELECT
        id, user_id, name, created_at, status
        FROM call_to_actions
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_all(&mut **transaction)
    .await?;
    let date = Utc::now() + Duration::milliseconds(600_000);
    cache
        .write()
        .await
        .insert(user_id.clone(), calls_to_action.clone(), Some(date))
        .await;
    Ok(calls_to_action)
}
