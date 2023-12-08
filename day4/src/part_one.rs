use nom::{
    bytes::complete::tag, 
    character::complete::{digit1, u8, space1}, 
    combinator::{all_consuming, recognize}, 
    IResult, 
    multi::separated_list1,
    sequence::{separated_pair, tuple, preceded}, 
};
use tinyset::Set64;
use rayon::prelude::*;

fn parse_card_id(input: &str) -> IResult<&str, &str> {
    recognize(tuple((
        tag("Card"),
        space1,
        digit1,
        tag(":"),
        space1
    )))(input)
}

fn parse_number_sequence(input: &str) -> IResult<&str, Vec<u8>> {
    separated_list1(
        space1, 
        u8,
    )(input)
}

#[derive(Debug)]
struct Card {
    winning: Set64<u8>,
    picked: Vec<u8>
}

fn parse_lottery_numbers(input: &str) -> IResult<&str, Card> {
    let (remainder, (winning_nums, picked)) = separated_pair(
        parse_number_sequence, 
        tuple((space1, tag("|"), space1)), 
        parse_number_sequence
    )(input)?;

    let winning = winning_nums.into_iter().collect();
    Ok((remainder, Card { winning, picked }))
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    all_consuming(preceded(
        parse_card_id, 
        parse_lottery_numbers
    ))(input)
}

pub fn parse_line(line: &str) -> usize {
    let res = parse_card(line);
    if let Ok((_, card)) = res {
        let winners = card.picked
            .into_par_iter()
            .filter(|pick| card.winning.contains(pick))
            .count();
        if winners > 0 { 2_usize.pow((winners - 1) as u32) }
        else { 0 }
    } else {
        println!("something went wrong with {}", line);
        println!("{:#?}", res);
        0
    }
}


#[cfg(test)]
mod tests {
    use super::*;
        
    #[test]
    fn score_from_line() {
        let lines = [
            ("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 8),
            ("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2),
            ("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 2),
            ("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1),
            ("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0),
            ("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 0),
        ];
        for (line, expectation) in lines {
            let result = parse_line(line);
            println!("{}", line);
            assert_eq!(result, expectation);
        }
    }
}
