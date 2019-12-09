use advent_of_code_2019::cpu::{parse_program, Execution, IntCode, Memory};
use advent_of_code_2019::{run, Problem, ProblemState};
use env_logger::Env;

struct Five {}

impl Problem for Five {
    type Input = Memory;
    type Extra = ();

    fn parse(s: &str, _state: &ProblemState<Self::Extra>) -> Self::Input {
        parse_program(s)
    }

    fn part_1(program: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        execute_program(program, 1)
    }

    fn part_2(program: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        execute_program(program, 5)
    }

    fn problem_number() -> usize {
        5
    }
}

fn execute_program(program: &Memory, input: IntCode) -> Option<String> {
    let mut execution: Execution = Execution::new_input(program.to_owned(), vec![input]);

    execution.run().expect("This should always work");

    let mut output = 0;
    while output == 0 {
        output = execution.output.pop_front().unwrap()
    }

    Some(format!("{}", output))
}

fn main() {
    env_logger::init_from_env(Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn"));

    run::<Five>((), include_str!("5_input.txt"));
}

#[cfg(test)]
mod five {
    use super::*;
    use advent_of_code_2019::assert_solution;

    #[test]
    fn test() {
        assert_solution::<Five>(include_str!("5_input.txt"), (), "5044655", "7408802");
    }
}
