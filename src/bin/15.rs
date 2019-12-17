use advent_of_code_2019::coordinates::two_d::PointLike;
use advent_of_code_2019::coordinates::Grid;
use advent_of_code_2019::cpu::{parse_program, Execution, IntCode, Memory};
use advent_of_code_2019::problem::{run, Problem, ProblemState};
use env_logger::Env;
use log::Level;
use std::collections::LinkedList;
use std::fmt;
use std::fmt::{Display, Formatter, Write};

struct Fifteen {}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Command {
    North,
    South,
    West,
    East,
}

impl Command {
    fn reverse(self) -> Command {
        match self {
            Command::North => Command::South,
            Command::South => Command::North,
            Command::West => Command::East,
            Command::East => Command::West,
        }
    }
}

impl From<Command> for IntCode {
    fn from(command: Command) -> Self {
        match command {
            Command::North => 1,
            Command::South => 2,
            Command::West => 3,
            Command::East => 4,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Status {
    Wall,
    Moved,
    OxygenSystem,
}

impl From<IntCode> for Status {
    fn from(code: IntCode) -> Self {
        match code {
            0 => Status::Wall,
            1 => Status::Moved,
            2 => Status::OxygenSystem,
            _ => panic!("bad status"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum MapBlock {
    Unknown,
    Vacuum(usize),
    Oxygen,
    Wall,
}

impl Default for MapBlock {
    fn default() -> Self {
        MapBlock::Unknown
    }
}

impl Display for MapBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MapBlock::Vacuum(_) => f.write_char(' ')?,
            MapBlock::Oxygen => f.write_char('.')?,
            MapBlock::Wall => f.write_char('#')?,
            MapBlock::Unknown => f.write_char('?')?,
        }

        Ok(())
    }
}

impl Problem for Fifteen {
    type Input = Memory;
    type Extra = ();

    fn parse(s: &str, _state: &ProblemState<Self::Extra>) -> Self::Input {
        parse_program(s)
    }

    fn part_1(program: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        let mut map = Grid::new(0, 0);

        let (oxygen_distance, _) = find_oxygen(program, &mut map);

        if log::log_enabled!(Level::Debug) {
            map.print_bottom_up();
        }

        Some(format!("{}", oxygen_distance))
    }

    fn part_2(program: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        let mut map = Grid::new(0, 0);
        let (_, oxygen_location) = find_oxygen(program, &mut map);

        let mut time = 0;
        let mut next_locations: LinkedList<(isize, isize)> = LinkedList::new();
        let mut last_locations: LinkedList<(isize, isize)> = LinkedList::new();

        map.set(oxygen_location.x(), oxygen_location.y(), MapBlock::Oxygen);
        last_locations.push_back(oxygen_location);

        while !last_locations.is_empty() {
            for location in last_locations.into_iter() {
                for move_delta in [(1, 0), (0, 1), (-1, 0), (0, -1)].iter() {
                    let next_location = location.add(move_delta);

                    // check if our oxygen can move into this location
                    if let MapBlock::Vacuum(_) = map.get(next_location.x(), next_location.y()) {
                        map.set(next_location.x(), next_location.y(), MapBlock::Oxygen);
                        next_locations.push_back(next_location);
                    }
                }
            }

            if log::log_enabled!(Level::Debug) {
                map.print_bottom_up();
            }

            time += 1;
            last_locations = next_locations;
            next_locations = LinkedList::new();
        }

        // if we didn't have anywhere to move to, we need to decrement time
        time -= 1;

        Some(format!("{}", time))
    }

    fn problem_number() -> usize {
        15
    }
}

fn find_oxygen(
    program: &<Fifteen as Problem>::Input,
    map: &mut Grid<MapBlock>,
) -> (usize, (isize, isize)) {
    let mut robot = Execution::new(program.clone());

    map.set(0, 0, MapBlock::Vacuum(0));

    find_oxygen_inner(&mut robot, map, (0, 0), 0).expect("We should have found oxygen")
}

fn find_oxygen_inner(
    robot: &mut Execution,
    map: &mut Grid<MapBlock>,
    location: (isize, isize),
    distance: usize,
) -> Option<(usize, (isize, isize))> {
    let mut result: Option<(usize, (isize, isize))> = None;
    let new_distance = distance + 1;
    for command in [Command::North, Command::South, Command::West, Command::East].iter() {
        if let Some((distance, location)) =
            check_direction(*command, robot, map, location, new_distance)
        {
            if let Some((old_distance, _)) = result {
                result = Some((old_distance.max(distance), location))
            } else {
                result = Some((distance, location))
            }
        }
    }

    result
}

fn check_direction(
    command: Command,
    robot: &mut Execution,
    map: &mut Grid<MapBlock>,
    location: (isize, isize),
    distance: usize,
) -> Option<(usize, (isize, isize))> {
    let (new_location, status) = move_robot(command, robot, location);

    match status {
        Status::Wall => {
            if let MapBlock::Vacuum(_) = map.get(new_location.x(), new_location.y()) {
                panic!("Robot's spatial awareness is wrong");
            }

            map.set(new_location.x(), new_location.y(), MapBlock::Wall);
            None
        }
        Status::Moved | Status::OxygenSystem => {
            let good_path = match map.get(new_location.x(), new_location.y()) {
                MapBlock::Unknown => true,
                MapBlock::Vacuum(old_distance) if *old_distance > distance => true,
                MapBlock::Vacuum(_) => false,
                MapBlock::Wall | MapBlock::Oxygen => panic!(
                    "Found either a wall or oxygen, when there should only be vacuum: {:?}",
                    new_location
                ),
            };

            let path_result = if good_path {
                map.set(
                    new_location.x(),
                    new_location.y(),
                    MapBlock::Vacuum(distance),
                );

                if status == Status::OxygenSystem {
                    Some((distance, new_location))
                } else {
                    find_oxygen_inner(robot, map, new_location, distance)
                }
            } else {
                None
            };

            // since we actually moved, put our robot back where we found it
            move_robot(command.reverse(), robot, new_location);

            path_result
        }
    }
}

fn move_robot(
    command: Command,
    robot: &mut Execution,
    location: (isize, isize),
) -> ((isize, isize), Status) {
    robot.input.push_back(command.into());
    robot.run().expect("The robot should work");
    let status = robot.expect_pop().into();

    let checked_location = match command {
        Command::North => location.add(&(0, 1)),
        Command::South => location.add(&(0, -1)),
        Command::West => location.add(&(-1, 0)),
        Command::East => location.add(&(1, 0)),
    };

    (checked_location, status)
}

fn main() {
    env_logger::init_from_env(Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn"));

    run::<Fifteen>((), include_str!("15_input.txt"));
}

#[cfg(test)]
mod fifteen {
    use super::*;
    use advent_of_code_2019::problem::assert_solution;

    #[test]
    fn test() {
        assert_solution::<Fifteen>(include_str!("15_input.txt"), (), "266", "274");
    }
}
