use std::{fs, time::Instant};
use anyhow::Result;
use rayon::prelude::*;

mod cli;
mod part_one;
mod part_two;


fn main() -> Result<()> {
    let args = cli::parse();
    let file = fs::read_to_string(&args.path)?;
    let lines = file.par_lines();

    let timer = Instant::now();
    let sum: i32 = match &args.part_two {
        false => lines.into_par_iter().map(part_one::parse_line).sum(),
        true => lines.into_par_iter().map(part_two::parse_line).sum(),
        // true => part_two::parse_input(&file),
    };
    let base_time = timer.elapsed();
    
    println!("Value is: {} calculated in {}", sum, base_time.as_micros());
    Ok(()) 
}
