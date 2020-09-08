#![deny(missing_docs)]
//! A simple key-value store.

pub use error::{Error, Result};
pub use kv_store::KvStore;

mod error;
mod kv_store;
