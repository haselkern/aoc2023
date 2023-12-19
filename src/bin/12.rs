use aoc2023::*;
use cached::proc_macro::cached;
use itertools::Itertools;
use std::{collections::VecDeque, fmt::Debug, iter};

const INPUT: &str = include_str!("../../input/12");

fn main() {
    assert_example!(part1, "12-test", 21);
    println!("Part 1: {}", part1(INPUT));
    assert_example!(part2, "12-test", 525152);
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(Row::parse)
        .map(possible_arrangements)
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(Row::parse)
        .map(Row::unfold)
        .map(possible_arrangements)
        .sum()
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Row {
    springs: VecDeque<Spring>,
    groups: VecDeque<usize>,
}

impl Row {
    fn parse(line: &str) -> Self {
        let (springs, groups) = line.split_once(' ').unwrap();
        let springs = springs.chars().map(Spring::from).collect();
        let groups = groups.split(',').map(|n| n.parse().unwrap()).collect();
        Self { springs, groups }
    }

    fn unfold(self) -> Self {
        let springs = iter::repeat(self.springs).take(5);
        let springs = Itertools::intersperse(springs, [Spring::Unknown].into())
            .flatten()
            .collect();
        let groups = iter::repeat(self.groups).take(5).flatten().collect();
        Self { springs, groups }
    }
}

#[cached]
fn possible_arrangements(mut row: Row) -> usize {
    // Pop all operational springs on the front and then get the first different one.
    let first = loop {
        match row.springs.pop_front() {
            None => return if row.groups.is_empty() { 1 } else { 0 },
            Some(Spring::Operational) => continue,
            Some(Spring::Broken) => break InterestingSpring::Broken,
            Some(Spring::Unknown) => break InterestingSpring::Unknown,
        }
    };

    match first {
        InterestingSpring::Broken => {
            // A broken spring requires a run that matches the group.
            let Some(mut group) = row.groups.pop_front() else {
                // There was a broken spring but no group
                return 0;
            };

            // We already encountered the first broken spring.
            group -= 1;

            // Pop all required springs or return if impossible.
            loop {
                match row.springs.pop_front() {
                    None if group > 0 => return 0,
                    None => break,
                    Some(Spring::Broken | Spring::Unknown) if group > 0 => group -= 1,
                    Some(Spring::Broken) => return 0,
                    Some(Spring::Operational) if group > 0 => return 0,
                    Some(Spring::Operational | Spring::Unknown) => break,
                }
            }

            possible_arrangements(row)
        }
        InterestingSpring::Unknown => {
            // We don't know what kind of spring is here, recursively try both options.
            let mut a = row.clone();
            let mut b = row;
            a.springs.push_front(Spring::Broken);
            b.springs.push_front(Spring::Operational);
            possible_arrangements(a) + possible_arrangements(b)
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum InterestingSpring {
    Broken,
    Unknown,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Spring {
    Broken,
    Unknown,
    Operational,
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Broken,
            '.' => Self::Operational,
            '?' => Self::Unknown,
            other => panic!("unknown spring '{other}'"),
        }
    }
}
