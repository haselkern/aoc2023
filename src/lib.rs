use std::fmt::{Debug, Display};
use std::str::FromStr;

/// Parse a whitespace separated list of things.
///
/// Panics on parse error.
pub fn parse_ws_separated<T>(s: &str) -> impl Iterator<Item = T> + '_
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    s.split_ascii_whitespace().map(|s| s.parse().unwrap())
}

/// Concatenate two things.
///
/// Panics if the concatenation is not be parseable.
///
/// ```rust
/// # use aoc2023::concat;
/// assert_eq!(concat(123, 456), 123456);
/// ```
pub fn concat<T>(a: T, b: T) -> T
where
    T: Display + FromStr,
    <T as FromStr>::Err: Debug,
{
    format!("{a}{b}").parse().unwrap()
}
