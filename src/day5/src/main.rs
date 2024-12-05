
type Rule<'a> = (&'a str, &'a str);

fn collect_rules<'a>(input: &'a str) -> Vec<Rule> {
    let mut rules = Vec::new();
    for line in input.lines() {
        if let Some((l, r)) = line.split_once('|') {
            rules.push((l, r));
        };
    }

    rules
}

fn find_violation<'a>(input: &'a str, rules: &[Rule]) -> Option<(usize, usize)> {
    for (first, later) in rules.iter() {
        match (input.find(first), input.find(later)) {
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

    let rules = collect_rules(rule_input);
    
    pages.lines().filter_map(|line| {
        if find_violation(line, &rules).is_some() {
            None
        } else {
            let mut numbers = line.split(",");
            // Cause apparently Split doesn't override size_hint
            let count = numbers.clone().count();
            numbers.nth(count / 2).map(|s| s.parse::<u32>().expect("should be a number"))
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

    let rules = collect_rules(rule_input);

    pages.lines().filter_map(|line| {
        if let Some((l, r)) = find_violation(&line, &rules) {
            let mut line = line.to_string();
            macro_rules! swap {
                ($l:expr, $r:expr) => {
                    unsafe {
                        // Safety: All valid inputs to the puzzle contain numbers
                        // that are exactly two digits long
                        let b = line.as_bytes_mut();
                        let tmp = b[$l];
                        b[$l] = b[$r];
                        b[$r] = tmp;
                    }
                };
            }

            swap!(l, r);
            swap!(l + 1, r + 1);

            loop {
                if let Some((l, r)) = find_violation(&line, &rules) {
                    swap!(l, r);
                    swap!(l + 1, r + 1);
                } else {
                    let mut numbers = line.split(",");
                    let count = numbers.clone().count();
                    break numbers.nth(count / 2).map(|s| s.parse::<u32>().expect("should be a number"))
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
