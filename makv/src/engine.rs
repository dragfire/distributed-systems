use crate::Result;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum Engine {
    Yakv,
    Sled,
}

// NOTE: look into arg_enum!() macro from clap as an alternative
impl FromStr for Engine {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, ()> {
        match s {
            "yakv" => Ok(Engine::Yakv),
            "sled" => Ok(Engine::Sled),
            _ => Err(()),
        }
    }
}

/// Define YakvEngine trait
pub trait YakvEngine {
    /// Sets the value of s string key to a string.
    fn set(&self, key: String, value: String) -> Result<()>;

    /// Gets the string value for a given key.
    fn get(&self, key: String) -> Result<Option<String>>;

    /// Removes the given key.
    fn remove(&self, key: String) -> Result<()>;
}

// /// YakvSledEngine implements YakvEngine trait
// pub struct YakvSledEngine {
//     db: Db,
// }
//
// impl YakvSledEngine {
//     /// Return Sled engine for the given path
//     pub fn open(path: &Path) -> Result<Self> {
//         let mut path = PathBuf::from(path);
//         path.push("engine_sled_data");
//         let db = sled::open(path)?;
//         Ok(YakvSledEngine { db })
//     }
// }
//
// impl YakvEngine for YakvSledEngine {
//     fn set(&mut self, key: String, value: String) -> Result<()> {
//         self.db.insert(key.as_bytes(), value.as_bytes())?;
//         self.db.flush()?;
//         Ok(())
//     }
//
//     fn get(&mut self, key: String) -> Result<Option<String>> {
//         let val = self
//             .db
//             .get(key.as_bytes())?
//             .map(|ivec| String::from_utf8(Vec::from(&*ivec)))
//             .and_then(|v| v.ok());
//         Ok(val)
//     }
//
//     fn remove(&mut self, key: String) -> Result<()> {
//         let result = self.db.remove(key.as_bytes())?;
//         self.db.flush()?;
//         if result.is_none() {
//             Err(YakvError::NotFoundError(key))
//         } else {
//             Ok(())
//         }
//     }
// }
