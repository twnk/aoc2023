use std::fs;
use anyhow::Result;

mod cli;
mod part_one;
mod part_two;


fn main() -> Result<()> {
    let args = cli::parse();
    let file = fs::read_to_string(&args.path)?;

    let sum: usize = match &args.part_two {
        false => part_one::parse_input(&file),
        true => part_two::parse_input(&file),
    };
    
    println!("Value is: {}", sum);
    Ok(()) 
}
