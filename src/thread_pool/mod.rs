/// ThreadPool in a naive approach.
pub use naive::NaiveThreadPool;

mod naive;

use crate::error::Result;

/// ThreadPool trait must be implemented by all thread pools.
pub trait ThreadPool {

    /// Create a new thread pool.
    fn new(threads: u32) -> Result<Self>
        where Self: Sized;

    /// Spawn the closure in one of the threads.
    fn spawn<F>(&self, job: F)
        where F: FnOnce() + Send + 'static;

}
