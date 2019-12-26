use advent_of_code_2019::cpu::{parse_program, Execution, IntCode, Memory};
use advent_of_code_2019::problem::{run, Problem, ProblemState};
use env_logger::Env;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashSet, VecDeque};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::mem;

static DANGEROUS_ITEMS: [&str; 5] = [
    "infinite loop",
    "photons",
    "giant electromagnet",
    "molten lava",
    "escape pod",
];

struct TwentyFive {}

impl Problem for TwentyFive {
    type Input = Memory;
    type Extra = bool;

    fn parse(s: &str, _state: &ProblemState<Self::Extra>) -> Self::Input {
        parse_program(s)
    }

    fn part_1(program: &Self::Input, state: &ProblemState<Self::Extra>) -> Option<String> {
        let mut explorer = Explorer::new(program);

        let last_move = explorer.collect_all_items(None);

        explorer.go_to_room("security checkpoint", Some(last_move));

        let response = explorer.try_items(Direction::North);

        if state.extra {
            loop {
                use std::io;

                println!("{:?}", explorer.path);

                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Please enter something");

                let response = explorer.send_command(Some(input.trim()));
                println!("{}", response);
            }
        }

        Some(response)
    }

    fn part_2(_program: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        None
    }

    fn problem_number() -> usize {
        25
    }
}

struct Explorer {
    execution: Execution,
    last_room: String,
    path: Vec<Direction>,
    moved: bool,
}

impl Explorer {
    fn new(program: &<TwentyFive as Problem>::Input) -> Explorer {
        Explorer {
            execution: Execution::new(program.clone()),
            last_room: String::new(),
            path: vec![],
            moved: false,
        }
    }

    fn try_items(&mut self, try_direction: Direction) -> String {
        let inventory_response = self.send_command(Some("inv"));
        if let Response::Inventory(_, all_items) = inventory_response {
            for i in 1..all_items.len() {
                for items in all_items.iter().combinations(i) {
                    for item in items.iter() {
                        let drop_command = format!("drop {}", item);
                        let drop_response = self.send_command(Some(&drop_command));
                        log::trace!("{}", drop_response);
                    }

                    let move_response = self.send_command(Some(try_direction.command()));
                    if let Response::Unknown(output) = move_response {
                        // assume we don't know the response once we open our door
                        return output;
                    } else {
                        log::trace!("{}", move_response);
                    }

                    for item in items.iter() {
                        let take_response = self.send_command(Some(&format!("take {}", item)));
                        log::trace!("{}", take_response);
                    }
                }
            }

            panic!("Couldn't open the door");
        } else {
            panic!("Unknown inventory response {:?}", inventory_response);
        }
    }

    fn collect_all_items(&mut self, maybe_previous_move: Option<Moved>) -> Moved {
        let mut last_move = self.extract_last_move(maybe_previous_move);

        let mut explored = HashSet::new();
        for &next_direction in last_move.directions.clone().iter() {
            if let Some(next_last_move) = self.collect_all_recursive(next_direction, &mut explored)
            {
                last_move = next_last_move;
            }
        }

        last_move
    }

    fn collect_all_recursive(
        &mut self,
        direction: Direction,
        explored: &mut HashSet<Vec<Direction>>,
    ) -> Option<Moved> {
        let response = self.send_command(Some(direction.command()));

        if let Response::Moved(_, moved) = response {
            if !explored.contains(&self.path) {
                explored.insert(self.path.clone());

                for item in moved
                    .items
                    .iter()
                    .filter(|&item| !DANGEROUS_ITEMS.iter().any(|danger| item == danger))
                {
                    self.send_command(Some(&format!("take {}", item)));
                }

                for &next_direction in moved.directions.iter() {
                    self.collect_all_recursive(next_direction, explored);
                }
            }

            // go back to where we came from
            if let Response::Moved(_, last_moved) =
                self.send_command(Some(direction.reverse().command()))
            {
                Some(last_moved)
            } else {
                None
            }
        } else {
            println!("{}", response);
            None
        }
    }

    fn go_to_room(&mut self, room: &str, maybe_previous_move: Option<Moved>) -> Moved {
        let last_move = self.extract_last_move(maybe_previous_move);

        let mut explored = HashSet::new();
        for &next_direction in last_move.directions.iter() {
            if let Some(next_last_move) =
                self.go_to_room_recursive(room, next_direction, &mut explored)
            {
                return next_last_move;
            }
        }

        panic!("Couldn't find room: {}", room);
    }

    fn go_to_room_recursive(
        &mut self,
        room: &str,
        direction: Direction,
        explored: &mut HashSet<Vec<Direction>>,
    ) -> Option<Moved> {
        let response = self.send_command(Some(direction.command()));

        if let Response::Moved(_, moved) = response {
            if room == moved.name.to_lowercase() {
                return Some(moved);
            }

            if !explored.contains(&self.path) {
                explored.insert(self.path.clone());

                for &next_direction in moved.directions.iter() {
                    if let Some(matched) = self.go_to_room_recursive(room, next_direction, explored)
                    {
                        return Some(matched);
                    }
                }
            }

            // go back to where we came from
            self.send_command(Some(direction.reverse().command()));
        } else {
            println!("{}", response);
        }

        None
    }

    fn check_movement(&mut self, command: &str, response: &Response) {
        self.moved = false;

        if let Some(direction) = Direction::new(command) {
            if let Response::Moved(_, moved) = &response {
                if moved.name != self.last_room {
                    if !self.path.is_empty() && self.path.last() == Some(&direction.reverse()) {
                        self.path.pop();
                    } else {
                        self.path.push(direction);
                    }

                    self.last_room = moved.name.clone();

                    self.moved = true;
                } else {
                    self.moved = false;
                }
            }
        }
    }

    fn send_command(&mut self, maybe_command: Option<&str>) -> Response {
        if let Some(command) = maybe_command {
            let mut raw_command: VecDeque<IntCode> =
                command.chars().map(|c| c as IntCode).collect();
            raw_command.push_back('\n' as IntCode);

            self.execution.input.append(&mut raw_command);
        }

        self.execution.run().ok();

        let response = Response::parse(&mut self.execution);

        if let Some(command) = maybe_command {
            self.check_movement(command, &response);
        }

        response
    }

    fn extract_last_move(&mut self, maybe_previous_move: Option<Moved>) -> Moved {
        maybe_previous_move.unwrap_or_else(|| {
            let response = self.send_command(None);

            if let Response::Moved(_, moved) = response {
                moved
            } else {
                panic!("We don't know how to start");
            }
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn new(s: &str) -> Option<Direction> {
        match s {
            "north" => Some(Direction::North),
            "east" => Some(Direction::East),
            "south" => Some(Direction::South),
            "west" => Some(Direction::West),
            _ => None,
        }
    }

    fn reverse(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

    fn command(self) -> &'static str {
        match self {
            Direction::North => "north",
            Direction::East => "east",
            Direction::South => "south",
            Direction::West => "west",
        }
    }
}

#[derive(Debug, Clone)]
enum Response {
    Moved(String, Moved),
    Inventory(String, Vec<String>),
    Impassable(String),
    Unknown(String),
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Response::Moved(output, _) => write!(f, "{}", output),
            Response::Inventory(output, _) => write!(f, "{}", output),
            Response::Impassable(output) => write!(f, "{}", output),
            Response::Unknown(output) => write!(f, "{}", output),
        }
    }
}

impl Response {
    fn parse(execution: &mut Execution) -> Response {
        lazy_static! {
            static ref MOVED_RE: Regex = Regex::new(r"== ([^=]+) ==\n(.+)\n\nDoors here lead:\n((?:- \w+\n)+)(?:\nItems here:\n((?:- .+\n)+))?\nCommand\?").unwrap();
            static ref INVENTORY_RE: Regex = Regex::new(r"Items in your inventory:\n((?:- .+\n)+)\nCommand\?").unwrap();
            static ref LIST_RE: Regex = Regex::new(r"- (.+)").unwrap();
        }

        let output = mem::replace(&mut execution.output, VecDeque::new())
            .iter()
            .map(|&i| i as u8 as char)
            .collect::<String>();

        if let Some(parsed) = MOVED_RE.captures(&output) {
            let directions = parsed[3]
                .split('\n')
                .filter_map(|row| {
                    LIST_RE
                        .captures(row)
                        .and_then(|parsed_row| Direction::new(&parsed_row[1]))
                })
                .collect();

            let items = parsed
                .get(4)
                .map(|found_items| {
                    found_items
                        .as_str()
                        .split('\n')
                        .filter_map(|row| {
                            LIST_RE
                                .captures(row)
                                .map(|parsed_row| parsed_row[1].trim().to_string())
                        })
                        .collect()
                })
                .unwrap_or_else(|| vec![]);

            let moved = Moved {
                name: parsed[1].to_string(),
                description: parsed[2].to_string(),
                directions,
                items,
            };

            Response::Moved(output, moved)
        } else if let Some(parsed) = INVENTORY_RE.captures(&output) {
            let items = parsed[1]
                .split('\n')
                .filter_map(|row| {
                    LIST_RE
                        .captures(row)
                        .map(|parsed_row| parsed_row[1].trim().to_string())
                })
                .collect();

            Response::Inventory(output, items)
        } else if "\nYou aren't carrying any items.\n\nCommand?\n" == &output {
            Response::Inventory(output, vec![])
        } else if "\nYou can't go that way.\n\nCommand?\n" == &output {
            Response::Impassable(output)
        } else {
            Response::Unknown(output)
        }
    }
}

#[derive(Debug, Clone)]
struct Moved {
    name: String,
    description: String,
    directions: Vec<Direction>,
    items: Vec<String>,
}

impl Display for Moved {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "== {} ==\n\t{}\n\tDirections: {:?}\n\tItems: {:?}",
            self.name, self.description, self.directions, self.items
        )
    }
}

fn main() {
    env_logger::init_from_env(Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn"));

    run::<TwentyFive>(false, include_str!("25_input.txt"));
}

#[cfg(test)]
mod twenty_five {
    use super::*;

    #[test]
    fn test() {
        let state = ProblemState {
            name: "test".into(),
            is_example: false,
            extra: false,
        };

        let input = TwentyFive::parse(include_str!("25_input.txt"), &state);

        assert_eq!(
            TwentyFive::part_1(&input, &state),
            Some(
                r#"


== Pressure-Sensitive Floor ==
Analyzing...

Doors here lead:
- south

A loud, robotic voice says "Analysis complete! You may proceed." and you enter the cockpit.
Santa notices your small droid, looks puzzled for a moment, realizes what has happened, and radios your ship directly.
"Oh, hello! You should be able to get in by typing 35717128 on the keypad at the main airlock."
"#.to_string()
            )
        );
    }
}
