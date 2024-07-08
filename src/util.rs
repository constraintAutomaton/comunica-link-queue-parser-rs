use std::collections::HashMap;

use crate::object::History;
use lazy_static::lazy_static;
use regex::Regex;
use serde_json;

use crate::object::LinkEvent;

pub fn process_line(
    line: &String,
    history: &mut HashMap<String, History>,
) -> Result<(), serde_json::Error> {
    lazy_static! {
        static ref RE_EVENT: Regex =
            Regex::new(r#".*?(?<jsonEvent>\{.+?"Link queue changed".*\})"#).unwrap();
        static ref RE_QUERY: Regex = Regex::new(r"(\r\n|\n|\r)").unwrap();
    };

    if let Some(caps) = RE_EVENT.captures(line) {
        let event: serde_json::Value = serde_json::from_str(&caps["jsonEvent"])?;
        let event = event.get("data").unwrap().as_object().unwrap();
        let one_line_query = RE_QUERY
            .replace_all(event.get("query").unwrap().as_str().unwrap(), " ")
            .to_string();

        let event_type = event.get("type").unwrap().as_str().unwrap();
        let link_event = LinkEvent {
            query: one_line_query.clone(),
            link: event.get("link").unwrap().as_object().unwrap().clone(),
            queue: event.get("queue").unwrap().as_object().unwrap().clone(),
        };
        let history_event = if history.contains_key(&one_line_query) {
            history.get_mut(&one_line_query).unwrap()
        } else {
            history.insert(
                one_line_query.clone(),
                History {
                    push_events: Vec::new(),
                    pop_events: Vec::new(),
                },
            );
            history.get_mut(&one_line_query).unwrap()
        };

        match event_type {
            "pushEvent" => history_event.push_events.push(link_event),
            "popEvent" => history_event.pop_events.push(link_event),
            _ => panic!("there is a link with the unknown event {event_type}"),
        }
    }
    Ok(())
}
