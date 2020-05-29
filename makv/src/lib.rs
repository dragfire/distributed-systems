#![deny(missing_docs)]
//! Yet another Key/Value store

pub use engines::{Command, Engine, KvStore, MakvEngine, SledStore};
pub use error::{MakvError, Result};
pub use protocol::{MakvMessage, Payload, PayloadType, Response};
pub use thread_pool::{NaiveThreadPool, ThreadPool};

mod engines;
mod error;
mod protocol;
mod thread_pool;
