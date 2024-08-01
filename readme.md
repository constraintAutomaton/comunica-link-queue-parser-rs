# comunica-link-queue-parser-rs

A small CLI tool to parse the occupancy of the link queue of the [comunica link traversal query engine](https://github.com/comunica/comunica-feature-link-traversal).

## Dependencies
- [Rust toolkit](https://www.rust-lang.org/fr)

## Installation

```bash
cargo build --release
```
## Example output
```json
{
    "SELECT ?messageId ?messageCreationDate ?messageContent WHERE {\\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/hasCreator> <https://solidbench.linkeddatafragments.org/pods/00000000000000000933/profile/card#me>.\\n  ?message <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/Post>.\\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/content> ?messageContent.\\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/creationDate> ?messageCreationDate.\\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/id> ?messageId.\\n}": {
        "push": [
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
                "timestamp": 1718631765370,
                "parent": "https://solidbench.linkeddatafragments.org/pods/00000000000000000933/profile/card",
                "queue": {
                    "size": 1,
                    "push": {
                        "urn:comunica:default:extract-links/actors#predicates-solid": 1
                    },
                    "pop": {}
                }
            },
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
                "timestamp": 1718631765472,
                "parent": "https://solidbench.linkeddatafragments.org/pods/00000000000000000933/",
                "queue": {
                    "size": 1,
                    "push": {
                        "urn:comunica:default:extract-links/actors#predicates-solid": 1,
                        "urn:comunica:default:extract-links/actors#predicates-ldp": 1
                    },
                    "pop": {
                        "urn:comunica:default:extract-links/actors#predicates-solid": 1
                    }
                }
            },
            ...
            {
                "url": "https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/data/forum00000000755914244125",
                "producedByActor": {
                  "name": "urn:comunica:default:extract-links/actors#predicates-common",
                  "metadata": {
                    "predicates": [
                      "http://www.w3.org/2000/01/rdf-schema#seeAlso",
                      "http://www.w3.org/2002/07/owl##sameAs",
                      "http://xmlns.com/foaf/0.1/isPrimaryTopicOf"
                    ],
                    "matchingPredicate": "http://www.w3.org/2000/01/rdf-schema#seeAlso",
                    "checkSubject": false
                  }
                },
                "timestamp": 1718631766440,
                "parent": "https://solidbench.linkeddatafragments.org/pods/00000000000000000933/posts/2011-11-17",
                "queue": {
                  "size": 1,
                  "push": {
                    "urn:comunica:default:extract-links/actors#predicates-solid": 1,
                    "urn:comunica:default:extract-links/actors#predicates-ldp": 91,
                    "urn:comunica:default:extract-links/actors#predicates-common": 32
                  },
                  "pop": {
                    "urn:comunica:default:extract-links/actors#predicates-solid": 1,
                    "urn:comunica:default:extract-links/actors#predicates-ldp": 91,
                    "urn:comunica:default:extract-links/actors#predicates-common": 31
                  }
                }
              }
        ],
        "pop": [
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
                "timestamp": 1718631765370,
                "queue": {
                    "size": 0,
                    "push": {
                        "urn:comunica:default:extract-links/actors#predicates-solid": 1
                    },
                    "pop": {
                        "urn:comunica:default:extract-links/actors#predicates-solid": 1
                    }
                }
            },
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
                "timestamp": 1718631765473,
                "queue": {
                    "size": 4,
                    "push": {
                        "urn:comunica:default:extract-links/actors#predicates-solid": 1,
                        "urn:comunica:default:extract-links/actors#predicates-ldp": 5
                    },
                    "pop": {
                        "urn:comunica:default:extract-links/actors#predicates-solid": 1,
                        "urn:comunica:default:extract-links/actors#predicates-ldp": 1
                    }
                }
            },
            ...
            {
                "url": "https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/data/forum00000000755914244125",
                "producedByActor": {
                  "name": "urn:comunica:default:extract-links/actors#predicates-common",
                  "metadata": {
                    "predicates": [
                      "http://www.w3.org/2000/01/rdf-schema#seeAlso",
                      "http://www.w3.org/2002/07/owl##sameAs",
                      "http://xmlns.com/foaf/0.1/isPrimaryTopicOf"
                    ],
                    "matchingPredicate": "http://www.w3.org/2000/01/rdf-schema#seeAlso",
                    "checkSubject": false
                  }
                },
                "timestamp": 1718631766440,
                "queue": {
                  "size": 0,
                  "push": {
                    "urn:comunica:default:extract-links/actors#predicates-solid": 1,
                    "urn:comunica:default:extract-links/actors#predicates-ldp": 91,
                    "urn:comunica:default:extract-links/actors#predicates-common": 32
                  },
                  "pop": {
                    "urn:comunica:default:extract-links/actors#predicates-solid": 1,
                    "urn:comunica:default:extract-links/actors#predicates-ldp": 91,
                    "urn:comunica:default:extract-links/actors#predicates-common": 32
                  }
                }
              }
        ]
    }
}
```
## Usage
The binary is located at `./target/release/comunica-link-queue-parser-rs` and can be executed.

```
A small CLI tool to parse the occupancy of the link queue of the comunica link traversal query engine

Usage: comunica-link-queue-parser-rs [OPTIONS]

Options:
  -i, --input <INPUT>    Path of logging file of comunica By default the value is [default: ./info]
  -o, --output <OUTPUT>  Path of the output occupancy file By default the value is [default: ./occupancy.json]
  -p, --print            Print the occupancy object [default: false]
  -h, --help             Print help
  -V, --version          Print version
```