# comunica-link-queue-parser-rs

A small CLI tool to parse the occupancy of the link queue of the [comunica link traversal query engine](https://github.com/comunica/comunica-feature-link-traversal).

## Dependencies
- [Rust toolkit](https://www.rust-lang.org/fr)

## Installation

```bash
cargo build --release
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
## Example of an execution

- Compile the code (see the [installation section](#installation))

- Run a query using [Comunica feature link traversal](https://github.com/comunica/comunica-feature-link-traversal) with the [Comunica Wrapper Info Occupancy RDF Resolve Hypermedia Links Queue Actor](https://github.com/comunica/comunica-feature-link-traversal/tree/master/packages/actor-rdf-resolve-hypermedia-links-queue-wrapper-info-occupancy) and a [`@comunica/logger-bunyan`](https://github.com/comunica/comunica/tree/master/packages/logger-bunyan) and pipe the log into a file. The resulting file should look like the example below but much longer.

```json
{"name":"comunica","streamProviders":[{"level":"trace"}],"hostname":"bryanelliott-latitude5530","pid":17725,"level":30,"headers":{"accept":"application/n-quads,application/trig;q=0.95,application/ld+json;q=0.9,application/n-triples;q=0.8,text/turtle;q=0.6,application/rdf+xml;q=0.5,text/n3;q=0.35,application/xml;q=0.3,image/svg+xml;q=0.3,text/xml;q=0.3,text/html;q=0.2,application/xhtml+xml;q=0.18,application/json;q=0.135,text/shaclc;q=0.1,text/shaclc-ext;q=0.05","user-agent":"Comunica/actor-http-fetch (Node.js v20.13.1; linux)"},"method":"GET","actor":"urn:comunica:default:http/actors#fetch","msg":"Requesting https://solidbench.linkeddatafragments.org/pods/00000000000000000933/profile/card","time":"2024-07-05T12:06:08.501Z","v":0}
{"name":"comunica","streamProviders":[{"level":"trace"}],"hostname":"bryanelliott-latitude5530","pid":17725,"level":30,"actor":"urn:comunica:default:query-source-identify-hypermedia/actors#none","msg":"Identified as file source: https://solidbench.linkeddatafragments.org/pods/00000000000000000933/profile/card","time":"2024-07-05T12:06:08.624Z","v":0}
{"name":"comunica","streamProviders":[{"level":"trace"}],"hostname":"bryanelliott-latitude5530","pid":17725,"level":10,"data":{"type":"pushEvent","link":{"url":"https://solidbench.linkeddatafragments.org/pods/00000000000000000933/","producedByActor":{"name":"urn:comunica:default:extract-links/actors#predicates-solid","metadata":{"predicates":["http://www.w3.org/ns/pim/space#storage"],"matchingPredicate":"http://www.w3.org/ns/pim/space#storage","checkSubject":true}},"timestamp":2912.4356,"parent":"https://solidbench.linkeddatafragments.org/pods/00000000000000000933/profile/card"},"query":"SELECT ?messageId ?messageCreationDate ?messageContent WHERE {\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/hasCreator> <https://solidbench.linkeddatafragments.org/pods/00000000000000000933/profile/card#me>.\n  ?message <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/Post>.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/content> ?messageContent.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/creationDate> ?messageCreationDate.\n  ?message <https://solidbench.linkeddatafragments.org/www.ldbc.eu/ldbc_socialnet/1.0/vocabulary/id> ?messageId.\n}","queue":{"size":1,"pushEvents":{"urn:comunica:default:extract-links/actors#predicates-solid":1},"popEvents":{}}},"msg":"Link queue changed","time":"2024-07-05T12:06:08.654Z","v":0}
```

- Run `comunica-link-queue-parser-rs` (see [usage section](#usage))

```
./target/release/comunica-link-queue-parser-rs -i "path/to/the/comunica/log" -o "desired/output/path.json"
```

It should output something similar to the example below.

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