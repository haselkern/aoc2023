use itertools::assert_equal;

const INPUT: &str = include_str!("../../input/01");
const TEST_INPUT: &str = include_str!("../../input/01-test");
const TEST_INPUT2: &str = include_str!("../../input/01-test-2");

fn main() {
    assert_eq!(part1(TEST_INPUT), 142);
    println!("Part 1: {}", part1(INPUT));

    assert_equal(replace_with_digits("eightwothree"), [8, 2, 3]);
    assert_equal(replace_with_digits("zoneight234"), [1, 8, 2, 3, 4]);

    assert_eq!(part2(TEST_INPUT2), 281);
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(parse_digits)
        .map(calibration_value_from_line)
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(replace_with_digits)
        .map(calibration_value_from_line)
        .sum()
}

fn calibration_value_from_line(line: Vec<u32>) -> u32 {
    let first = line.first().copied().unwrap();
    let last = line.last().copied().unwrap_or(first);
    first * 10 + last
}

/// Parse all digits from the input
fn parse_digits(line: &str) -> Vec<u32> {
    line.chars().flat_map(|c| c.to_digit(10)).collect()
}

/// Replace all words with digits, keep digits and discard everything else.
fn replace_with_digits(line: &str) -> Vec<u32> {
    let replacements = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let mut result = Vec::new();
    let mut buf = line.to_string();

    while !buf.is_empty() {
        if let Some(c) = buf.chars().next() {
            if let Some(num) = c.to_digit(10) {
                result.push(num);
                buf.remove(0);
                continue;
            }
        }

        for (find, replace) in replacements {
            if buf.starts_with(find) {
                result.push(replace);
                buf.remove(0);
                continue;
            }
        }

        buf.remove(0);
    }

    result
}
