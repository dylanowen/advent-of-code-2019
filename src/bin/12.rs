use advent_of_code_2019::{example, run, Problem, ProblemState, RunFor};
use env_logger::Env;
use num::Integer;
use regex::Regex;

struct Twelve {}

trait System {
    fn step(&mut self);
}

#[derive(Debug, Clone)]
struct Moons {
    x: Vec<Axis>,
    y: Vec<Axis>,
    z: Vec<Axis>,
}

impl Moons {
    fn energy(&self) -> isize {
        (0..self.x.len())
            .map(|i| self.potential_energy(i) * self.kinetic_energy(i))
            .sum()
    }

    fn potential_energy(&self, moon: usize) -> isize {
        self.x[moon].location.abs() + self.y[moon].location.abs() + self.z[moon].location.abs()
    }

    fn kinetic_energy(&self, moon: usize) -> isize {
        self.x[moon].velocity.abs() + self.y[moon].velocity.abs() + self.z[moon].velocity.abs()
    }

    fn push(&mut self, (x, y, z): (isize, isize, isize)) {
        self.x.push(Axis {
            location: x,
            velocity: 0,
        });
        self.y.push(Axis {
            location: y,
            velocity: 0,
        });
        self.z.push(Axis {
            location: z,
            velocity: 0,
        });
    }
}

impl System for Moons {
    fn step(&mut self) {
        self.x.step();
        self.y.step();
        self.z.step();
    }
}

impl System for Vec<Axis> {
    fn step(&mut self) {
        // apply velocity
        for i in 0..self.len() {
            let mut velocity = 0;
            for j in 0..self.len() {
                if i != j {
                    velocity += velocity_delta(self[i].location, self[j].location)
                }
            }
            self[i].velocity += velocity;
        }

        // update the location
        for moon in self.iter_mut() {
            moon.location += moon.velocity;
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Axis {
    location: isize,
    velocity: isize,
}

fn velocity_delta(a: isize, b: isize) -> isize {
    if a < b {
        1
    } else if a > b {
        -1
    } else {
        0
    }
}

impl Problem for Twelve {
    type Input = Moons;
    type Extra = usize;

    fn parse(s: &str, _state: &ProblemState<Self::Extra>) -> Self::Input {
        let re = Regex::new(r"<x=(-?\d+),\sy=(-?\d+),\sz=(-?\d+)>").unwrap();

        s.split('\n')
            .map(|row| {
                let parsed_row = re.captures(row).unwrap();
                let x = parsed_row[1].parse::<isize>().expect("Parse error");
                let y = parsed_row[2].parse::<isize>().expect("Parse error");
                let z = parsed_row[3].parse::<isize>().expect("Parse error");

                (x, y, z)
            })
            .fold(
                Moons {
                    x: vec![],
                    y: vec![],
                    z: vec![],
                },
                |mut moons, location| {
                    moons.push(location);

                    moons
                },
            )
    }

    fn part_1(original_moons: &Self::Input, state: &ProblemState<Self::Extra>) -> Option<String> {
        let mut moons = original_moons.clone();

        log::trace!("{:?}", moons);
        for _ in 0..state.extra {
            moons.step();
        }

        //let total_energy: isize = moons.iter().map(Moon::energy).sum();

        Some(format!("{}", moons.energy()))
    }

    fn part_2(original_moons: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        let mut moons = original_moons.clone();

        let mut x_cycle = 0;
        let mut y_cycle = 0;
        let mut z_cycle = 0;

        let mut step: usize = 0;
        while x_cycle == 0 || y_cycle == 0 || z_cycle == 0 {
            moons.step();
            step += 1;

            if x_cycle == 0 && original_moons.x == moons.x {
                x_cycle = step
            }
            if y_cycle == 0 && original_moons.y == moons.y {
                y_cycle = step
            }
            if z_cycle == 0 && original_moons.z == moons.z {
                z_cycle = step
            }
        }

        let steps = x_cycle.lcm(&y_cycle).lcm(&z_cycle);

        Some(format!("{}", steps))
    }

    fn problem_number() -> usize {
        11
    }
}

fn main() {
    env_logger::init_from_env(Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn"));

    example!(Twelve;
        RunFor::Both, 10, r#"<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>"#,
        RunFor::Both, 100, r#"<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>"#);
    run::<Twelve>(1000, include_str!("12_input.txt"));
}

#[cfg(test)]
mod twelve {
    use super::*;
    use advent_of_code_2019::assert_solution;

    #[test]
    fn test() {
        assert_solution::<Twelve>(
            include_str!("12_input.txt"),
            1000,
            "14809",
            "282270365571288",
        );
    }
}
