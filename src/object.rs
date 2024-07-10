use serde::Serialize;
use serde_json::{self, Map};
use std::vec::Vec;

#[derive(Debug, Serialize, PartialEq)]
pub struct History {
    pub push_events: Vec<LinkEvent>,
    pub pop_events: Vec<LinkEvent>,
}
#[derive(Debug, Serialize, PartialEq)]
pub struct LinkEvent {
    pub link: Map<String, serde_json::Value>,
    pub queue: Map<String, serde_json::Value>,
}
