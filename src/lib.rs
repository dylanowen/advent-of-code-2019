use std::time::Instant;

pub mod coordinates;
pub mod cpu;

pub trait Problem {
    type Input;

    fn parse(s: &str) -> Self::Input;
    fn part_1(input: &Self::Input, name: &str, is_example: bool) -> Option<String>;
    fn part_2(input: &Self::Input, name: &str, is_example: bool) -> Option<String>;

    fn problem_number() -> usize;
}

pub fn run<P: Problem>(is_example: bool, raw_input: &str) {
    run_with_name::<P>(is_example, "", raw_input)
}

pub fn run_with_name<P: Problem>(is_example: bool, name: &str, raw_input: &str) {
    let input = P::parse(raw_input);
    let problem_type = if !is_example { "Problem" } else { "Example" };
    let full_name = &*format!("{}.1 {} {}", P::problem_number(), problem_type, name);

    // give our output a random color
    let random_color_index = (rand::random::<u8>() % 5) + 2;
    let color = format!("\u{001B}[3{}m", random_color_index);

    benchmark(&color, full_name, || {
        P::part_1(&input, full_name, is_example)
    });
    benchmark(&color, full_name, || {
        P::part_2(&input, full_name, is_example)
    });
}

/// Can be used to run the same type of problem with multiple inputs and unique names
#[macro_export]
macro_rules! run {
    ( $problem:ty; $is_example:expr, $( $input:expr ),+ ) => {
        let mut count = 1;
        $(
            $crate::run_with_name::<$problem>($is_example, &*count.to_string(), $input);
            count += 1;
        )*
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
            "{}{}:\u{001B}[0m {:01}.{:03}s",
            color,
            name,
            elapsed.as_secs(),
            elapsed.subsec_millis()
        );

        println!("{}", result);
    }
}
