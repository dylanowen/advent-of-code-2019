use advent_of_code_2019::coordinates::Grid;
use advent_of_code_2019::cpu::{parse_program, Execution, IntCode, Memory};
use advent_of_code_2019::{example, run, Problem, ProblemState, RunFor};
use env_logger::Env;
use std::fmt::{Display, Error, Formatter};

struct Ten {}

impl Problem for Ten {
    type Input = Grid<bool>;
    type Extra = ();

    fn parse(s: &str, _state: &ProblemState<Self::Extra>) -> Self::Input {
        let raw_asteroids: Vec<Vec<bool>> = s
            .split('\n')
            .map(|row| {
                row.chars()
                    .map(|c| match c {
                        '#' => true,
                        '.' => false,
                        invalid => panic!("Invalid asteroid {}", invalid),
                    })
                    .collect()
            })
            .collect();

        let mut grid: Grid<bool> = Grid::new_with_dimensions(
            0..=raw_asteroids[0].len() as isize - 1,
            0..=raw_asteroids.len() as isize - 1,
        );

        for (y, row) in raw_asteroids.iter().enumerate() {
            for (x, asteroid) in row.iter().enumerate() {
                grid.set(x as isize, y as isize, *asteroid);
            }
        }

        grid
    }

    fn part_1(asteroids: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        let (_, _, best_detection) = find_best_location(asteroids);
        Some(format!("{}", best_detection))
    }

    fn part_2(asteroids: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        let (station_x, station_y, _) = find_best_location(asteroids);

        println!("station {},{}", station_x, station_y);

        let mut targets: Vec<(isize, isize, f64)> = asteroids
            .enumerate()
            .filter(|(x, y, a)| **a && !(*x == station_x && *y == station_y))
            .map(|(x, y, _)| {
                // calculate the angle
                let v_x = x - station_x;
                let v_y = y - station_y;

                let mut angle = (v_y as f64).atan2(v_x as f64) + std::f64::consts::FRAC_PI_2;

                // since up is 0 for us, normalize our angle (this could probably be more elegant)
                if angle < 0.0 {
                    angle += std::f64::consts::PI * 2.0;
                }

                (x, y, angle)
            })
            .collect();

        targets.sort_by(|(_, _, a_angle), (_, _, b_angle)| a_angle.partial_cmp(b_angle).unwrap());

        // cluster them into Vec<Vec<(isize, isize, f64)>> that are inline with each other.
        // loop 0..200
        // get the next cluster, find the closest
        // next

        println!("{:?}", targets);

        None
    }

    fn problem_number() -> usize {
        10
    }
}

fn find_best_location(asteroids: &Grid<bool>) -> (isize, isize, usize) {
    let mut best_x = 0;
    let mut best_y = 0;
    let mut best_detection = 0;
    for (potential_x, potential_y, _) in asteroids.enumerate().filter(|(_, _, a)| **a) {
        let other_asteroids = asteroids
            .enumerate()
            .filter(|(x, y, a)| **a && !(*x == potential_x && *y == potential_y));

        let mut detected = 0;
        for (other_x, other_y, _) in other_asteroids {
            let (inc_x, inc_y) = reduce(other_x - potential_x, other_y - potential_y);
            let mut x = potential_x + inc_x;
            let mut y = potential_y + inc_y;

            while !*asteroids.get(x, y) {
                x += inc_x;
                y += inc_y;
            }

            // if we ended on our other asteroid, that means there was no obstruction in sight
            if x == other_x && y == other_y {
                detected += 1;
            }
        }

        if detected > best_detection {
            best_detection = detected;
            best_x = potential_x;
            best_y = potential_y;
            log::trace!("{},{} detected {}", best_y, potential_y, best_detection);
        }
    }

    (best_x, best_y, best_detection)
}

fn print_asteroids(grid: &Grid<bool>) {
    for y in grid.y_min()..grid.y_max() {
        for x in grid.x_min()..grid.x_max() {
            print!(
                "{}",
                if *grid.get(x as isize, y as isize) {
                    '#'
                } else {
                    '.'
                }
            );
        }
        println!();
    }
}

fn reduce(mut one: isize, mut two: isize) -> (isize, isize) {
    if one == 0 {
        (0, two / two.abs())
    } else if two == 0 {
        (one / one.abs(), 0)
    } else {
        loop {
            let gcd = find_gcd(one, two);
            one = one / gcd;
            two = two / gcd;

            if gcd == one || gcd == two || gcd == 1 {
                return (one, two);
            }
        }
    }
}

/// https://en.wikipedia.org/wiki/Greatest_common_divisor#Euclid.27s_algorithm
fn find_gcd(mut one: isize, mut two: isize) -> isize {
    one = one.abs();
    two = two.abs();
    let mut upper = one.max(two);
    let mut lower = one.min(two);

    if lower == 0 {
        upper
    } else if upper == 0 {
        lower
    } else {
        loop {
            let remainder = upper % lower;
            if remainder == 0 {
                return lower;
            } else {
                upper = lower;
                lower = remainder;
            }
        }
    }
}

fn main() {
    env_logger::init_from_env(Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn"));

    example!(Ten;
    //        RunFor::Part1, (), r#".#..#
    //.....
    //#####
    //....#
    //...##"#,
    //        RunFor::Both, (), r#".#..##.###...#######
    //##.############..##.
    //.#.######.########.#
    //.###.#######.####.#.
    //#####.##.#.##.###.##
    //..#####..#.#########
    //####################
    //#.####....###.#.#.##
    //##.#################
    //#####.##.###..####..
    //..######..##.#######
    //####.##.####...##..#
    //.#####..#.######.###
    //##...#.##########...
    //#.##########.#######
    //.####.#.###.###.#.##
    //....##.##.###..#####
    //.#.#.###########.###
    //#.#.#.#####.####.###
    //###.##.####.##.#..##"#,
            RunFor::Part2, (), r#".#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
..#.#.....#....##"#
        );
    //run::<Ten>((), include_str!("10_input.txt"));
}

#[cfg(test)]
mod ten {
    use super::*;
    use advent_of_code_2019::assert_solution;

    #[test]
    fn test() {
        assert_solution::<Ten>(include_str!("9_input.txt"), (), "3280416268", "80210");
    }

    #[test]
    fn euclids_gcd() {
        assert_eq!(find_gcd(48, 18), 6);
        assert_eq!(find_gcd(18, 48), 6);
        assert_eq!(find_gcd(-48, 18), 6);
        assert_eq!(find_gcd(-48, -18), 6);
        assert_eq!(find_gcd(12, 0), 12);
    }

    #[test]
    fn reduce_test() {
        assert_eq!(reduce(3, 2), (3, 2));
        assert_eq!(reduce(48, 18), (8, 3));
        assert_eq!(reduce(-3, 0), (-1, 0));
        assert_eq!(reduce(0, -3), (0, -1));
    }
}
