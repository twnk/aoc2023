#[cfg(test)]
const GRID_SIZE: usize = 10;
#[cfg(not(test))]
const GRID_SIZE: usize = 140;

pub type Grid = [char; GRID_SIZE * GRID_SIZE];

fn radius_search(grid: &Grid, x: usize, y: usize, index: usize, c: char, in_number_since: usize, part_number_acc: &str) -> usize {

    // we were in a number at last char
    // search radius:
    // - the position after the number (current pos)
    // - the position before the number
    // - the row above the number
    // - the row below the number 

    let mut found = false;
    // current position & position before
    if c != '.' && c.is_ascii_punctuation() { found = true }
    else if in_number_since < y { // bounds check
        let chr = grid[index - (in_number_since + 1)];
        if chr != '.' && chr.is_ascii_punctuation() { found = true } 
    }

    // rows above and below
    let y_start = y.saturating_sub(in_number_since + 1);
    let y_diff = y - y_start;

    // row above
    if !found && x > 0 {
        let search_start = ((x - 1) * GRID_SIZE) + y_start;
        let search_stop = search_start + y_diff;
        for idx in search_start..=search_stop {
            let chr = grid[idx];
            if chr != '.' && chr.is_ascii_punctuation() { found = true; break }
        }
    }

    // row below
    if !found && x + 1 < GRID_SIZE {
        let search_start = ((x + 1) * GRID_SIZE) + y_start;
        let search_stop = search_start + y_diff;
        for idx in search_start..=search_stop {
            let chr = grid[idx];
            if chr != '.' && chr.is_ascii_punctuation() { found = true; break }
        }
    }

    if found {
        println!("found symbol around {}", &part_number_acc);
        let part_number = usize::from_str_radix(&part_number_acc, 10).unwrap_or(0);
        part_number
    } else { 0 }
    
}

pub fn search_grid(grid: Grid) -> usize {
    let mut total = 0;
    let mut part_number_acc = String::with_capacity(3);
    let mut in_number_since: usize = 0;
    for x in 0..GRID_SIZE {
        println!("row {}", x + 1);
        for y in 0..GRID_SIZE {
            let index = (x * GRID_SIZE) + y;
            let c = grid[index];
            if c.is_ascii_digit() {
                part_number_acc.push(c);
                in_number_since += 1;
                // handle final column!
                if y + 1 == GRID_SIZE {
                    total += radius_search(&grid, x, y + 1, index + 1, c, in_number_since, &part_number_acc);
                    // clear the number flag and string
                    in_number_since = 0;
                    part_number_acc.clear();
                }
            } else {
                // not in number or symbol
                if in_number_since != 0 {
                    total += radius_search(&grid, x, y, index, c, in_number_since, &part_number_acc);
                    // clear the number flag and string
                    in_number_since = 0;
                    part_number_acc.clear();
                }
            }
        }
    };
    
    total
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
