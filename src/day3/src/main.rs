
// Super basic state machine
fn match_mul(input: &[u8], index: usize) -> Option<(usize, String, String)> {
    let mut i = index;
    while i < input.len() {
        let c = input[i];
        if c == b'm' && input[i..i+4] == *b"mul(" {
            let mut l = Vec::new();
            let mut n = i + 4;
            while input[n].is_ascii_digit() {
                l.push(input[n] as char);
                n += 1;
            }
            if l.len() == 0 || input[n] != b',' {
                i = n;
                continue;
            }
            let mut r = Vec::new();
            n += 1;
            while input[n].is_ascii_digit() {
                r.push(input[n] as char);
                n += 1;
            }
            if r.len() == 0 || input[n] != b')' {
                i = n;
                continue;
            }
            return Some((n + 1, l.into_iter().collect(), r.into_iter().collect()))
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

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", solve_part_one(input));
}
