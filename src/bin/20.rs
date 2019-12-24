use advent_of_code_2019::coordinates::two_d::{Point, PointLike};
use advent_of_code_2019::coordinates::{two_d, Grid};
use advent_of_code_2019::example;
use advent_of_code_2019::problem::{run, Problem, ProblemState, RunFor};
use core::mem;
use env_logger::Env;
use log::Level;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter, Write};
use wasm_bindgen::__rt::std::collections::HashSet;

static START_ID: usize = encode_id('A', 'A');
static END_ID: usize = encode_id('Z', 'Z');

struct Twenty {}

#[derive(Debug, Copy, Clone, PartialEq)]
enum PortalType {
    Inward,
    Outward,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum MapBlock {
    Empty,
    Wall,
    Passage,
    Start,
    End,
    Portal(usize, Point, PortalType),
}

impl Default for MapBlock {
    fn default() -> Self {
        MapBlock::Empty
    }
}

impl Display for MapBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            MapBlock::Empty => f.write_char(' '),
            MapBlock::Wall => f.write_char('#'),
            MapBlock::Passage => f.write_char('.'),
            MapBlock::Start => f.write_char('A'),
            MapBlock::End => f.write_char('Z'),
            MapBlock::Portal(_, _, PortalType::Inward) => f.write_char('v'),
            MapBlock::Portal(_, _, PortalType::Outward) => f.write_char('^'),
        }
    }
}

impl Problem for Twenty {
    type Input = (Point, Grid<MapBlock>);
    type Extra = ();

    fn parse(s: &str, _state: &ProblemState<Self::Extra>) -> Self::Input {
        let mut grid = Grid::new_from_inclusive_range(0..=8, 0..=8);
        for (y, row) in s.split('\n').rev().enumerate() {
            for (x, c) in row.chars().enumerate() {
                grid.set(
                    x as isize,
                    y as isize,
                    match c {
                        ' ' => MapBlock::Empty,
                        '#' => MapBlock::Wall,
                        '.' => MapBlock::Passage,
                        _ => MapBlock::Portal(c as usize, Point::new(0, 0), PortalType::Inward),
                    },
                );
            }
        }

        let mut start = Point::new(0, 0);
        let mut portals = HashMap::new();
        for point in grid.indices() {
            if let MapBlock::Portal(mut c, _, _) = *grid.get_point(point) {
                for delta in &two_d::NEIGHBOR_DELTAS {
                    if let MapBlock::Passage = grid.get_point(point.add(delta)) {
                        let other_point = point.sub(delta);
                        if let MapBlock::Portal(mut other_c, _, _) = *grid.get_point(other_point) {
                            if point.x() > other_point.x() || point.y() < other_point.y() {
                                mem::swap(&mut c, &mut other_c);
                            }

                            grid.set_point(other_point, MapBlock::Empty);

                            let portal_id = encode_id(c as u8 as char, other_c as u8 as char);

                            if portal_id == START_ID {
                                grid.set_point(point, MapBlock::Start);
                                start = point;
                            } else if portal_id == END_ID {
                                grid.set_point(point, MapBlock::End);
                            } else {
                                let (portal_type, other_portal_type) = if point.x <= 2
                                    || (grid.x_max() - point.x) <= 2
                                    || point.y <= 2
                                    || (grid.y_max() - point.y) <= 2
                                {
                                    (PortalType::Outward, PortalType::Inward)
                                } else {
                                    (PortalType::Inward, PortalType::Outward)
                                };

                                grid.set_point(
                                    point,
                                    MapBlock::Portal(portal_id, point, PortalType::Inward),
                                );

                                if let Some(&other_end) = portals.get(&portal_id) {
                                    // join our portals together
                                    grid.set_point(
                                        other_end,
                                        MapBlock::Portal(portal_id, point, other_portal_type),
                                    );
                                    grid.set_point(
                                        point,
                                        MapBlock::Portal(portal_id, other_end, portal_type),
                                    );
                                } else {
                                    portals.insert(portal_id, point);
                                }
                            }
                        } else {
                            panic!("A portal should have 2 ids")
                        }
                    }
                }
            }
        }

        if log::log_enabled!(Level::Debug) {
            grid.print_bottom_up();
        }

        (start, grid)
    }

    fn part_1((start, map): &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        // start at negative 1 since we list our location in the portal, but really we start in front of it
        let min_distance =
            shortest_normal_path(*start, &mut Grid::new_from_grid_size(map), map, -1);

        Some(min_distance.to_string())
    }

    fn part_2((start, map): &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        let min_distance = shortest_recursive_path(*start, map);

        Some(min_distance.to_string())
    }

    fn problem_number() -> usize {
        20
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Searched(isize);

impl Default for Searched {
    fn default() -> Self {
        Searched(std::isize::MAX)
    }
}

fn shortest_normal_path(
    location: Point,
    searched: &mut Grid<Searched>,
    map: &Grid<MapBlock>,
    distance: isize,
) -> isize {
    log::trace!("Checking {:?} at {}", location, distance);
    searched.set_point(location, Searched(distance));

    let mut min_distance = std::isize::MAX;

    for &(mut next_location) in &location.neighbors() {
        let mut next_distance = 0;
        let found_path = match map.get_point(next_location) {
            MapBlock::Passage => {
                next_distance = distance + 1;
                true
            }
            &MapBlock::Portal(id, other_end, _) => {
                next_location = other_end;
                next_distance = distance;

                log::trace!(
                    "Might Use Portal:({}) {} -> {}",
                    decode_id(id),
                    location,
                    next_location
                );
                true
            }
            MapBlock::End => {
                min_distance = min_distance.min(distance);

                log::debug!("Found Goal: {} in {}", next_location, min_distance);

                // if we found the end, no need to continue
                false
            }
            _ => false,
        };

        if found_path && next_distance < searched.get_point(next_location).0 {
            min_distance = min_distance.min(shortest_normal_path(
                next_location,
                searched,
                map,
                next_distance,
            ))
        }
    }

    min_distance
}

fn shortest_recursive_path(start: Point, map: &Grid<MapBlock>) -> isize {
    let mut searched: Vec<Grid<Searched>> = vec![Grid::new_from_grid_size(map)];
    let mut next_breadth = HashSet::new();
    next_breadth.insert((start, 0));

    // start at negative 1 since we list our location in the portal, but really we start in front of it
    let mut distance = -1;
    while !next_breadth.is_empty() {
        log::trace!("{:?}", next_breadth);
        let last_breadth = mem::replace(&mut next_breadth, HashSet::new());

        for (location, depth) in last_breadth.into_iter() {
            log::trace!("Searching {:?} {:?}", location, depth);
            searched[depth].set_point(location, Searched(distance));

            for &(mut next_location) in &location.neighbors() {
                let mut next_depth = depth;
                let found_path = match map.get_point(next_location) {
                    MapBlock::Passage => true,
                    &MapBlock::Portal(_, _, PortalType::Outward) if depth == 0 => {
                        // leaving the recursive maze won't get us anywhere
                        false
                    }
                    &MapBlock::Portal(_, other_end, portal_type) => {
                        match portal_type {
                            PortalType::Inward => next_depth += 1,
                            PortalType::Outward => next_depth -= 1,
                        }

                        // find an actual passage from the portal
                        for &other_passage in &other_end.neighbors() {
                            if *map.get_point(other_passage) == MapBlock::Passage {
                                next_location = other_passage;
                                break;
                            }
                        }

                        log::trace!("\tPortal {:?} {:?}", next_location, next_depth);

                        true
                    }
                    MapBlock::End if next_depth == 0 => {
                        // if we found the end, no need to continue
                        return distance;
                    }
                    _ => false,
                };

                // make sure we have something to search in
                if next_depth >= searched.len() {
                    searched.push(Grid::new_from_grid_size(map));
                }

                if found_path && distance < searched[next_depth].get_point(next_location).0 {
                    log::trace!("\tNext {:?} {:?}", next_location, next_depth);
                    next_breadth.insert((next_location, next_depth));
                }
            }
        }

        distance += 1;
    }

    std::isize::MAX
}

const fn encode_id(left: char, right: char) -> usize {
    (left as usize) << 8 | (right as usize)
}

fn decode_id(id: usize) -> String {
    format!("{}{}", (id >> 8) as u8 as char, (id & 0xFF) as u8 as char)
}

fn main() {
    env_logger::init_from_env(Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn"));

    example!(Twenty;
        RunFor::Both, (), r#"         A
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       "#,
         RunFor::Part1, (), r#"                   A
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P               "#,
         RunFor::Part2, (), r#"             Z L X W       C
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     "#
    );

    run::<Twenty>((), include_str!("20_input.txt"));
}

#[cfg(test)]
mod twenty {
    use super::*;
    use advent_of_code_2019::problem::assert_solution;

    #[test]
    fn test() {
        assert_solution::<Twenty>(include_str!("20_input.txt"), (), "528", "6214");
    }
}
