use rayon::{prelude::*, str::Split};

type Hash = usize;
type Label<'a> = &'a [u8];
type Lens = usize;

fn hash_label(label: Label) -> Hash {
    let mut val: u8 = 0;
    for byte in label {
        val = val.wrapping_add(*byte);
        val = val.wrapping_mul(17);
    }
    val as Hash
}

enum Instruction<'a> {
    Remove(Hash, Label<'a>),
    Add(Hash, Label<'a>, Lens)
}

pub fn process_steps(steps: Split<'_, char>) -> usize {
    let mut hashmap: Vec<Vec<(Label, Lens)>> = vec![Vec::with_capacity(8); 256];

    let instructions: Vec<Instruction> = steps
        .into_par_iter()
        .map(|step| {
            let bytes = step.as_bytes();
            let (last, rest) = bytes.split_last().unwrap();

            match last {
                b'-' => {
                    Instruction::Remove(hash_label(rest), rest)
                },
                _ => {
                    let (_, label) = rest.split_last().unwrap();
                    let focal_length = (last - 48) as usize; // ascii 0-9 is 48 to 57
                    Instruction::Add(hash_label(label), label, focal_length)
                }
            }
        })
        .collect();
    
    for instruction in instructions {
        match instruction {
            Instruction::Remove(hash, label) => {
                let container = &mut hashmap[hash];
                if let Some(idx) = container.iter().position(|(lbl, _)| *lbl == label) {
                    container.remove(idx);
                }
            },
            Instruction::Add(hash, label, lens) => {
                let container = &mut hashmap[hash];
                if let Some(idx) = container.iter().position(|(lbl, _)| *lbl == label) {
                    container[idx] = (label, lens);
                } else {
                    container.push((label, lens));
                }
            },
        }
    }

    hashmap
        .into_par_iter()
        .enumerate()
        .map(|(box_idx, container)| {
            container
                .iter()
                .enumerate()
                .map(|(idx, (_, lens))| {
                    (box_idx + 1) * (idx + 1) * lens
                })
                .sum::<usize>()
        })
        .sum()
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
            let actual = hash_label(input.as_bytes());
            println!("expecting {} actual {}", expected, actual);
            assert_eq!(actual, expected);
        }
        
    }

}
