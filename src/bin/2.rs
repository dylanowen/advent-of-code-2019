use advent_of_code_2019::cpu::{parse_program, Execution, Memory};
use advent_of_code_2019::example;
use advent_of_code_2019::problem::{run, Problem, ProblemState, RunFor};
use env_logger::Env;

struct Two {}

impl Problem for Two {
    type Input = Memory;
    type Extra = ();

    fn parse(s: &str, _state: &ProblemState<Self::Extra>) -> Self::Input {
        parse_program(s)
    }

    fn part_1(input: &Memory, state: &ProblemState<Self::Extra>) -> Option<String> {
        let mut execution: Execution = input.clone().into();

        if !state.is_example {
            execution[1] = 12;
            execution[2] = 2;
        }

        execution.run().expect("This should always work");

        Some(format!("{}", execution[0]))
    }

    fn part_2(input: &Self::Input, state: &ProblemState<Self::Extra>) -> Option<String> {
        let goal = if state.is_example { 1202 } else { 19_690_720 };

        let mut result = 0;
        'outer: for noun in 0..=99 {
            for verb in 0..=99 {
                let mut execution: Execution = input.clone().into();
                execution[1] = noun;
                execution[2] = verb;

                execution.run().expect("This should always work");
                if goal == execution[0] {
                    result = 100 * noun + verb;
                    break 'outer;
                }
            }
        }

        Some(format!("{}", result))
    }

    fn problem_number() -> usize {
        2
    }
}

fn main() {
    env_logger::init_from_env(Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn"));

    example!(Two; RunFor::Part1, (), "1,9,10,3,2,3,11,0,99,30,40,50");
    run::<Two>((), include_str!("2_input.txt"));
}

#[cfg(test)]
mod two {
    use super::*;
    use advent_of_code_2019::problem::assert_solution;

    #[test]
    fn test() {
        assert_solution::<Two>(include_str!("2_input.txt"), (), "6327510", "4112");
    }
}
