use chrono::{DateTime, Utc};
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

impl<K: Eq + Hash + Clone, V: Clone> DashMapWithExpiry<K, V> {
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
        self.map.remove(key);
        self.map.remove(key)
    }

    fn _get(self, key: &K) -> Option<V> {
        match self.map.get(key) {
            Some(value) => {
                let v = value.value().clone();
                Some(v)
            }
            None => None,
        }
    }

    pub fn get(self, key: &K) -> Option<V> {
        let e = self.expiry.get(key);
        match e {
            Some(expiry) => {
                let expiry_date_time = expiry.value();
                if Utc::now() > *expiry_date_time {
                    self.remove(key);
                    None
                } else {
                    self._get(key)
                }
            }
            None => self._get(key),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration as ChronoDuration;
    use std::thread;
    use std::time::Duration;

    // use time::Duration as TimeDuration;

    #[test]
    fn no_expiry() {
        let map: DashMapWithExpiry<u64, u64> = DashMapWithExpiry::new();
        map.insert(10, 10, None);
        let v = map.get(&10).unwrap();
        assert_eq!(10, v);
    }

    #[test]
    fn with_expiry() {
        let map: DashMapWithExpiry<u64, u64> = DashMapWithExpiry::new();
        let date = Utc::now() + ChronoDuration::milliseconds(1000);
        map.insert(10, 10, Some(date));
        let v = map.get(&10).unwrap();
        assert_eq!(10, v);

        let two_sec = Duration::new(2, 0);
        thread::sleep(two_sec);
        let v = map.get(&10);
        assert_eq!(None, v);
    }

    #[test]
    fn remove() {
        let map: DashMapWithExpiry<u64, u64> = DashMapWithExpiry::new();
        map.insert(10, 10, None);
        let v = map.get(&10).unwrap();
        assert_eq!(10, v);
        map.remove(&10);
        let v = map.get(&10);
        assert_eq!(None, v);
    }
}
