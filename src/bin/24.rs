use advent_of_code_2019::coordinates::two_d::PointLike;
use advent_of_code_2019::coordinates::Grid;
use advent_of_code_2019::example;
use advent_of_code_2019::problem::{run, Problem, ProblemState, RunFor};
use env_logger::Env;
use log::Level;
use std::fmt;
use std::fmt::{Debug, Display, Formatter, Write};
use wasm_bindgen::__rt::std::collections::{HashSet, VecDeque};

struct TwentyFour {}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Tile {
    Empty,
    Bug,
    Recursive,
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Empty
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Empty => f.write_char('.'),
            Tile::Bug => f.write_char('#'),
            Tile::Recursive => f.write_char('?'),
        }
    }
}

impl Problem for TwentyFour {
    type Input = Grid<Tile>;
    type Extra = usize;

    fn parse(s: &str, _state: &ProblemState<Self::Extra>) -> Self::Input {
        let mut grid = Grid::new_from_range(0..5, 0..5);
        for (y, row) in s.split('\n').enumerate() {
            for (x, c) in row.chars().enumerate() {
                grid.set(
                    x as isize,
                    y as isize,
                    match c {
                        '.' => Tile::Empty,
                        '#' => Tile::Bug,
                        _ => panic!("Invalid tile: {}", c),
                    },
                );
            }
        }

        grid
    }

    fn part_1(initial: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        let mut past = HashSet::new();

        let mut last_eris = initial.clone();
        loop {
            let next = simulate_eris(&last_eris);
            if past.contains(&last_eris) {
                break;
            } else {
                past.insert(last_eris);
            }
            last_eris = next;
        }

        let biodiversity = calculate_biodiversity(&last_eris);

        Some(biodiversity.to_string())
    }

    fn part_2(initial: &Self::Input, state: &ProblemState<Self::Extra>) -> Option<String> {
        let mut recursive_initial = initial.clone();
        setup_recursive(&mut recursive_initial);

        let mut world = VecDeque::new();
        world.push_back(recursive_initial);

        for minute in 0..state.extra {
            if log::log_enabled!(Level::Debug) {
                println!("Minute: {}", minute);
                for eris in world.iter() {
                    eris.print_top_down();
                    println!();
                }
            }

            world = simulate_recursive_eris(world);
        }

        if log::log_enabled!(Level::Debug) {
            println!("Minute: {}", state.extra);
            for eris in world.iter() {
                eris.print_top_down();
                println!();
            }
        }

        let total_bugs: usize = world
            .iter()
            .map(|eris| {
                eris.enumerate()
                    .filter(|(_, &tile)| Tile::Bug == tile)
                    .count()
            })
            .sum();

        Some(total_bugs.to_string())
    }

    fn problem_number() -> usize {
        24
    }
}

fn simulate_recursive_eris(mut last_erises: VecDeque<Grid<Tile>>) -> VecDeque<Grid<Tile>> {
    // add some surrounding recursive grids
    last_erises.push_front(new_recursive());
    last_erises.push_back(new_recursive());

    // create our next world state
    let mut next = VecDeque::with_capacity(last_erises.len());

    for i in 0..last_erises.len() {
        // just pretend we're our own parent since they're both empty anyway
        let last_parent = if i == 0 {
            &last_erises[i]
        } else {
            &last_erises[i - 1]
        };
        let last_eris = &last_erises[i];
        // just pretend we're our own child since they're both empty anyway
        let last_child = if i == last_erises.len() - 1 {
            &last_erises[i]
        } else {
            &last_erises[i + 1]
        };

        log::debug!("Checking depth {}", i);

        let (next_eris, has_bugs) = simulate_window(last_parent, last_eris, last_child);
        // if we're at the start or end we dont have to add bugs
        if has_bugs || (i != 0 && i != last_erises.len() - 1) {
            next.push_back(next_eris)
        }
    }

    next
}

fn simulate_window(
    last_parent: &Grid<Tile>,
    last_eris: &Grid<Tile>,
    last_child: &Grid<Tile>,
) -> (Grid<Tile>, bool) {
    let mut next_eris = new_recursive();
    let mut has_bugs = false;

    for (point, &tile) in last_eris
        .enumerate()
        .filter(|(_, &tile)| tile != Tile::Recursive)
    {
        let bug_sum = sum_bugs(&point, last_parent, last_eris, last_child);

        has_bugs = update_tile(point, tile, bug_sum, &mut next_eris) || has_bugs;
    }

    (next_eris, has_bugs)
}

fn sum_bugs<P>(
    point: &P,
    last_parent: &Grid<Tile>,
    last_eris: &Grid<Tile>,
    last_child: &Grid<Tile>,
) -> usize
where
    P: PointLike + Copy + Debug,
{
    log::debug!("Checkin {:?}", point);

    let mut bug_sum = 0;
    for &neighbor in &point.neighbors() {
        match last_eris.get_point(neighbor) {
            Tile::Bug => bug_sum += 1,
            Tile::Recursive => {
                // check parents
                let maybe_parent_tile = if neighbor.x() == -1 {
                    Some(last_parent.get(1, 2))
                } else if neighbor.x() == 5 {
                    Some(last_parent.get(3, 2))
                } else if neighbor.y() == -1 {
                    Some(last_parent.get(2, 1))
                } else if neighbor.y() == 5 {
                    Some(last_parent.get(2, 3))
                } else {
                    None
                };

                log::debug!("\tParent {:?}", maybe_parent_tile);

                if let Some(&parent_tile) = maybe_parent_tile {
                    // we found a parent so see if it's a bug
                    if Tile::Bug == parent_tile {
                        bug_sum += 1;
                    }
                } else {
                    // check children
                    let children_range: Vec<(isize, isize)> = if point.y() == 3 {
                        (0..5).map(|x| (x, 4)).collect()
                    } else if point.x() == 3 {
                        (0..5).map(|y| (4, y)).collect()
                    } else if point.y() == 1 {
                        (0..5).map(|x| (x, 0)).collect()
                    } else if point.x() == 1 {
                        (0..5).map(|y| (0, y)).collect()
                    } else {
                        panic!("Invalid Recursive Location: {:?}", point)
                    };

                    bug_sum += children_range
                        .iter()
                        .filter(|&&point| *last_child.get_point(point) == Tile::Bug)
                        .count();
                }
            }
            _ => (),
        }
    }

    log::debug!("\t {:?} has {} bugs", point, bug_sum);

    bug_sum
}

fn new_recursive() -> Grid<Tile> {
    let mut new = Grid::new_from_inclusive_range(-1..=5, -1..=5);
    setup_recursive(&mut new);
    new
}

fn setup_recursive(grid: &mut Grid<Tile>) {
    grid.set(2, 2, Tile::Recursive);

    for &y in &[-1, 5] {
        for x in -1..=5 {
            grid.set(x, y, Tile::Recursive);
        }
    }

    for &x in &[-1, 5] {
        for y in 0..=4 {
            grid.set(x, y, Tile::Recursive);
        }
    }
}

fn simulate_eris(last_eris: &Grid<Tile>) -> Grid<Tile> {
    let mut next = Grid::new_from_grid_size(last_eris);

    for (point, &tile, neighbors) in last_eris
        .enumerate()
        .map(|(point, tile)| (point, tile, point.neighbors()))
    {
        let bug_sum = neighbors
            .iter()
            .filter(|&&neighbor| *last_eris.get_point(neighbor) == Tile::Bug)
            .count();

        update_tile(point, tile, bug_sum, &mut next);
    }

    next
}

fn update_tile<P>(point: P, tile: Tile, bug_sum: usize, next: &mut Grid<Tile>) -> bool
where
    P: PointLike + Copy,
{
    match tile {
        Tile::Empty if bug_sum == 1 || bug_sum == 2 => {
            next.set_point(point, Tile::Bug);
            true
        }
        Tile::Bug if bug_sum != 1 => {
            next.set_point(point, Tile::Empty);
            false
        }
        _ => {
            next.set_point(point, tile);
            tile == Tile::Bug
        }
    }
}

fn calculate_biodiversity(eris: &Grid<Tile>) -> usize {
    let mut power = 1;
    let mut result = 0;
    for (_, &tile) in eris.enumerate() {
        if Tile::Bug == tile {
            result += power;
        }
        power *= 2;
    }

    result
}

fn main() {
    env_logger::init_from_env(Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn"));

    example!(TwentyFour;
        RunFor::Both, 10, r#"....#
#..#.
#..##
..#..
#...."#
    );

    run::<TwentyFour>(200, include_str!("24_input.txt"));
}

#[cfg(test)]
mod twenty_four {
    use super::*;
    use advent_of_code_2019::problem::assert_solution;

    #[test]
    fn test() {
        assert_solution::<TwentyFour>(include_str!("24_input.txt"), 200, "2130474", "1923");
    }
}
