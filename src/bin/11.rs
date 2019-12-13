use advent_of_code_2019::coordinates::two_d::Point;
use advent_of_code_2019::coordinates::Grid;
use advent_of_code_2019::cpu::{parse_program, Execution, ExecutionState, Memory};
use advent_of_code_2019::problem::{run, Problem, ProblemState};
use env_logger::Env;
use std::fmt::{Display, Error, Formatter, Write};

struct Eleven {}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum PanelColor {
    Black,
    White,
}

impl Default for PanelColor {
    fn default() -> Self {
        PanelColor::Black
    }
}

impl Display for PanelColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match *self {
            PanelColor::Black => f.write_char('.')?,
            PanelColor::White => f.write_char('#')?,
        }

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Rotate {
    Left,
    Right,
}

impl Problem for Eleven {
    type Input = Memory;
    type Extra = ();

    fn parse(s: &str, _state: &ProblemState<Self::Extra>) -> Self::Input {
        parse_program(s)
    }

    fn part_1(program: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        let mut robot_program = Execution::new(program.clone());
        let mut panels: Grid<(PanelColor, bool)> = Grid::new(0, 0);
        let mut direction = Direction::Up;
        let mut x = 0;
        let mut y = 0;

        while let Some((paint_color, rotation)) = step_robot(&mut robot_program, panels.get(x, y).0)
        {
            panels.set(x, y, (paint_color, true));

            let (next_direction, next_x, next_y) = next_robot_state(direction, rotation, x, y);
            direction = next_direction;
            x = next_x;
            y = next_y;
        }

        let painted_panels = panels
            .enumerate()
            .filter(|(_, (_, painted))| *painted)
            .count();

        Some(format!("{}", painted_panels))
    }

    fn part_2(program: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        let mut robot_program = Execution::new(program.clone());
        let mut panels: Grid<PanelColor> = Grid::new(0, 0);
        let mut direction = Direction::Up;
        let mut x = 0;
        let mut y = 0;

        panels.set(0, 0, PanelColor::White);

        while let Some((paint_color, rotation)) = step_robot(&mut robot_program, *panels.get(x, y))
        {
            panels.set(x, y, paint_color);

            let (next_direction, next_x, next_y) = next_robot_state(direction, rotation, x, y);
            direction = next_direction;
            x = next_x;
            y = next_y;
        }

        Some(render_panels(&panels))
    }

    fn problem_number() -> usize {
        11
    }
}

fn next_robot_state(
    start_direction: Direction,
    rotation: Rotate,
    start_x: isize,
    start_y: isize,
) -> (Direction, isize, isize) {
    let direction = match start_direction {
        Direction::Up => match rotation {
            Rotate::Left => Direction::Left,
            Rotate::Right => Direction::Right,
        },
        Direction::Right => match rotation {
            Rotate::Left => Direction::Up,
            Rotate::Right => Direction::Down,
        },
        Direction::Down => match rotation {
            Rotate::Left => Direction::Right,
            Rotate::Right => Direction::Left,
        },
        Direction::Left => match rotation {
            Rotate::Left => Direction::Down,
            Rotate::Right => Direction::Up,
        },
    };

    let (x, y) = match direction {
        Direction::Up => (start_x, start_y + 1),
        Direction::Right => (start_x + 1, start_y),
        Direction::Down => (start_x, start_y - 1),
        Direction::Left => (start_x - 1, start_y),
    };

    (direction, x, y)
}

fn step_robot(execution: &mut Execution, panel: PanelColor) -> Option<(PanelColor, Rotate)> {
    let input = match panel {
        PanelColor::Black => 0,
        PanelColor::White => 1,
    };

    execution.input.push_back(input);

    let state = execution.run().expect("CPU Error");

    if state == ExecutionState::Halted && execution.output.is_empty() {
        None
    } else {
        let paint_color = match execution.output.pop_front().expect("CPU Error") {
            0 => PanelColor::Black,
            1 => PanelColor::White,

            _ => panic!("Invalid output"),
        };

        let rotate_direction = match execution.output.pop_front().expect("CPU Error") {
            0 => Rotate::Left,
            1 => Rotate::Right,
            _ => panic!("Invalid output"),
        };

        Some((paint_color, rotate_direction))
    }
}

fn render_panels(panels: &Grid<PanelColor>) -> String {
    let mut min_x = std::isize::MAX;
    let mut max_x = std::isize::MIN;
    let mut min_y = std::isize::MAX;
    let mut max_y = std::isize::MIN;

    for (Point { x, y }, color) in panels.enumerate() {
        if PanelColor::White == *color {
            min_x = min_x.min(x);
            max_x = max_x.max(x);
            min_y = min_y.min(y);
            max_y = max_y.max(y);
        }
    }

    let mut output = String::new();
    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            let c = match panels.get(x, y) {
                PanelColor::Black => ' ',
                PanelColor::White => '#',
            };
            output.push(c);
        }
        output.push('\n');
    }

    output
}

fn main() {
    env_logger::init_from_env(Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn"));

    run::<Eleven>((), include_str!("11_input.txt"));
}

#[cfg(test)]
mod eleven {
    use super::*;
    use advent_of_code_2019::problem::assert_solution;

    #[test]
    fn test() {
        assert_solution::<Eleven>(
            include_str!("11_input.txt"),
            (),
            "2415",
            r#"###  #### ###  #  # #### #  # ###   ## 
#  # #    #  # #  #    # #  # #  # #  #
###  ###  #  # #  #   #  #  # #  # #   
#  # #    ###  #  #  #   #  # ###  #   
#  # #    #    #  # #    #  # #    #  #
###  #    #     ##  ####  ##  #     ## 
"#,
        );
    }
}
