use crate::error::{Error, Result};
use failure::_core::ops::Range;
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

const COMPACTION_THRESHOLD: u64 = 1024 * 1024;

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
    path: PathBuf,
    writer: BufWriter<File>,
    readers: HashMap<u64, BufReader<File>>,
    index: HashMap<String, CommandOffset>,
    current_gen: u64,
    uncompacted: u64,
}

impl KvStore {
    /// Open the KvStore at a given path.
    /// Return the KvStore.
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let path = path.into();
        fs::create_dir_all(&path)?;

        let mut readers = HashMap::new();
        let mut index = HashMap::new();

        let gens = generations(&path)?;
        for gen in gens.iter() {
            let path = db_path(&path, *gen);
            let mut reader = BufReader::new(File::open(path)?);

            load_index(*gen, &mut reader, &mut index)?;
            readers.insert(*gen, reader);
        }

        let current_gen = gens.last().unwrap_or(&0) + 1;
        let (writer, reader) = new_db_log(&db_path(&path, current_gen))?;
        readers.insert(current_gen, reader);

        Ok(KvStore {
            path,
            writer,
            readers,
            index,
            current_gen,
            uncompacted: 0,
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
        let command = Command::Set {
            key: key.clone(),
            value,
        };

        let pos = self.writer.seek(SeekFrom::End(0))?;
        serde_json::to_writer(&mut self.writer, &command)?;

        let new_pos = self.writer.seek(SeekFrom::Current(0))?;
        if let Some(cmd) = self
            .index
            .insert(key, From::from((self.current_gen, pos..new_pos)))
        {
            self.uncompacted += cmd.len;

            if self.uncompacted >= COMPACTION_THRESHOLD {
                self.compact()?;
            }
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
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        if let Some(CommandOffset { gen, pos, len }) = self.index.get(&key) {
            let reader = self
                .readers
                .get_mut(&gen)
                .expect("Can not find the log reader");
            reader.seek(SeekFrom::Start(*pos))?;

            let mut buffer = vec![0u8; *len as usize];
            reader.read_exact(&mut buffer)?;

            let cmd: Command = serde_json::from_slice(&buffer)?;
            if let Command::Set { key: _, value } = cmd {
                Ok(Some(value))
            } else {
                unreachable!()
            }
        } else {
            Ok(None)
        }
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
                let command = Command::Remove { key: key.clone() };

                self.writer.seek(SeekFrom::End(0))?;
                serde_json::to_writer(&mut self.writer, &command)?;

                if let Some(CommandOffset {
                    gen: _,
                    pos: _,
                    len,
                }) = self.index.remove(&key)
                {
                    self.uncompacted += len;

                    if self.uncompacted >= COMPACTION_THRESHOLD {
                        self.compact()?;
                    }
                }

                Ok(())
            }
            None => Err(Error::KeyNotFound),
        }
    }

    /// Compacting the log file.
    /// To support concurrent, use generation to maintain the log files.
    pub fn compact(&mut self) -> Result<()> {
        let (mut compact_writer, compact_reader) =
            new_db_log(&db_path(&self.path, self.current_gen + 1))?;
        let (new_writer, new_reader) = new_db_log(&db_path(&self.path, self.current_gen + 2))?;

        self.current_gen += 2;
        self.writer = new_writer;
        self.readers.insert(self.current_gen - 1, compact_reader);
        self.readers.insert(self.current_gen, new_reader);

        for (_, value) in self.index.iter_mut() {
            let CommandOffset { gen, pos, len } = value;
            let reader = self
                .readers
                .get_mut(&gen)
                .expect("Can not find reader of the log file");

            reader.seek(SeekFrom::Start(*pos))?;
            let mut buffer = vec![0; *len as usize];
            reader.read_exact(&mut buffer)?;

            *pos = compact_writer.seek(SeekFrom::Current(0))?;
            *gen = self.current_gen - 1;
            compact_writer.write_all(&buffer)?;
        }

        let stale_gens = generations(&self.path)?
            .into_iter()
            .filter(|gen| *gen <= self.current_gen - 2)
            .collect::<Vec<u64>>();

        for ref gen in stale_gens {
            let path = db_path(&self.path, *gen);
            self.readers.remove(gen);
            fs::remove_file(path)?;
        }

        Ok(())
    }
}

fn db_path(path: &PathBuf, gen: u64) -> PathBuf {
    let file_name = format!("{}.log", gen);
    path.join(file_name)
}

fn new_db_log(path: &PathBuf) -> Result<(BufWriter<File>, BufReader<File>)> {
    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(&path)?;

    let writer = BufWriter::new(file.try_clone()?);
    let reader = BufReader::new(file);

    Ok((writer, reader))
}

fn generations(path: &PathBuf) -> Result<Vec<u64>> {
    let mut gens = fs::read_dir(path)?
        .flat_map(|entry| -> Result<_> { Ok(entry?.path()) })
        .filter(|path| path.is_file() && path.extension() == Some("log".as_ref()))
        .flat_map(|path| {
            path.file_name()
                .and_then(OsStr::to_str)
                .map(|str| str.trim_end_matches(".log"))
                .map(str::parse::<u64>)
        })
        .flatten()
        .collect::<Vec<u64>>();

    gens.sort_unstable();
    Ok(gens)
}

fn load_index(
    gen: u64,
    reader: &mut BufReader<File>,
    index: &mut HashMap<String, CommandOffset>,
) -> Result<()> {
    let mut pos = reader.seek(SeekFrom::Start(0))?;
    let mut stream = Deserializer::from_reader(reader).into_iter::<Command>();
    while let Some(cmd) = stream.next() {
        let new_pos = stream.byte_offset() as u64;

        match cmd? {
            Command::Set { key, value: _ } => {
                index.insert(key, From::from((gen, pos..new_pos)));
            }
            Command::Remove { key } => {
                index.remove(&key);
            }
        }

        pos = new_pos;
    }

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}

#[derive(Debug)]
struct CommandOffset {
    gen: u64,
    pos: u64,
    len: u64,
}

impl From<(u64, Range<u64>)> for CommandOffset {
    fn from((gen, range): (u64, Range<u64>)) -> Self {
        CommandOffset {
            gen,
            pos: range.start,
            len: range.end - range.start,
        }
    }
}
