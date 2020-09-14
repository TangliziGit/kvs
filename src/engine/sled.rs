use crate::engine::KvsEngine;
use crate::error::ErrorKind;
use crate::Result;
use sled::Db;
use std::path::PathBuf;

/// Used to store a string key to a string value with sled engine.
pub struct SledKvsEngine {
    db: Db,
}

impl SledKvsEngine {
    /// Open the SledKvsEngine at a given path.
    pub fn open(path: impl Into<PathBuf>) -> Result<Self> {
        let path = path.into();
        let path = path.join("sled.db");

        let db = sled::open(path)?;

        Ok(SledKvsEngine { db })
    }
}

impl KvsEngine for SledKvsEngine {
    /// Sets the value of a string key to a string.
    /// Return an error if the value is not written successfully.
    fn set(&mut self, key: String, value: String) -> Result<()> {
        self.db.insert(key, value.as_bytes())?;
        self.db.flush()?;
        Ok(())
    }

    /// Gets the string value of the a string key.
    /// If the key does not exist, return None.
    /// Return an error if the value is not read successfully.
    fn get(&mut self, key: String) -> Result<Option<String>> {
        let ivec = self.db.get(key)?;
        let ivec = match ivec {
            Some(ivec) => Some(String::from_utf8(ivec.to_vec())?),
            None => None,
        };

        self.db.flush()?;
        Ok(ivec)
    }

    /// Removes a given key.
    /// Return an error if the key does not exist or is not removed successfully.
    fn remove(&mut self, key: String) -> Result<()> {
        self.db.remove(key)?.ok_or(ErrorKind::KeyNotFound)?;
        self.db.flush()?;
        Ok(())
    }
}
