use std::collections::BTreeMap;

use nom::{
    character::complete::alpha1,
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
        alpha1,
        tag(" = "), 
        delimited(
            tag("("), 
            separated_pair(
                alpha1, 
                tag(", "), 
                alpha1
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
    let mut pos = "AAA";
    loop {
        for idx in 0..max_direction {
            let (left, right) = map.get(pos).unwrap();
            pos = match directions[idx] {
                Direction::Left => *left,
                Direction::Right => *right,
            };
            if pos == "ZZZ" { 
                return (iterations * max_direction) + idx + 1
             }
        }
        iterations += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn sample_input_1() {
        let input = "RL\n\nAAA = (BBB, CCC)\nBBB = (DDD, EEE)\nCCC = (ZZZ, GGG)\nDDD = (DDD, DDD)\nEEE = (EEE, EEE)\nGGG = (GGG, GGG)\nZZZ = (ZZZ, ZZZ)\n";
        let steps = parse_input(input);
        assert_eq!(steps, 2);
    }

    #[test]
    fn sample_input_2() {
        let input = "LLR\n\nAAA = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)\n";
        let steps = parse_input(input);
        assert_eq!(steps, 6);
    }
}
