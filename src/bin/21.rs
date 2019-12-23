use advent_of_code_2019::cpu::{parse_program, Execution, IntCode, Memory};
use advent_of_code_2019::problem::{run, Problem, ProblemState};
use env_logger::Env;
use log::Level;
use std::collections::VecDeque;
use std::convert::TryFrom;
use std::mem;

struct TwentyOne {}

impl Problem for TwentyOne {
    type Input = Memory;
    type Extra = ();

    fn parse(s: &str, _state: &ProblemState<Self::Extra>) -> Self::Input {
        parse_program(s)
    }

    fn part_1(program: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        run_springdroid(
            r#"
            OR A T
            AND B T
            AND C T
            NOT T T
            AND D T
            OR T J
            WALK
                "#,
            program,
        )
    }

    fn part_2(program: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        run_springdroid(
            r#"
            OR A T
            AND B T
            AND C T
            NOT T T
            AND D T
            OR T J

            AND H T
            OR E T
            AND T J
            RUN
                "#,
            program,
        )
    }

    fn problem_number() -> usize {
        21
    }
}

fn run_springdroid(script: &str, program: &Memory) -> Option<String> {
    let mut spring_script = compile_spring_script(script);

    let mut execution = Execution::new(program.clone());
    execution.input.append(&mut spring_script);

    execution.run().expect("The program should work");

    process_output(&mut execution).map(|out| out.to_string())
}

fn process_output(execution: &mut Execution) -> Option<IntCode> {
    let mut damage = None;

    let mut output = String::with_capacity(execution.output.len());
    for out in mem::replace(&mut execution.output, VecDeque::new()) {
        match u8::try_from(out).ok().map(|n| n as char) {
            Some(ascii) => {
                if log::log_enabled!(Level::Info) {
                    output.push(ascii)
                }
            }
            None => damage = Some(out),
        }
    }

    log::info!("{}", output);

    damage
}

fn compile_spring_script(s: &str) -> VecDeque<IntCode> {
    s.trim()
        .split('\n')
        .filter_map(|mut row| {
            row = row.trim();
            if row.is_empty() {
                None
            } else {
                Some(format!("{}\n", row))
            }
        })
        .flat_map(|s| s.chars().map(|c| (c as IntCode)).collect::<Vec<_>>())
        .collect()
}

fn main() {
    env_logger::init_from_env(Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn"));

    run::<TwentyOne>((), include_str!("21_input.txt"));
}

#[cfg(test)]
mod twenty_one {
    use super::*;
    use advent_of_code_2019::problem::assert_solution;

    #[test]
    fn test() {
        assert_solution::<TwentyOne>(include_str!("21_input.txt"), (), "19352493", "1141896219");
    }
}
