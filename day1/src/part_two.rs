use anyhow::Result;
use winnow::prelude::*;
use winnow::{
    ascii::digit1,
    combinator::{alt, dispatch, fail, success},
    token::{any, one_of, tag},
};

const PLACEHOLDER_CHAR: char = ' ';

fn parse_word_number<'s>(input: &mut &'s str) -> PResult<char> {
        dispatch!(any;
            "one"   => success('1'),
            "two"   => success('2'),
            "three" => success('3'),
            "four"  => success('4'),
            "five"  => success('5'),
            "six"   => success('6'),
            "seven" => success('7'),
            "eight" => success('8'),
            "nine"  => success('9'),
            _ => fail::<_, _, _>
        )
    .parse_next(input)
}

fn parse_digit_number<'s>(input: &mut &'s str) -> PResult<char> {
    one_of('0'..='9').parse_next(input)
}

fn parse_number<'s>(input: &mut &'s str) -> PResult<char> {
    alt((
        digit1,
        parse_word_number
    )).parse_next(input)
}

pub fn parse_line(line: &str) -> usize {
    let mut acc = String::with_capacity(2);
    let mut last_seen = PLACEHOLDER_CHAR;
    for chr in line.chars() {
        match chr {
            '0'..='9' => {
                if last_seen == PLACEHOLDER_CHAR {
                    acc.push(chr);
                }
                last_seen = chr;
            }
            _ => {}
        }
    }
    acc.push(last_seen);
    match usize::from_str_radix(&acc, 10) {
        Ok(v) => v,
        Err(_) => { println!("err parsing {}, from line {}", acc, line); 0 }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
        
    #[test]
    fn calibration_value_from_line() {
        let lines = [
            ("1abc2", 12),
            ("pqr3stu8vwx", 38),
            ("a1b2c3d4e5f", 15),
            ("treb7uchet", 77),
        ];
        for (line, expectation) in lines {
            let result = parse_line(line);
            assert_eq!(result, expectation);
        }
    }

    #[test]
    fn parses_a_digit() {
        let lines = [
            ("lalaone", "one"),
            ("onon2three", "2"),
        ];

        for (line, expectation) in lines {
            let result = parse_number(&mut &line).unwrap();
            assert_eq!(result, expectation);
        }
    }
}
