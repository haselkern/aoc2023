//! This library contains useful helper functions that may be useful in several problems.

use std::ops::{Add, AddAssign};
use std::{
    fmt::{Debug, Display},
    ops::{Div, Mul, Rem},
    str::FromStr,
};

/// Parse a whitespace separated list of things.
///
/// This works with any type that implements [`FromStr`].
///
/// Panics on parse error to keep things simple.
///
/// ```rust
/// # use aoc2023::parse_ws_separated;
/// let mut nums = parse_ws_separated("1 2 3");
/// assert_eq!(nums.next(), Some(1));
/// assert_eq!(nums.next(), Some(2));
/// assert_eq!(nums.next(), Some(3));
/// assert_eq!(nums.next(), None);
/// ```
///
/// ```rust
/// # use aoc2023::parse_ws_separated;
/// # use std::net::Ipv4Addr;
/// let mut ips = parse_ws_separated("127.0.0.1      10.0.0.1");
/// assert_eq!(ips.next(), Some(Ipv4Addr::new(127, 0, 0, 1)));
/// assert_eq!(ips.next(), Some(Ipv4Addr::new(10, 0, 0, 1)));
/// assert_eq!(ips.next(), None);
/// ```
pub fn parse_ws_separated<T>(s: &str) -> impl DoubleEndedIterator<Item = T> + '_
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    s.split_ascii_whitespace().map(|s| s.parse().unwrap())
}

/// Concatenate two things.
///
/// Panics if the concatenation is not be parseable to keep things simple.
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
/// <https://en.wikipedia.org/wiki/Euclidean_algorithm>
///
/// ```rust
/// # use aoc2023::gcd;
/// assert_eq!(gcd(12, 8), 4);
/// # assert_eq!(gcd(105, 252), 21);
/// # assert_eq!(gcd(252, 105), 21);
/// ```
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
/// <https://en.wikipedia.org/wiki/Least_common_multiple>
///
/// ```rust
/// # use aoc2023::lcm;
/// assert_eq!(lcm(4, 6), 12);
/// # assert_eq!(lcm(6, 4), 12);
/// ```
pub fn lcm<T>(a: T, b: T) -> T
where
    T: PartialEq + Default + Rem<Output = T> + Div<Output = T> + Mul<Output = T> + Copy,
{
    let gcd = gcd(a, b);
    a / gcd * b
}

/// Given a function and a name of a file in the `input` directory,
/// assert that the function applied to the contents of the file returns the expected result.
/// ```
#[macro_export]
macro_rules! assert_example {
    ($solve:ident, $file:expr, $expected:expr) => {
        assert_eq!(
            $solve(include_str!(concat!("../../input/", $file))),
            $expected,
            "{}, {}",
            stringify!($solve),
            $file,
        )
    };
}

#[derive(Default, Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Vec2<T>
where
    T: Ord,
{
    /// Returns the component-wise maximum of this vector and the other.
    ///
    /// ```rust
    /// # use aoc2023::Vec2;
    /// let a = Vec2::new(1, 5);
    /// let b = Vec2::new(2, -5);
    /// assert_eq!(a.max(b), Vec2::new(2, 5));
    /// ```
    pub fn max(self, other: Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }
}

impl<T> Add for Vec2<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> AddAssign for Vec2<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
