use advent_of_code_2019::coordinates::Grid;
use advent_of_code_2019::cpu::{parse_program, Execution, ExecutionState, IntCode, Memory};
use advent_of_code_2019::problem::{run, Problem, ProblemState};
use advent_of_code_2019::thirteen::*;
use env_logger::Env;
use log::Level;
use std::fmt;
use std::fmt::{Display, Error, Formatter, Write};

struct Nineteen {}

#[derive(Debug, Copy, Clone, PartialEq)]
enum BeamEffect {
    Stationary,
    Pulled,
}

impl Default for BeamEffect {
    fn default() -> Self {
        BeamEffect::Stationary
    }
}

impl Display for BeamEffect {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            BeamEffect::Stationary => f.write_char('.'),
            BeamEffect::Pulled => f.write_char('#'),
        }
    }
}

impl Problem for Nineteen {
    type Input = Memory;
    type Extra = ();

    fn parse(s: &str, _state: &ProblemState<Self::Extra>) -> Self::Input {
        parse_program(s)
    }

    fn part_1(program: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        let mut tractor_map = Grid::new_with_dimensions(0..=50, 0..=50);

        for y in tractor_map.y_min()..tractor_map.y_max() {
            for x in tractor_map.x_min()..tractor_map.x_max() {
                tractor_map.set(x, y, read_beam(x, y, program));
            }
        }

        let affected_area = tractor_map
            .enumerate()
            .filter(|(_, &effect)| effect == BeamEffect::Pulled)
            .count();

        tractor_map.print_top_down();

        Some(affected_area.to_string())
    }

    fn part_2(program: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        None
    }

    fn problem_number() -> usize {
        19
    }
}

fn read_beam(x: isize, y: isize, program: &Memory) -> BeamEffect {
    let mut drone = Execution::new(program.clone());

    drone.input.push_back(x as IntCode);
    drone.input.push_back(y as IntCode);

    drone.run().expect("This program should run");

    if drone.expect_pop() == 1 {
        BeamEffect::Pulled
    } else {
        BeamEffect::Stationary
    }
}

fn main() {
    env_logger::init_from_env(Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn"));

    run::<Nineteen>((), include_str!("19_input.txt"));
}

#[cfg(test)]
mod nineteen {
    use super::*;
    use advent_of_code_2019::problem::assert_solution;

    #[test]
    fn test() {
        assert_solution::<Nineteen>(include_str!("19_input"), (), "318", "16309");
    }
}
