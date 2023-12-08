use std::fs;
use rayon::prelude::*;
use anyhow::Result;

mod cli;
mod part_one;
mod part_two;


fn main() -> Result<()> {
    let args = cli::parse();
    let file = fs::read_to_string(&args.path)?;
    let lines = file.par_lines();

    let sum: usize = match &args.part_two {
        false => part_one::parse_input(lines),
        true => part_two::parse_input(lines),
    };
    
    println!("Value is: {}", sum);
    Ok(()) 
}
