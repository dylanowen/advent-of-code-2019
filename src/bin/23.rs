use advent_of_code_2019::cpu::{parse_program, Execution, IntCode, Memory};
use advent_of_code_2019::problem::{run, Problem, ProblemState};
use env_logger::Env;
use std::collections::VecDeque;
use std::mem;

struct TwentyThree {}

#[derive(Debug, Clone)]
struct Computer {
    address: IntCode,
    net_buffer: VecDeque<IntCode>,
    execution: Execution,
}

impl Computer {
    fn new(address: IntCode, program: &[IntCode]) -> Computer {
        let mut execution = Execution::new(program.to_owned());
        execution.input.push_back(address);

        Computer {
            address,
            net_buffer: VecDeque::new(),
            execution,
        }
    }

    fn run(&mut self) -> VecDeque<IntCode> {
        if self.net_buffer.is_empty() {
            // let the computer know we don't have anything for it
            self.execution.input.push_back(-1);
        } else {
            self.execution.input.append(&mut self.net_buffer);
        }

        self.execution.run().expect("The network never stops");

        let mut output = VecDeque::new();
        mem::swap(&mut output, &mut self.execution.output);

        output
    }
}

impl Problem for TwentyThree {
    type Input = Memory;
    type Extra = ();

    fn parse(s: &str, _state: &ProblemState<Self::Extra>) -> Self::Input {
        parse_program(s)
    }

    fn part_1(program: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        let mut network = Vec::with_capacity(50);

        // assign addresses and start our computers
        for i in 0..50 {
            network.push(Computer::new(i as IntCode, program));
        }

        let final_y;
        'outer: loop {
            for i in 0..50 {
                let mut output = network[i].run();
                while !output.is_empty() {
                    let destination = output.pop_front().unwrap();
                    let x = output.pop_front().unwrap();
                    let y = output.pop_front().unwrap();

                    if destination == 255 {
                        final_y = y;
                        break 'outer;
                    } else {
                        network[destination as usize].execution.input.push_back(x);
                        network[destination as usize].execution.input.push_back(y);
                    }
                }
            }
        }

        Some(final_y.to_string())
    }

    fn part_2(program: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        let mut network = Vec::with_capacity(50);

        // assign addresses and start our computers
        for i in 0..50 {
            network.push(Computer::new(i as IntCode, program));
        }

        let mut last_nat_y = 0;
        let mut nat = (0, 0);
        let final_y;
        'outer: loop {
            let mut idle = true;
            for i in 0..50 {
                let mut output = network[i].run();
                while !output.is_empty() {
                    idle = false;
                    let destination = output.pop_front().unwrap();
                    let x = output.pop_front().unwrap();
                    let y = output.pop_front().unwrap();

                    if destination == 255 {
                        nat = (x, y);
                    } else {
                        network[destination as usize].execution.input.push_back(x);
                        network[destination as usize].execution.input.push_back(y);
                    }
                }
            }

            if idle {
                if last_nat_y == nat.1 {
                    final_y = nat.1;
                    break 'outer;
                }
                network[0].execution.input.push_back(nat.0);
                network[0].execution.input.push_back(nat.1);
                last_nat_y = nat.1;
            }
        }

        Some(final_y.to_string())
    }

    fn problem_number() -> usize {
        23
    }
}

fn main() {
    env_logger::init_from_env(Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn"));

    run::<TwentyThree>((), include_str!("23_input.txt"));
}

#[cfg(test)]
mod twenty_three {
    use super::*;
    use advent_of_code_2019::problem::assert_solution;

    #[test]
    fn test() {
        assert_solution::<TwentyThree>(include_str!("23_input.txt"), (), "18966", "14370");
    }
}
