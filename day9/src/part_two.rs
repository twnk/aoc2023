use nom::{
    bytes::complete::tag,
    character::complete::i32,
    IResult, 
    multi::separated_list1, 
};
use packed_simd::{i32x4, shuffle};
// use rayon::prelude::*;

fn parse_line_to_nums(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(tag(" "), i32)(input)
}

fn calculate_next_value(raw_nums: Vec<i32>) -> i32 {
    let input_size = raw_nums.len();
    // Establish how many chunks of 4 i32s we will need
    let number_of_chunks = input_size.div_ceil(4);

    // Pre-allocate an array with enough space for the input
    let mut subtrahends: Vec<i32x4> = Vec::with_capacity(number_of_chunks);
    let mut minuends: Vec<i32x4> = Vec::with_capacity(number_of_chunks);

    // Chop up the input into chunks of 4 subtrahends & minuends
    let mut sub_chunks = raw_nums[0..input_size-1].chunks_exact(4);
    for slice in &mut sub_chunks {
        let chunk = i32x4::from_slice_aligned(slice);
        subtrahends.push(chunk);
    }

    let mut min_chunks = raw_nums[1..input_size].chunks_exact(4);
    for slice in &mut min_chunks {
        let chunk = i32x4::from_slice_unaligned(slice);
        minuends.push(chunk);
    }

    // Handle the remainders
    let remainder = sub_chunks.remainder();
    if !remainder.is_empty() {
        let mut last_chunk = [0; 4];
        last_chunk[..remainder.len()].copy_from_slice(remainder);
        let chunk = i32x4::from_slice_unaligned(&last_chunk);
        subtrahends.push(chunk);
    }

    let remainder = min_chunks.remainder();
    if !remainder.is_empty() {
        let mut last_chunk = [0; 4];
        last_chunk[..remainder.len()].copy_from_slice(remainder);
        let chunk = i32x4::from_slice_unaligned(&last_chunk);
        minuends.push(chunk);
    }

    // "minuend" - "subtrahend" = "difference"
    //  __________  __________ sub
    // [a, b, c, d, e, f, g, h, i]
    // min __________  __________
    //
    // min [b, c, d, e] [f, g, h, i]
    // sub [a, b, c, d] [e, f, g, h]
    // dif [p, q, r, s] [t, u, v, w]
    //
    //      __________   __________ sub
    // dif [p, q, r, s] [t, u, v, w]
    //     min __________  ___________
    //
    
    // Each round, we will reduce the number of entries by 1
    // so we need to track where and when to trim the array
    let mut remainder_size = input_size % 4 - 1;
    if remainder_size == 0 { remainder_size = 4}; // bruh

    let mut walk_backwards = Vec::new();

    loop {
        let zeros = i32x4::splat(0);
        let mut all_zeros: bool;
        let mut carry: i32x4;
        let iterations = minuends.len();
        let prediction = subtrahends[0].extract(0);
        walk_backwards.push(prediction);

        // handle the first subtraction differently
        // because each round loses one number from the left
        // so we need to shift one minuend forward to the next chunk
        // which means we need to calculate the 2nd difference before
        // we can form the first minuend for the next round
        {
            let minuend = minuends[0];
            let subtrahend = subtrahends[0];
            let difference = minuend - subtrahend;

            all_zeros = difference == zeros;
            subtrahends[0] = difference;
            carry = difference;
        }
        for idx in 1..iterations {
            let minuend = minuends[idx];
            let subtrahend = subtrahends[idx];
            let difference =  minuend - subtrahend;

            all_zeros &= difference == zeros;
            subtrahends[idx] = difference;
            minuends[idx - 1] = shuffle!(carry, difference, [1, 2, 3, 4]);
            carry = difference;
        }

        // handle final minuend, return if we're done
        {
            // trim final minuend and extract prediction value
            remainder_size -= 1;
            let idx = iterations - 1;

            if all_zeros { 
                println!("{:?}", walk_backwards);
                return walk_backwards.into_iter().rev().fold(0, |acc, i| i - acc);
            }

            minuends[idx] = shuffle!(carry, zeros, [1, 2, 3, 4]);

            if remainder_size == 0 {
                // We shifted down a whole chunk, we need to trim it!
                minuends.pop();
                subtrahends.pop();
                remainder_size = 4;
            } else if idx == 0 {
                // we are down to the final chunk and need to start trimming subtrahend too
                // e.g. for rem = 3
                //      _______ sub
                // dif [a, b, c, d]
                //     min _______

                subtrahends[0] = subtrahends[0].replace(remainder_size, 0);
            }
        }       
    }
}

pub fn parse_line(line: &str) -> i32 {
    match parse_line_to_nums(line) {
        Ok((_, nums)) => calculate_next_value(nums),
        Err(e) => {
            println!("{}", e); 
            return 0
            // panic!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_line() {
        let inputs = [
            ("10 13 16 21 30 45", 5),
            ("0 3 6 9 12 15", -3),
            ("1 3 6 10 15 21", 0),
            
        ];
        for (input, expected) in inputs {
            let actual = parse_line(input);
            println!("expecting {} actual {}", expected, actual);
            assert_eq!(actual, expected);
        }
        
    }

}
