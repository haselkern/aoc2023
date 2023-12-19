use aoc2023::*;
use std::collections::VecDeque;

const INPUT: &str = include_str!("../../input/12");

fn main() {
    assert_example!(part1, "12-test", 21);
    println!("Part 1: {}", part1(INPUT));
    assert_example!(part2, "12-test", 0);
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(Row::parse)
        .map(Row::possible_arrangements)
        .sum()
}

fn part2(input: &str) -> usize {
    0
}

#[derive(Clone, Debug)]
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

    fn possible_arrangements(mut self) -> usize {
        // Pop all operational springs on the front and then get the first different one.
        let first = loop {
            match self.springs.pop_front() {
                None => return if self.groups.is_empty() { 1 } else { 0 },
                Some(Spring::Operational) => continue,
                Some(Spring::Broken) => break InterestingSpring::Broken,
                Some(Spring::Unknown) => break InterestingSpring::Unknown,
            }
        };

        match first {
            InterestingSpring::Broken => {
                // A broken spring requires a run that matches the group.
                let Some(mut group) = self.groups.pop_front() else {
                    // There was a broken spring but no group
                    return 0;
                };

                // We already encountered the first broken spring.
                group -= 1;

                // Pop all required springs or return if impossible.
                loop {
                    match self.springs.pop_front() {
                        None if group > 0 => return 0,
                        None if group == 0 => break,
                        Some(Spring::Broken | Spring::Unknown) if group > 0 => group -= 1,
                        Some(Spring::Broken) if group == 0 => return 0,
                        Some(Spring::Operational) if group > 0 => return 0,
                        Some(Spring::Operational | Spring::Unknown) => break,
                        other => panic!("group detection weird case {other:?}, group={group}"),
                    }
                }

                self.possible_arrangements()
            }
            InterestingSpring::Unknown => {
                // We don't know what kind of spring is here, recursively try both options.
                let mut a = self.clone();
                let mut b = self;
                a.springs.push_front(Spring::Broken);
                b.springs.push_front(Spring::Operational);
                a.possible_arrangements() + b.possible_arrangements()
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum InterestingSpring {
    Broken,
    Unknown,
}

#[derive(Copy, Clone, Debug)]
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
