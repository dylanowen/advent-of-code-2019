use advent_of_code_2019::cpu::{parse_program, Execution, IntCode, Memory};
use advent_of_code_2019::{example, run, Problem, ProblemState, RunFor};
use env_logger::Env;

struct Nine {}

impl Problem for Nine {
    type Input = Memory;
    type Extra = ();

    fn parse(s: &str, _state: &ProblemState<Self::Extra>) -> Self::Input {
        parse_program(s)
    }

    fn part_1(program: &Self::Input, state: &ProblemState<Self::Extra>) -> Option<String> {
        let input = if state.is_example { vec![] } else { vec![1] };
        execute_program(program, input)
    }

    fn part_2(program: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        execute_program(program, vec![2])
    }

    fn problem_number() -> usize {
        9
    }
}

fn execute_program(program: &[IntCode], input: Vec<IntCode>) -> Option<String> {
    let mut execution: Execution = Execution::new_input(program.to_owned(), input);

    execution.run().expect("This should always work");

    log::debug!("{:?}", execution.output);

    Some(format!("{}", execution.output.pop_front().unwrap()))
}

fn main() {
    env_logger::init_from_env(Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn"));

    example!(Nine;
        RunFor::Part1, (), "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99",
        RunFor::Part1, (), "1102,34915192,34915192,7,4,7,99,0",
        RunFor::Part1, (), "104,1125899906842624,99"
    );
    run::<Nine>((), include_str!("9_input.txt"));
}

#[cfg(test)]
mod nine {
    use super::*;
    use advent_of_code_2019::assert_solution;

    #[test]
    fn test() {
        assert_solution::<Nine>(include_str!("9_input.txt"), (), "3280416268", "80210");
    }
}
