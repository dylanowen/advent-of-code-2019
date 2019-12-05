use advent_of_code_2019::{run, Problem};
use env_logger::Env;
use std::ops::Range;

struct Four {}

impl Problem for Four {
    type Input = Range<usize>;

    fn parse(s: &str) -> Self::Input {
        let ranges = s
            .split('-')
            .map(|n| n.parse::<usize>().expect("parse error"))
            .collect::<Vec<usize>>();

        ranges[0]..ranges[1]
    }

    fn part_1(range: &Self::Input, _name: &str, _is_example: bool) -> Option<String> {
        let valid_numbers = range
            .clone()
            .map(split)
            .filter(|num| has_adjacent_digits(num) && is_increasing(num));

        Some(format!("{}", valid_numbers.count()))
    }

    fn part_2(range: &Self::Input, _name: &str, _is_example: bool) -> Option<String> {
        let valid_numbers = range
            .clone()
            .map(split)
            .filter(|num| has_double_grouping(num) && is_increasing(num));

        Some(format!("{}", valid_numbers.count()))
    }

    fn problem_number() -> usize {
        4
    }
}

fn split(num: usize) -> [usize; 6] {
    [
        (num / 100000) % 10,
        (num / 10000) % 10,
        (num / 1000) % 10,
        (num / 100) % 10,
        (num / 10) % 10,
        num % 10,
    ]
}

fn has_adjacent_digits(num: &[usize; 6]) -> bool {
    let mut last = num[0];
    for digit in &num[1..] {
        if last == *digit {
            return true;
        }
        last = *digit;
    }

    false
}

fn is_increasing(num: &[usize; 6]) -> bool {
    let mut last = num[0];
    for digit in &num[1..] {
        if last > *digit {
            return false;
        }
        last = *digit;
    }

    true
}

fn has_double_grouping(num: &[usize; 6]) -> bool {
    let mut last = num[0];
    let mut group_size = 1;
    for digit in &num[1..] {
        if last == *digit {
            group_size += 1;
        } else if group_size == 2 {
            // we found a double
            return true;
        } else {
            group_size = 1;
        }
        last = *digit;
    }

    group_size == 2
}

fn main() {
    env_logger::init_from_env(Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn"));

    run::<Four>(false, "278384-824795");
}

#[cfg(test)]
mod four {
    use super::*;

    use advent_of_code_2019::assert_solution;

    #[test]
    fn test() {
        assert_solution::<Four>("278384-824795", "921", "603");
    }

    #[test]
    fn part_1() {
        let mut num = split(111111);
        assert_eq!(true, has_adjacent_digits(&num) && is_increasing(&num));

        num = split(223450);
        assert_eq!(false, has_adjacent_digits(&num) && is_increasing(&num));

        num = split(123789);
        assert_eq!(false, has_adjacent_digits(&num) && is_increasing(&num));
    }

    #[test]
    fn part_2() {
        let mut num = split(112233);
        assert_eq!(true, has_double_grouping(&num) && is_increasing(&num));

        num = split(123444);
        assert_eq!(false, has_double_grouping(&num) && is_increasing(&num));

        num = split(111122);
        assert_eq!(true, has_double_grouping(&num) && is_increasing(&num));
    }
}
