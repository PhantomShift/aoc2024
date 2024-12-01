use std::collections::HashMap;


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

fn solve_part_two(input: &str) -> u32 {
    let lines = input.lines()
        .into_iter()
        .map(|s| {
            let mut s = s.trim_end().split("   ");
            (s.next().expect("left side should exist"), s.next().expect("right side should exist"))
        });
    let mut left_count = HashMap::new();
    let mut right_count = HashMap::new();
    for (l, r) in lines {
        left_count.entry(l).and_modify(|c: &mut u32| *c += 1).or_insert(1);
        right_count.entry(r).and_modify(|c: &mut u32| *c += 1).or_insert(1);
    }

    left_count.into_iter().map(|(key, count)| {
        let id: u32 = key.parse().expect("should be a number");
        let r = right_count.get(key).map(|v| v.to_owned()).unwrap_or_default();
        id * count * r
    }).sum()
}

#[test]
fn test_part_two() {
    let input = include_str!("test.txt");
    assert_eq!(31, solve_part_two(input))
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", solve_part_one(input));
    println!("Part 2: {}", solve_part_two(input));
}
