use chrono::{DateTime, Utc};
use dashmap::DashMap;
use dashmap::DashSet;
use std::hash::Hash;

pub struct DashSetWithExpiry<K> {
    set: DashSet<K>,
    expiry: DashMap<K, DateTime<Utc>>,
}

impl<K: Eq + Hash + Clone> Default for DashSetWithExpiry<K> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, K: Eq + Hash + Clone> DashSetWithExpiry<K> {
    pub fn new() -> Self {
        DashSetWithExpiry {
            set: DashSet::new(),
            expiry: DashMap::new(),
        }
    }

    pub fn insert(&self, key: K, expiry: Option<DateTime<Utc>>) -> bool {
        match expiry {
            Some(e) => {
                self.expiry.insert(key.clone(), e);
                self.set.insert(key)
            }
            None => self.set.insert(key),
        }
    }

    pub fn remove(&self, key: &K) -> Option<K> {
        self.expiry.remove(key);
        self.set.remove(key)
    }

    fn _get(&'a self, key: &K) -> Option<K> {
        match self.set.get(key) {
            Some(value) => {
                let v = value.key().clone();
                Some(v)
            }
            None => None,
        }
    }

    pub fn get(&'a self, key: &K) -> Option<K> {
        let e = self.expiry.get(key);
        let (v, expired) = match e {
            Some(expiry) => {
                let expiry_date_time = expiry.value();
                if Utc::now() > *expiry_date_time {
                    self.set.remove(key);
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
        let map: DashSetWithExpiry<u64> = DashSetWithExpiry::new();
        map.insert(10, None);
        let v = map.get(&10).unwrap();
        assert_eq!(10, v);
    }

    #[tokio::test]
    #[traced_test]
    async fn with_expiry() {
        let map: DashSetWithExpiry<u64> = DashSetWithExpiry::new();
        let date = Utc::now() + ChronoDuration::milliseconds(100);
        map.insert(10, Some(date));
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
        let map: DashSetWithExpiry<u64> = DashSetWithExpiry::new();
        map.insert(10, None);
        let v = map.get(&10).unwrap();
        assert_eq!(10, v);
        map.remove(&10);
        let v = map.get(&10);
        assert_eq!(None, v);
    }
}
