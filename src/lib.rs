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

pub fn run<P: Problem>(is_example: bool, name: &str, raw_input: &str) {
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

//pub fn run<P: Problem, I>(inputs: I) where I: FixedSizeArray(bool, &str)] + Sized  {
//// give our output a random color
//let random_color_index = (rand::random::<u8>() % 5) + 2;
//let color = format!("\u{001B}[3{}m", random_color_index);
//
//for (is_example, raw_input) in inputs {
//let input = P::parse(raw_input);
//let name = if !is_example { "Problem" } else { "Example" };
//
//
//benchmark(
//&color,
//format!("{}.1 {}", P::problem_number(), name),
//|| P::part_1(&input, is_example),
//);
//benchmark(
//&color,
//format!("{}.2 {}", P::problem_number(), name),
//|| P::part_2(&input, is_example),
//);
//}
//}

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
