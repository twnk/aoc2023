use rayon::prelude::*;

pub type Grid = Vec<Vec<char>>;

struct Coord {
    chr: char,
    x: usize,
    y: usize
}

enum C {
    Digit(Coord),
    Gear(Coord)
} 

pub fn gear_shift(grid: Grid) -> usize {
    let max_size = grid.len();

    let (digits, gears): (Vec<_>, Vec<_>) = grid
        .par_iter()
        .enumerate()
        .flat_map(|(x, row)| {
            row
                .into_par_iter()
                .enumerate()
                .filter_map(move |(y, chr)| match chr {
                    '0'..='9' => Some(C::Digit(Coord { chr: *chr, x, y })),
                    '*' => Some(C::Gear(Coord { chr: *chr, x, y })),
                    _ => None,
                })
        })
        .partition_map(|c| match c {
            C::Digit(c) => rayon::iter::Either::Left(c),
            C::Gear(g) => rayon::iter::Either::Right(g),
        });

    gears
        .into_par_iter()
        .fold(
            || 0 as usize, 
            |acc, gear| {
            // after 
            let y = gear.y + 1;
            if y < max_size && grid[gear.x][y].is_numeric() {
                for n in y..(max_size.min(y + 2)) {}
            }
            acc + 1
        })
        .sum()
    
}


#[cfg(test)]
mod tests {
    use super::*;
        
    #[test]
    fn sum_part_numbers() {
        let lines = [
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ];
        let chars: Vec<char> = lines.into_iter().map(|l| l.chars()).flatten().collect();

        let grid = chars.try_into().unwrap();

        let result = search_grid(grid);
        assert_eq!(result, 4361);
    }
}
