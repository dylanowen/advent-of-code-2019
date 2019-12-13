use advent_of_code_2019::coordinates::two_d::{Point, PointLike};
use advent_of_code_2019::coordinates::Grid;
use advent_of_code_2019::example;
use advent_of_code_2019::problem::{run, Problem, ProblemState, RunFor};
use env_logger::Env;
use num::Integer;

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

        #[allow(clippy::range_minus_one)]
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
        let (_, best_detection) = find_best_location(asteroids);
        Some(format!("{}", best_detection))
    }

    fn part_2(asteroids: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        log::trace!("{}", render_asteroids(asteroids));

        let (station, _) = find_best_location(asteroids);

        let mut targets: Vec<(Point, f64)> = asteroids
            .enumerate()
            .filter(|(point, a)| **a && !point.eq(&station))
            .map(|(point, _)| {
                // calculate the angle
                let v = point.sub(&station);

                let mut angle = (v.y() as f64).atan2(v.x() as f64) + std::f64::consts::FRAC_PI_2;

                // since up is 0 for us, normalize our angle (this could probably be more elegant)
                if angle < 0.0 {
                    angle += std::f64::consts::PI * 2.0;
                }

                (point, angle)
            })
            .collect();

        targets.sort_by(|(_, a_angle), (_, b_angle)| a_angle.partial_cmp(b_angle).unwrap());

        let mut clustered: Vec<Vec<Point>> = vec![vec![(targets[0].0)]];
        let mut last_inc = reduce(
            targets[0].0.x() - station.x(),
            targets[0].0.y() - station.y(),
        );
        let mut i = 0;
        for (other, _) in targets.iter().skip(1) {
            let inc = reduce(other.x() - station.x(), other.y() - station.y());
            if inc.x() == last_inc.x() && inc.y() == last_inc.y() {
                clustered[i].push(*other)
            } else {
                clustered[i].sort_by(|a, b| b.distance(&station).cmp(&a.distance(&station)));
                clustered.push(vec![*other]);
                i += 1;
            }
            last_inc = inc;
        }

        let mut i = 0;
        let mut last = Point::new(0, 0);
        for _ in 0..200 {
            if i > clustered.len() {
                i = 0;
            }

            last = clustered[i].pop().unwrap();

            if clustered[i].is_empty() {
                clustered.remove(i);
            } else {
                i += 1;
            }
        }

        Some(format!("{}", last.x() * 100 + last.y()))
    }

    fn problem_number() -> usize {
        10
    }
}

fn find_best_location(asteroids: &Grid<bool>) -> (Point, usize) {
    let mut best = Point::new(0, 0);
    let mut best_detection = 0;
    for (potential, _) in asteroids.enumerate().filter(|(_, a)| **a) {
        let other_asteroids = asteroids.enumerate().filter(|(point, a)| {
            **a && !(point.x() == potential.x() && point.y() == potential.y())
        });

        let mut detected = 0;
        for (other, _) in other_asteroids {
            let delta = reduce(other.x() - potential.x(), other.y() - potential.y());
            let mut next = potential.add(&delta);

            while !*asteroids.get(next.x(), next.y()) {
                next.inc(&delta);
            }

            // if we ended on our other asteroid, that means there was no obstruction in sight
            if next == other {
                detected += 1;
            }
        }

        if detected > best_detection {
            best_detection = detected;
            best = potential;
            log::trace!("{},{} detected {}", best.x(), best.y(), best_detection);
        }
    }

    (best, best_detection)
}

fn render_asteroids(grid: &Grid<bool>) -> String {
    let mut output = String::new();
    for y in grid.y_min()..grid.y_max() {
        for x in grid.x_min()..grid.x_max() {
            if *grid.get(x as isize, y as isize) {
                output.push('#');
            } else {
                output.push('.');
            }
        }
        output.push('\n');
    }

    output
}

fn reduce(mut one: isize, mut two: isize) -> (isize, isize) {
    if one == 0 {
        (0, two / two.abs())
    } else if two == 0 {
        (one / one.abs(), 0)
    } else {
        loop {
            let gcd = one.gcd(&two);
            one /= gcd;
            two /= gcd;

            if gcd == one || gcd == two || gcd == 1 {
                return (one, two);
            }
        }
    }
}

fn main() {
    env_logger::init_from_env(Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn"));

    example!(Ten;
        RunFor::Part1, (), r#".#..#
.....
#####
....#
...##"#,
    RunFor::Both, (), r#".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##"#
    );
    run::<Ten>((), include_str!("10_input.txt"));
}

#[cfg(test)]
mod ten {
    use super::*;
    use advent_of_code_2019::problem::assert_solution;

    #[test]
    fn test() {
        assert_solution::<Ten>(include_str!("10_input.txt"), (), "319", "517");
    }

    #[test]
    fn reduce_test() {
        assert_eq!(reduce(3, 2), (3, 2));
        assert_eq!(reduce(48, 18), (8, 3));
        assert_eq!(reduce(-3, 0), (-1, 0));
        assert_eq!(reduce(0, -3), (0, -1));
    }
}
