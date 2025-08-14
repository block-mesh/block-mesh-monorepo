use bcrypt::verify;
use dash_with_expiry::hash_map_with_expiry::HashMapWithExpiry;
use std::sync::Arc;
use tokio::sync::OnceCell;

type VerifyMap = Arc<HashMapWithExpiry<(String, String), bool>>;

static CACHE: OnceCell<VerifyMap> = OnceCell::const_new();

#[tracing::instrument(name = "get_cache", skip_all)]
pub async fn get_cache<'a>() -> &'a VerifyMap {
    CACHE
        .get_or_init(|| async { Arc::new(HashMapWithExpiry::new(1_000)) })
        .await
}

#[tracing::instrument(name = "verify_with_cache", skip_all)]
pub async fn verify_with_cache(password: &str, hash: &str) -> bool {
    let key = (password.to_string(), hash.to_string());
    let cache = get_cache().await;
    if let Some(entry) = cache.get(&key).await {
        return entry;
    }
    if let Ok(result) = verify::<&str>(password, hash) {
        cache.insert(key, result, None).await;
        return result;
    }
    false
}
