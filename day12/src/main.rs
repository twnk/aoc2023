#![feature(slice_as_chunks)]
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
    let sum: usize = match &args.part_two {
        false => lines.into_par_iter().map(part_one::parse_and_calc_line).sum(),
        true => lines.into_par_iter().map(part_two::parse_and_calc_line).sum()
    };
    let base_time = timer.elapsed();
    
    println!("Value is: {} calculated in {}", sum, base_time.as_micros());
    Ok(()) 
}
