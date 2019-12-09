use advent_of_code_2019::cpu::{parse_program, Execution, ExecutionState, Memory};
use advent_of_code_2019::{example, run, Problem, ProblemState, RunFor};
use env_logger::Env;
use permutohedron::LexicalPermutation;

struct Seven {}

impl Problem for Seven {
    type Input = Memory;
    type Extra = ();

    fn parse(s: &str, _state: &ProblemState<Self::Extra>) -> Self::Input {
        parse_program(s)
    }

    fn part_1(program: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        let mut max_thrust = 0;
        let mut phase_settings = [0, 1, 2, 3, 4];
        loop {
            let mut output = 0;
            for phase in phase_settings.iter() {
                let mut execution: Execution =
                    Execution::new_input(program.clone(), vec![*phase, output]);

                execution.run().expect("This should always work");

                output = execution.output.pop_front().unwrap()
            }

            max_thrust = max_thrust.max(output);

            if !phase_settings.next_permutation() {
                break;
            }
        }

        Some(format!("{}", max_thrust))
    }

    fn part_2(program: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        let mut max_thrust = 0;
        let mut phase_settings = [5, 6, 7, 8, 9];
        loop {
            let mut executions =
                vec![Execution::new_input(program.clone(), vec![]); phase_settings.len()];

            for (i, phase) in phase_settings.iter().enumerate() {
                executions[i].input.push_back(*phase);
            }

            let mut output = 0;
            'feedback: loop {
                let mut state = ExecutionState::Running;
                for execution in executions.iter_mut() {
                    execution.input.push_back(output);
                    state = execution.run().expect("This should always work");
                    output = execution.output.pop_front().unwrap();
                }

                if ExecutionState::Halted == state {
                    break 'feedback;
                }
            }

            max_thrust = max_thrust.max(output);

            if !phase_settings.next_permutation() {
                break;
            }
        }

        Some(format!("{}", max_thrust))
    }

    fn problem_number() -> usize {
        7
    }
}

fn main() {
    env_logger::init_from_env(Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn"));

    example!(Seven;
        RunFor::Part1, (), "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0",
        RunFor::Part2, (), "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
    );
    run::<Seven>((), include_str!("7_input.txt"));
}

#[cfg(test)]
mod seven {
    use super::*;
    use advent_of_code_2019::assert_solution;

    #[test]
    fn test() {
        assert_solution::<Seven>(include_str!("7_input.txt"), (), "101490", "61019896");
    }
}
