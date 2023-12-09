use aoc2023::*;

const INPUT: &str = include_str!("../../input/09");
const TEST_INPUT: &str = include_str!("../../input/09-test");

fn main() {
    let (test1, test2) = solve(TEST_INPUT);
    let (part1, part2) = solve(INPUT);

    assert_eq!(test1, 114, "Part 1");
    println!("Part 1: {}", part1);
    assert_eq!(test2, 2, "Part 2");
    println!("Part 2: {}", part2);
}

fn solve(input: &str) -> (i64, i64) {
    let (forward, backward): (Vec<i64>, Vec<i64>) = input
        .lines()
        .map(parse_ws_separated)
        .map(extrapolate)
        .unzip();
    (forward.into_iter().sum(), backward.into_iter().sum())
}

/// Extrapolate (forward, backward).
fn extrapolate(mut input: impl Iterator<Item = i64>) -> (i64, i64) {
    let start = input.next().unwrap();

    // We do not need to store all values,
    // only the first and last difference values are important.
    let mut forward = vec![start];
    let mut backward = vec![start];

    // Build the list of differences
    for next in input {
        let mut propagate_diff = next;
        for diff in &mut forward {
            let previous = *diff;
            *diff = propagate_diff;
            propagate_diff -= previous;
        }
        let double_0 = *forward.last().unwrap() == 0 && propagate_diff == 0;
        if !double_0 {
            forward.push(propagate_diff);
            backward.push(propagate_diff);
        }
    }

    // Extrapolate
    let forward = forward.into_iter().sum();
    let backward = backward.into_iter().rev().fold(0, |a, b| b - a);

    (forward, backward)
}
