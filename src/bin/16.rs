use advent_of_code_2019::example;
use advent_of_code_2019::problem::{run, Problem, ProblemState, RunFor};
use core::mem;
use env_logger::Env;

struct Sixteen {}

impl Problem for Sixteen {
    type Input = Vec<isize>;
    type Extra = usize;

    fn parse(s: &str, _state: &ProblemState<Self::Extra>) -> Self::Input {
        s.chars()
            .map(|c| c.to_digit(10).expect("Bad input digit") as isize)
            .collect()
    }

    fn part_1(signal: &Self::Input, state: &ProblemState<Self::Extra>) -> Option<String> {
        let result = run_phases(signal.clone(), state.extra);

        let output: String = result.into_iter().take(8).map(|i| i.to_string()).collect();

        Some(output)
    }

    fn part_2(signal: &Self::Input, state: &ProblemState<Self::Extra>) -> Option<String> {
        // grab the first 7 digits of the signal
        let offset = signal
            .iter()
            .take(7)
            .map(|i| i.to_string())
            .collect::<String>()
            .parse::<usize>()
            .expect("Our signal should be all digits");

        // since our offset is assumed to be past the midpoint, we can optimize our algorithm
        let duplicated_signal: Self::Input = {
            let full_signal_len = 10_000 * signal.len();
            let partial_signal_len = full_signal_len - offset;
            let mut needed_duplicates = partial_signal_len / signal.len();
            let duplicates_remainder = partial_signal_len % signal.len();
            if duplicates_remainder > 0 {
                needed_duplicates += 1;
            }

            let mut result = vec![];
            for _ in 0..needed_duplicates {
                result.append(&mut signal.clone());
            }
            result
                .into_iter()
                .skip(signal.len() - duplicates_remainder)
                .collect()
        };

        let result = {
            let phases = state.extra;
            let mut input = duplicated_signal;
            let mut next: Vec<isize> = vec![0; input.len()];

            for _phase in 0..phases {
                let mut sum: isize = input.iter().sum();
                next[0] = sum % 10;
                for i in 1..input.len() {
                    sum -= input[i - 1];

                    next[i] = sum % 10;
                }

                mem::swap(&mut input, &mut next);
            }

            input
        };

        let output: String = result.into_iter().take(8).map(|i| i.to_string()).collect();

        Some(output)
    }

    fn problem_number() -> usize {
        16
    }
}

fn run_phases(signal: <Sixteen as Problem>::Input, phases: usize) -> Vec<isize> {
    let mut input = signal;
    let mut next: Vec<isize> = vec![0; input.len()];
    let mut iterator;

    for _phase in 0..phases {
        for (output_index, next_value) in next.iter_mut().enumerate() {
            iterator = PatternIterator::new(output_index);
            let mut next_digit = 0;
            for value in &input {
                let next_pattern = iterator.next().expect("Our pattern is forever");
                next_digit += value * next_pattern;
            }

            *next_value = next_digit.abs() % 10;
        }

        mem::swap(&mut input, &mut next);
    }

    input
}

struct PatternIterator {
    output_position: usize,
    offset: usize,
}

impl PatternIterator {
    fn new(output_index: usize) -> PatternIterator {
        PatternIterator {
            output_position: output_index + 1,
            offset: 1,
        }
    }
}

impl Iterator for PatternIterator {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        let result = Some(
            match (self.offset % (self.output_position * 4)) / self.output_position {
                0 => 0,
                1 => 1,
                2 => 0,
                3 => -1,
                _ => panic!("Overflowed iterator"),
            },
        );

        self.offset += 1;

        result
    }
}

fn main() {
    env_logger::init_from_env(Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn"));

    example!(Sixteen;
        RunFor::Part1, 4, "12345678",
        RunFor::Part2, 100, "03036732577212944063491565474664",
        RunFor::Part2, 100, "02935109699940807407585447034323",
        RunFor::Part2, 100, "03081770884921959731165446850517"
    );

    run::<Sixteen>(100, include_str!("16_input.txt"));
}

#[cfg(test)]
mod sixteen {
    use super::*;
    use advent_of_code_2019::problem::assert_solution;

    #[test]
    fn test() {
        assert_solution::<Sixteen>(include_str!("16_input.txt"), 100, "25131128", "53201602");
    }

    #[test]
    fn pattern_1() {
        assert_eq!(
            PatternIterator::new(0).take(10).collect::<Vec<isize>>(),
            vec![1, 0, -1, 0, 1, 0, -1, 0, 1, 0]
        )
    }

    #[test]
    fn pattern_2() {
        assert_eq!(
            PatternIterator::new(1).take(10).collect::<Vec<isize>>(),
            vec![0, 1, 1, 0, 0, -1, -1, 0, 0, 1]
        )
    }

    #[test]
    fn pattern_3() {
        assert_eq!(
            PatternIterator::new(2).take(10).collect::<Vec<isize>>(),
            vec![0, 0, 1, 1, 1, 0, 0, 0, -1, -1]
        )
    }
}
