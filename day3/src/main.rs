use std::fs;
use rayon::prelude::*;
use anyhow::Result;


mod cli;
mod part_one;
mod part_two;


fn main() -> Result<()> {
    let args = cli::parse();
    let file = fs::read_to_string(&args.path)?;
    match args.part_two {
        false => {
            let chars: Vec<char> = file
                .par_lines()
                .map(|l| l.par_chars())
                .flatten()
                .collect();

            let grid = chars.try_into().unwrap();
            
            let sum: usize = part_one::search_grid(grid);
            println!("Value is: {}", sum);
        }
        true => {
            let chars: part_two::Grid = file
                .par_lines()
                .map(|l| l.par_chars().collect())
                .collect();

            let val = part_two::gear_shift(chars);
            println!("Value is: {}", val);
        }
    }
    
    Ok(()) 
}
