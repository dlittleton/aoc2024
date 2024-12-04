use std::{fmt::Debug, str::FromStr};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE_INTEGERS: Regex = Regex::new(r"(-?\d+)").unwrap();
}

pub fn get_all_numbers<T>(s: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    RE_INTEGERS
        .find_iter(s)
        .map(|s| s.as_str().parse::<T>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_all_numbers() {
        assert_eq!(
            get_all_numbers::<i32>("1 2 abc 3 def -5"),
            vec![1, 2, 3, -5]
        );
    }
}
