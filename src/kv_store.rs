use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, BufWriter, Seek, SeekFrom, Write};
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
pub struct KvStore {
    writer: BufWriter<File>,
    reader: BufReader<File>,
    index: HashMap<String, CommandOffset>,
}

impl KvStore {
    /// Open the KvStore at a given path.
    /// Return the KvStore.
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let path = path.into();
        fs::create_dir_all(&path)?;

        let path = db_path(&path)?;
        let writer = BufWriter::new(OpenOptions::new().write(true).open(&path)?);
        let mut reader = BufReader::new(File::open(&path)?);
        let index = load_index(&mut reader)?;

        Ok(KvStore {
            writer,
            reader,
            index,
        })
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
        let command = Command::Set { key, value };
        let content = serde_json::to_string(&command)?;

        self.writer.seek(SeekFrom::End(0))?;
        self.writer.write_all(content.as_bytes())?;
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
    pub fn get(&self, _key: String) -> Result<Option<String>> {
        unimplemented!()
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
        match self.index.get(&key) {
            Some(_) => {
                let command = Command::Remove { key };
                let content = serde_json::to_string(&command)?;

                self.writer.seek(SeekFrom::End(0))?;
                self.writer.write_all(content.as_bytes())?;
                Ok(())
            }
            None => Err(Error::KeyNotFound),
        }
    }
}

fn db_path(path: &PathBuf) -> Result<PathBuf> {
    let path = path.join("kvs.db");

    if !path.exists() {
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)?;
    }

    Ok(path)
}

fn load_index(reader: &mut BufReader<File>) -> Result<HashMap<String, CommandOffset>> {
    let mut pos = reader.seek(SeekFrom::Start(0))?;
    let mut index = HashMap::new();
    let mut stream = Deserializer::from_reader(reader).into_iter::<Command>();
    while let Some(cmd) = stream.next() {
        let new_pos = stream.byte_offset() as u64;

        match cmd? {
            Command::Set { key, value: _ } => {
                index.insert(key, From::from((pos, new_pos - 1)));
            }
            Command::Remove { key } => {
                index.remove(&key);
            }
        }

        pos = new_pos;
    }

    Ok(index)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}

#[derive(Debug)]
struct CommandOffset {
    pos: u64,
    len: u64,
}

impl From<(u64, u64)> for CommandOffset {
    fn from((pos, len): (u64, u64)) -> Self {
        CommandOffset { pos, len }
    }
}
