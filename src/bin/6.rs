use advent_of_code_2019::example;
use advent_of_code_2019::problem::{run, Problem, ProblemState, RunFor};
use env_logger::Env;
use std::collections::{HashMap, VecDeque};

struct Six {}

impl Problem for Six {
    type Input = HashMap<String, String>;
    type Extra = ();

    fn parse(s: &str, _state: &ProblemState<Self::Extra>) -> Self::Input {
        s.split('\n')
            .map(|row| {
                let orbit = row.split(')').collect::<Vec<&str>>();
                (orbit[1].to_string(), orbit[0].to_string())
            })
            .collect()
    }

    fn part_1(orbits: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        let mut count = 0;
        for (_orbiter, orbitee) in orbits.iter() {
            let mut maybe_current = Some(orbitee);
            while let Some(current) = maybe_current {
                maybe_current = orbits.get(current);
                count += 1;
            }
        }

        Some(format!("{}", count))
    }

    fn part_2(orbits: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        let build_chain = |start: String| {
            let mut chain = VecDeque::new();
            let mut maybe_current = Some(start);
            while let Some(current) = maybe_current {
                maybe_current = orbits.get(&current).cloned();
                chain.push_front(current);
            }

            chain
        };

        let mut you_chain: VecDeque<String> = build_chain("YOU".into());
        let mut san_chain: VecDeque<String> = build_chain("SAN".into());

        // drop matching orbits
        while you_chain.front() == san_chain.front() {
            you_chain.pop_front();
            san_chain.pop_front();
        }

        // drop our own locations
        you_chain.pop_back();
        san_chain.pop_back();

        log::debug!("{:?}", you_chain);
        log::debug!("{:?}", san_chain);

        Some(format!("{}", you_chain.len() + san_chain.len()))
    }

    fn problem_number() -> usize {
        6
    }
}

fn main() {
    env_logger::init_from_env(Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn"));

    example!(Six;
        RunFor::Part1, (), "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L",
        RunFor::Part2, (), "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN"
    );
    run::<Six>((), include_str!("6_input.txt"));
}

#[cfg(test)]
mod six {
    use super::*;
    use advent_of_code_2019::problem::assert_solution;

    #[test]
    fn test() {
        assert_solution::<Six>(include_str!("6_input.txt"), (), "294191", "424");
    }
}
