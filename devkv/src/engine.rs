use crate::Result;
use crate::YakvError;
use sled::Db;
use std::fs;
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
pub struct YakvSledEngine {
    db: Db,
}

impl YakvSledEngine {
    /// Return Sled engine for the given path
    pub fn open(path: PathBuf) -> Result<Self> {
        let mut path = path;
        path.push("engine_sled_data");
        let db = sled::open(path)?;
        Ok(YakvSledEngine { db })
    }
}

impl YakvEngine for YakvSledEngine {
    fn set(&mut self, key: String, value: String) -> Result<()> {
        self.db.insert(key.as_bytes(), value.as_bytes())?;
        Ok(())
    }

    fn get(&mut self, key: String) -> Result<Option<String>> {
        let val = self
            .db
            .get(key.as_bytes())?
            .map(|ivec| String::from_utf8(Vec::from(&*ivec)));
        let res = val.and_then(|v| v.ok());
        Ok(res)
    }

    fn remove(&mut self, key: String) -> Result<()> {
        let result = self.db.remove(key.as_bytes())?;
        if result.is_none() {
            Err(YakvError::NotFoundError(key))
        } else {
            Ok(())
        }
    }
}
