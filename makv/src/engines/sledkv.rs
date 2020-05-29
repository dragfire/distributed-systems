use crate::engines::MakvEngine;
use crate::{MakvError, Result};
use sled::Db;
use std::path::{Path, PathBuf};

/// SledStore implements YakvEngine trait
pub struct SledStore {
    db: Db,
}

impl SledStore {
    /// Return Sled engine for the given path
    pub fn open(path: &Path) -> Result<Self> {
        let mut path = PathBuf::from(path);
        path.push("engine_sled_data");
        let db = sled::open(path)?;
        Ok(SledStore { db })
    }
}

impl MakvEngine for SledStore {
    fn set(&mut self, key: String, value: String) -> Result<()> {
        self.db.insert(key.as_bytes(), value.as_bytes())?;
        self.db.flush()?;
        Ok(())
    }

    fn get(&mut self, key: String) -> Result<Option<String>> {
        let val = self
            .db
            .get(key.as_bytes())?
            .map(|ivec| String::from_utf8(Vec::from(&*ivec)))
            .and_then(|v| v.ok());
        Ok(val)
    }

    fn remove(&mut self, key: String) -> Result<()> {
        let result = self.db.remove(key.as_bytes())?;
        self.db.flush()?;
        if result.is_none() {
            Err(MakvError::NotFoundError(key))
        } else {
            Ok(())
        }
    }
}
