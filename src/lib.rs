use std::collections::HashMap;

pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {

    #[inline]
    pub fn new() -> KvStore {
        let map = HashMap::new();

        KvStore { map }
    }

    /// Set the value of a string key to a string
    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    /// Get the string value of the a string key. If the key does not exist, return None.
    pub fn get(&self, key: String) -> Option<String> {
        self.map.get(&key)
            .map(Clone::clone)
    }

    /// Remove a given key.
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}

