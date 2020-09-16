pub(crate) mod kvs;
pub(crate) mod sled;

use crate::Result;

/// KvsEngine trait provides key-value store methods.
pub trait KvsEngine: Clone + Send + 'static {
    /// Sets the value of a string key to a string.
    /// Return an error if the value is not written successfully.
    fn set(&self, key: String, value: String) -> Result<()>;

    /// Gets the string value of the a string key.
    /// If the key does not exist, return None.
    /// Return an error if the value is not read successfully.
    fn get(&self, key: String) -> Result<Option<String>>;

    /// Removes a given key.
    /// Return an error if the key does not exist or is not removed successfully.
    fn remove(&self, key: String) -> Result<()>;
}
