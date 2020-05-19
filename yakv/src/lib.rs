#![allow(warnings)]
use anyhow;
use std::env;
use std::io::{Write, Read, SeekFrom, Seek, BufRead, BufReader};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs::{File, OpenOptions, create_dir_all};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use serde::{Serialize, Deserialize};

pub type Result<T> = anyhow::Result<T>;
pub type LogIndex = HashMap<String, (u64, usize)>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct Log {
    command: String, 
    args: Vec<String>,
}

impl Log {
    fn new(command: String, args: Vec<String>) -> Self {
        Log {
            command,
            args,
        }
    }
}

/// KvStore stores string key/value pairs.
///
/// Key/Value pairs are stored in a `HashMap` in memory and not persisted to disk.
///
/// Example:
///
/// ```rust
/// # use kvs::KvStore;
/// let mut store = KvStore::new();
/// store.set("key".to_owned(), "value".to_owned());
/// let val = store.get("key".to_owned());
/// assert_eq!(val, Some("value".to_owned()));
/// ```
pub struct KvStore {
    pub log_index: LogIndex,
    pub path: PathBuf,
    pub log_file: File,
    pub index_file: File,
}

impl KvStore {
    /// Creates a KvStore
    pub fn new() -> Result<Self> {
        KvStore::open(env::current_dir()?)
    }

    /// Sets the value of a key to a string.
    ///
    /// It overwrites the value if key is already in the store.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let log = Log::new("set".to_string(), vec![key.to_owned(), value.to_owned()]);
        let log_bytes = &serde_json::to_vec(&log)?;

        self.log_index.insert(key, (self.log_file.metadata()?.len(), log_bytes.len()));

        self.log_file.write_all(log_bytes)?;
        self.save_log_index();

        Ok(())
    }

    /// Gets the string value of a given key string.
    ///
    /// Returns `None` if the key does not exist.
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        if let Some(log_index) = self.log_index.get(&key) {
            self.log_file.seek(SeekFrom::Start(log_index.0));

            let mut buf = vec![0u8; log_index.1];
            self.log_file.read_exact(&mut buf)?;

            let log: Log = serde_json::from_slice(&buf)?;
            let val = log.args[1].to_owned();

            return Ok(Some(val));
        }
        Ok(None)
    }

    /// Removes a given key
    pub fn remove(&mut self, key: String) -> Result<()> {
        if let Some(log_index) = self.log_index.get(&key) {
            let log = Log::new("rm".to_string(), vec![key.to_owned()]);
            self.log_file.write_all(&serde_json::to_vec(&log)?)?;
            self.log_index.remove(&key).ok_or("Log Index: Key not found");
            self.save_log_index();

            return Ok(());
        } 

        Err(anyhow::Error::msg("Key not found"))
    }

    fn build_log_index(file: File) -> Result<LogIndex> {
        let file = BufReader::new(file);
        Ok(serde_json::from_reader(file)?)
    }

    /// Serialize LogIndex and save it on disk
    pub fn save_log_index(&self) -> Result<()> {
        let mut file = KvStore::get_file(self.path.clone(), "index.log", false)?;
        //println!("log_index: {:?}", self.log_index);
        file.write_all(&serde_json::to_vec(&self.log_index)?)?;
        Ok(())
    }

    /// Open the KvStore at a given path
    ///
    /// Return the KvStore
    pub fn open<T: Into<PathBuf>>(path: T) -> Result<Self> {
        let data_path: PathBuf = PathBuf::from("data/ ");
        let mut path = path.into();
        path.push(data_path);
        let log_file = KvStore::get_file(path.clone(), "store.log", true)?;
        let index_file = KvStore::get_file(path.clone(), "index.log", false)?;
        let log_index = KvStore::build_log_index(index_file.try_clone()?).or_else(|e| {
            //println!("{:?}", e);
            Err(e)
        }).unwrap_or(HashMap::new());

        Ok(KvStore {
            log_index, 
            path,
            log_file,
            index_file,
        })
    }

    fn get_file(path: PathBuf, filename: &str, append: bool) -> Result<File> {
        create_dir_all(&path)?;
        let mut file_path = path;
        file_path.set_file_name(filename);

        let mut options = OpenOptions::new();
        options.read(true)
            .write(true)
            .create(true);

        if append {
            options.append(append);
        }

        options.open(&file_path)
            .map_err(|e| anyhow::Error::new(e))
    }
}
