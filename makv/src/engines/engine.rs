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

/// Define MakvEngine trait
pub trait MakvEngine: Clone + Send + 'static {
    /// Sets the value of s string key to a string.
    fn set(&self, key: String, value: String) -> Result<()>;

    /// Gets the string value for a given key.
    fn get(&self, key: String) -> Result<Option<String>>;

    /// Removes the given key.
    fn remove(&self, key: String) -> Result<()>;
}
