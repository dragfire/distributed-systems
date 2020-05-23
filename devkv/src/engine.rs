use crate::Result;
use std::path::PathBuf;

/// Define YakvEngine trait
pub trait YakvEngine {
    /// Sets the value of s string key to a string.
    fn set(&mut self, key: String, value: String) -> Result<()>;

    /// Gets the string value for a given key.
    fn get(&mut self, key: String) -> Result<Option<String>>;

    /// Removes the given key.
    fn remove(&mut self, key: String) -> Result<()>;
}

/// YakvSledEngine implements YakvEngine trait
pub struct YakvSledEngine {}

impl YakvSledEngine {
    fn new(path: PathBuf) -> Result<Self> {
        Ok(YakvSledEngine {})
    }
}

impl YakvEngine for YakvSledEngine {
    fn set(&mut self, key: String, value: String) -> Result<()> {
        panic!();
    }

    fn get(&mut self, key: String) -> Result<Option<String>> {
        panic!();
    }

    fn remove(&mut self, key: String) -> Result<()> {
        panic!();
    }
}
