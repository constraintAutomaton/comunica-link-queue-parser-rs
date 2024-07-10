use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
/// The CLI arguments
pub(crate) struct Cli {
    /// Path of logging file of comunica
    /// By default the value is [default: ./info]
    #[arg(short, long)]
    pub input: Option<PathBuf>,

    /// Path of the output occupancy file
    /// By default the value is [default: ./occupancy.json]
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Print the occupancy object [default: false]
    #[arg(short, long,default_value_t=false, action = clap::ArgAction::SetTrue)]
    pub print: bool,
}
