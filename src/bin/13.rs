use advent_of_code_2019::coordinates::Grid;
use advent_of_code_2019::cpu::{parse_program, Execution, ExecutionState, Memory};
use advent_of_code_2019::problem::{run, Problem, ProblemState};
use advent_of_code_2019::thirteen::*;
use env_logger::Env;
use log::Level;

struct Thirteen {}

impl Problem for Thirteen {
    type Input = Memory;
    type Extra = ();

    fn parse(s: &str, _state: &ProblemState<Self::Extra>) -> Self::Input {
        parse_program(s)
    }

    fn part_1(program: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        let mut screen: Grid<Tile> = Grid::new(0, 0);

        let mut execution = Execution::new(program.clone());

        execution.run().expect("No errors");
        while !execution.output.is_empty() {
            let x = execution.output.pop_front().expect("Should have output");
            let y = execution.output.pop_front().expect("Should have output");
            let tile_id = Tile::new(execution.output.pop_front().expect("Should have output"));

            screen.set(x as isize, y as isize, tile_id);
        }

        let blocks = screen
            .enumerate()
            .filter(|(_, tile_id)| **tile_id == Tile::Block)
            .count();

        Some(format!("{}", blocks))
    }

    fn part_2(program: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        let mut screen: Grid<Tile> = Grid::new_with_dimensions(0..=44, 0..=45);

        let mut paid_program = program.clone();
        // pay 2 "quarters" for our game
        paid_program[0] = 2;

        let mut execution = Execution::new_input(
            paid_program,
            parse_program(include_str!("../thirteen/13_perfect_game.txt")),
        );

        while execution.run().expect("No errors") != ExecutionState::Halted {
            let (score, _, _) = read_output(&mut execution, &mut screen);

            if log::log_enabled!(Level::Info) {
                screen.print_top_down();
                println!("score: {}", score);
            }

            let joystick_direction;
            loop {
                use std::io;

                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Please enter something");

                joystick_direction = match input.trim() {
                    "a" => -1,
                    "d" => 1,
                    "" | "s" => 0,
                    _ => continue,
                };
                break;
            }

            execution.input.push_back(joystick_direction);
        }

        let (score, _, _) = read_output(&mut execution, &mut screen);

        Some(format!("{}", score))
    }

    fn problem_number() -> usize {
        13
    }
}

fn main() {
    env_logger::init_from_env(Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn"));

    run::<Thirteen>((), include_str!("../thirteen/13_input.txt"));
}

#[cfg(test)]
mod thirteen {
    use super::*;
    use advent_of_code_2019::problem::assert_solution;

    #[test]
    fn test() {
        assert_solution::<Thirteen>(include_str!("../thirteen/13_input.txt"), (), "318", "16309");
    }
}
