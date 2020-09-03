pub struct KvStore;

impl KvStore {

    #[inline]
    pub fn new() -> KvStore {
        KvStore
    }

    /// Set the value of a string key to a string
    pub fn set(&mut self, key: String, value: String) {
        unimplemented!();
    }

    /// Get the string value of the a string key. If the key does not exist, return None.
    pub fn get(&self, key: String) -> Option<String> {
        unimplemented!();
    }

    /// Remove a given key.
    pub fn remove(&mut self, key: String) {
        unimplemented!();
    }
}

