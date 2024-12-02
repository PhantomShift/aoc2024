
fn solve_part_one(input: &str) -> usize {
    input.lines()
        .filter(|line| !line.trim().is_empty())
        .filter(|line| {
            let v: Vec<_> = line.split_ascii_whitespace()
                .map(|s| s.parse::<i8>().expect("should only be numbers"))
                .collect();
            let mut w = v.windows(2);
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
        })
        .count()
}

#[test]
fn test_part_one() {
    let input = include_str!("test.txt");
    assert_eq!(2, solve_part_one(input));
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", solve_part_one(input));
}
