use itertools::Itertools;
use std::cmp::Ordering;

const INPUT: &str = include_str!("../../input/07");
const TEST_INPUT: &str = include_str!("../../input/07-test");

fn main() {
    assert_eq!(part1(TEST_INPUT), 6440, "Part 1");
    println!("Part 1: {}", part1(INPUT));
    assert_eq!(part2(TEST_INPUT), 5905, "Part 2");
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    solve(input, false)
}

fn part2(input: &str) -> usize {
    solve(input, true)
}

fn solve(input: &str, jokers_are_wildcards: bool) -> usize {
    parse(input, jokers_are_wildcards)
        .sorted()
        .enumerate()
        .map(|(rank0, hand)| (rank0 + 1) * hand.bid)
        .sum()
}

#[derive(Eq, Debug)]
struct Hand {
    cards: [Card; 5],
    bid: usize,
}

impl Hand {
    fn kind(&self) -> HandKind {
        let counted = self
            .cards
            .iter()
            .filter(|card| !card.is_wildcard())
            .counts_by(|card| card.symbol);

        let wildcards = self.cards.iter().filter(|card| card.is_wildcard()).count();
        let same_cards = counted.values().copied().max().unwrap_or(0);
        let pairs = counted.values().copied().filter(|&c| c == 2).count();

        if same_cards == 5
            || same_cards == 4 && wildcards == 1
            || same_cards == 3 && wildcards == 2
            || pairs == 1 && wildcards == 3
            || wildcards >= 4
        {
            HandKind::Five
        } else if same_cards == 4
            || same_cards == 3 && wildcards == 1
            || pairs == 1 && wildcards == 2
            || wildcards >= 3
        {
            HandKind::Four
        } else if same_cards == 3 && pairs == 1
            || pairs == 2 && wildcards == 1
            || pairs == 1 && wildcards == 2
        {
            HandKind::FullHouse
        } else if same_cards == 3 || pairs == 1 && wildcards == 1 || wildcards == 2 {
            HandKind::Three
        } else if pairs == 2 || wildcards == 2 {
            HandKind::TwoPair
        } else if pairs == 1 || wildcards == 1 {
            HandKind::OnePair
        } else {
            HandKind::HighCard
        }
    }

    fn parse(line: &str, jokers_are_wildcards: bool) -> Self {
        let (cards, bid) = line.split_once(' ').unwrap();
        let cards = cards
            .chars()
            .map(|symbol| Card {
                symbol,
                jokers_are_wildcards,
            })
            .collect_vec()
            .try_into()
            .unwrap();
        let bid = bid.parse().unwrap();
        Self { cards, bid }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.kind().cmp(&other.kind()) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => self.cards.iter().cmp(&other.cards),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
struct Card {
    symbol: char,
    jokers_are_wildcards: bool,
}

impl Card {
    fn is_wildcard(&self) -> bool {
        self.jokers_are_wildcards && self.symbol == 'J'
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        let order = |card: &Card| match card.symbol {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' if card.jokers_are_wildcards => 1,
            'J' => 11,
            'T' => 10,
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2,
            other => panic!("unknown card symbol '{other}'"),
        };

        order(self).cmp(&order(other))
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &str, jokers_are_wildcards: bool) -> impl Iterator<Item = Hand> + '_ {
    input
        .lines()
        .map(move |l| Hand::parse(l, jokers_are_wildcards))
}
