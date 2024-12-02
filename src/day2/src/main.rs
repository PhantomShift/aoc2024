
fn is_safe(s: &[i8]) -> bool {
    let mut w = s.windows(2);
    match w.next() {
        Some(first) if (1..=3).contains(&first[0].abs_diff(first[1])) => {
            let sign = (first[0] - first[1]).signum();
            for pair in w {
                let diff = pair[0] - pair[1];
                if diff.signum() != sign || !(1..=3).contains(&diff.abs()) {
                    return false
                }
            }
        }
        _ => {
            return false
        }
    }
    true
}

fn solve_part_one(input: &str) -> usize {
    input.lines()
        .filter(|line| !line.trim().is_empty())
        .filter(|line| {
            let v: Vec<_> = line.split_ascii_whitespace()
                .map(|s| s.parse::<i8>().expect("should only be numbers"))
                .collect();
            is_safe(&v)
        })
        .count()
}

#[test]
fn test_part_one() {
    let input = include_str!("test.txt");
    assert_eq!(2, solve_part_one(input));
}

fn solve_part_two(input: &str) -> usize {
    input.lines()
        .filter(|line| {
            let v: Vec<_> = line.split_ascii_whitespace()
                .map(|s| s.parse::<i8>().expect("should only be numbers"))
                .collect();

            if !is_safe(&v) {
                // Brute force approach because I'm too lazy to consider
                // how to solve this cleverly
                (0..v.len()).any(|i| {
                    is_safe(&[&v[..i], &v[i+1..]].concat())
                })
            } else {
                true
            }
        })
        .count()
}

#[test]
fn test_part_two() {
    let input = include_str!("test.txt");
    assert_eq!(4, solve_part_two(input));
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", solve_part_one(input));
    println!("Part 2: {}", solve_part_two(input));
}
