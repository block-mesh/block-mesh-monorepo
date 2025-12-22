use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;
use std::time::Duration;
use time::OffsetDateTime;
use tokio::sync::RwLock;

pub struct HashMapWithExpiry<K, V> {
    map: Arc<RwLock<HashMap<K, V>>>,
    expiry: Arc<RwLock<HashMap<K, OffsetDateTime>>>,
    size_limit: usize,
}

impl<K: Eq + Hash + Clone + Sync + Send + 'static, V: Clone + Sync + Send + 'static> Clone
    for HashMapWithExpiry<K, V>
{
    fn clone(&self) -> Self {
        Self {
            size_limit: self.size_limit,
            map: self.map.clone(),
            expiry: self.expiry.clone(),
        }
    }
}

impl<K: Eq + Hash + Clone + Sync + Send + 'static, V: Clone + Sync + Send + 'static> Default
    for HashMapWithExpiry<K, V>
{
    fn default() -> Self {
        Self::new(1_000)
    }
}

impl<'a, K: Eq + Hash + Clone + Sync + Send + 'static, V: Clone + Sync + Send + 'static>
    HashMapWithExpiry<K, V>
{
    pub async fn is_empty(&self) -> bool {
        self.map.read().await.len() == 0
    }

    pub async fn len(&self) -> usize {
        self.map.read().await.len()
    }

    pub async fn clear(&self) {
        self.map.write().await.clear();
        self.expiry.write().await.clear();
    }

    pub async fn keys(&self) -> Vec<K> {
        let mut keys: Vec<K> = Vec::new();
        for key in self.map.read().await.keys() {
            keys.push(key.clone());
        }
        keys
    }

    pub async fn period_cleanup(
        size_limit: usize,
        map: Arc<RwLock<HashMap<K, V>>>,
        expiry: Arc<RwLock<HashMap<K, OffsetDateTime>>>,
    ) {
        loop {
            let clear = map.read().await.len() >= size_limit;
            if clear {
                map.write().await.clear();
                expiry.write().await.clear();
            }
            tokio::time::sleep(Duration::from_millis(10_000)).await;
        }
    }

    pub fn new(size_limit: usize) -> Self {
        let inst = HashMapWithExpiry {
            size_limit,
            map: Arc::new(RwLock::new(HashMap::new())),
            expiry: Arc::new(RwLock::new(HashMap::new())),
        };
        tokio::spawn(Self::period_cleanup(
            inst.size_limit,
            inst.map.clone(),
            inst.expiry.clone(),
        ));
        inst
    }

    pub async fn insert(&self, key: K, value: V, expiry: Option<OffsetDateTime>) -> Option<V> {
        match expiry {
            Some(e) => {
                self.expiry.write().await.insert(key.clone(), e);
                self.map.write().await.insert(key, value)
            }
            None => self.map.write().await.insert(key, value),
        }
    }

    pub async fn remove(&self, key: &K) -> Option<V> {
        self.expiry.write().await.remove(key);
        self.map.write().await.remove(key)
    }

    async fn _get(&'a self, key: &K) -> Option<V> {
        match self.map.read().await.get(key) {
            Some(value) => {
                let v = value.clone();
                Some(v)
            }
            None => None,
        }
    }

    pub async fn get(&'a self, key: &K) -> Option<V> {
        let e = self.expiry.read().await.get(key).cloned();
        let (v, expired) = match e {
            Some(expiry) => {
                if OffsetDateTime::now_utc() > expiry {
                    self.map.write().await.remove(key);
                    (None, true)
                } else {
                    (self._get(key).await, false)
                }
            }
            None => (self._get(key).await, false),
        };
        if expired {
            self.expiry.write().await.remove(key);
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use time::Duration as TimeDuration;
    use tokio::time::sleep;
    use tracing_test::traced_test;

    #[tokio::test]
    #[traced_test]
    async fn no_expiry() {
        let map: HashMapWithExpiry<u64, u64> = HashMapWithExpiry::new(1_000);
        map.insert(10, 10, None).await;
        let v = map.get(&10).await.unwrap();
        assert_eq!(10, v);
    }

    #[tokio::test]
    #[traced_test]
    async fn with_expiry() {
        let map: HashMapWithExpiry<u64, u64> = HashMapWithExpiry::new(1_000);
        let date = OffsetDateTime::now_utc() + TimeDuration::milliseconds(1000);
        map.insert(10, 10, Some(date)).await;
        let v = map.get(&10).await.unwrap();
        assert_eq!(10, v);
        let two_sec = Duration::new(2, 0);
        let _ = sleep(two_sec).await;
        let v = map.get(&10).await;
        assert_eq!(None, v);
    }

    #[tokio::test]
    #[traced_test]
    async fn remove() {
        let map: HashMapWithExpiry<u64, u64> = HashMapWithExpiry::new(1_000);
        map.insert(10, 10, None).await;
        let v = map.get(&10).await.unwrap();
        assert_eq!(10, v);
        map.remove(&10).await;
        let v = map.get(&10).await;
        assert_eq!(None, v);
    }
}
