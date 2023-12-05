use aho_corasick::{AhoCorasick, AhoCorasickBuilder};
use phf::phf_map;
use {
    once_cell::sync::Lazy,
    regex::Regex,
};

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"one|two|three|four|five|six|seven|eight|nine|[0-9]").unwrap());

const PATTERNS: [&str; 19] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine"
    ];

static AHO: Lazy<AhoCorasick> = Lazy::new(|| AhoCorasickBuilder::new().build(PATTERNS).unwrap());

static DECODE: phf::Map<&'static str, usize> = phf_map! {
    "0" => 0,
    "1" => 1,
    "2" => 2,
    "3" => 3,
    "4" => 4,
    "5" => 5,
    "6" => 6,
    "7" => 7,
    "8" => 8,
    "9" => 9,
    "one" => 1,
    "two" => 2,
    "three" => 3,
    "four" => 4,
    "five" => 5,
    "six" => 6,
    "seven" => 7,
    "eight" => 8,
    "nine" => 9
};


pub fn parse_line_re(line: &str) -> usize {
    let mut matches = RE.find_iter(line);
    let partial = if let Some(m) = matches.next() {
        match DECODE.get(m.as_str()) {
            Some(n) => n,
            None => {println!("err no phf match for {}, from line {}", m.as_str(), line); return 0}
        }
    } else {
        println!("err no regex matches from line {}", line);
        return 0
    };

    if let Some(m) = matches.last() {
        match DECODE.get(m.as_str()) {
            Some(n) => {
                let rtn = (partial * 10) + n;
                println!("ok: {}, from line {}", rtn, line);
                rtn
            },
            None => {
                println!("err no phf match for {}, from line {}", m.as_str(), line); 
                0
            }
        }
    } else {
        println!("ok: {}{}, from line {}", partial, partial, line);
        (partial * 10) + partial
    }
}

pub fn parse_line_aho(line: &str) -> usize {
    let mut matches = AHO.find_overlapping_iter(line);
    let partial = if let Some(m) = matches.next() {
        let n = m.pattern().as_usize();
        if n < 10 { n } else { n - 9 }
    } else {
        println!("err no aho matches from line {}", line);
        return 0
    };

    if let Some(m) = matches.last() {
        let n = m.pattern().as_usize();
        if n < 10 { (partial * 10) + n } else { (partial * 10) + (n - 9) }
    } else {
        (partial * 10) + partial
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    const SIMPLE: [(&str, usize); 4] = [
        ("1abc2", 12),
        ("pqr3stu8vwx", 38),
        ("a1b2c3d4e5f", 15),
        ("treb7uchet", 77),
    ];
        
    #[test]
    fn calibration_simple_re() {
        for (line, expectation) in SIMPLE {
            let result = parse_line_re(line);
            assert_eq!(result, expectation);
        }
    }

    #[test]
    fn calibration_simple_aho() {
        for (line, expectation) in SIMPLE {
            let result = parse_line_aho(line);
            assert_eq!(result, expectation);
        }
    }

    const WORDY: [(&str, usize); 7] = [
            ("two1nine", 29),
            ("eightwothree", 83),
            ("abcone2threexyz", 13),
            ("xtwone3four", 24),
            ("4nineeightseven2", 42),
            ("zoneight234", 14),
            ("7pqrstsixteen", 76),
        ];

    #[test]
    fn calibration_wordy_re() {
        for (line, expectation) in WORDY {
            let result = parse_line_re(line);
            assert_eq!(result, expectation);
        }
    }

    #[test]
    fn calibration_wordy_aho() {
        for (line, expectation) in WORDY {
            let result = parse_line_aho(line);
            assert_eq!(result, expectation);
        }
    }

    const OVERLAPPING: [(&str, usize); 2] = [
        ("twone3threeight", 28),
        ("nineight4sevenine", 99)
    ];

    #[test]
    fn calibration_overlapping_re() {
        for (line, expectation) in OVERLAPPING {
            let result = parse_line_re(line);
            assert_ne!(result, expectation);
        }
    }

    #[test]
    fn calibration_overlapping_aho() {
        for (line, expectation) in OVERLAPPING {
            let result = parse_line_aho(line);
            assert_eq!(result, expectation);
        }
    }


}
