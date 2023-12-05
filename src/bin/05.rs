use itertools::Itertools;

const INPUT: &str = include_str!("../../input/05");
const TEST_INPUT: &str = include_str!("../../input/05-test");

fn main() {
    assert_eq!(part1(TEST_INPUT), 35);
    println!("Part 1: {}", part1(INPUT));
    assert_eq!(part2(TEST_INPUT), 46);
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> u64 {
    let garden = Garden::parse(input);
    garden
        .simple_seeds
        .iter()
        .copied()
        .map(|seed| garden.map(seed))
        .min()
        .unwrap()
}

fn part2(input: &str) -> u64 {
    0
}

#[derive(Debug)]
struct Garden {
    simple_seeds: Vec<u64>,
    seed_ranges: Vec<SeedRange>,
    maps: Vec<Map>,
}

impl Garden {
    fn parse(input: &str) -> Self {
        let mut blocks = input.split("\n\n");
        let seeds = blocks.next().unwrap().strip_prefix("seeds: ").unwrap();
        let maps = blocks.map(Map::parse).collect();

        let simple_seeds = parse_ws_numbers(seeds);
        let seed_ranges = simple_seeds
            .iter()
            .copied()
            .tuples()
            .map(|(start, length)| SeedRange { start, length })
            .collect();

        Self {
            simple_seeds,
            seed_ranges,
            maps,
        }
    }

    /// Look up n in every map.
    fn map(&self, mut n: u64) -> u64 {
        for map in &self.maps {
            n = map.map(n);
        }
        n
    }
}

#[derive(Debug)]
struct SeedRange {
    start: u64,
    length: u64,
}

#[derive(Debug)]
struct Map {
    ranges: Vec<MappedRange>,
}

impl Map {
    fn map(&self, n: u64) -> u64 {
        for range in &self.ranges {
            if let Some(mapped) = range.map(n) {
                return mapped;
            }
        }

        n
    }

    fn parse(block: &str) -> Self {
        let lines = block.lines().skip(1);
        let ranges = lines.map(MappedRange::parse).collect();
        Self { ranges }
    }
}

#[derive(Debug)]
struct MappedRange {
    destination: u64,
    source: u64,
    length: u64,
}

impl MappedRange {
    fn map(&self, n: u64) -> Option<u64> {
        let source_start = self.source;
        let source_end = source_start + self.length;
        if (source_start..source_end).contains(&n) {
            Some(n - source_start + self.destination)
        } else {
            None
        }
    }

    fn parse(line: &str) -> Self {
        let [destination, source, length] = parse_ws_numbers(line).try_into().unwrap();
        Self {
            destination,
            source,
            length,
        }
    }
}

fn parse_ws_numbers(s: &str) -> Vec<u64> {
    s.split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}
