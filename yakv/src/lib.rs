use anyhow;
use std::env;
use std::io::{Write, BufRead, BufReader};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs::{File, OpenOptions, create_dir_all};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use serde::{Serialize, Deserialize};

pub type Result<T> = anyhow::Result<T>;

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
    pub log_index: HashMap<u64, u64>,
    pub log_file: Result<File>,
    pub index_file: Result<File>,
}

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

impl KvStore {
    /// Creates a KvStore
    pub fn new() -> Result<Self> {
        let path = PathBuf::from("data/ ");
        KvStore::open(path)
    }

    /// Sets the value of a key to a string.
    ///
    /// It overwrites the value if key is already in the store.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let log = Log::new("set".to_string(), vec![key.to_owned(), value.to_owned()]);
        // log file
        // index file
        let mut store = KvStore::new()?;
        store.log_index.insert(KvStore::calculate_hash(&log), 0);

        println!("{:?}", store.log_index);

        let mut log_file = store.log_file?;

        log_file.write_all(&serde_json::to_vec(&log)?)?;

        Ok(())
    }

    /// Gets the string value of a given key string.
    ///
    /// Returns `None` if the key does not exist.
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        unimplemented!();
    }

    /// Removes a given key
    pub fn remove(&mut self, key: String) -> Result<()> {
        unimplemented!();
    }

    /// Open the KvStore at a given path
    ///
    /// Return the KvStore
    pub fn open<T: Into<PathBuf>>(path: T) -> Result<Self> {
        let path = path.into();
        let log_file = KvStore::get_file(path.clone(), "store.log");
        let index_file = KvStore::get_file(path, "index.log");

        Ok(KvStore {
            log_index: HashMap::new(),
            log_file,
            index_file,
        })
    }

    fn get_file(path: PathBuf, filename: &str) -> Result<File> {
        create_dir_all(&path)?;
        let mut file_path = path;
        file_path.set_file_name(filename);
        // println!("Path: {:?}", file_path);

        OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(&file_path)
            .map_err(|e| anyhow::Error::new(e))
    }

    fn calculate_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }
}
