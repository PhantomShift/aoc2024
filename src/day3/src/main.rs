
enum Instruction {
    Do,
    Dont,
    Mul(u32, u32),
}


// Super basic state machine
fn match_mul(input: &[u8], index: usize) -> Option<(usize, String, String)> {
    let mut i = index;
    while i < input.len() {
        let c = input[i];
        if c == b'm' && input[i..i+4] == *b"mul(" {
            let mut l = String::new();
            let mut n = i + 4;
            while input[n].is_ascii_digit() {
                l.push(input[n] as char);
                n += 1;
            }
            if l.len() == 0 || input[n] != b',' {
                i = n;
                continue;
            }
            let mut r = String::new();
            n += 1;
            while input[n].is_ascii_digit() {
                r.push(input[n] as char);
                n += 1;
            }
            if r.len() == 0 || input[n] != b')' {
                i = n;
                continue;
            }
            return Some((n + 1, l, r))
        }
        i += 1;
    }
    return None
}

fn solve_part_one(input: &str) -> u32 {
    let mut sum = 0;
    let mut index = 0;
    while let Some((i, l, r)) = match_mul(input.as_bytes(), index) {
        sum += l.parse::<u32>().expect("should be ascii digits") * r.parse::<u32>().expect("should be ascii digits");

        index = i;
    }

    sum
}

#[test]
fn test_part_one() {
    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    assert_eq!(161, solve_part_one(input));
}


fn match_instruction(input: &[u8], index: usize, enable: bool) -> Option<(usize, Instruction)> {
    let mut i = index;
    while i < input.len() {
        match input[i] {
            b'd' if !enable && input[i..i+4] == *b"do()" => {
                return Some((i + 4, Instruction::Do))
            },
            b'd' if enable && input[i..i+7] == *b"don't()" => {
                return Some((i + 7, Instruction::Dont))
            },
            b'm' if enable && input[i..i+4] == *b"mul(" => {
                // Just copy and paste because
                // I don't want to bother re-organizing to reuse
                let mut l = String::new();
                let mut n = i + 4;
                while input[n].is_ascii_digit() {
                    l.push(input[n] as char);
                    n += 1;
                }
                if l.len() == 0 || input[n] != b',' {
                    i = n;
                    continue;
                }
                let mut r = String::new();
                n += 1;
                while input[n].is_ascii_digit() {
                    r.push(input[n] as char);
                    n += 1;
                }
                if r.len() == 0 || input[n] != b')' {
                    i = n;
                    continue;
                }
                return Some((n + 1, Instruction::Mul(
                    l.parse().expect("should be ascii digits"),
                    r.parse().expect("should be ascii digits"),
                )))
            }
            _ => i += 1,
        }
    }

    None
}

fn solve_part_two(input: &str) -> u32 {
    let mut sum = 0;
    let mut index = 0;
    let mut enable = true;
    while let Some((i, instr)) = match_instruction(input.as_bytes(), index, enable) {
        match instr {
            Instruction::Do => enable = true,
            Instruction::Dont => enable = false,
            Instruction::Mul(l, r) => {
                sum += l * r;
            }
        }

        index = i;
    }

    sum
}

#[test]
fn test_part_two() {
    let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    assert_eq!(48, solve_part_two(input));
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", solve_part_one(input));
    println!("Part 2: {}", solve_part_two(input));
}
