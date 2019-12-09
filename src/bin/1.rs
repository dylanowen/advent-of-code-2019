use advent_of_code_2019::{example, run, Problem, ProblemState, RunFor};
use env_logger::Env;

struct One {}

impl Problem for One {
    type Input = Vec<isize>;
    type Extra = ();

    fn parse(s: &str, _state: &ProblemState<Self::Extra>) -> Self::Input {
        s.split('\n')
            .map(|mass| mass.parse::<isize>().expect("parse error"))
            .collect()
    }

    fn part_1(modules_mass: &Vec<isize>, _state: &ProblemState<Self::Extra>) -> Option<String> {
        let fuel_requirements: isize = modules_mass.iter().map(|mass| mass / 3 - 2).sum();

        Some(format!("{}", fuel_requirements))
    }

    fn part_2(modules_mass: &Vec<isize>, _state: &ProblemState<Self::Extra>) -> Option<String> {
        let fuel_requirements: isize = modules_mass
            .iter()
            .map(|mass| {
                let mut result = 0;
                let mut last_fuel = mass / 3 - 2;
                while last_fuel > 0 {
                    result += last_fuel;
                    last_fuel = last_fuel / 3 - 2;
                }

                result
            })
            .sum();

        Some(format!("{}", fuel_requirements))
    }

    fn problem_number() -> usize {
        1
    }
}

fn main() {
    env_logger::init_from_env(Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn"));

    example!(One; RunFor::Both, (), "100756");
    run::<One>((), include_str!("1_input.txt"));
}

#[cfg(test)]
mod one {
    use super::*;
    use advent_of_code_2019::assert_solution;

    #[test]
    fn test() {
        assert_solution::<One>(include_str!("1_input.txt"), (), "3361299", "5039071");
    }
}
