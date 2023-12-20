use std::{fs, time::Instant};
use anyhow::Result;
use rayon::prelude::*;

mod cli;
mod part_one;
mod part_two;


fn main() -> Result<()> {
    let args = cli::parse();
    let file = fs::read_to_string(&args.path)?;
    let steps = file.par_split(',');

    let timer = Instant::now();
    let sum: usize = match &args.part_two {
        false => steps.into_par_iter().map(part_one::hash_step).sum::<u32>() as usize,
        true => part_two::process_steps(steps),
        // true => part_two::parse_input(&file),
    };
    let base_time = timer.elapsed();
    
    println!("Value is: {} calculated in {}", sum, base_time.as_micros());
    Ok(()) 
}
