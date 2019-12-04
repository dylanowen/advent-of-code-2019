use advent_of_code_2019::{run, Problem};

struct One {}

impl Problem for One {
    type Input = Vec<isize>;

    fn parse(s: &str) -> Vec<isize> {
        s.split('\n')
            .map(|mass| mass.parse::<isize>().expect("parse error"))
            .collect()
    }

    fn part_1(modules_mass: &Vec<isize>, _name: &str, _is_example: bool) -> Option<String> {
        let fuel_requirements: isize = modules_mass.iter().map(|mass| mass / 3 - 2).sum();

        Some(format!("{}", fuel_requirements))
    }

    fn part_2(modules_mass: &Vec<isize>, _name: &str, _is_example: bool) -> Option<String> {
        let fuel_requirements: isize = modules_mass
            .iter()
            .map(|mass| {
                let mut result = 0;
                let mut last_fuel = mass / 3 - 2;
                while last_fuel > 0 {
                    result += last_fuel;
                    last_fuel = last_fuel / 3 - 2;
                }

                result
            })
            .sum();

        Some(format!("{}", fuel_requirements))
    }

    fn problem_number() -> usize {
        1
    }
}

fn main() {
    run::<One>(true, "100756");
    run::<One>(false, include_str!("1_input.txt"));
}
