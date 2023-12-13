use itertools::Itertools;
use nom::{
    branch::alt, 
    bytes::complete::tag, 
    character::complete::u8,
    combinator::value, 
    IResult, 
    multi::{separated_list1, many1}, 
    sequence::separated_pair, Finish, 
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Spring {
    Operational, // .
    Damaged, // #
    Unknown // ?
}

fn parse_spring(input: &str) -> IResult<&str, Spring> {
    alt((
        value(Spring::Operational, tag(".")),
        value(Spring::Damaged, tag("#")),
        value(Spring::Unknown, tag("?"))
    ))(input)
}


fn parse_line(input: &str) -> IResult<&str, (Vec<Spring>, Vec<u8>)> {
    separated_pair(
        many1(parse_spring), 
        tag(" "), 
        separated_list1(
            tag(","), 
            u8
        )
    )(input)
}

#[derive(Debug, PartialEq, Eq)]
enum Validation {
    FailedAt(usize),
    Passed
}

fn validate_combination(springs: &[Spring], counts: &[u8], damaged: &[usize]) -> Validation {
    // println!("testing {:?} against {:?}", damaged, counts);
    // println!("springs {:?}", springs);
    let max_idx_contiguous = counts.len() - 1;
    let mut idx_contiguous = 0;
    let mut size_contiguous = 0;
    let mut idx_damaged = 0;
    for idx in 0..springs.len() {
        let spring = springs[idx];
        let damaged = match spring {
            Spring::Operational => false,
            Spring::Damaged => true,
            Spring::Unknown => {
                if idx_damaged < damaged.len() && damaged[idx_damaged] == idx {
                    idx_damaged += 1;
                    true
                } else { 
                    false 
                }
            }
        };

        // println!(
        //     "idx {} spring {:?} damaged {} size_cont {} idx_cont {} idx_damg {}",
        //     idx, spring, damaged, size_contiguous, idx_contiguous, idx_damaged
        // );

        // if we are in a contiguous block
        if size_contiguous > 0 {
            let target_size = counts[idx_contiguous];
            if damaged {
                // failure condition: 
                // found contiguous block larger than specified
                if size_contiguous >= target_size {
                    // println!("failure condition: found contiguous block larger than specified");
                    return Validation::FailedAt(idx_damaged);
                }
                size_contiguous += 1;
            } else {
                // failure condition:
                // found contiguous block smaller than specified
                if size_contiguous != target_size {
                    // println!("failure condition: found contiguous block smaller than specified");
                    return Validation::FailedAt(idx_damaged);
                }
                idx_contiguous += 1;
                size_contiguous = 0;
            }
        } else {
            if damaged {
                // failure condition:
                // found more contiguous blocks than specified
                if idx_contiguous > max_idx_contiguous {
                    // println!("failure condition: found more contiguous blocks than specified");
                    return Validation::FailedAt(idx_damaged);
                }
                size_contiguous = 1;
            }
        }
    }
    // we looped through all springs without short-circuiting
    // final check: have we found all expected groups?
    // println!("no short circuit, but final check: {}", idx_contiguous == max_idx_contiguous );
    // return idx_contiguous == max_idx_contiguous;
    Validation::Passed
}

fn calc(springs: Vec<Spring>, counts: Vec<u8>) -> usize {
    // algorithm
    //
    // work out target number of springs?
    // partition the springs into chunks that *must* be separated
    let target = counts.iter().sum::<u8>() as usize;
    let current = springs.iter().filter(|p| **p == Spring::Damaged).count();

    // println!(
    //     "springs {:?}, counts {:?}, current {}, target {}", 
    //     springs, counts, current, target
    // );

    let combinations = springs
            .iter()
            .enumerate()
            .filter_map(
                |(i, s)| match s {
                    Spring::Unknown => Some(i),
                    _ => None,
                })
            .combinations(target - current);

    
    
    let mut skip_from_combination: Vec<usize>;
    let mut skip_prefix: Option<&[usize]> = None;
    // let mut skipped = 0;
    let mut count = 0;
    
    for combination in combinations {
        if let Some(prefix) = skip_prefix {
            if combination.starts_with(prefix) {
                // skipped += 1;
                continue;
            } else {
                // we have passed this prefix
                skip_prefix = None;
            }
        }
        if let Validation::FailedAt(idx) = validate_combination(&springs, &counts, &combination) {
            skip_from_combination = combination;
            skip_prefix = Some(&skip_from_combination[0..idx]);
            // println!("validation failed for {:?} skipping {:?}", skip_from_combination, skip_prefix);
        } else {
            count += 1;
        };
    }

    count
}

fn explode(springs: &mut Vec<Spring>, counts: &mut Vec<u8>) {
  let spring_len = springs.len();
  let counts_len = counts.len();

  springs.reserve_exact((spring_len * 5) + 4);
  counts.reserve_exact(counts_len * 5);

  for _ in 0..4 {
    springs.push(Spring::Unknown);
    springs.extend_from_within(0..spring_len);
    counts.extend_from_within(0..counts_len);
  }
}

pub fn parse_and_calc_line(input: &str) -> usize {
    let (_, (mut springs, mut counts)) = parse_line(input).finish().unwrap();
    explode(&mut springs, &mut counts);
    calc(springs, counts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "?#?#?..#?#? 1,3,1,6";
        let (_, (springs, counts)) = parse_line(input).unwrap();
        assert_eq!(springs, vec![
            Spring::Unknown, Spring::Damaged, Spring::Unknown, Spring::Damaged, Spring::Unknown, 
            Spring::Operational, Spring::Operational,
            Spring::Damaged, Spring::Unknown, Spring::Damaged, Spring::Unknown,
            ]);
        assert_eq!(counts, vec![1, 3, 1, 6]);
    }
    
    #[test]
    fn test_parse_and_calc_line() {
        let inputs = [
            ("???.### 1,1,3", 1),
            (".??..??...?##. 1,1,3", 16384),
            ("?#?#?#?#?#?#?#? 1,3,1,6", 1),
            ("????.#...#... 4,1,1", 16),
            ("????.######..#####. 1,6,5", 2500),
            ("?###???????? 3,2,1", 506250),
        ];
        for (line, expected) in inputs {
            let actual = parse_and_calc_line(line);
            assert_eq!(actual, expected);
        };
    }

}
