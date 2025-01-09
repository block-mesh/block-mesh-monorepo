use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{OnceCell, RwLock};

type EnvVarMap = Arc<RwLock<HashMap<String, String>>>;

static CACHE: OnceCell<EnvVarMap> = OnceCell::const_new();

#[tracing::instrument(name = "get_cache", skip_all)]
pub async fn get_cache<'a>() -> &'a EnvVarMap {
    CACHE
        .get_or_init(|| async { Arc::new(RwLock::new(HashMap::new())) })
        .await
}

#[tracing::instrument(name = "get_envar", skip_all)]
pub async fn get_envar(name: &str) -> String {
    let cache = get_cache().await;
    if let Some(entry) = cache.read().await.get(name) {
        return entry.clone();
    }
    let value = std::env::var(name).unwrap_or_default();
    cache.write().await.insert(name.to_string(), value.clone());
    value
}
