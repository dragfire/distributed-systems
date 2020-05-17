#![deny(missing_docs)]
//! A simple key/value store

use std::collections::HashMap;

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
#[derive(Default)]
pub struct KvStore {
    data: HashMap<String, String>,
}

impl KvStore {
    /// Creates a KvStore
    pub fn new() -> Self {
        KvStore {
            data: HashMap::new(),
        }
    }

    /// Sets the value of a key to a string.
    ///
    /// It overwrites the value if key is already in the store.
    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    /// Gets the string value of a given key string.
    ///
    /// Returns `None` if the key does not exist.
    pub fn get(&mut self, key: String) -> Option<String> {
        self.data.get(&key).map(ToOwned::to_owned)
    }

    /// Removes a given key
    pub fn remove(&mut self, key: String) {
        self.data.remove(&key);
    }
}
