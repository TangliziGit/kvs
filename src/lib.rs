#![deny(missing_docs)]
//! A simple key-value store.

pub use client::KvsClient;
pub use engine::{kvs::KvStore, sled::SledKvsEngine, KvsEngine};
pub use error::{Error, Result};
pub use protocol::{Request, Response};
pub use server::KvsServer;

mod client;
mod engine;
mod error;
mod protocol;
mod server;

/// The thread_pool modular.
pub mod thread_pool;
