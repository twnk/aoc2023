pub fn hash_step(step: &str) -> u32 {
    let bytes = step.as_bytes();
    let mut val: u8 = 0;
    for byte in bytes {
        val = val.wrapping_add(*byte);
        val = val.wrapping_mul(17);
    }
    val as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_line() {
        let inputs = [
            ("rn=1", 30),
            ("cm-", 253),
            ("qp=3", 97),
            ("cm=2", 47),
            ("qp-", 14),
            ("pc=4", 180),
            ("ot=9", 9),
            ("ab=5", 197),
            ("pc-", 48),
            ("pc=6", 214),
            ("ot=7", 231),
        ];
        for (input, expected) in inputs {
            let actual = hash_step(input);
            println!("expecting {} actual {}", expected, actual);
            assert_eq!(actual, expected);
        }
        
    }

}
