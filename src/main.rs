use clap::Parser;
use cli::Cli;
use object::History;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, prelude::*, BufReader};
use std::path::PathBuf;
mod cli;
mod object;
mod util;

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let input_file = cli.input.unwrap_or(PathBuf::from("./info"));
    let output_file = cli.output.unwrap_or(PathBuf::from("./occupancy.json"));

    let file = File::open(input_file)?;
    let reader = BufReader::new(file);
    let mut history: HashMap<String, History> = HashMap::new();
    for line in reader.lines() {
        let _ = util::process_line(&line?, &mut history);
    }

    let json_string = serde_json::to_string(&history)?;

    if cli.print {
        println!("{json_string}");
    }

    fs::write(output_file, json_string).expect("Unable to write file");

    Ok(())
}
