use std::fs;
use anyhow::Result;

mod cli;
mod part_one;
mod part_two;


fn main() -> Result<()> {
    let args = cli::parse();
    let file = fs::read_to_string(&args.path)?;
    let lines = file.split('\n');

    let parse_line = match &args.part_two {
        false => part_one::parse_line,
        true => part_two::parse_line
    };
    
    let sum: usize = lines.map(parse_line).sum();
    println!("Value is: {}", sum);
    Ok(()) 
}
