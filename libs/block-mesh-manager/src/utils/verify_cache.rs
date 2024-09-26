use bcrypt::verify;
use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::OnceCell;

type VerifyMap = Arc<DashMap<(String, String), bool>>;

static CACHE: OnceCell<VerifyMap> = OnceCell::const_new();

#[tracing::instrument(name = "get_cache", skip_all)]
pub async fn get_cache<'a>() -> &'a VerifyMap {
    CACHE
        .get_or_init(|| async { Arc::new(DashMap::new()) })
        .await
}

#[tracing::instrument(name = "verify_with_cache", skip_all)]
pub async fn verify_with_cache(password: &str, hash: &str) -> bool {
    let key = (password.to_string(), hash.to_string());
    let cache = get_cache().await;
    if let Some(entry) = cache.get(&key) {
        return entry.value().clone();
    }
    if let Ok(result) = verify::<&str>(password, hash) {
        cache.insert(key, result);
        return result;
    }
    false
}
