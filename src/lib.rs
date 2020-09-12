#![deny(missing_docs)]
//! A simple key-value store.

pub use error::{Error, Result};
pub use kv_store::KvStore;
pub use protocol::Protocol;

mod error;
mod kv_store;
mod protocol;
