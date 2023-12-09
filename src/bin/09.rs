use aoc2023::*;

const INPUT: &str = include_str!("../../input/09");
const TEST_INPUT: &str = include_str!("../../input/09-test");

fn main() {
    assert_eq!(part1(TEST_INPUT), 114, "Part 1");
    println!("Part 1: {}", part1(INPUT));
    assert_eq!(part2(TEST_INPUT), 2, "Part 2");
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> i64 {
    input.lines().map(parse_ws_separated).map(extrapolate).sum()
}

fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(parse_ws_separated)
        .map(|numbers| numbers.rev())
        .map(extrapolate)
        .sum()
}

fn extrapolate(mut input: impl Iterator<Item = i64>) -> i64 {
    let start = input.next().unwrap();

    // We do not need to store all values,
    // only the difference values are important.
    let mut differences = vec![start];

    // Build the list of differences
    for next in input {
        let mut propagate_diff = next;
        for diff in &mut differences {
            let previous = *diff;
            *diff = propagate_diff;
            propagate_diff -= previous;
        }
        let double_0 = *differences.last().unwrap() == 0 && propagate_diff == 0;
        if !double_0 {
            differences.push(propagate_diff);
        }
    }

    // Extrapolate
    differences.into_iter().sum()
}
