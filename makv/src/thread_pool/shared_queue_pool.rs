use std::thread;

use super::ThreadPool;
use crate::Result;

use crossbeam::channel::{self, Receiver, Sender};

#[allow(missing_docs)]
pub struct SharedQueueThreadPool {
    tx: Sender<Box<dyn FnOnce() + Send + 'static>>,
}

impl ThreadPool for SharedQueueThreadPool {
    fn new(threads: u32) -> Result<Self> {
        let (tx, rx) = channel::unbounded::<Box<dyn FnOnce() + Send + 'static>>();
        for _ in 0..threads {
            let rx = TaskReceiver(rx.clone());
            thread::Builder::new().spawn(move || run_tasks(rx))?;
        }
        Ok(SharedQueueThreadPool { tx })
    }

    /// Spawns a function into the thread pool.
    ///
    /// # Panics
    ///
    /// Panics if the thread pool has no thread.
    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.tx
            .send(Box::new(job))
            .expect("The thread pool has no thread.");
    }
}

#[derive(Clone)]
struct TaskReceiver(Receiver<Box<dyn FnOnce() + Send + 'static>>);

impl Drop for TaskReceiver {
    fn drop(&mut self) {
        if thread::panicking() {
            let rx = self.clone();
            if let Err(e) = thread::Builder::new().spawn(move || run_tasks(rx)) {
                eprint!("Failed to spawn a thread: {}", e);
            }
        }
    }
}

fn run_tasks(rx: TaskReceiver) {
    loop {
        match rx.0.recv() {
            Ok(task) => {
                task();
            }
            Err(_) => eprint!("Thread exits because the thread pool is destroyed."),
        }
    }
}

// use crate::{Result, ThreadPool};
// use std::sync::{mpsc, Arc, Mutex};
// use std::thread;
//
// type Job = Box<dyn FnOnce() + Send + 'static>;
// type Receiver = Arc<Mutex<mpsc::Receiver<Job>>>;
//
// #[derive(Clone)]
// struct JobReceiver(Receiver);
//
// impl Drop for JobReceiver {
//     fn drop(&mut self) {
//         if thread::panicking() {
//             let rx = self.clone();
//             if let Err(e) = thread::Builder::new().spawn(move || execute_job(rx)) {
//                 eprint!("Failed to spawn a thread: {}", e);
//             }
//         }
//     }
// }
//
// fn execute_job(worker: JobReceiver) {
//     loop {
//         if let Ok(rx) = worker.0.lock() {
//             if let Ok(job) = rx.recv() {
//                 job();
//             } else {
//                 break;
//             }
//         } else {
//             break;
//         }
//     }
// }
//
// #[allow(missing_docs)]
// pub struct SharedQueueThreadPool {
//     sender: mpsc::Sender<Job>,
// }
//
// impl ThreadPool for SharedQueueThreadPool {
//     fn new(size: u32) -> Result<Self> {
//         assert!(size > 0);
//         let size = size as usize;
//         let (sender, receiver) = mpsc::channel::<Job>();
//         let receiver = Arc::new(Mutex::new(receiver));
//
//         for _ in 0..size {
//             let rx = receiver.clone();
//             thread::Builder::new().spawn(move || execute_job(JobReceiver(rx)))?;
//         }
//
//         Ok(SharedQueueThreadPool { sender })
//     }
//
//     fn spawn<F>(&self, f: F)
//     where
//         F: FnOnce() + Send + 'static,
//     {
//         let job = Box::new(f);
//         self.sender
//             .send(job)
//             .expect("The thread pool has no thread.");
//     }
// }
