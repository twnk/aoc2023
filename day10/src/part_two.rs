use rayon::{prelude::*, str::Lines};

// Grid Size 5, but smallest chunk is 16
#[cfg(test)]
const ROW_SIZE: usize = 20;
#[cfg(test)]
const GRID_ROWS: usize = 20;

// Grid size 140 * 140 
// 16 * 9 = 144 
#[cfg(not(test))]
const ROW_SIZE: usize = 144; 
#[cfg(not(test))]
const GRID_ROWS: usize = 144;

const ALLOCATION: usize = ROW_SIZE * GRID_ROWS;

#[derive(PartialEq, Eq, Copy, Clone, Default, Debug)]
enum Tile {
    #[default]
    Ground, // .
    NorthSouth, // |
    EastWest, // -
    NorthEast, // L
    NorthWest, // J
    SouthWest, // 7
    SouthEast, // F
    Start, // S
}

fn parse_line_to_tiles<const T: usize>(input: &[u8], out: &mut [Tile; ROW_SIZE]) {
    let mut out_idx = 0;
    for chunk in input.chunks(T) {
        for idx in 0..chunk.len() {
            let tile = match chunk[idx] {
                b'|' => Tile::NorthSouth,
                b'-' => Tile::EastWest, // -
                b'L' => Tile::NorthEast, // L
                b'J' => Tile::NorthWest, // J
                b'7' => Tile::SouthWest, // 7
                b'F' => Tile::SouthEast, // F
                b'S' => Tile::Start, // S
                _ => Tile::Ground, // .
            };
            out[out_idx + idx] = tile;
        }
        out_idx += T;
    };
}

pub fn parse_lines(lines: Lines) -> usize {
    let rows_as_bytes: Vec<_> = lines.into_par_iter().map(|l| l.as_bytes()).collect();
    let mut full_grid = [Tile::default(); ALLOCATION];

    let (mut_slices, _) = full_grid.as_chunks_mut::<ROW_SIZE>();

    rows_as_bytes
        .into_par_iter()
        .zip(mut_slices)
        .for_each(|(input, mut out)| {
            parse_line_to_tiles::<16>(input, &mut out);
        });

    let (start, _) = full_grid.par_iter().enumerate().find_first(|(_, t)| **t == Tile::Start).unwrap();

    let pipe_start = 'pipe_start: {
        let start_south = start + ROW_SIZE;
        match full_grid[start_south] {
            Tile::NorthSouth | Tile::NorthWest | Tile::NorthEast => {
                break 'pipe_start start_south;
            },
            _ => {}
        };

        let start_north = start - ROW_SIZE;
        match full_grid[start_north] {
            Tile::NorthSouth | Tile::SouthWest | Tile::SouthEast => {
                break 'pipe_start start_north;
            },
            _ => {}
        };

        // there have to be 2 entrances, so...
        let start_east = start - 1;
        // let start_west = start + 1;
        start_east
    };

    // println!("start at {} {}", start / ROW_SIZE, start % ROW_SIZE);

    // for i in 0..GRID_ROWS {
    //     println!("{:?}", &full_grid[i..(i*ROW_SIZE)]);
    // }

    let mut area: isize = 0;
    let mut pos = pipe_start;
    let mut last_pos = start;
    let mut y1 = start/ ROW_SIZE;
    let mut x1 = start % ROW_SIZE;
    let mut length = 1;

    loop {
        // println!(
        //     "now at {} {} tile {:?}", 
        //     pos / ROW_SIZE, 
        //     pos % ROW_SIZE,
        //     full_grid[pos],
        // );
        match full_grid[pos] {
            Tile::NorthEast |
            Tile::NorthWest |
            Tile::SouthEast |
            Tile::SouthWest => {
                // Corner, so calc shoelace formula next determinant 
                let y2 = pos / ROW_SIZE;
                let x2 = pos % ROW_SIZE;

                // | x1  x2 |
                // |        | = x1.y2 - x2.y1
                // | y1  y2 | 
                area += (x1 * y2) as isize - (x2 * y1) as isize;
                x1 = x2;
                y1 = y2;
            }
            _ => {}
        }
        let new_pos = match full_grid[pos] {
            Tile::Ground => panic!("ran aground!"),
            Tile::NorthSouth => match last_pos < pos {
                true => pos + ROW_SIZE, // we were north, go south
                false => pos - ROW_SIZE, // we were south, go north
            },
            Tile::EastWest => match last_pos < pos {
                true => pos + 1, // we were west, go east
                false => pos - 1, // we were east, go west
            },
            Tile::NorthEast => match last_pos < pos {
                true => pos + 1, // we were north, go east
                false => pos - ROW_SIZE, // we were east, go north
            },
            Tile::NorthWest => match last_pos == pos - 1 {
                true => pos - ROW_SIZE, // we were west, go north
                false => pos - 1, // we were north, go west
            },
            Tile::SouthWest => match last_pos < pos {
                true => pos + ROW_SIZE, // we were west, go south
                false => pos - 1, // we were south, go west
            },
            Tile::SouthEast => match last_pos == pos + 1 {
                true => pos + ROW_SIZE, // we were east, go south
                false => pos + 1, // we were south, go east
            },
            Tile::Start => {break;},
        };
        last_pos = pos;
        pos = new_pos;
        length += 1;
    }
    
    // Handle last corner 
    {
        let y2 = start/ ROW_SIZE;
        let x2 = start % ROW_SIZE;

        // | x1  x2 |
        // |        | = x1.y2 - x2.y1
        // | y1  y2 | 
        area += (x1 * y2) as isize - (x2 * y1) as isize;
    }
    println!("length {}", length);
    (2 + area.unsigned_abs() - length) / 2
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_line() {
        let input = "FF7FSF7F7F7F7F7F---7\nL|LJ||||||||||||F--J\nFL-7LJLJ||||||LJL-77\nF--JF--7||LJLJ7F7FJ-\nL---JF-JLJ.||-FJLJJ7\n|F|F-JF---7F7-L7L|7|\n|FFJF7L7F-JF7|JL---7\n7-L-JL7||F7|L7F-7F7|\nL.L7LFJ|||||FJL7||LJ\nL7JLJL-JLJLJL--JLJ.L\n";
        let actual = parse_lines(input.par_lines());

        assert_eq!(actual, 10);
        
    }

}
