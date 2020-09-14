pub(crate) mod kvs;
pub(crate) mod sled;

use crate::Result;

/// KvsEngine trait provides key-value store methods.
pub trait KvsEngine {
    /// Sets the value of a string key to a string.
    /// Return an error if the value is not written successfully.
    fn set(&mut self, key: String, value: String) -> Result<()>;

    /// Gets the string value of the a string key.
    /// If the key does not exist, return None.
    /// Return an error if the value is not read successfully.
    fn get(&mut self, key: String) -> Result<Option<String>>;

    /// Removes a given key.
    /// Return an error if the key does not exist or is not removed successfully.
    fn remove(&mut self, key: String) -> Result<()>;
}
