use bcrypt::verify;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{OnceCell, RwLock};

type VerifyMap = Arc<RwLock<HashMap<(String, String), bool>>>;

static CACHE: OnceCell<VerifyMap> = OnceCell::const_new();

#[tracing::instrument(name = "get_cache", skip_all)]
pub async fn get_cache<'a>() -> &'a VerifyMap {
    CACHE
        .get_or_init(|| async { Arc::new(RwLock::new(HashMap::new())) })
        .await
}

#[tracing::instrument(name = "verify_with_cache", skip_all)]
pub async fn verify_with_cache(password: &str, hash: &str) -> bool {
    let key = (password.to_string(), hash.to_string());
    let cache = get_cache().await;
    if let Some(entry) = cache.read().await.get(&key) {
        return *entry;
    }
    if let Ok(result) = verify::<&str>(password, hash) {
        cache.write().await.insert(key, result);
        return result;
    }
    false
}
