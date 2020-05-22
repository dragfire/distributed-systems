use crate::{KvStore, Result};
use std::path::PathBuf;

trait Engine {
    fn set(&mut self, key: String, value: String) -> Result<()>;

    fn get(&mut self, key: String) -> Result<Option<String>>;

    fn remove(&mut self, key: String) -> Result<()>;
}

/// KvsEngine implements Engine trait
pub struct KvsEngine {
    store: KvStore,
}

impl KvsEngine {
    fn new(path: PathBuf) -> Result<Self> {
        Ok(KvsEngine {
            store: KvStore::open(path)?,
        })
    }
}

impl Engine for KvsEngine {
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
