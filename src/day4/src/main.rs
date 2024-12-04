fn get_dimensions(s: &str) -> (usize, usize) {
    (
        s.lines().next().map(|l| l.len()).unwrap_or_default(),
        s.split_whitespace().count()
    )
}

#[test]
fn test_get_dimensions() {
    let input = include_str!("test.txt");
    assert_eq!((10, 10), get_dimensions(input));
}

fn solve_part_one(input: &str) -> usize {
    let (max_x, max_y) = get_dimensions(input);
    let flat = input.replace('\n', "");
    let mut total = 0;

    // Horizontal
    for line in input.lines() {
        for x in 0..max_x-3 {
            match &line[x..x+4] {
                "XMAS" | "SAMX" => total += 1,
                _ => ()
            }
        }
    }

    macro_rules! flat_get {
        ($x:expr, $y:expr) => {
            flat.as_bytes()[$x + $y * max_x]
        };
    }

    macro_rules! check_flat {
        ($x0:expr, $y0:expr, $x1:expr, $y1:expr, $x2:expr, $y2:expr, $x3:expr, $y3:expr) => {
            let s = &[
                flat_get!($x0, $y0),
                flat_get!($x1, $y1),
                flat_get!($x2, $y2),
                flat_get!($x3, $y3),
            ];
            match s {
                b"XMAS" | b"SAMX" => total += 1,
                _ => ()
            }
        };
    }

    // Vertical
    for x in 0..max_x {
        for y in 0..max_y-3 {
            check_flat!(
                x, y,
                x, y + 1,
                x, y + 2,
                x, y + 3
            );
        }
    }

    // Diagonal left to right
    for x in 0..max_x-3 {
        for y in 0..max_y-3 {
            check_flat!(
                x, y,
                x + 1, y + 1,
                x + 2, y + 2,
                x + 3, y + 3
            );
        }
    }

    // Diagonal right to left
    for x in 3..max_x {
        for y in 0..max_y-3 {
            check_flat!(
                x, y,
                x - 1, y + 1,
                x - 2, y + 2,
                x - 3, y + 3
            );
        }
    }

    total
}

#[test]
fn test_part_one() {
    let input = include_str!("test.txt");
    assert_eq!(18, solve_part_one(input));
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", solve_part_one(input));
}
