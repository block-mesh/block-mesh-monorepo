use chrono::{DateTime, Utc};
use dashmap::try_result::TryResult::Present;
use dashmap::DashMap;
use std::hash::Hash;

pub struct DashMapWithExpiry<K, V> {
    map: DashMap<K, V>,
    expiry: DashMap<K, DateTime<Utc>>,
}

impl<K: Eq + Hash + Clone, V: Clone> Default for DashMapWithExpiry<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, K: Eq + Hash + Clone, V: Clone> DashMapWithExpiry<K, V> {
    pub fn new() -> Self {
        DashMapWithExpiry {
            map: DashMap::new(),
            expiry: DashMap::new(),
        }
    }

    pub fn insert(&self, key: K, value: V, expiry: Option<DateTime<Utc>>) -> Option<V> {
        match expiry {
            Some(e) => {
                self.expiry.insert(key.clone(), e);
                self.map.insert(key, value)
            }
            None => self.map.insert(key, value),
        }
    }

    pub fn remove(&self, key: &K) -> Option<(K, V)> {
        self.expiry.remove(key);
        self.map.remove(key)
    }

    fn _get(&'a self, key: &K) -> Option<V> {
        match self.map.get(key) {
            Some(value) => {
                let v = value.value().clone();
                Some(v)
            }
            None => None,
        }
    }

    fn _try_get(&'a self, key: &K) -> Option<V> {
        match self.map.try_get(key) {
            Present(value) => {
                let v = value.value().clone();
                Some(v)
            }
            _ => None,
        }
    }

    pub fn try_get(&self, key: &K) -> Option<V> {
        let e = self.expiry.get(key);
        let (v, expired) = match e {
            Some(expiry) => {
                let expiry_date_time = expiry.value();
                if Utc::now() > *expiry_date_time {
                    self.map.remove(key);
                    (None, true)
                } else {
                    (self._try_get(key), false)
                }
            }
            None => (self._try_get(key), false),
        };
        if expired {
            self.expiry.remove(key);
        }
        v
    }

    pub fn get(&'a self, key: &K) -> Option<V> {
        let e = self.expiry.get(key);
        let (v, expired) = match e {
            Some(expiry) => {
                let expiry_date_time = expiry.value();
                if Utc::now() > *expiry_date_time {
                    self.map.remove(key);
                    (None, true)
                } else {
                    (self._get(key), false)
                }
            }
            None => (self._get(key), false),
        };
        if expired {
            self.expiry.remove(key);
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
        let map: DashMapWithExpiry<u64, u64> = DashMapWithExpiry::new();
        map.insert(10, 10, None);
        let v = map.get(&10).unwrap();
        assert_eq!(10, v);
    }

    #[tokio::test]
    #[traced_test]
    async fn with_expiry() {
        let map: DashMapWithExpiry<u64, u64> = DashMapWithExpiry::new();
        let date = Utc::now() + ChronoDuration::milliseconds(1000);
        map.insert(10, 10, Some(date));
        let v = map.get(&10).unwrap();
        assert_eq!(10, v);
        let two_sec = Duration::new(2, 0);
        let _ = sleep(two_sec).await;
        let v = map.get(&10);
        assert_eq!(None, v);
    }

    #[tokio::test]
    #[traced_test]
    async fn remove() {
        let map: DashMapWithExpiry<u64, u64> = DashMapWithExpiry::new();
        map.insert(10, 10, None);
        let v = map.get(&10).unwrap();
        assert_eq!(10, v);
        map.remove(&10);
        let v = map.get(&10);
        assert_eq!(None, v);
    }

    #[tokio::test]
    #[traced_test]
    async fn try_get_base() {
        let map: DashMap<String, String> = DashMap::new();
        map.insert("10".to_string(), "10".to_string());
        let v = { map.try_get("10").unwrap().value().clone() };
        assert_eq!("10".to_string(), v);
        map.insert("11".to_string(), "11".to_string());
        let v = { map.try_get("11").unwrap().value().clone() };
        assert_eq!("11".to_string(), v);
        map.remove("10");
    }

    #[tokio::test]
    #[traced_test]
    async fn try_no_expiry() {
        let map: DashMapWithExpiry<u64, u64> = DashMapWithExpiry::new();
        map.insert(10, 10, None);
        let v = map.try_get(&10).unwrap();
        assert_eq!(10, v);
    }

    #[tokio::test]
    #[traced_test]
    async fn try_with_expiry() {
        let map: DashMapWithExpiry<u64, u64> = DashMapWithExpiry::new();
        let date = Utc::now() + ChronoDuration::milliseconds(1000);
        map.insert(10, 10, Some(date));
        let v = map.try_get(&10).unwrap();
        assert_eq!(10, v);
        let two_sec = Duration::new(2, 0);
        let _ = sleep(two_sec).await;
        let v = map.try_get(&10);
        assert_eq!(None, v);
    }

    #[tokio::test]
    #[traced_test]
    async fn try_remove() {
        let map: DashMapWithExpiry<u64, u64> = DashMapWithExpiry::new();
        map.insert(10, 10, None);
        let v = map.try_get(&10).unwrap();
        assert_eq!(10, v);
        map.remove(&10);
        let v = map.try_get(&10);
        assert_eq!(None, v);
    }
}
