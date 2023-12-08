use std::{
    fmt::{Debug, Display},
    ops::{Div, Mul, Rem},
    str::FromStr,
};

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

/// Return the greatest common divisor.
///
/// https://en.wikipedia.org/wiki/Euclidean_algorithm
pub fn gcd<T>(mut a: T, mut b: T) -> T
where
    T: PartialEq + Default + Rem<Output = T> + Copy,
{
    let zero = T::default();
    while b != zero {
        (a, b) = (b, a % b);
    }

    a
}

/// Return the least common multiple.
///
/// https://en.wikipedia.org/wiki/Least_common_multiple
pub fn lcm<T>(a: T, b: T) -> T
where
    T: PartialEq + Default + Rem<Output = T> + Div<Output = T> + Mul<Output = T> + Copy,
{
    let gcd = gcd(a, b);
    a / gcd * b
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(252, 105), 21);
        assert_eq!(gcd(105, 252), 21);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(6, 4), 12);
    }
}
