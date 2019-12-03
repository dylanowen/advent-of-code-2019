use std::time::Instant;

pub mod cpu;

pub trait Problem {
    type Input;

    fn parse(s: &str) -> Self::Input;
    fn part_1(input: &Self::Input, is_example: bool) -> Option<String>;
    fn part_2(input: &Self::Input, is_example: bool) -> Option<String>;

    fn problem_number() -> usize;
}

pub fn run<P: Problem>(is_example: bool, raw_input: &str) {
    let input = P::parse(raw_input);
    let name = if !is_example { "Problem" } else { "Example" };

    // give our output a random color
    let random_color_index = (rand::random::<u8>() % 5) + 2;
    let color = format!("\u{001B}[3{}m", random_color_index);

    benchmark(
        &color,
        format!("{}.1 {}", P::problem_number(), name),
        || P::part_1(&input, is_example),
    );
    benchmark(
        &color,
        format!("{}.2 {}", P::problem_number(), name),
        || P::part_2(&input, is_example),
    );
}

fn benchmark<C>(color: &str, name: String, runner: C)
where
    C: Fn() -> Option<String>,
{
    //println!("{}{}:\u{001B}[0m", color, name);

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
