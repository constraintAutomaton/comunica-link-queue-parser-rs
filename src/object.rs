use serde::Serialize;
use serde_json::{self, Map};
use std::vec::Vec;

#[derive(Debug, Serialize, PartialEq)]
pub struct History {
    pub push: Vec<LinkEvent>,
    pub pop: Vec<LinkEvent>,
}
#[derive(Debug, Serialize, PartialEq)]
pub struct LinkEvent {
    pub link: Map<String, serde_json::Value>,
    pub queue: Map<String, serde_json::Value>,
}
