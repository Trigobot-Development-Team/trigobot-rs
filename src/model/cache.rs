use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

use serde::{Deserialize, Serialize};

/// Custom implementation of cache that is serializable
#[derive(Default, Deserialize, Serialize)]
pub(crate) struct Cache<K: Hash + Eq + Clone, V> {
    storage: VecDeque<K>,
    keys: HashMap<K, V>,
    capacity: usize,
}

impl<K: Hash + Eq + Clone, V> Cache<K, V> {
    pub fn new(capacity: usize) -> Self {
        Cache {
            storage: VecDeque::new(),
            keys: HashMap::new(),
            capacity,
        }
    }

    pub fn put(&mut self, key: K, val: V) {
        if self.keys.contains_key(&key) {
            // Remove old key
            let pos = self.storage.iter().position(|e| e == &key).unwrap();
            self.storage.remove(pos).unwrap();
        } else if self.keys.len() == self.capacity {
            let key = self.storage.pop_back().unwrap();
            self.keys.remove(&key);
        }

        self.storage.push_front(key.clone());
        self.keys.insert(key, val);
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.keys.get(key)
    }
}
