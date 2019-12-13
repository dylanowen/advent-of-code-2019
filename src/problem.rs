use std::time::Instant;

pub struct ProblemState<T: Sized + Default> {
    pub name: String,
    pub is_example: bool,
    pub extra: T,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RunFor {
    Part1,
    Part2,
    Both,
}

pub trait Problem {
    type Input;
    type Extra: Sized + Default;

    fn parse(s: &str, state: &ProblemState<Self::Extra>) -> Self::Input;
    fn part_1(input: &Self::Input, state: &ProblemState<Self::Extra>) -> Option<String>;
    fn part_2(input: &Self::Input, state: &ProblemState<Self::Extra>) -> Option<String>;

    fn problem_number() -> usize;
}

pub fn run<P: Problem>(extra: P::Extra, input: &str) {
    run_with_name::<P>(" ", false, RunFor::Both, extra, input)
}

/// Can be used to run examples for the problem
#[macro_export]
macro_rules! example {
    ( $problem:ty; $( $run_for:expr, $extra:expr, $input:expr ),+ ) => {
        let mut count = 1;
        $(
            $crate::problem::run_with_name::<$problem>(&*count.to_string(), true, $run_for, $extra, $input);
            count += 1;
        )*
    };
}

pub fn assert_solution<P: Problem>(s: &str, extra: P::Extra, expected_1: &str, expected_2: &str) {
    let state = ProblemState {
        name: "test".into(),
        is_example: false,
        extra,
    };

    let input = P::parse(s, &state);

    assert_eq!(P::part_1(&input, &state), Some(expected_1.to_string()));
    assert_eq!(P::part_2(&input, &state), Some(expected_2.to_string()));
}

pub fn run_with_name<P: Problem>(
    name: &str,
    is_example: bool,
    run_for: RunFor,
    extra: P::Extra,
    raw_input: &str,
) {
    let problem_type = if !is_example { "Problem" } else { "Example" };

    let mut state = ProblemState {
        name: format!("{} {} {}", P::problem_number(), problem_type, name),
        is_example,
        extra,
    };

    let input = P::parse(raw_input, &state);

    // give our output a random color
    let random_color_index = (rand::random::<u8>() % 5) + 2;
    let color = format!("\u{001B}[3{}m", random_color_index);

    if run_for != RunFor::Part2 {
        state.name = format!("{}.1 {} {}", P::problem_number(), problem_type, name);

        benchmark(&color, &*state.name, || P::part_1(&input, &state));
    }
    if run_for != RunFor::Part1 {
        state.name = format!("{}.2 {} {}", P::problem_number(), problem_type, name);

        benchmark(&color, &*state.name, || P::part_2(&input, &state));
    }
}

fn benchmark<C>(color: &str, name: &str, runner: C)
where
    C: Fn() -> Option<String>,
{
    let now = Instant::now();
    let maybe_result = runner();
    let elapsed = now.elapsed();

    if let Some(result) = maybe_result {
        println!(
            "{}{}:\u{001B}[0m {:2}.{:09}s",
            color,
            name,
            elapsed.as_secs(),
            elapsed.subsec_nanos()
        );

        println!("{}", result);
    }
}
