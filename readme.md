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

- Run a query using [Comunica feature link traversal](https://github.com/comunica/comunica-feature-link-traversal) with the [Comunica Wrapper Info Occupancy RDF Resolve Hypermedia Links Queue Actor](https://github.com/comunica/comunica-feature-link-traversal/tree/master/packages/actor-rdf-resolve-hypermedia-links-queue-wrapper-info-occupancy) and a [`@comunica/logger-bunyan`](https://github.com/comunica/comunica/tree/master/packages/logger-bunyan) and pipe the log into a file.

```js
import { LoggerBunyan, BunyanStreamProviderStdout } from '@comunica/logger-bunyan';
import { QueryEngineFactory } from '@comunica/query-sparql-link-traversal-solid';

const query = `
PREFIX foat: <http://xmlns.com/foaf/0.1/>
PREFIX ruben: <https://ruben.verborgh.org/profile/#>
PREFIX rubent: <https://www.rubensworks.net/#>
SELECT DISTINCT * WHERE {
    rubent:me foaf:knows ?person.
    ruben:me foaf:knows ?person.
    ?person foaf:name ?name.
}`;

const source = 'https://www.rubensworks.net/';
const configPath = './config.json';
const engine = await new QueryEngineFactory().create({ configPath });

const streamProvider = new BunyanStreamProviderStdout({ level: 'trace' });
const loggerParams = {
  name: 'comunica',
  level: 'trace',
  streamProviders: [ streamProvider ],
};
const logger = new LoggerBunyan(loggerParams);

const bindingsStream = await engine.queryBindings(query, {
  sources:[source],
  lenient: true,
  log: logger,
});

const res = await bindingsStream.toArray();
```
where the `./config` is defined as follow.

```json
{
    "@context": [
      "https://linkedsoftwaredependencies.org/bundles/npm/@comunica/config-query-sparql/^3.0.0/components/context.jsonld",
      "https://linkedsoftwaredependencies.org/bundles/npm/@comunica/config-query-sparql-link-traversal/^0.0.0/components/context.jsonld"
    ],
    "import": [
      "ccqslt:config/config-base.json",
      "ccqslt:config/extract-links/actors/content-policies-conditional.json",
      "ccqslt:config/extract-links/actors/quad-pattern-query.json",
      "ccqslt:config/rdf-resolve-hypermedia-links/actors/traverse-replace-conditional.json",
      "ccqslt:config/rdf-resolve-hypermedia-links-queue/actors/wrapper-limit-count.json",
      "ccqslt:config/rdf-resolve-hypermedia-links-queue/actors/wrapper-info-occupancy.json"
    ]
  }
  
```

 The resulting file should look like the example below but much longer.

```json
{"name":"comunica","streamProviders":[{"level":"trace"}],"hostname":"bryanelliott-latitude5530","pid":22297,"level":30,"headers":{"accept":"application/n-quads,application/trig;q=0.95,application/ld+json;q=0.9,application/n-triples;q=0.8,text/turtle;q=0.6,application/rdf+xml;q=0.5,text/n3;q=0.35,application/xml;q=0.3,image/svg+xml;q=0.3,text/xml;q=0.3,text/html;q=0.2,application/xhtml+xml;q=0.18,application/json;q=0.135,text/shaclc;q=0.1,text/shaclc-ext;q=0.05","user-agent":"Comunica/actor-http-fetch (Node.js v20.13.1; linux)"},"method":"GET","actor":"urn:comunica:default:http/actors#fetch","msg":"Requesting https://www.rubensworks.net/","time":"2024-08-07T08:21:44.158Z","v":0}
{"name":"comunica","streamProviders":[{"level":"trace"}],"hostname":"bryanelliott-latitude5530","pid":22297,"level":30,"actor":"urn:comunica:default:query-source-identify-hypermedia/actors#none","msg":"Identified as file source: https://www.rubensworks.net/","time":"2024-08-07T08:21:44.408Z","v":0}
{"name":"comunica","streamProviders":[{"level":"trace"}],"hostname":"bryanelliott-latitude5530","pid":22297,"level":20,"entries":3,"variables":[["person"],["person"],["person","name"]],"costs":{"inner-bind":2423100.896815718,"inner-multi-smallest":59100},"coefficients":{"inner-bind":{"iterations":0.08968157181571816,"persistedItems":0,"blockingItems":0,"requestTime":24231},"inner-multi-smallest":{"iterations":0,"persistedItems":0,"blockingItems":0,"requestTime":591}},"msg":"Determined physical join operator 'inner-multi-smallest'","time":"2024-08-07T08:21:44.417Z","v":0}
{"name":"comunica","streamProviders":[{"level":"trace"}],"hostname":"bryanelliott-latitude5530","pid":22297,"level":20,"entries":2,"variables":[["person"],["person"]],"costs":{"inner-hash":40193,"inner-symmetric-hash":40071,"inner-nested-loop":39400,"inner-bind":1221400},"coefficients":{"inner-hash":{"iterations":61,"persistedItems":61,"blockingItems":61,"requestTime":394},"inner-symmetric-hash":{"iterations":61,"persistedItems":61,"blockingItems":0,"requestTime":394},"inner-nested-loop":{"iterations":0,"persistedItems":0,"blockingItems":0,"requestTime":394},"inner-bind":{"iterations":0,"persistedItems":0,"blockingItems":0,"requestTime":12214}},"msg":"Determined physical join operator 'inner-nested-loop'","time":"2024-08-07T08:21:44.418Z","v":0}
{"name":"comunica","streamProviders":[{"level":"trace"}],"hostname":"bryanelliott-latitude5530","pid":22297,"level":20,"entries":2,"variables":[["person","name"],["person"]],"costs":{"inner-hash":20506,"inner-symmetric-hash":20382,"inner-nested-loop":19700,"inner-bind":19700},"coefficients":{"inner-hash":{"iterations":62,"persistedItems":62,"blockingItems":62,"requestTime":197},"inner-symmetric-hash":{"iterations":62,"persistedItems":62,"blockingItems":0,"requestTime":197},"inner-nested-loop":{"iterations":0,"persistedItems":0,"blockingItems":0,"requestTime":197},"inner-bind":{"iterations":0,"persistedItems":0,"blockingItems":0,"requestTime":197}},"msg":"Determined physical join operator 'inner-nested-loop'","time":"2024-08-07T08:21:44.419Z","v":0}
{"name":"comunica","streamProviders":[{"level":"trace"}],"hostname":"bryanelliott-latitude5530","pid":22297,"level":10,"data":{"type":"push","link":{"url":"https://data.verborgh.org/people/anastasia_dimou","producedByActor":{"name":"urn:comunica:default:extract-links/actors#quad-pattern-query","metadata":{"onlyVariables":true}},"timestamp":2553.299712,"parent":"https://www.rubensworks.net/"},"query":"SELECT DISTINCT ?name ?person WHERE {\n  <https://www.rubensworks.net/#me> <http://xmlns.com/foaf/0.1/knows> ?person.\n  <https://ruben.verborgh.org/profile/#me> <http://xmlns.com/foaf/0.1/knows> ?person.\n  ?person <http://xmlns.com/foaf/0.1/name> ?name.\n}","queue":{"size":1,"push":{"urn:comunica:default:extract-links/actors#quad-pattern-query":1},"pop":{}}},"msg":"Link queue changed","time":"2024-08-07T08:21:44.426Z","v":0}
```

- Run `comunica-link-queue-parser-rs` (see [usage section](#usage))

```
./target/release/comunica-link-queue-parser-rs -i path/to/the/comunica/log -o desired/output/path.json
```

It should output something similar to the shorten example below.

```json
{
    "SELECT DISTINCT ?name ?person WHERE {   <https://www.rubensworks.net/#me> <http://xmlns.com/foaf/0.1/knows> ?person.   <https://ruben.verborgh.org/profile/#me> <http://xmlns.com/foaf/0.1/knows> ?person.   ?person <http://xmlns.com/foaf/0.1/name> ?name. }": {
        "push": [
            {
                "link": {
                    "parent": "https://www.rubensworks.net/",
                    "producedByActor": {
                        "metadata": {
                            "onlyVariables": true
                        },
                        "name": "urn:comunica:default:extract-links/actors#quad-pattern-query"
                    },
                    "timestamp": 2553.299712,
                    "url": "https://data.verborgh.org/people/anastasia_dimou"
                },
                "queue": {
                    "pop": {},
                    "push": {
                        "urn:comunica:default:extract-links/actors#quad-pattern-query": 1
                    },
                    "size": 1
                }
            },
            {
                "link": {
                    "parent": "https://www.rubensworks.net/",
                    "producedByActor": {
                        "metadata": {
                            "onlyVariables": true
                        },
                        "name": "urn:comunica:default:extract-links/actors#quad-pattern-query"
                    },
                    "timestamp": 2553.459799,
                    "url": "https://data.verborgh.org/people/arne_gevaert"
                },
                "queue": {
                    "pop": {},
                    "push": {
                        "urn:comunica:default:extract-links/actors#quad-pattern-query": 2
                    },
                    "size": 2
                }
            },
            ...
            {
                "link": {
                    "parent": "https://data.verborgh.org/ruben?predicate=http%3A%2F%2Fxmlns.com%2Ffoaf%2F0.1%2Fname&page=14",
                    "timestamp": 45449.248452,
                    "url": "https://data.verborgh.org/ruben?predicate=http%3A%2F%2Fxmlns.com%2Ffoaf%2F0.1%2Fname&page=15"
                },
                "queue": {
                    "pop": {
                        "unknown": 15,
                        "urn:comunica:default:extract-links/actors#quad-pattern-query": 407
                    },
                    "push": {
                        "unknown": 16,
                        "urn:comunica:default:extract-links/actors#quad-pattern-query": 407
                    },
                    "size": 1
                }
            }
        ],
        "pop": [
            {
                "link": {
                    "producedByActor": {
                        "metadata": {
                            "onlyVariables": true
                        },
                        "name": "urn:comunica:default:extract-links/actors#quad-pattern-query"
                    },
                    "timestamp": 2558.003431,
                    "url": "https://data.verborgh.org/people/anastasia_dimou"
                },
                "queue": {
                    "pop": {
                        "urn:comunica:default:extract-links/actors#quad-pattern-query": 1
                    },
                    "push": {
                        "urn:comunica:default:extract-links/actors#quad-pattern-query": 60
                    },
                    "size": 59
                }
            },
            {
                "link": {
                    "producedByActor": {
                        "metadata": {
                            "onlyVariables": true
                        },
                        "name": "urn:comunica:default:extract-links/actors#quad-pattern-query"
                    },
                    "timestamp": 2558.042332,
                    "url": "https://data.verborgh.org/people/arne_gevaert"
                },
                "queue": {
                    "pop": {
                        "urn:comunica:default:extract-links/actors#quad-pattern-query": 2
                    },
                    "push": {
                        "urn:comunica:default:extract-links/actors#quad-pattern-query": 60
                    },
                    "size": 58
                }
            },
            ...
            {
                "link": {
                    "timestamp": 45458.530643,
                    "url": "https://data.verborgh.org/ruben?predicate=http%3A%2F%2Fxmlns.com%2Ffoaf%2F0.1%2Fname&page=15"
                },
                "queue": {
                    "pop": {
                        "unknown": 16,
                        "urn:comunica:default:extract-links/actors#quad-pattern-query": 407
                    },
                    "push": {
                        "unknown": 16,
                        "urn:comunica:default:extract-links/actors#quad-pattern-query": 407
                    },
                    "size": 0
                }
            }
        ]
    }
}
```