use advent_of_code_2019::cpu::{parse_program, Execution, Program};
use advent_of_code_2019::{run, Problem};
use env_logger::Env;

struct Two {}

impl Problem for Two {
    type Input = Program;

    fn parse(s: &str) -> Program {
        parse_program(s)
    }

    fn part_1(input: &Program, _name: &str, is_example: bool) -> Option<String> {
        let mut execution: Execution = input.clone().into();

        if !is_example {
            execution[1] = 12;
            execution[2] = 2;
        }

        execution.run().expect("This should always work");

        Some(format!("{}", execution[0]))
    }

    fn part_2(input: &Program, _name: &str, is_example: bool) -> Option<String> {
        if is_example {
            // we don't have a good example for this problem
            return None;
        }

        let goal = if is_example { 1202 } else { 19_690_720 };

        let mut result = 0;
        'outer: for noun in 0..=99 {
            for verb in 0..=99 {
                let mut execution: Execution = input.clone().into();
                execution[1] = noun;
                execution[2] = verb;

                execution.run().expect("This should always work");
                if goal == execution[0] {
                    result = 100 * noun + verb;
                    break 'outer;
                }
            }
        }

        Some(format!("{}", result))
    }

    fn problem_number() -> usize {
        2
    }
}

fn main() {
    env_logger::init_from_env(Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn"));

    run::<Two>(true, "", "1,9,10,3,2,3,11,0,99,30,40,50");
    run::<Two>(false, "", include_str!("2_input.txt"));
}
