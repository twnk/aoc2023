use nom::{
    branch::alt, 
    bytes::complete::tag, 
    IResult, 
    multi::separated_list1,
    sequence::{delimited, separated_pair, pair}, 
    combinator::{value, all_consuming}, 
};

type GameID = u16;

fn parse_game_id(input: &str) -> IResult<&str, GameID> {
    delimited(
        tag("Game "),
        nom::character::complete::u16,
        tag(": ")
    )(input)
}


#[derive(Clone)]
enum Colour {
    Green,
    Blue,
    Red
}

fn parse_colour(input: &str) -> IResult<&str, Colour> {
    alt((
        value(Colour::Red, tag("red")),
        value(Colour::Green, tag("green")),
        value(Colour::Blue, tag("blue"))
    ))(input)
}

type DiceCount = u16;

fn parse_one_dice_number(input: &str) -> IResult<&str, (DiceCount, Colour)> {
    separated_pair(
        nom::character::complete::u16, 
        tag(" "), 
        parse_colour,
    )(input)
}

type Round = Vec<(DiceCount, Colour)>;

fn parse_one_round(input: &str) -> IResult<&str, Round> {
    separated_list1(
        tag(", "), 
        parse_one_dice_number
    )(input)
}

#[derive(Debug)]
struct GameMaxima {
    red: DiceCount,
    green: DiceCount,
    blue: DiceCount
}

fn parse_all_rounds(input: &str) -> IResult<&str, GameMaxima> {
    let (remainder, rounds) = separated_list1(
        tag("; "),
        parse_one_round
    )(input)?;

    let counts = rounds.into_iter().flatten();
    let maxima = counts.fold(
        GameMaxima { red: 0, green: 0, blue: 0 },
        |mut acc, (count, colour)| {
            match colour {
                Colour::Green => { acc.green = count.max(acc.green); },
                Colour::Blue => { acc.blue = count.max(acc.blue); },
                Colour::Red => { acc.red = count.max(acc.red); },
            };
            acc
        }
    );
    Ok((remainder, maxima))
}

type Game = (GameID, GameMaxima);

fn parse_game(input: &str) -> IResult<&str, Game> {
    all_consuming(pair(
        parse_game_id, 
        parse_all_rounds
    ))(input)
}

const MAX_RED: u16 = 12;
const MAX_GREEN: u16 = 13;
const MAX_BLUE: u16 = 14;


pub fn parse_line(line: &str) -> usize {
    let res = parse_game(line);
    if let Ok((_, (game_id, maxima))) = res {
        if maxima.red <= MAX_RED && maxima.green <= MAX_GREEN && maxima.blue <= MAX_BLUE {
            game_id.into()
        } else {
            0
        }
    } else {
        println!("something went wrong");
        println!("{:#?}", res);
        0
    }
}


#[cfg(test)]
mod tests {
    use super::*;
        
    #[test]
    fn game_id_value_from_line() {
        let lines = [
            ("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 1),
            ("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 2),
            ("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", 0),
            ("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", 0),
            ("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 5),
        ];
        for (line, expectation) in lines {
            let result = parse_line(line);
            assert_eq!(result, expectation);
        }
    }
}
