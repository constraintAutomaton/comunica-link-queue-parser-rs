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