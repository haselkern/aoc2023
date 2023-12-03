use std::collections::HashMap;

const INPUT: &str = include_str!("../../input/03");
const TEST_INPUT: &str = include_str!("../../input/03-test");

fn main() {
    assert_eq!(part1(TEST_INPUT), 4361);
    println!("Part 1: {}", part1(INPUT));
    assert_eq!(part2(TEST_INPUT), 467835);
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> u64 {
    parse_part_numbers(input)
        .into_iter()
        .map(|part| part.value)
        .sum()
}

fn part2(input: &str) -> u64 {
    let parts_with_gear = parse_part_numbers(input)
        .into_iter()
        .filter(|part| part.symbol == '*');

    let mut gears: HashMap<(usize, usize), Vec<PartNumber>> = HashMap::new();
    for part in parts_with_gear {
        gears.entry(part.symbol_index).or_default().push(part);
    }

    gears.values().flat_map(gear_ratio).sum()
}

/// Return the gear ratio for the parts if there are exactly two parts.
/// Returns None otherwise.
fn gear_ratio(parts: impl AsRef<[PartNumber]>) -> Option<u64> {
    let parts = parts.as_ref();
    if parts.len() == 2 {
        Some(parts.iter().map(|part| part.value).product())
    } else {
        None
    }
}

/// A part number is a number beside a symbol.
struct PartNumber {
    value: u64,
    /// (col, row) where the symbol for this part is.
    symbol_index: (usize, usize),
    symbol: char,
}

fn parse_part_numbers(input: &str) -> Vec<PartNumber> {
    let lines: Vec<&str> = input.lines().collect();

    let mut number = String::new();
    let mut numbers: Vec<PartNumber> = Vec::new();

    for (line_index, line) in lines.iter().enumerate() {
        for (c_index, c) in line.chars().enumerate() {
            if let CharType::Number(c) = c.into() {
                number.push(c);
            }

            let line_end = c_index == line.len() - 1;
            let number_end = match c.into() {
                CharType::Number(_) => false,
                CharType::Empty | CharType::Symbol(_) => true,
            };
            let has_number = !number.is_empty();
            let stop_number_parsing = has_number && (line_end || number_end);
            if !stop_number_parsing {
                continue;
            }

            // Find any symbol around number.
            let x_range = (c_index - number.len()).saturating_sub(1)..=c_index;
            let y_range = line_index.saturating_sub(1)..=(line_index + 1);
            'find_symbol: for x in x_range {
                for y in y_range.clone() {
                    let c = lines.get(y).and_then(|line| line.chars().nth(x));
                    let Some(c) = c else {
                        continue;
                    };

                    if let CharType::Symbol(c) = c.into() {
                        // The number is a part number.
                        numbers.push(PartNumber {
                            value: number.parse().unwrap(),
                            symbol_index: (x, y),
                            symbol: c,
                        });
                        break 'find_symbol;
                    }
                }
            }

            // Clear before next number is parsed.
            number.clear();
        }
    }

    numbers
}

enum CharType {
    Number(char),
    Empty,
    Symbol(char),
}

impl From<char> for CharType {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            c if c.is_numeric() => Self::Number(c),
            other => Self::Symbol(other),
        }
    }
}
