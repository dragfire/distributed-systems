use crate::Result;
use std::thread;

#[allow(missing_docs)]
pub trait ThreadPool {
    /// Creates a new thread pool, immediately spawning the specified number of threads.
    ///
    /// Returns an error if any thread fails to spawn. All previously-spawned threads are terminated.
    fn new(threads: u32) -> Result<Self>
    where
        Self: Sized;

    /// Spawn a function into the threadpool.
    ///
    /// Spawning always succeeds, but if the function panics the threadpool continues to operate with the same number of threads
    /// — the thread count is not reduced nor is the thread pool destroyed, corrupted or invalidated.
    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static;
}

#[allow(missing_docs)]
pub struct SharedQueueThreadPool;

impl ThreadPool for SharedQueueThreadPool {
    fn new(_threads: u32) -> Result<Self> {
        Ok(SharedQueueThreadPool)
    }

    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        thread::spawn(|| {
            job();
        });
    }
}

#[allow(missing_docs)]
pub struct RayonThreadPool;

impl ThreadPool for RayonThreadPool {
    fn new(_threads: u32) -> Result<Self> {
        Ok(Self)
    }

    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        thread::spawn(|| {
            job();
        });
    }
}
