use object::History;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, prelude::*, BufReader};
mod object;
mod util;

fn main() -> io::Result<()> {
    let file = File::open("./info")?;
    let reader = BufReader::new(file);
    let mut history: HashMap<String, History> = HashMap::new();
    for line in reader.lines() {
        let _ = util::process_line(&line?, &mut history);
    }

    let json_string = serde_json::to_string(&history)?;

    fs::write("./occupancy.json", json_string).expect("Unable to write file");

    Ok(())
}
