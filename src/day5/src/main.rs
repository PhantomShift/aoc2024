
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

fn solve_part_one(input: &str) -> u32 {
    let (rule_input, pages) = input.split_once("\n\n")
        .expect("input should contain rules and pages");

    let rules = collect_rules(rule_input);
    
    pages.lines().filter_map(|line| {
        for (first, later) in rules.iter() {
            match (line.find(first), line.find(later)) {
                (Some(l), Some(r)) if l > r => {
                    return None
                }
                _ => ()
            }
        }

        let mut numbers = line.split(",");
        // Cause apparently Split doesn't override size_hint
        let count = numbers.clone().count();
        numbers.nth(count / 2).map(|s| s.parse::<u32>().expect("should be a number"))
    }).sum()
}

#[test]
fn test_part_one() {
    let input = include_str!("test.txt");
    assert_eq!(143, solve_part_one(input));
}


fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", solve_part_one(input));
}
