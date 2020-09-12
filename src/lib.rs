#![deny(missing_docs)]
//! A simple key-value store.

pub use error::{Error, Result};
pub use kv_store::KvStore;
pub use protocol::{Request, Response};
pub use server::KvsServer;
pub use client::KvsClient;

mod error;
mod kv_store;
mod protocol;
mod server;
mod client;
