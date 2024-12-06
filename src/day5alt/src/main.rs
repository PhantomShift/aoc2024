// Differences from original implementation
// 1) Operation on vectors instead of using the string directly
// for search, swap and indexing operations.
// 2) Since only the middle value of incorrectly-ordered updates
// is necessary, only do swaps such that the middle value's
// ordering becomes valid.
// Surprisingly this approach works on the test and real input
// (I'm too dumb to really elaborate on why)
// yet provides a significant speed up compared to the initial implementation.

use std::collections::HashSet;

type Rule<'a> = (&'a str, &'a str);

fn find_pos<'a, T>(s: &'a [T], f: T) -> Option<usize>
where T: PartialEq
{
    s.iter().position(|v| *v == f)
}

fn collect_rules<'a>(input: &'a str) -> (Vec<Rule>, HashSet<&'a str>) {
    let mut rules = Vec::new();
    let mut set = HashSet::new();
    for line in input.lines() {
        if let Some((l, r)) = line.split_once('|') {
            rules.push((l, r));
            set.insert(l);
            set.insert(r);
        };
    }

    (rules, set)
}

fn find_violation<'a>(input: &[&'a str], rules: &[Rule]) -> Option<(usize, usize)> {
    for (first, later) in rules.iter() {
        match (find_pos(input, first), find_pos(input, later)) {
            (Some(l), Some(r)) if l > r => {
                return Some((l, r))
            }
            _ => ()
        }
    }
    None
}

fn solve_part_one(input: &str) -> u32 {
    let (rule_input, pages) = input.split_once("\n\n")
        .expect("input should contain rules and pages");
    let (rules, _) = collect_rules(rule_input);

    pages.lines().filter_map(|line| {
        let numbers: Vec<_> = line.split(',').collect();
        if find_violation(&numbers, &rules).is_none() {
            Some(numbers[numbers.len() / 2].parse::<u32>().expect("should be a number"))
        } else {
            None
        }
    }).sum()
}

#[test]
fn test_part_one() {
    let input = include_str!("test.txt");
    assert_eq!(143, solve_part_one(input));
}

fn solve_part_two(input: &str) -> u32 {
    let (rule_input, pages) = input.split_once("\n\n")
        .expect("input should contain rules and pages");
    let (rules, set) = collect_rules(rule_input);

    pages.lines().filter_map(|line| {
        let mut numbers: Vec<_> = line.split(',').collect();
        if find_violation(&numbers, &rules).is_some() {
            let mid = numbers.len() / 2;
            loop {
                let middle = numbers[mid];
                if set.contains(middle) {
                    let mut rules_iter = rules.iter();
                    let corrected = loop {
                        match rules_iter.next() {
                            Some(&(first, after)) if first == middle => {
                                match find_pos(&numbers, after) {
                                    Some(pos) if pos > mid => {
                                        numbers.swap(mid, pos);
                                        break true
                                    }
                                    _ => ()
                                }
                            },
                            Some(&(first, after)) if after == middle => {
                                match find_pos(&numbers, first) {
                                    Some(pos) if pos < mid => {
                                        numbers.swap(mid, pos);
                                        break true
                                    }
                                    _ => ()
                                }
                            },
                            None => break false,
                            _ => (),
                        }
                    };
                    if !corrected {
                        break Some(middle.parse::<u32>().expect("should be a number"))
                    }
                }
            }
        } else {
            None
        }
    }).sum()
}

#[test]
fn test_part_two() {
    let input = include_str!("test.txt");
    assert_eq!(123, solve_part_two(input));
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", solve_part_one(input));
    println!("Part 2: {}", solve_part_two(input));
}
