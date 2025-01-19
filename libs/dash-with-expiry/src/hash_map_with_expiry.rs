use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct HashMapWithExpiry<K, V> {
    map: Arc<RwLock<HashMap<K, V>>>,
    expiry: Arc<RwLock<HashMap<K, DateTime<Utc>>>>,
}

impl<K: Eq + Hash + Clone, V: Clone> Clone for HashMapWithExpiry<K, V> {
    fn clone(&self) -> Self {
        Self {
            map: self.map.clone(),
            expiry: self.expiry.clone(),
        }
    }
}

impl<K: Eq + Hash + Clone, V: Clone> Default for HashMapWithExpiry<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, K: Eq + Hash + Clone, V: Clone> HashMapWithExpiry<K, V> {
    pub fn new() -> Self {
        HashMapWithExpiry {
            map: Arc::new(RwLock::new(HashMap::new())),
            expiry: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn insert(&self, key: K, value: V, expiry: Option<DateTime<Utc>>) -> Option<V> {
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
                if Utc::now() > expiry {
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
    use chrono::Duration as ChronoDuration;
    use std::time::Duration;
    use tokio::time::sleep;
    use tracing_test::traced_test;

    #[tokio::test]
    #[traced_test]
    async fn no_expiry() {
        let map: HashMapWithExpiry<u64, u64> = HashMapWithExpiry::new();
        map.insert(10, 10, None).await;
        let v = map.get(&10).await.unwrap();
        assert_eq!(10, v);
    }

    #[tokio::test]
    #[traced_test]
    async fn with_expiry() {
        let map: HashMapWithExpiry<u64, u64> = HashMapWithExpiry::new();
        let date = Utc::now() + ChronoDuration::milliseconds(1000);
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
        let map: HashMapWithExpiry<u64, u64> = HashMapWithExpiry::new();
        map.insert(10, 10, None).await;
        let v = map.get(&10).await.unwrap();
        assert_eq!(10, v);
        map.remove(&10).await;
        let v = map.get(&10).await;
        assert_eq!(None, v);
    }
}
