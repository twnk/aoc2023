const PLACEHOLDER_CHAR: char = ' ';

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
}
