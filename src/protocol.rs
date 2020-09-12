#![allow(missing_docs)]
use serde::{Deserialize, Serialize};

/// Used to communicate between clients and server.
#[derive(Debug, Serialize, Deserialize)]
pub enum Protocol {
    Set { key: String, value: String },
    Get { key: String },
    Remove { key: String },
}
