#![deny(missing_docs)]
//! Yet another Key/Value store

pub use engine::KvsEngine;
pub use error::{Result, YakvError};
pub use yakv::KvStore;

mod engine;
mod error;
mod yakv;
