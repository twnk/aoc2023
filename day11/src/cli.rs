use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// File to parse
    pub path: String,
    /// Part two
    #[arg(long)]
    pub part_two: bool,
}

pub fn parse() -> Cli {
    Cli::parse()
}
