use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::OnceCell;

type EnvVarMap = Arc<DashMap<String, String>>;

static CACHE: OnceCell<EnvVarMap> = OnceCell::const_new();

#[tracing::instrument(name = "get_cache", skip_all)]
pub async fn get_cache<'a>() -> &'a EnvVarMap {
    CACHE
        .get_or_init(|| async { Arc::new(DashMap::new()) })
        .await
}

pub async fn get_envar(name: &str) -> String {
    let cache = get_cache().await;
    if let Some(entry) = cache.get(name) {
        return entry.value().clone();
    }
    let value = std::env::var(name).unwrap_or_default();
    cache.insert(name.to_string(), value.clone());
    value
}
