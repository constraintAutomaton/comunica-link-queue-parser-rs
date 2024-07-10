use std::collections::HashMap;

use crate::object::History;
use lazy_static::lazy_static;
use regex::Regex;
use serde_json;

use crate::object::LinkEvent;

/// convert a line into an [`HashMap<String, History>`] object
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
            link: event.get("link").unwrap().as_object().unwrap().clone(),
            queue: event.get("queue").unwrap().as_object().unwrap().clone(),
        };
        let history_event = if history.contains_key(&one_line_query) {
            history.get_mut(&one_line_query).unwrap()
        } else {
            history.insert(
                one_line_query.clone(),
                History {
                    push: Vec::new(),
                    pop: Vec::new(),
                },
            );
            history.get_mut(&one_line_query).unwrap()
        };

        match event_type {
            "push" => history_event.push.push(link_event),
            "pop" => history_event.pop.push(link_event),
            _ => panic!("there is a link with the unknown event {event_type}"),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref RE_EVENT: Regex =
            Regex::new(r#".*?(?<jsonEvent>\{.+?"Link queue changed".*\})"#).unwrap();
        static ref RE_QUERY: Regex = Regex::new(r"(\r\n|\n|\r)").unwrap();
    }

    #[test]
    fn should_not_process_an_empty_line() {
        let mut history: HashMap<String, History> = HashMap::new();
        let line = "".to_string();
        process_line(&line, &mut history).unwrap();
        assert_eq!(history.len(), 0);
    }

    #[test]
    fn should_not_process_an_unrelated_line() {
        let mut history: HashMap<String, History> = HashMap::new();
        let line = "adwiadawoidqwawio dwadiawjd{ }".to_string();
        process_line(&line, &mut history).unwrap();
        assert_eq!(history.len(), 0);
    }

    #[test]
    fn should_not_process_an_unrelated_json() {
        let mut history: HashMap<String, History> = HashMap::new();
        let line = r#"{"name":"comunica","streamProviders":[{"level":"trace"}],"hostname":"bryanelliott-latitude5530","pid":17725,"level":30,"headers":{"accept":"application/n-quads,application/trig;q=0.95,application/ld+json;q=0.9,application/n-triples;q=0.8,text/turtle;q=0.6,application/rdf+xml;q=0.5,text/n3;q=0.35,application/xml;q=0.3,image/svg+xml;q=0.3,text/xml;q=0.3,text/html;q=0.2,application/xhtml+xml;q=0.18,application/json;q=0.135,text/shaclc;q=0.1,text/shaclc-ext;q=0.05","user-agent":"Comunica/actor-http-fetch (Node.js v20.13.1; linux)"},"method":"GET","actor":"urn:comunica:default:http/actors#fetch","msg":"Requesting https://solidbench.linkeddatafragments.org/pods/00000000000000000933/profile/card","time":"2024-07-05T12:06:08.501Z","v":0}"#.to_string();
        process_line(&line, &mut history).unwrap();
        assert_eq!(history.len(), 0);
    }

    #[test]
    fn should_process_a_push_event() {
        let query = "SELECT ?messageId ?messageCreationDate ?messageContent WHERE {\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/hasCreator> <https://solidbench.linkeddatafragments.org/pods/00000000000000000933/profile/card#me>.\n  ?message <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/Post>.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/content> ?messageContent.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/creationDate> ?messageCreationDate.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/id> ?messageId.\n}";
        let query = RE_QUERY.replace_all(query, " ").to_string();

        let link:serde_json::Value = serde_json::from_str(
            r#"
            {
        "url": "https://solidbench.linkeddatafragments.org/pods/00000000000000000933/",
        "producedByActor": {
            "name": "urn:comunica:default:extract-links/actors#predicates-solid",
            "metadata": {
                "predicates": [
                    "http://www.w3.org/ns/pim/space#storage"
                ],
                "matchingPredicate": "http://www.w3.org/ns/pim/space#storage",
                "checkSubject": true
            }
        },
        "timestamp": 2912.4356,
        "parent": "https://solidbench.linkeddatafragments.org/pods/00000000000000000933/profile/card"
    }"#
        ).unwrap();

        let queue: serde_json::Value = serde_json::from_str(
            r#"
             {
        "size": 1,
        "push": {
            "urn:comunica:default:extract-links/actors#predicates-solid": 1
        },
        "pop": {}
    }"#,
        )
        .unwrap();

        let link_event = LinkEvent {
            link: link.as_object().unwrap().clone(),
            queue: queue.as_object().unwrap().clone(),
        };

        let expected_history = History {
            push: vec![link_event],
            pop: Vec::new(),
        };

        let expected_history_by_query: HashMap<String, History> =
            HashMap::from([(query.clone(), expected_history)]);

        let mut history: HashMap<String, History> = HashMap::new();
        let line = r#"{"name":"comunica","streamProviders":[{"level":"trace"}],"hostname":"bryanelliott-latitude5530","pid":17725,"level":10,"data":{"type":"push","link":{"url":"https://solidbench.linkeddatafragments.org/pods/00000000000000000933/","producedByActor":{"name":"urn:comunica:default:extract-links/actors#predicates-solid","metadata":{"predicates":["http://www.w3.org/ns/pim/space#storage"],"matchingPredicate":"http://www.w3.org/ns/pim/space#storage","checkSubject":true}},"timestamp":2912.4356,"parent":"https://solidbench.linkeddatafragments.org/pods/00000000000000000933/profile/card"},"query":"SELECT ?messageId ?messageCreationDate ?messageContent WHERE {\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/hasCreator> <https://solidbench.linkeddatafragments.org/pods/00000000000000000933/profile/card#me>.\n  ?message <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/Post>.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/content> ?messageContent.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/creationDate> ?messageCreationDate.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/id> ?messageId.\n}","queue":{"size":1,"push":{"urn:comunica:default:extract-links/actors#predicates-solid":1},"pop":{}}},"msg":"Link queue changed","time":"2024-07-05T12:06:08.654Z","v":0}"#.to_string();

        process_line(&line, &mut history).unwrap();

        assert_eq!(history.len(), 1);
        assert!(history.get(&query).is_some());
        assert_eq!(
            history, expected_history_by_query,
            "got \n{:?} expected\n {:?}",
            history, expected_history_by_query
        );
    }

    #[test]
    fn should_process_a_pop_event() {
        let query = "SELECT ?messageId ?messageCreationDate ?messageContent WHERE {\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/hasCreator> <https://solidbench.linkeddatafragments.org/pods/00000000000000000933/profile/card#me>.\n  ?message <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/Post>.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/content> ?messageContent.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/creationDate> ?messageCreationDate.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/id> ?messageId.\n}";
        let query = RE_QUERY.replace_all(query, " ").to_string();

        let link: serde_json::Value = serde_json::from_str(
            r#"{
            "url": "https://solidbench.linkeddatafragments.org/pods/00000000000000000933/",
            "producedByActor": {
                "name": "urn:comunica:default:extract-links/actors#predicates-solid",
                "metadata": {
                    "predicates": [
                        "http://www.w3.org/ns/pim/space#storage"
                    ],
                    "matchingPredicate": "http://www.w3.org/ns/pim/space#storage",
                    "checkSubject": true
                }
            },
            "timestamp": 2913.561066
        }"#,
        )
        .unwrap();

        let queue: serde_json::Value = serde_json::from_str(
            r#"{
            "size": 0,
            "push": {
                "urn:comunica:default:extract-links/actors#predicates-solid": 1
            },
            "pop": {
                "urn:comunica:default:extract-links/actors#predicates-solid": 1
            }
        }"#,
        )
        .unwrap();

        let link_event = LinkEvent {
            link: link.as_object().unwrap().clone(),
            queue: queue.as_object().unwrap().clone(),
        };

        let expected_history = History {
            push: Vec::new(),
            pop: vec![link_event],
        };

        let expected_history_by_query: HashMap<String, History> =
            HashMap::from([(query.clone(), expected_history)]);

        let mut history: HashMap<String, History> = HashMap::new();
        let line = r#"{"name":"comunica","streamProviders":[{"level":"trace"}],"hostname":"bryanelliott-latitude5530","pid":17725,"level":10,"data":{"type":"pop","link":{"url":"https://solidbench.linkeddatafragments.org/pods/00000000000000000933/","producedByActor":{"name":"urn:comunica:default:extract-links/actors#predicates-solid","metadata":{"predicates":["http://www.w3.org/ns/pim/space#storage"],"matchingPredicate":"http://www.w3.org/ns/pim/space#storage","checkSubject":true}},"timestamp":2913.561066},"query":"SELECT ?messageId ?messageCreationDate ?messageContent WHERE {\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/hasCreator> <https://solidbench.linkeddatafragments.org/pods/00000000000000000933/profile/card#me>.\n  ?message <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/Post>.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/content> ?messageContent.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/creationDate> ?messageCreationDate.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/id> ?messageId.\n}","queue":{"size":0,"push":{"urn:comunica:default:extract-links/actors#predicates-solid":1},"pop":{"urn:comunica:default:extract-links/actors#predicates-solid":1}}},"msg":"Link queue changed","time":"2024-07-05T12:06:08.655Z","v":0}"#.to_string();

        process_line(&line, &mut history).unwrap();

        assert_eq!(history.len(), 1);
        assert!(history.get(&query).is_some());
        assert_eq!(
            history, expected_history_by_query,
            "got \n\n{:?} expected\n\n {:?}",
            history, expected_history_by_query
        );
    }

    #[test]
    fn should_process_multiple_queries() {
        let query = "SELECT ?messageId ?messageCreationDate ?messageContent WHERE {\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/hasCreator> <https://solidbench.linkeddatafragments.org/pods/00000000000000000933/profile/card#me>.\n  ?message <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/Post>.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/content> ?messageContent.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/creationDate> ?messageCreationDate.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/id> ?messageId.\n}";
        let query = RE_QUERY.replace_all(query, " ").to_string();

        let other_query = "SELECT ?messageId1 ?messageCreationDate ?messageContent WHERE {\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/hasCreator> <https://solidbench.linkeddatafragments.org/pods/00000000000000000933/profile/card#me>.\n  ?message <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/Post>.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/content> ?messageContent.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/creationDate> ?messageCreationDate.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/id> ?messageId1.\n}";
        let other_query = RE_QUERY.replace_all(other_query, " ").to_string();

        let line_push_1 = r#"{"name":"comunica","streamProviders":[{"level":"trace"}],"hostname":"bryanelliott-latitude5530","pid":17725,"level":10,"data":{"type":"push","link":{"url":"https://solidbench.linkeddatafragments.org/pods/00000000000000000933/","producedByActor":{"name":"urn:comunica:default:extract-links/actors#predicates-solid","metadata":{"predicates":["http://www.w3.org/ns/pim/space#storage"],"matchingPredicate":"http://www.w3.org/ns/pim/space#storage","checkSubject":true}},"timestamp":2912.4356,"parent":"https://solidbench.linkeddatafragments.org/pods/00000000000000000933/profile/card"},"query":"SELECT ?messageId1 ?messageCreationDate ?messageContent WHERE {\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/hasCreator> <https://solidbench.linkeddatafragments.org/pods/00000000000000000933/profile/card#me>.\n  ?message <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/Post>.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/content> ?messageContent.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/creationDate> ?messageCreationDate.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/id> ?messageId1.\n}","queue":{"size":1,"push":{"urn:comunica:default:extract-links/actors#predicates-solid":1},"pop":{}}},"msg":"Link queue changed","time":"2024-07-05T12:06:08.654Z","v":0}"#.to_string();
        let line_push_2 = r#"{"name":"comunica","streamProviders":[{"level":"trace"}],"hostname":"bryanelliott-latitude5530","pid":17725,"level":10,"data":{"type":"push","link":{"url":"https://solidbench.linkeddatafragments.org/pods/00000000000000000933/profile/","producedByActor":{"name":"urn:comunica:default:extract-links/actors#predicates-ldp","metadata":{"predicates":["http://www.w3.org/ns/ldp#contains"],"matchingPredicate":"http://www.w3.org/ns/ldp#contains","checkSubject":true}},"timestamp":3003.617209,"parent":"https://solidbench.linkeddatafragments.org/pods/00000000000000000933/"},"query":"SELECT ?messageId ?messageCreationDate ?messageContent WHERE {\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/hasCreator> <https://solidbench.linkeddatafragments.org/pods/00000000000000000933/profile/card#me>.\n  ?message <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/Post>.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/content> ?messageContent.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/creationDate> ?messageCreationDate.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/id> ?messageId.\n}","queue":{"size":1,"push":{"urn:comunica:default:extract-links/actors#predicates-solid":1,"urn:comunica:default:extract-links/actors#predicates-ldp":1},"pop":{"urn:comunica:default:extract-links/actors#predicates-solid":1}}},"msg":"Link queue changed","time":"2024-07-05T12:06:08.745Z","v":0}"#.to_string();
        let line_pop_1 = r#"{"name":"comunica","streamProviders":[{"level":"trace"}],"hostname":"bryanelliott-latitude5530","pid":17725,"level":10,"data":{"type":"pop","link":{"url":"https://solidbench.linkeddatafragments.org/pods/00000000000000000933/","producedByActor":{"name":"urn:comunica:default:extract-links/actors#predicates-solid","metadata":{"predicates":["http://www.w3.org/ns/pim/space#storage"],"matchingPredicate":"http://www.w3.org/ns/pim/space#storage","checkSubject":true}},"timestamp":2913.561066},"query":"SELECT ?messageId ?messageCreationDate ?messageContent WHERE {\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/hasCreator> <https://solidbench.linkeddatafragments.org/pods/00000000000000000933/profile/card#me>.\n  ?message <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/Post>.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/content> ?messageContent.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/creationDate> ?messageCreationDate.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/id> ?messageId.\n}","queue":{"size":0,"push":{"urn:comunica:default:extract-links/actors#predicates-solid":1},"pop":{"urn:comunica:default:extract-links/actors#predicates-solid":1}}},"msg":"Link queue changed","time":"2024-07-05T12:06:08.655Z","v":0}"#.to_string();
        let line_pop_2 = r#"{"name":"comunica","streamProviders":[{"level":"trace"}],"hostname":"bryanelliott-latitude5530","pid":17725,"level":10,"data":{"type":"pop","link":{"url":"https://solidbench.linkeddatafragments.org/pods/00000000000000000933/profile/","producedByActor":{"name":"urn:comunica:default:extract-links/actors#predicates-ldp","metadata":{"predicates":["http://www.w3.org/ns/ldp#contains"],"matchingPredicate":"http://www.w3.org/ns/ldp#contains","checkSubject":true}},"timestamp":3004.183666},"query":"SELECT ?messageId ?messageCreationDate ?messageContent WHERE {\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/hasCreator> <https://solidbench.linkeddatafragments.org/pods/00000000000000000933/profile/card#me>.\n  ?message <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/Post>.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/content> ?messageContent.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/creationDate> ?messageCreationDate.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/id> ?messageId.\n}","queue":{"size":4,"push":{"urn:comunica:default:extract-links/actors#predicates-solid":1,"urn:comunica:default:extract-links/actors#predicates-ldp":5},"pop":{"urn:comunica:default:extract-links/actors#predicates-solid":1,"urn:comunica:default:extract-links/actors#predicates-ldp":1}}},"msg":"Link queue changed","time":"2024-07-05T12:06:08.746Z","v":0}"#.to_string();

        let link_event_push_1 = {
            let link:serde_json::Value = serde_json::from_str(
                r#"
                {
            "url": "https://solidbench.linkeddatafragments.org/pods/00000000000000000933/",
            "producedByActor": {
                "name": "urn:comunica:default:extract-links/actors#predicates-solid",
                "metadata": {
                    "predicates": [
                        "http://www.w3.org/ns/pim/space#storage"
                    ],
                    "matchingPredicate": "http://www.w3.org/ns/pim/space#storage",
                    "checkSubject": true
                }
            },
            "timestamp": 2912.4356,
            "parent": "https://solidbench.linkeddatafragments.org/pods/00000000000000000933/profile/card"
        }"#
            ).unwrap();

            let queue: serde_json::Value = serde_json::from_str(
                r#"
                 {
            "size": 1,
            "push": {
                "urn:comunica:default:extract-links/actors#predicates-solid": 1
            },
            "pop": {}
        }"#,
            )
            .unwrap();

            LinkEvent {
                link: link.as_object().unwrap().clone(),
                queue: queue.as_object().unwrap().clone(),
            }
        };
        let link_event_push_2 = {
            let link: serde_json::Value = serde_json::from_str(
                r#"
                {
            "url": "https://solidbench.linkeddatafragments.org/pods/00000000000000000933/profile/",
            "producedByActor": {
                "name": "urn:comunica:default:extract-links/actors#predicates-ldp",
                "metadata": {
                    "predicates": [
                        "http://www.w3.org/ns/ldp#contains"
                    ],
                    "matchingPredicate": "http://www.w3.org/ns/ldp#contains",
                    "checkSubject": true
                }
            },
            "timestamp": 3003.617209,
            "parent": "https://solidbench.linkeddatafragments.org/pods/00000000000000000933/"
        }"#,
            )
            .unwrap();

            let queue: serde_json::Value = serde_json::from_str(
                r#"{
            "size": 1,
            "push": {
                "urn:comunica:default:extract-links/actors#predicates-solid": 1,
                "urn:comunica:default:extract-links/actors#predicates-ldp": 1
            },
            "pop": {
                "urn:comunica:default:extract-links/actors#predicates-solid": 1
            }
        }"#,
            )
            .unwrap();

            LinkEvent {
                link: link.as_object().unwrap().clone(),
                queue: queue.as_object().unwrap().clone(),
            }
        };

        let link_event_pop_1 = {
            let link: serde_json::Value = serde_json::from_str(
                r#"{
                "url": "https://solidbench.linkeddatafragments.org/pods/00000000000000000933/",
                "producedByActor": {
                    "name": "urn:comunica:default:extract-links/actors#predicates-solid",
                    "metadata": {
                        "predicates": [
                            "http://www.w3.org/ns/pim/space#storage"
                        ],
                        "matchingPredicate": "http://www.w3.org/ns/pim/space#storage",
                        "checkSubject": true
                    }
                },
                "timestamp": 2913.561066
            }"#,
            )
            .unwrap();

            let queue: serde_json::Value = serde_json::from_str(
                r#"{
                "size": 0,
                "push": {
                    "urn:comunica:default:extract-links/actors#predicates-solid": 1
                },
                "pop": {
                    "urn:comunica:default:extract-links/actors#predicates-solid": 1
                }
            }"#,
            )
            .unwrap();

            LinkEvent {
                link: link.as_object().unwrap().clone(),
                queue: queue.as_object().unwrap().clone(),
            }
        };
        let link_event_pop_2 = {
            let link: serde_json::Value = serde_json::from_str(
                r#"{
            "url": "https://solidbench.linkeddatafragments.org/pods/00000000000000000933/profile/",
            "producedByActor": {
                "name": "urn:comunica:default:extract-links/actors#predicates-ldp",
                "metadata": {
                    "predicates": [
                        "http://www.w3.org/ns/ldp#contains"
                    ],
                    "matchingPredicate": "http://www.w3.org/ns/ldp#contains",
                    "checkSubject": true
                }
            },
            "timestamp": 3004.183666
        }"#,
            )
            .unwrap();

            let queue: serde_json::Value = serde_json::from_str(
                r#"{
            "size": 4,
            "push": {
                "urn:comunica:default:extract-links/actors#predicates-solid": 1,
                "urn:comunica:default:extract-links/actors#predicates-ldp": 5
            },
            "pop": {
                "urn:comunica:default:extract-links/actors#predicates-solid": 1,
                "urn:comunica:default:extract-links/actors#predicates-ldp": 1
            }
        }"#,
            )
            .unwrap();

            LinkEvent {
                link: link.as_object().unwrap().clone(),
                queue: queue.as_object().unwrap().clone(),
            }
        };

        let expected_history = History {
            push: vec![link_event_push_2],
            pop: vec![link_event_pop_1, link_event_pop_2],
        };

        let expected_history_other_query = History {
            push: vec![link_event_push_1],
            pop: vec![],
        };
        let expected_history_by_query: HashMap<String, History> =
            HashMap::from([(query.clone(), expected_history), (other_query.clone(), expected_history_other_query)]);

        let mut history: HashMap<String, History> = HashMap::new();

        process_line(&line_push_1, &mut history).unwrap();
        process_line(&line_push_2, &mut history).unwrap();
        process_line(&line_pop_1, &mut history).unwrap();
        process_line(&line_pop_2, &mut history).unwrap();

        assert_eq!(history.len(), 2);
        assert!(history.get(&query).is_some());
        assert!(history.get(&other_query).is_some());
        assert_eq!(
            history, expected_history_by_query,
            "got \n\n{:?} expected\n\n {:?}",
            history, expected_history_by_query
        );
    }

    #[test]
    fn should_process_multiple_event() {
        let query = "SELECT ?messageId ?messageCreationDate ?messageContent WHERE {\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/hasCreator> <https://solidbench.linkeddatafragments.org/pods/00000000000000000933/profile/card#me>.\n  ?message <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/Post>.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/content> ?messageContent.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/creationDate> ?messageCreationDate.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/id> ?messageId.\n}";
        let query = RE_QUERY.replace_all(query, " ").to_string();

        let line_push_1 = r#"{"name":"comunica","streamProviders":[{"level":"trace"}],"hostname":"bryanelliott-latitude5530","pid":17725,"level":10,"data":{"type":"push","link":{"url":"https://solidbench.linkeddatafragments.org/pods/00000000000000000933/","producedByActor":{"name":"urn:comunica:default:extract-links/actors#predicates-solid","metadata":{"predicates":["http://www.w3.org/ns/pim/space#storage"],"matchingPredicate":"http://www.w3.org/ns/pim/space#storage","checkSubject":true}},"timestamp":2912.4356,"parent":"https://solidbench.linkeddatafragments.org/pods/00000000000000000933/profile/card"},"query":"SELECT ?messageId ?messageCreationDate ?messageContent WHERE {\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/hasCreator> <https://solidbench.linkeddatafragments.org/pods/00000000000000000933/profile/card#me>.\n  ?message <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/Post>.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/content> ?messageContent.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/creationDate> ?messageCreationDate.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/id> ?messageId.\n}","queue":{"size":1,"push":{"urn:comunica:default:extract-links/actors#predicates-solid":1},"pop":{}}},"msg":"Link queue changed","time":"2024-07-05T12:06:08.654Z","v":0}"#.to_string();
        let line_push_2 = r#"{"name":"comunica","streamProviders":[{"level":"trace"}],"hostname":"bryanelliott-latitude5530","pid":17725,"level":10,"data":{"type":"push","link":{"url":"https://solidbench.linkeddatafragments.org/pods/00000000000000000933/profile/","producedByActor":{"name":"urn:comunica:default:extract-links/actors#predicates-ldp","metadata":{"predicates":["http://www.w3.org/ns/ldp#contains"],"matchingPredicate":"http://www.w3.org/ns/ldp#contains","checkSubject":true}},"timestamp":3003.617209,"parent":"https://solidbench.linkeddatafragments.org/pods/00000000000000000933/"},"query":"SELECT ?messageId ?messageCreationDate ?messageContent WHERE {\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/hasCreator> <https://solidbench.linkeddatafragments.org/pods/00000000000000000933/profile/card#me>.\n  ?message <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/Post>.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/content> ?messageContent.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/creationDate> ?messageCreationDate.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/id> ?messageId.\n}","queue":{"size":1,"push":{"urn:comunica:default:extract-links/actors#predicates-solid":1,"urn:comunica:default:extract-links/actors#predicates-ldp":1},"pop":{"urn:comunica:default:extract-links/actors#predicates-solid":1}}},"msg":"Link queue changed","time":"2024-07-05T12:06:08.745Z","v":0}"#.to_string();
        let line_pop_1 = r#"{"name":"comunica","streamProviders":[{"level":"trace"}],"hostname":"bryanelliott-latitude5530","pid":17725,"level":10,"data":{"type":"pop","link":{"url":"https://solidbench.linkeddatafragments.org/pods/00000000000000000933/","producedByActor":{"name":"urn:comunica:default:extract-links/actors#predicates-solid","metadata":{"predicates":["http://www.w3.org/ns/pim/space#storage"],"matchingPredicate":"http://www.w3.org/ns/pim/space#storage","checkSubject":true}},"timestamp":2913.561066},"query":"SELECT ?messageId ?messageCreationDate ?messageContent WHERE {\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/hasCreator> <https://solidbench.linkeddatafragments.org/pods/00000000000000000933/profile/card#me>.\n  ?message <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/Post>.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/content> ?messageContent.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/creationDate> ?messageCreationDate.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/id> ?messageId.\n}","queue":{"size":0,"push":{"urn:comunica:default:extract-links/actors#predicates-solid":1},"pop":{"urn:comunica:default:extract-links/actors#predicates-solid":1}}},"msg":"Link queue changed","time":"2024-07-05T12:06:08.655Z","v":0}"#.to_string();
        let line_pop_2 = r#"{"name":"comunica","streamProviders":[{"level":"trace"}],"hostname":"bryanelliott-latitude5530","pid":17725,"level":10,"data":{"type":"pop","link":{"url":"https://solidbench.linkeddatafragments.org/pods/00000000000000000933/profile/","producedByActor":{"name":"urn:comunica:default:extract-links/actors#predicates-ldp","metadata":{"predicates":["http://www.w3.org/ns/ldp#contains"],"matchingPredicate":"http://www.w3.org/ns/ldp#contains","checkSubject":true}},"timestamp":3004.183666},"query":"SELECT ?messageId ?messageCreationDate ?messageContent WHERE {\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/hasCreator> <https://solidbench.linkeddatafragments.org/pods/00000000000000000933/profile/card#me>.\n  ?message <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/Post>.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/content> ?messageContent.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/creationDate> ?messageCreationDate.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/id> ?messageId.\n}","queue":{"size":4,"push":{"urn:comunica:default:extract-links/actors#predicates-solid":1,"urn:comunica:default:extract-links/actors#predicates-ldp":5},"pop":{"urn:comunica:default:extract-links/actors#predicates-solid":1,"urn:comunica:default:extract-links/actors#predicates-ldp":1}}},"msg":"Link queue changed","time":"2024-07-05T12:06:08.746Z","v":0}"#.to_string();

        let link_event_push_1 = {
            let link:serde_json::Value = serde_json::from_str(
                r#"
                {
            "url": "https://solidbench.linkeddatafragments.org/pods/00000000000000000933/",
            "producedByActor": {
                "name": "urn:comunica:default:extract-links/actors#predicates-solid",
                "metadata": {
                    "predicates": [
                        "http://www.w3.org/ns/pim/space#storage"
                    ],
                    "matchingPredicate": "http://www.w3.org/ns/pim/space#storage",
                    "checkSubject": true
                }
            },
            "timestamp": 2912.4356,
            "parent": "https://solidbench.linkeddatafragments.org/pods/00000000000000000933/profile/card"
        }"#
            ).unwrap();

            let queue: serde_json::Value = serde_json::from_str(
                r#"
                 {
            "size": 1,
            "push": {
                "urn:comunica:default:extract-links/actors#predicates-solid": 1
            },
            "pop": {}
        }"#,
            )
            .unwrap();

            LinkEvent {
                link: link.as_object().unwrap().clone(),
                queue: queue.as_object().unwrap().clone(),
            }
        };
        let link_event_push_2 = {
            let link: serde_json::Value = serde_json::from_str(
                r#"
                {
            "url": "https://solidbench.linkeddatafragments.org/pods/00000000000000000933/profile/",
            "producedByActor": {
                "name": "urn:comunica:default:extract-links/actors#predicates-ldp",
                "metadata": {
                    "predicates": [
                        "http://www.w3.org/ns/ldp#contains"
                    ],
                    "matchingPredicate": "http://www.w3.org/ns/ldp#contains",
                    "checkSubject": true
                }
            },
            "timestamp": 3003.617209,
            "parent": "https://solidbench.linkeddatafragments.org/pods/00000000000000000933/"
        }"#,
            )
            .unwrap();

            let queue: serde_json::Value = serde_json::from_str(
                r#"{
            "size": 1,
            "push": {
                "urn:comunica:default:extract-links/actors#predicates-solid": 1,
                "urn:comunica:default:extract-links/actors#predicates-ldp": 1
            },
            "pop": {
                "urn:comunica:default:extract-links/actors#predicates-solid": 1
            }
        }"#,
            )
            .unwrap();

            LinkEvent {
                link: link.as_object().unwrap().clone(),
                queue: queue.as_object().unwrap().clone(),
            }
        };

        let link_event_pop_1 = {
            let link: serde_json::Value = serde_json::from_str(
                r#"{
                "url": "https://solidbench.linkeddatafragments.org/pods/00000000000000000933/",
                "producedByActor": {
                    "name": "urn:comunica:default:extract-links/actors#predicates-solid",
                    "metadata": {
                        "predicates": [
                            "http://www.w3.org/ns/pim/space#storage"
                        ],
                        "matchingPredicate": "http://www.w3.org/ns/pim/space#storage",
                        "checkSubject": true
                    }
                },
                "timestamp": 2913.561066
            }"#,
            )
            .unwrap();

            let queue: serde_json::Value = serde_json::from_str(
                r#"{
                "size": 0,
                "push": {
                    "urn:comunica:default:extract-links/actors#predicates-solid": 1
                },
                "pop": {
                    "urn:comunica:default:extract-links/actors#predicates-solid": 1
                }
            }"#,
            )
            .unwrap();

            LinkEvent {
                link: link.as_object().unwrap().clone(),
                queue: queue.as_object().unwrap().clone(),
            }
        };
        let link_event_pop_2 = {
            let link: serde_json::Value = serde_json::from_str(
                r#"{
            "url": "https://solidbench.linkeddatafragments.org/pods/00000000000000000933/profile/",
            "producedByActor": {
                "name": "urn:comunica:default:extract-links/actors#predicates-ldp",
                "metadata": {
                    "predicates": [
                        "http://www.w3.org/ns/ldp#contains"
                    ],
                    "matchingPredicate": "http://www.w3.org/ns/ldp#contains",
                    "checkSubject": true
                }
            },
            "timestamp": 3004.183666
        }"#,
            )
            .unwrap();

            let queue: serde_json::Value = serde_json::from_str(
                r#"{
            "size": 4,
            "push": {
                "urn:comunica:default:extract-links/actors#predicates-solid": 1,
                "urn:comunica:default:extract-links/actors#predicates-ldp": 5
            },
            "pop": {
                "urn:comunica:default:extract-links/actors#predicates-solid": 1,
                "urn:comunica:default:extract-links/actors#predicates-ldp": 1
            }
        }"#,
            )
            .unwrap();

            LinkEvent {
                link: link.as_object().unwrap().clone(),
                queue: queue.as_object().unwrap().clone(),
            }
        };

        let expected_history = History {
            push: vec![link_event_push_1, link_event_push_2],
            pop: vec![link_event_pop_1, link_event_pop_2],
        };
        let expected_history_by_query: HashMap<String, History> =
            HashMap::from([(query.clone(), expected_history)]);

        let mut history: HashMap<String, History> = HashMap::new();

        process_line(&line_push_1, &mut history).unwrap();
        process_line(&line_push_2, &mut history).unwrap();
        process_line(&line_pop_1, &mut history).unwrap();
        process_line(&line_pop_2, &mut history).unwrap();

        assert_eq!(history.len(), 1);
        assert!(history.get(&query).is_some());
        assert_eq!(
            history, expected_history_by_query,
            "got \n\n{:?} expected\n\n {:?}",
            history, expected_history_by_query
        );
    }
}
