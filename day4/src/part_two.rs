use nom::{
    bytes::complete::tag, 
    character::complete::{u8, space1}, 
    combinator::all_consuming, 
    IResult, 
    multi::separated_list1,
    sequence::{delimited, separated_pair, tuple, pair},
};
use tinyset::Set64;
use rayon::{prelude::*, str::Lines};

fn parse_card_id(input: &str) -> IResult<&str, u8> {
    delimited(
        tuple((tag("Card"), space1)),
        u8,
        tuple((tag(":"), space1)),
    )(input)
}

fn parse_number_sequence(input: &str) -> IResult<&str, Vec<u8>> {
    separated_list1(
        space1, 
        u8,
    )(input)
}

fn parse_lottery_numbers(input: &str) -> IResult<&str, (Vec<u8>, Vec<u8>)> {
    separated_pair(
        parse_number_sequence, 
        tuple((space1, tag("|"), space1)), 
        parse_number_sequence
    )(input)
}

#[derive(Debug)]
struct Card {
    id: u8,
    winning: Set64<u8>,
    picked: Vec<u8>
}

fn parse_card(input: &str) -> IResult<&str, Card> {
   let (remainder, (id, (winning_nums, picked))) = all_consuming(pair(
        parse_card_id, 
        parse_lottery_numbers
    ))(input)?;

    let winning = winning_nums.into_iter().collect();
    Ok((remainder, Card { id, winning, picked }))
}

type Score = usize;

fn score_line(line: &str) -> Score {
    let res = parse_card(line);
    if let Ok((_, card)) = res {
        let score = card.picked
            .into_par_iter()
            .filter(|pick| card.winning.contains(pick))
            .count();
        score
    } else {
        panic!("something went wrong with {}: {:#?}", line, res);
    }
}

pub fn parse_input(lines: Lines) -> u32 {
    let scores: Vec<_> = lines.map(score_line).collect();
    let max = scores.len();
    let mut counts: Vec<u32> = vec![1; max];

    for (id, score) in scores.into_iter().enumerate() {
        let next_card = max.min(id + 1);
        let more_cards_up_to = max.min(id + score);
        
        let count = counts[id];

        for idx in next_card..=more_cards_up_to {
            counts[idx] += count
        }
    }

    counts.into_par_iter().sum()
}


#[cfg(test)]
mod tests {
    use super::*;
        
    #[test]
    fn score_from_line() {
        let lines = [
            ("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 4),
            ("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2),
            ("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 2),
            ("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1),
            ("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0),
            ("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 0),
        ];
        for (line, expectation) in lines {
            let result = score_line(line);
            println!("{}", line);
            assert_eq!(result, expectation);
        }
    }
}
