#![deny(missing_docs)]
//! Yet another Key/Value store

pub use engine::KvsEngine;
pub use yakv::{KvStore, Result};

mod engine;
mod yakv;
