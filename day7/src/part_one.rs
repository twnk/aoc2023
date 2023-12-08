use nom::{
    character::complete::{space1, u16},
    IResult, 
    sequence::separated_pair, multi::count, combinator::value, branch::alt, bytes::complete::tag,
};
use rayon::{prelude::*, str::Lines};
use std::cmp::Ordering;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Kind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A
}

#[derive(Eq, Debug)]
struct Hand {
    bid: u16,
    kind: Kind,
    cards: [Card; 5]
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.kind.cmp(&other.kind) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.cards.cmp(&other.cards),
            Ordering::Greater => Ordering::Greater,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.cards == other.cards
    }
}

fn kind_from_cards(cards: [Card; 5]) -> Kind {
    let mut reps = vec![1; 5];
    for i in 0..5 {
        for j in (i + 1)..5 {
            if reps[j] != 0 && cards[i] == cards[j] {
                reps[i] += 1;
                reps[j] = 0;
            }
        }
    }

    reps.sort();
    match reps[4] {
        5 => Kind::FiveOfAKind,
        4 => Kind::FourOfAKind,
        3 => match reps[3] {
            2 => Kind::FullHouse,
            _ => Kind::ThreeOfAKind
        },
        2 => match reps[3] {
            2 => Kind::TwoPair,
            _ => Kind::OnePair
        },
        _ => Kind::HighCard
    }
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    alt((
        value(Card::Two, tag("2")),
        value(Card::Three, tag("3")),
        value(Card::Four, tag("4")),
        value(Card::Five, tag("5")),
        value(Card::Six, tag("6")),
        value(Card::Seven, tag("7")),
        value(Card::Eight, tag("8")),
        value(Card::Nine, tag("9")),
        value(Card::T, tag("T")),
        value(Card::J, tag("J")),
        value(Card::Q, tag("Q")),
        value(Card::K, tag("K")),
        value(Card::A, tag("A")),
    ))(input)
}

fn parse_cards(input: &str) -> IResult<&str, [Card; 5]> {
    let (remainder, cards_vec) = count(parse_card, 5)(input)?;
    Ok((remainder, cards_vec.try_into().unwrap()))
}

fn parse_line(input: &str) -> IResult<&str, ([Card; 5], u16)> {
    separated_pair(
        parse_cards,
        space1,
        u16
    )(input)
}

pub fn parse_input(lines: Lines) -> usize {
    let mut hands: Vec<_> = lines
        .into_par_iter()
        .filter_map(|l| match parse_line(l) {
            Ok((_, (cards, bid))) => Some(
                Hand { 
                    cards, 
                    kind: kind_from_cards(cards), 
                    bid 
                }),
            Err(e) => {println!("{:#?}", e); None},
        })
        .collect();

    

    hands.par_sort();

    for idx in 0..hands.len() { 
        let hand = &hands[idx];

        println!("{} {:?} score: {}",idx, hand, hand.bid as usize * (idx + 1))
    }

    hands.par_iter()
        .enumerate()
        .map(|(i, hand)| {(i + 1) * hand.bid as usize})
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;
        
    #[test]
    fn rank_and_bid_from_line() {
        let lines = [
            ("32T3K 765", Kind::OnePair),
            ("T55J5 684", Kind::ThreeOfAKind),
            ("KK677 28", Kind::TwoPair),
            ("KTJJT 220", Kind::TwoPair),
            ("QQQJA 483", Kind::ThreeOfAKind),
        ];
        for (line, expectation) in lines {
            let (_, (cards, _)) = parse_line(line).unwrap();
            let kind = kind_from_cards(cards);
            println!("{:?} {:#?}", cards, kind);
            assert_eq!(kind, expectation);
        }
    }

    #[test]
    fn winnings() {
        let input = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483\n";
        let winnings = parse_input(input.par_lines());
        assert_eq!(winnings, 6440);
    }
}
