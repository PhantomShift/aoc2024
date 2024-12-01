
fn solve_part_one(input: &str) -> i32 {
    let lines = input.lines()
        .into_iter()
        .map(|s| {
            if let [l, r] = s.trim_end().split("   ").collect::<Vec<_>>()[..] {
                (l.parse().expect("should be a number"), r.parse().expect("should be a number"))
            } else {
                unreachable!()
            }
        });
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for (l, r) in lines {
        left.push(l);
        right.push(r);
    }
    left.sort();
    right.sort();

    left.into_iter().zip(right.into_iter()).map(|(l, r)| l.abs_diff(r) as i32).sum()
}

#[test]
fn test_part_one() {
    let input = include_str!("test.txt");
    assert_eq!(11, solve_part_one(input))
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", solve_part_one(input));
}
