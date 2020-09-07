use crate::error::{ErrorKind, Result};
use std::collections::HashMap;
use std::path::PathBuf;

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

    /// Open the KvStore at a given path.
    /// Return the KvStore.
    pub fn open(_path: impl Into<PathBuf>) -> Result<KvStore> {
        unimplemented!()
    }

    /// Sets the value of a string key to a string.
    /// Return an error if the value is not written successfully.
    ///
    /// # Example
    ///
    /// ```
    /// # use kvs::KvStore;
    /// let mut kvs = KvStore::new();
    /// kvs.set("key".to_string(), "value".to_string());
    /// ```
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        if self.map.insert(key, value).is_none() {
            return Err(ErrorKind::KeyNotFound.into());
        }

        Ok(())
    }

    /// Gets the string value of the a string key.
    /// If the key does not exist, return None.
    /// Return an error if the value is not read successfully.
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
    pub fn get(&self, key: String) -> Result<Option<String>> {
        Ok(self.map.get(&key).map(Clone::clone))
    }

    /// Removes a given key.
    /// Return an error if the key does not exist or is not removed successfully.
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
    pub fn remove(&mut self, key: String) -> Result<()> {
        self.map.remove(&key);
        Ok(())
    }
}
