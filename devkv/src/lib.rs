#![deny(missing_docs)]
//! Yet another Key/Value store

pub use engine::YakvEngine;
pub use error::{Result, YakvError};
pub use protocol::{Payload, PayloadType, Response, YakvMessage};
pub use yakv::{Command, KvStore};

mod engine;
mod error;
mod protocol;
mod yakv;
