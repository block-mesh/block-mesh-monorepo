use chrono::{DateTime, Utc};
use dashmap::DashMap;
use std::hash::Hash;

pub struct DashMapWithExpiry<K, V> {
    map: DashMap<K, V>,
    expiry: DashMap<K, DateTime<Utc>>,
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
        self.map.remove(key);
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

    pub fn get(&'a self, key: &K) -> Option<V> {
        let e = self.expiry.get(key);
        match e {
            Some(expiry) => {
                let expiry_date_time = expiry.value();
                if Utc::now() > *expiry_date_time {
                    self.remove(key);
                    None
                } else {
                    self._get(&key)
                }
            }
            None => self._get(&key),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_expiry() {}
}
