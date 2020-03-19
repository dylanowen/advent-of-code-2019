use advent_of_code_2019::coordinates::two_d::{Point, PointLike};
use advent_of_code_2019::coordinates::Grid;
use advent_of_code_2019::example;
use advent_of_code_2019::problem::{run, Problem, ProblemState, RunFor};
use env_logger::Env;
use log::Level;
use std::collections::{HashSet, LinkedList};
use std::fmt::{Display, Formatter, Write};
use std::{fmt, mem};
use wasm_bindgen::__rt::std::collections::HashMap;

struct Eighteen {}

static START_KEY: char = '@';

#[derive(Debug, Copy, Clone, PartialEq)]
enum MapBlock {
    OpenPassage,
    Wall,
    Entrance,
    Door(char),
    Key(char),
}

impl MapBlock {
    fn new(c: char) -> MapBlock {
        match c {
            '.' => MapBlock::OpenPassage,
            '#' => MapBlock::Wall,
            '@' => MapBlock::Entrance,
            _ => {
                if c.is_uppercase() {
                    MapBlock::Door(c.to_ascii_lowercase())
                } else {
                    MapBlock::Key(c)
                }
            }
        }
    }
}

impl Default for MapBlock {
    fn default() -> Self {
        MapBlock::OpenPassage
    }
}

impl Display for MapBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MapBlock::OpenPassage => f.write_char(' ')?,
            MapBlock::Wall => f.write_char('#')?,
            MapBlock::Entrance => f.write_char('@')?,
            &MapBlock::Door(c) => f.write_char(c.to_ascii_uppercase())?,
            &MapBlock::Key(c) => f.write_char(c)?,
        }

        Ok(())
    }
}

impl Problem for Eighteen {
    type Input = Grid<MapBlock>;
    type Extra = ();

    fn parse(s: &str, _state: &ProblemState<Self::Extra>) -> Self::Input {
        let mut map = Grid::new_with_dimensions(0..=10, 0..=10);
        for (y, row) in s.split('\n').rev().enumerate() {
            for (x, c) in row.chars().enumerate() {
                map.set(x as isize, y as isize, MapBlock::new(c));
            }
        }
        map
    }

    fn part_1(map: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        let mut keys = HashMap::new();
        //let mut doors = vec![];
        let mut entrance: Point = Point::new(0, 0);
        for (location, block) in map.enumerate() {
            match block {
                MapBlock::Entrance => entrance = location,
                //&MapBlock::Door(c) => doors.push((c, location)),
                &MapBlock::Key(c) => {
                    keys.insert(c, location);
                }
                _ => (),
            }
        }

        let shortest_path = find_optimized_distance(&entrance, keys, map);

        Some(shortest_path.to_string())
    }

    fn part_2(map: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        None
    }

    fn problem_number() -> usize {
        18
    }
}

fn find_optimized_distance(
    entrance: &Point,
    keys: HashMap<char, Point>,
    map: &<Eighteen as Problem>::Input,
) -> usize {
    let key_distances = find_all_key_distances(entrance, &keys, map);

    let mut needed_keys = keys.keys().cloned().collect();
    find_optimized_distance_inner(
        START_KEY,
        &mut needed_keys,
        &key_distances,
        0,
        std::usize::MAX,
    )
    //find_optimized_distance_inner(entrance, &mut HashSet::new(), &mut keys, map, 0)
}

fn find_optimized_distance_inner(
    from_key: char,
    needed_keys: &mut HashSet<char>,
    key_distances: &HashMap<char, HashMap<char, KeyDistance>>,
    distance: usize,
    current_best: usize,
) -> usize {
    log::debug!("Searching Key {}", from_key);

    if needed_keys.len() == 0 {
        distance
    } else if current_best < distance {
        std::usize::MAX
    } else {
        let mut min_distance = std::usize::MAX;

        // check whether we need the key AND if we can reach it
        let mut valid_keys: Vec<(&char, &KeyDistance)> = key_distances
            .get(&from_key)
            .expect("Our key should be in the map")
            .iter()
            .filter(|(k, d)| needed_keys.contains(k) && d.can_pass(needed_keys))
            .collect();

        valid_keys.sort_by(|(_, left), (_, right)| left.distance.cmp(&right.distance));

        valid_keys.truncate(2);
        //println!("{} valid keys {:?}", from_key, valid_keys);

        for (&key, key_distance) in valid_keys.iter() {
            needed_keys.remove(&key);

            let next_distance = distance + key_distance.distance;

            let found_distance = find_optimized_distance_inner(
                key,
                needed_keys,
                key_distances,
                next_distance,
                min_distance,
            );
            min_distance = min_distance.min(found_distance);

            needed_keys.insert(key);
        }

        min_distance
    }
}

#[derive(Debug, Clone, PartialEq)]
struct KeyDistance {
    distance: usize,
    blocking_doors: HashSet<char>,
}

impl KeyDistance {
    fn can_pass(&self, needed_keys: &HashSet<char>) -> bool {
        !self
            .blocking_doors
            .iter()
            .any(|door| needed_keys.contains(door))
    }
}

fn find_all_key_distances(
    start: &Point,
    keys: &HashMap<char, Point>,
    map: &<Eighteen as Problem>::Input,
) -> HashMap<char, HashMap<char, KeyDistance>> {
    let mut key_distances = HashMap::new();
    for (&key, location) in keys {
        let distances = find_key_distances(location, map);

        key_distances.insert(key, distances);
    }
    key_distances.insert(START_KEY, find_key_distances(start, map));

    key_distances
}

fn find_key_distances(
    start: &Point,
    map: &<Eighteen as Problem>::Input,
) -> HashMap<char, KeyDistance> {
    let mut search_space: Grid<Option<usize>> =
        Grid::new_with_dimensions(0..=map.width() as isize, 0..=map.height() as isize);

    search_space.set(start.x(), start.y(), Some(0));

    find_key_distances_inner(start, &mut HashSet::new(), &mut search_space, map, 0)
        .expect("We always expect to be able to find paths to other keys")
}
fn find_key_distances_inner(
    location: &Point,
    blocking_doors: &mut HashSet<char>,
    search_space: &mut Grid<Option<usize>>,
    map: &<Eighteen as Problem>::Input,
    distance: usize,
) -> Option<HashMap<char, KeyDistance>> {
    let mut key_distances = None;
    for step in [(1, 0), (0, 1), (-1, 0), (0, -1)].iter() {
        let next_location = location.add(step);
        let next_distance = distance + 1;
        mem::drop(distance);

        let block = map.get(next_location.x(), next_location.y());
        if *block != MapBlock::Wall
            && is_current_path_shorter(&next_location, search_space, next_distance)
        {
            log::trace!("Searching: {:?}", next_location);

            match block {
                &MapBlock::Door(key) => {
                    blocking_doors.insert(key);
                    log::debug!(
                        "Added Door {} {:?}",
                        key.to_ascii_uppercase(),
                        next_location
                    );

                    key_distances = merge_key_distances(
                        key_distances,
                        continue_down_path(
                            &next_location,
                            blocking_doors,
                            search_space,
                            map,
                            next_distance,
                        ),
                    );

                    blocking_doors.remove(&key);
                }
                block => {
                    // we can move through anything else

                    // if we have a key, add it to our results
                    if let &MapBlock::Key(key) = block {
                        log::debug!("Found Key {} Blocked By: {:?}", key, blocking_doors);
                        key_distances = Some(add_create_key_distance(
                            key_distances,
                            key,
                            KeyDistance {
                                distance: next_distance,
                                blocking_doors: blocking_doors.clone(),
                            },
                        ));
                    }

                    key_distances = merge_key_distances(
                        key_distances,
                        continue_down_path(
                            &next_location,
                            blocking_doors,
                            search_space,
                            map,
                            next_distance,
                        ),
                    );
                }
            }
        }
    }

    key_distances
}

fn merge_key_distances(
    maybe_key_distances: Option<HashMap<char, KeyDistance>>,
    maybe_merged: Option<HashMap<char, KeyDistance>>,
) -> Option<HashMap<char, KeyDistance>> {
    //    println!("{:?}", maybe_key_distances);
    //    println!("{:?}", maybe_merged);

    match (maybe_key_distances, maybe_merged) {
        (Some(mut key_distances), Some(merged)) => {
            for (merge_key, merge_distance) in merged.into_iter() {
                key_distances = add_key_distance(key_distances, merge_key, merge_distance);
            }

            Some(key_distances)
        }
        (found @ Some(_), None) | (None, found @ Some(_)) => found,
        (None, None) => None,
    }
}

fn add_create_key_distance(
    maybe_key_distances: Option<HashMap<char, KeyDistance>>,
    key: char,
    distance: KeyDistance,
) -> HashMap<char, KeyDistance> {
    let mut key_distances = maybe_key_distances.unwrap_or_else(|| HashMap::new());

    add_key_distance(key_distances, key, distance)
}

fn add_key_distance(
    mut key_distances: HashMap<char, KeyDistance>,
    key: char,
    distance: KeyDistance,
) -> HashMap<char, KeyDistance> {
    if let Some(existing) = key_distances.get(&key) {
        if existing.distance > distance.distance {
            key_distances.insert(key, distance);
        }
    } else {
        key_distances.insert(key, distance);
    }

    key_distances
}

fn continue_down_path(
    location: &Point,
    blocking_doors: &mut HashSet<char>,
    search_space: &mut Grid<Option<usize>>,
    map: &<Eighteen as Problem>::Input,
    distance: usize,
) -> Option<HashMap<char, KeyDistance>> {
    let good_path = match search_space.get(location.x(), location.y()) {
        None => true,
        &Some(been_here) if been_here > distance => true,
        _ => false,
    };

    if good_path {
        search_space.set(location.x(), location.y(), Some(distance));
        find_key_distances_inner(&location, blocking_doors, search_space, map, distance)
    } else {
        None
    }
}

fn is_current_path_shorter(
    location: &Point,
    search_space: &mut Grid<Option<usize>>,
    distance: usize,
) -> bool {
    match search_space.get(location.x(), location.y()) {
        None => true,
        &Some(been_here) if been_here > distance => true,
        _ => false,
    }
}

fn main() {
    env_logger::init_from_env(Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn"));

    example!(Eighteen;
        RunFor::Part1, (), r#"########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################"#,
        RunFor::Part1, (), r#"########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################"#
    //    RunFor::Part1, (), r#"#################
    //#i.G..c...e..H.p#
    //########.########
    //#j.A..b...f..D.o#
    //########@########
    //#k.E..a...g..B.n#
    //########.########
    //#l.F..d...h..C.m#
    //#################"#,
    //    RunFor::Part1, (), r#"########################
    //#@..............ac.GI.b#
    //###d#e#f################
    //###A#B#C################
    //###g#h#i################
    //########################"#
            );

    run::<Eighteen>((), include_str!("18_input.txt"));
}

#[cfg(test)]
mod eighteen {
    use super::*;
    use advent_of_code_2019::problem::assert_solution;

    #[test]
    fn test() {
        assert_solution::<Fifteen>(include_str!("18_input.txt"), (), "266", "274");
    }
}

fn find_optimized_distance_inner_old(
    start: &Point,
    owned_keys: &mut HashSet<char>,
    needed_keys: &mut HashMap<char, Point>,
    map: &<Eighteen as Problem>::Input,
    distance: usize,
) -> usize {
    //    println!("startn\n{:?}", owned_keys);
    //    println!("{:?}", needed_keys);

    if needed_keys.is_empty() {
        distance
    } else {
        let mut min_distance = std::usize::MAX;
        for (key, key_distance) in find_key_distances_old(start, owned_keys, needed_keys, map) {
            //            println!("{} {}", key, key_distance);

            let key_location = needed_keys
                .remove(&key)
                .expect("We should always need the key we found");
            owned_keys.insert(key);
            let next_distance = distance + key_distance;

            let found_distance = find_optimized_distance_inner_old(
                &key_location,
                owned_keys,
                needed_keys,
                map,
                next_distance,
            );
            min_distance = min_distance.min(found_distance);

            owned_keys.remove(&key);
            needed_keys.insert(key, key_location);
        }

        min_distance
    }
}

fn find_key_distances_old(
    start: &Point,
    owned_keys: &HashSet<char>,
    needed_keys: &HashMap<char, Point>,
    map: &<Eighteen as Problem>::Input,
) -> HashMap<char, usize> {
    let mut search_space: Grid<Option<usize>> =
        Grid::new_with_dimensions(0..=map.width() as isize, 0..=map.height() as isize);

    search_space.set(start.x(), start.y(), Some(0));

    find_key_distances_inner_old(start, owned_keys, &mut search_space, map, 0);

    needed_keys
        .iter()
        .filter_map(|(&k, &p)| search_space.get(p.x(), p.y()).map(|distance| (k, distance)))
        .collect()
}

fn find_key_distances_inner_old(
    location: &Point,
    owned_keys: &HashSet<char>,
    search_space: &mut Grid<Option<usize>>,
    map: &<Eighteen as Problem>::Input,
    distance: usize,
) {
    for step in [(1, 0), (0, 1), (-1, 0), (0, -1)].iter() {
        let next_location = location.add(step);
        let next_distance = distance + 1;
        mem::drop(distance);
        //        println!(
        //            "{},{} {:?} {:?}",
        //            next_location.x(),
        //            next_location.y(),
        //            map.get(next_location.x(), next_location.y()),
        //            search_space.get(next_location.x(), next_location.y())
        //        );

        let good_path = match map.get(next_location.x(), next_location.y()) {
            MapBlock::Wall => false,
            MapBlock::Door(key) => {
                // if we have a key, we can move through this door, so check our path distance
                owned_keys.contains(key)
                    && is_current_path_shorter(&next_location, search_space, next_distance)
            }
            _ => {
                // we can move through anything else
                is_current_path_shorter(&next_location, search_space, next_distance)
            }
        };

        if good_path {
            search_space.set(next_location.x(), next_location.y(), Some(next_distance));
            find_key_distances_inner_old(
                &next_location,
                owned_keys,
                search_space,
                map,
                next_distance,
            );
        }
    }
}
