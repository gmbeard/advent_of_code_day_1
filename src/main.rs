use std::env;

mod digits;
mod neighbour;

use digits::*;
use neighbour::*;

fn get_sum(value: &str) -> i32 {
    as_digits(value)
        .filter_map(|n| match n {
            IsDigit::Yes(v) => Some(v),
            IsDigit::No => None
        })
        .as_neighbour()
        .filter_map(|(l, r)| {
            if l == r {
                Some(l)
            }
            else {
                None
            }
        })
        .sum()
}

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();

    for val in args.iter().map(|s| &*s) {
        print!("{}, ", get_sum(val));
    }
}

#[cfg(test)]
mod example_tests {
    use super::*;

    #[test]
    fn test_1122() {
        assert_eq!(3, get_sum("1122"));
    }

    #[test]
    fn test_1234() {
        assert_eq!(0, get_sum("1234"));
    }
    
    #[test]
    fn test_1231() {
        assert_eq!(1, get_sum("1231"));
    }

    #[test]
    fn test_1() {
        assert_eq!(1, get_sum("1"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(0, get_sum(""));
    }
}

