#![deny(missing_docs)]
//! Yet another Key/Value store

pub use engine::{Engine, MakvEngine};
pub use error::{Result, YakvError};
pub use protocol::{Payload, PayloadType, Response, YakvMessage};
pub use thread_pool::{NaiveThreadPool, RayonThreadPool, SharedQueueThreadPool, ThreadPool};
pub use yakv::{Command, KvStore};

mod engine;
mod error;
mod protocol;
mod thread_pool;
mod yakv;
