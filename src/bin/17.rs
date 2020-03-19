use advent_of_code_2019::coordinates::two_d::{Point, PointLike};
use advent_of_code_2019::coordinates::Grid;
use advent_of_code_2019::cpu::{parse_program, Execution, ExecutionState, IntCode, Memory};
use advent_of_code_2019::example;
use advent_of_code_2019::problem::{run, Problem, ProblemState, RunFor};
use core::mem;
use env_logger::Env;
use log::Level;
use std::fmt;
use std::fmt::{Display, Formatter, Write};
use wasm_bindgen::__rt::std::collections::VecDeque;

struct Seventeen {}

#[derive(Debug, Copy, Clone, PartialEq)]
enum MapBlock {
    EmptySpace,
    Scaffold,
    Robot(RobotState),
}

impl MapBlock {
    fn new(input: char) -> MapBlock {
        match input {
            '#' => MapBlock::Scaffold,
            '.' => MapBlock::EmptySpace,
            _ => MapBlock::Robot(RobotState::new(input)),
        }
    }
}

impl Default for MapBlock {
    fn default() -> Self {
        MapBlock::EmptySpace
    }
}

impl Display for MapBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MapBlock::EmptySpace => f.write_char('.')?,
            MapBlock::Scaffold => f.write_char('#')?,
            MapBlock::Robot(direction) => direction.fmt(f)?,
        }

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum RobotState {
    North,
    East,
    South,
    West,
    Fallen,
}

impl RobotState {
    fn new(input: char) -> RobotState {
        match input {
            '^' => RobotState::North,
            '>' => RobotState::East,
            'v' => RobotState::South,
            '<' => RobotState::West,
            'X' => RobotState::Fallen,
            _ => panic!("Invalid robot direction"),
        }
    }
}

impl Display for RobotState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RobotState::North => f.write_char('^')?,
            RobotState::East => f.write_char('>')?,
            RobotState::South => f.write_char('v')?,
            RobotState::West => f.write_char('<')?,
            RobotState::Fallen => f.write_char('X')?,
        }

        Ok(())
    }
}

impl Problem for Seventeen {
    type Input = Memory;
    type Extra = ();

    fn parse(s: &str, _state: &ProblemState<Self::Extra>) -> Self::Input {
        parse_program(s)
    }

    fn part_1(program: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        let map = load_map(program);

        if log::log_enabled!(Level::Debug) {
            map.print_bottom_up();
        }

        let mut alignment_parameters = 0;
        for (location, value) in map
            .enumerate()
            .filter(|(_, &value)| value != MapBlock::EmptySpace)
        {
            if [(1, 0), (0, 1), (-1, 0), (0, -1)].iter().all(|p| {
                let test_location = location.add(p);
                *map.get(test_location.x(), test_location.y()) != MapBlock::EmptySpace
            }) {
                alignment_parameters += location.x() * (map.height() as isize - location.y() - 1);
            }
        }

        Some(alignment_parameters.to_string())
    }

    fn part_2(signal: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        None
    }

    fn problem_number() -> usize {
        17
    }
}

fn load_map(program: &Memory) -> Grid<MapBlock> {
    let mut execution = Execution::new(program.clone());
    assert_eq!(
        execution.run().expect("Our program should work"),
        ExecutionState::Halted
    );

    //    let output: Vec<IntCode> = r#"..#..........
    //..#..........
    //#######...###
    //#.#...#...#.#
    //#############
    //..#...#...#..
    //..#####...^.."#
    //        .chars()
    //        .map(|c| c as IntCode)
    //        .collect();
    let output = execution.output;
    let width = output.iter().position(|&b| b == 10).unwrap() as isize;
    let height = output.len() as isize / (width + 1);

    let mut map = Grid::new_with_dimensions(0..=width, 0..=height);

    //    println!("{},{}", width, height);

    let mut x = 0;
    let mut y = height;
    for raw_block in output.iter().map(|&b| b as u8 as char) {
        match raw_block {
            '\n' => {
                //                println!("{},{}", x, y);
                x = 0;
                y -= 1;
            }
            _ => {
                map.set(x, y, MapBlock::new(raw_block));
                x += 1;
            }
        }
    }

    //    println!("{},{}", x, y);
    //    println!("{},{}", map.width(), map.height());

    map
}
//
//fn update_map(output: &VecDeque<IntCode>)

fn main() {
    env_logger::init_from_env(Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn"));

    run::<Seventeen>((), include_str!("17_input.txt"));
}

#[cfg(test)]
mod seventeen {
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
