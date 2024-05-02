use std::collections::HashMap;
use std::hash::Hash;

pub struct Cache<K, V> {
    map: HashMap<K, V>,
}

impl<K, V> Cache<K, V>
where
    K: Eq + Hash,
{
    pub fn new() -> Cache<K, V> {
        Cache {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }
}