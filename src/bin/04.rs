use std::collections::HashSet;

const INPUT: &str = include_str!("../../input/04");
const TEST_INPUT: &str = include_str!("../../input/04-test");

fn main() {
    assert_eq!(part1(TEST_INPUT), 13);
    println!("Part 1: {}", part1(INPUT));
    assert_eq!(part2(TEST_INPUT), 30);
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> u64 {
    parse_cards(input).map(|card| card.score()).sum()
}

fn part2(input: &str) -> u64 {
    let mut cards: Vec<Card> = parse_cards(input).collect();

    for i in 0..cards.len() {
        let winning = cards[i].count_winning();
        for win in 1..=winning {
            let win = i + win;
            if win < cards.len() {
                cards[win].copies += cards[i].copies;
            }
        }
    }

    cards.iter().map(|card| card.copies).sum()
}

#[derive(Clone)]
struct Card {
    copies: u64,
    winning: HashSet<u64>,
    owned: HashSet<u64>,
}

impl Card {
    fn count_winning(&self) -> usize {
        self.winning.intersection(&self.owned).count()
    }

    fn score(&self) -> u64 {
        let got_winning = self.count_winning();
        if got_winning > 0 {
            2u64.pow(got_winning as u32 - 1)
        } else {
            0
        }
    }

    fn parse(line: &str) -> Self {
        let (_id, line) = line.split_once(": ").unwrap();
        let (winning, owned) = line.split_once(" | ").unwrap();

        Self {
            copies: 1,
            winning: parse_numbers(winning),
            owned: parse_numbers(owned),
        }
    }
}

fn parse_cards(input: &str) -> impl Iterator<Item = Card> + '_ {
    input.lines().map(Card::parse)
}

fn parse_numbers(line: &str) -> HashSet<u64> {
    line.split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}
