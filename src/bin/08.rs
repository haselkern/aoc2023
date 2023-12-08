use aoc2023::lcm;
use std::collections::HashMap;

const INPUT: &str = include_str!("../../input/08");
const TEST_INPUT_1: &str = include_str!("../../input/08-test-1");
const TEST_INPUT_2: &str = include_str!("../../input/08-test-2");

fn main() {
    assert_eq!(part1(TEST_INPUT_1), 6, "Part 1");
    println!("Part 1: {}", part1(INPUT));
    assert_eq!(part2(TEST_INPUT_2), 6, "Part 2");
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    let start = "AAA".to_string();
    let end = |location: &str| location == "ZZZ";
    Map::parse(input).count_steps(start, end)
}

fn part2(input: &str) -> usize {
    let map = Map::parse(input);
    let end = |location: &str| location.ends_with('Z');

    map.starting_locations()
        .map(|start| map.count_steps(start, end))
        .reduce(lcm)
        .unwrap()
}

struct Map {
    transitions: HashMap<String, Directions>,
    instructions: Vec<Instruction>,
}

impl Map {
    /// Apply a list of instructions.
    fn follow(&self, from: String) -> String {
        self.instructions
            .iter()
            .fold(from, |location, &instruction| {
                self.step(location, instruction)
            })
    }

    /// Apply a single instruction.
    fn step(&self, from: String, instruction: Instruction) -> String {
        let direction = self.transitions.get(&from).unwrap();
        match instruction {
            Instruction::Left => direction.left.clone(),
            Instruction::Right => direction.right.clone(),
        }
    }

    fn starting_locations(&self) -> impl Iterator<Item = String> + '_ {
        self.transitions
            .keys()
            .filter(|k| k.ends_with('A'))
            .map(|k| k.to_string())
    }

    fn count_steps(&self, start: String, end: impl Fn(&str) -> bool) -> usize {
        let mut location = start;
        let mut steps = 0;

        while !end(&location) {
            location = self.follow(location);
            steps += self.instructions.len();
        }

        steps
    }

    fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        let instructions = lines
            .next()
            .unwrap()
            .chars()
            .map(Instruction::parse)
            .collect();
        let transitions = lines
            .skip(1)
            .map(Transition::parse)
            .map(|t| (t.from, t.directions))
            .collect();
        Self {
            instructions,
            transitions,
        }
    }
}

struct Directions {
    left: String,
    right: String,
}

impl Directions {
    fn parse(tuple: &str) -> Self {
        let (left, right) = tuple
            .strip_prefix('(')
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .split_once(", ")
            .unwrap();
        Self {
            left: left.to_string(),
            right: right.to_string(),
        }
    }
}

#[derive(Copy, Clone)]
enum Instruction {
    Left,
    Right,
}

impl Instruction {
    fn parse(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            other => panic!("unknown instruction '{other}'"),
        }
    }
}

struct Transition {
    from: String,
    directions: Directions,
}

impl Transition {
    fn parse(line: &str) -> Self {
        let (from, directions) = line.split_once(" = ").unwrap();
        Self {
            from: from.to_string(),
            directions: Directions::parse(directions),
        }
    }
}
