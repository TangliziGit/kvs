use std::thread;
use crate::error::Result;
use crate::thread_pool::ThreadPool;

/// Thread pool in a Naive approach, which not reuse threads.
pub struct NaiveThreadPool;

impl ThreadPool for NaiveThreadPool {

    fn new(_threads: u32) -> Result<Self> {
        Ok(NaiveThreadPool)
    }

    fn spawn<F>(&self, job: F) where F: FnOnce() + Send + 'static {
        thread::spawn(job);
    }
}
