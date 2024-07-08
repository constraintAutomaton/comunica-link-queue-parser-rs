use serde::Serialize;
use serde_json::{self, Map};
use std::vec::Vec;

#[derive(Debug, Serialize)]
pub struct History {
    pub push_events: Vec<LinkEvent>,
    pub pop_events: Vec<LinkEvent>,
}
#[derive(Debug, Serialize)]
pub struct LinkEvent {
    pub link: Map<String, serde_json::Value>,
    pub query: String,
    pub queue: Map<String, serde_json::Value>,
}
