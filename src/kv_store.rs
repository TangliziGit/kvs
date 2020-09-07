use std::collections::HashMap;

/// Used to store a string key to a string value.
///
/// # Example
///
/// ```
/// # use kvs::KvStore;
/// let mut kvs = KvStore::new();
///
/// kvs.set("key".to_string(), "value".to_string());
///
/// let val = kvs.get("key".to_string());
/// assert_eq!(val, Some("value".to_string()));
///
/// kvs.remove("key".to_string());
/// let val = kvs.get("key".to_string());
/// assert_eq!(val, None);
/// ```
#[derive(Default)]
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    /// Creates a `KvStore`
    #[inline]
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }

    /// Sets the value of a string key to a string
    ///
    /// # Example
    ///
    /// ```
    /// # use kvs::KvStore;
    /// let mut kvs = KvStore::new();
    /// kvs.set("key".to_string(), "value".to_string());
    /// ```
    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    /// Gets the string value of the a string key. If the key does not exist, return None.
    ///
    /// # Example
    ///
    /// ```
    /// # use kvs::KvStore;
    /// let kvs = KvStore::new();
    /// let value = kvs.get("key".to_string());
    ///
    /// assert_eq!(value, None);
    /// ```
    pub fn get(&self, key: String) -> Option<String> {
        self.map.get(&key).map(Clone::clone)
    }

    /// Removes a given key.
    ///
    /// # Example
    ///
    /// ```
    /// # use kvs::KvStore;
    /// let mut kvs = KvStore::new();
    /// kvs.set("key".to_string(), "value".to_string());
    /// kvs.remove("key".to_string());
    ///
    /// let value = kvs.get("key".to_string());
    /// assert_eq!(value, None);
    /// ```
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}
