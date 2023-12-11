use aoc2023::*;
use itertools::Itertools;

const INPUT: &str = include_str!("../../input/11");

fn main() {
    assert_example!(part1, "11-test", 374);
    println!("Part 1: {}", part1(INPUT));
    assert_example!(part2, "11-test", 82000210);
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    let mut universe = Universe::parse(input);
    universe.expand(2);
    universe.sum_of_distances()
}

fn part2(input: &str) -> usize {
    let mut universe = Universe::parse(input);
    universe.expand(1_000_000);
    universe.sum_of_distances()
}

type Galaxy = Vec2<usize>;

struct Universe {
    galaxies: Vec<Galaxy>,
}

impl Universe {
    fn expand(&mut self, factor: usize) {
        let add = factor - 1;
        let dimensions = self.dimensions();

        // Add additional columns
        for x in (0..=dimensions.x).rev() {
            let any_galaxies = self.galaxies.iter().any(|g| g.x == x);
            if any_galaxies {
                continue;
            }

            self.galaxies
                .iter_mut()
                .filter(|g| g.x > x)
                .for_each(|g| g.x += add);
        }

        // Add additional rows
        for y in (0..=dimensions.y).rev() {
            let any_galaxies = self.galaxies.iter().any(|g| g.y == y);
            if any_galaxies {
                continue;
            }

            self.galaxies
                .iter_mut()
                .filter(|g| g.y > y)
                .for_each(|g| g.y += add);
        }
    }

    fn dimensions(&self) -> Galaxy {
        self.galaxies.iter().fold(Galaxy::default(), |a, b| Galaxy {
            x: a.x.max(b.x),
            y: a.y.max(b.y),
        })
    }

    fn sum_of_distances(&self) -> usize {
        let pairs = self
            .galaxies
            .iter()
            .copied()
            .combinations_with_replacement(2)
            .map(|list| (list[0], list[1]));
        pairs.map(distance).sum()
    }

    fn parse(input: &str) -> Self {
        let galaxies = input.lines().enumerate().flat_map(parse_line).collect();
        Self { galaxies }
    }
}

fn parse_line((line_index, line): (usize, &str)) -> impl Iterator<Item = Galaxy> + '_ {
    line.chars()
        .enumerate()
        .filter(|&(_x, c)| c == '#')
        .map(move |(x, _c)| Vec2::new(x, line_index))
}

/// Taxi cab distance between two galaxies.
fn distance((a, b): (Galaxy, Galaxy)) -> usize {
    a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
}
