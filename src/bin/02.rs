const INPUT: &str = include_str!("../../input/02");
const TEST_INPUT: &str = include_str!("../../input/02-test");

fn main() {
    assert_eq!(part1(TEST_INPUT), 8);
    println!("Part 1: {}", part1(INPUT));

    assert_eq!(part2(TEST_INPUT), 2286);
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> u32 {
    parse(input)
        .filter(|game| game.is_possible())
        .map(|game| game.id)
        .sum()
}

fn part2(input: &str) -> u32 {
    parse(input)
        .map(|game| game.max_set())
        .map(|set| set.power())
        .sum()
}

struct Game {
    id: u32,
    revealed: Vec<Set>,
}

impl Game {
    fn is_possible(&self) -> bool {
        self.revealed
            .iter()
            .all(|set| set.r <= 12 && set.g <= 13 && set.b <= 14)
    }

    fn max_set(&self) -> Set {
        self.revealed.iter().fold(Set::default(), |a, b| Set {
            r: a.r.max(b.r),
            g: a.g.max(b.g),
            b: a.b.max(b.b),
        })
    }

    fn parse(line: &str) -> Self {
        let (game, revealed) = line.split_once(": ").unwrap();
        let id = game
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let revealed = revealed.split("; ").map(Set::parse).collect();

        Self { id, revealed }
    }
}

#[derive(Default)]
struct Set {
    r: u32,
    g: u32,
    b: u32,
}

impl Set {
    fn power(&self) -> u32 {
        self.r * self.g * self.b
    }

    fn parse(s: &str) -> Self {
        let colors = s.split(", ");
        let mut result = Self::default();

        for color in colors {
            let (count, color) = color.split_once(" ").unwrap();
            let count = count.parse().unwrap();
            match color {
                "red" => result.r = count,
                "green" => result.g = count,
                "blue" => result.b = count,
                other => panic!("unknown color: {other}"),
            }
        }

        result
    }
}

fn parse(input: &str) -> impl Iterator<Item = Game> + '_ {
    input.lines().map(Game::parse)
}
