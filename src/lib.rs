pub mod template;

use itertools::Itertools;
use once_cell::sync::Lazy;
use std::str::FromStr;

pub trait StringParsing {
    fn extract_numbers<T>(&self) -> impl Iterator<Item = T>
    where
        T: FromStr,
        T::Err: std::fmt::Debug;
}

impl StringParsing for str {
    fn extract_numbers<T>(&self) -> impl Iterator<Item = T>
    where
        T: FromStr,
        T::Err: std::fmt::Debug,
    {
        static RE: Lazy<regex::Regex> = Lazy::new(|| regex::Regex::new(r"-?\d+").unwrap());
        RE.find_iter(self)
            .map(|x| x.as_str().parse::<T>().unwrap())
            .collect_vec()
            .into_iter()
    }
}

pub fn extract_array<T: FromStr, const N: usize>(s: &str) -> [T; N]
where
    T::Err: std::fmt::Debug,
{
    s.extract_numbers::<T>().collect_array().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tuple() {
        let (u, v) = "123 456".extract_numbers::<i64>().collect_tuple().unwrap();
        assert_eq!(u, 123);
        assert_eq!(v, 456);
    }

    #[test]
    fn test_extract_numbers() {
        let s = "foo 123 bar -456 baz";
        let numbers = s.extract_numbers::<i64>().collect_vec();
        assert_eq!(numbers, vec![123, -456]);
        let (x, y) = s.extract_numbers::<i64>().collect_tuple().unwrap();
        assert_eq!((x, y), (123, -456));
    }
}
