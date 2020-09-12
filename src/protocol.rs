#![allow(missing_docs)]
use serde::{Deserialize, Serialize};
use failure::_core::fmt::Display;

/// Used to communicate between clients and server.
#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    Set { key: String, value: String },
    Get { key: String },
    Remove { key: String },
}

/// Used to communicate between clients and server.
#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    // impl Trait can not be written here.
    Set(Result<(), String>),
    Get(Result<Option<String>, String>),
    Remove(Result<(), String>)
}

impl Response {
    pub fn set(result: Result<(), impl Display>) -> Self {
        Response::Set(result.map_err(|e| e.to_string()))
    }

    pub fn get(result: Result<Option<String>, impl Display>) -> Self {
        Response::Get(result.map_err(|e| e.to_string()))
    }

    pub fn remove(result: Result<(), impl Display>) -> Self {
        Response::Remove(result.map_err(|e| e.to_string()))
    }
}
