use aoc2023::{concat, parse_ws_separated};
use itertools::Itertools;
use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../../input/06");
const TEST_INPUT: &str = include_str!("../../input/06-test");

fn main() {
    assert_eq!(part1(TEST_INPUT), 288);
    println!("Part 1: {}", part1(INPUT));
    assert_eq!(part2(TEST_INPUT), 71503);
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> u64 {
    parse(input).map(how_to_beat).map(len).product()
}

fn part2(input: &str) -> u64 {
    let race = parse(input).reduce(Race::concat).unwrap();
    let range = how_to_beat(race);
    len(range)
}

/// Range of time the button might be pressed to beat a record.
fn how_to_beat(race: Race) -> RangeInclusive<u64> {
    let time = race.time as f64;
    let record = race.record as f64;

    // The distance travelled d is dependant on the time t the button is pressed: d = time*t - t^2.
    // Solve d = record for min and max t:
    let offset = time * 0.5;
    let add = ((-time * 0.5).powf(2.0) - record).sqrt();
    let min = offset - add;
    let max = offset + add;

    // Shrink the interval to only return times to beat the record.
    let min = min.floor() as u64 + 1;
    let max = max.ceil() as u64 - 1;

    min..=max
}

fn len(range: RangeInclusive<u64>) -> u64 {
    range.try_len().unwrap() as u64
}

struct Race {
    time: u64,
    record: u64,
}

impl Race {
    fn concat(self, other: Self) -> Self {
        Self {
            time: concat(self.time, other.time),
            record: concat(self.record, other.record),
        }
    }
}

fn parse(input: &str) -> impl Iterator<Item = Race> + '_ {
    let mut lines = input.lines();
    let mut parse_next_line =
        |prefix: &str| parse_ws_separated(lines.next().unwrap().trim_start_matches(prefix));
    let times = parse_next_line("Time:      ");
    let records = parse_next_line("Distance:  ");
    times
        .zip(records)
        .map(|(time, record)| Race { time, record })
}
