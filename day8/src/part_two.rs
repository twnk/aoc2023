use std::collections::BTreeMap;

use nom::{
    character::complete::alphanumeric1,
    IResult, 
    sequence::{separated_pair, delimited}, multi::many1, combinator::value, branch::alt, bytes::complete::tag,
};
use rayon::prelude::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum Direction {
    Left,
    Right
}

fn parse_directions(input: &str) -> IResult<&str, Vec<Direction>> {
    many1(
        alt((
            value(Direction::Left, tag("L")), 
            value(Direction::Right, tag("R"))
        ))
    )(input)
}

fn parse_map(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    separated_pair(
        alphanumeric1,
        tag(" = "), 
        delimited(
            tag("("), 
            separated_pair(
                alphanumeric1, 
                tag(", "), 
                alphanumeric1
            ), 
            tag(")")
        )
    )(input)
}

pub fn parse_input(input: &str) -> usize {
    let lines: Vec<_> = input.par_lines().collect();

    let (directions_raw, maps_raw) = lines.split_at(2);

    let (_, directions) = parse_directions(directions_raw[0]).unwrap();
    let maps = maps_raw
        .into_par_iter()
        .filter_map(|l| match parse_map(*l) {
            Ok((_, n)) => Some(n),
            Err(e) => panic!("{}", e),
        });

    let map = BTreeMap::from_par_iter(maps);

    let max_direction = directions.len();
    let mut iterations = 0;
    let mut positions: Vec<&str> = map.keys().filter(|k| k.ends_with('A')).map(|s| *s).collect();
    let start_positions = positions.clone();
    let target = positions.len();
    let mut periods = vec![Vec::new(); target];
    loop {
        for idx in 0..max_direction {
            let direction = directions[idx];
            positions = positions
                .into_par_iter()
                .map(|p| {
                    let (left, right) = map.get(p).unwrap();
                    match direction {
                        Direction::Left => *left,
                        Direction::Right => *right,
                    }
                })
                .collect();
            
            let iter_count = (iterations * max_direction) + idx + 1;
            let ends = positions
                .iter()
                .enumerate()
                .filter(| (_, pos)| pos.ends_with('Z'));

            for (n, _) in ends {
                periods.get_mut(n).unwrap().push(iter_count)
            }
        }
        iterations += 1;
        println!("{:?}", periods);
        if periods.iter().filter(|ps| ps.len() > 4).count() == target {
            break;
        };
    };

    let mut simple_periods = Vec::new();
    for (prds, start) in periods.iter().zip(start_positions) {
        println!("Start {} reached Z at {}", start, prds[0]);
        simple_periods.push(prds[0]);
        for idx in 1..prds.len() {
            let a = prds[idx - 1];
            let b = prds[idx];
            let gap = b - a;
            println!("Start {} reached Z again at {} after {}", start, b, gap);
        }
    }

    println!("Calculating LCM");
    let lcm = simple_periods.into_iter().reduce(|a, b| num_integer::lcm(a, b));

    lcm.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn sample_input() {
        let input = "LR\n\n11A = (11B, XXX)\n11B = (XXX, 11Z)\n11Z = (11B, XXX)\n22A = (22B, XXX)\n22B = (22C, 22C)\n22C = (22Z, 22Z)\n22Z = (22B, 22B)\nXXX = (XXX, XXX)\n";
        let steps = parse_input(input);
        assert_eq!(steps, 6);
    }
}
