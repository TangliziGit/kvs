#![deny(missing_docs)]
//! A simple key-value store.

pub use error::{Error, Result};
pub use engine::kvs::KvStore;
pub use protocol::{Request, Response};
pub use server::KvsServer;
pub use client::KvsClient;

mod error;
mod protocol;
mod server;
mod client;
mod engine;
