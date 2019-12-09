use advent_of_code_2019::{example, run, Problem, ProblemState, RunFor};
use env_logger::Env;

struct Eight {}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Pixel {
    White,
    Black,
    Transparent,
}

impl Problem for Eight {
    type Input = Vec<Vec<Pixel>>;
    type Extra = (usize, usize);

    fn parse(s: &str, state: &ProblemState<Self::Extra>) -> Self::Input {
        let (width, height) = state.extra;
        let raw_image: Vec<Pixel> = s
            .chars()
            .map(|c| match c.to_digit(10).unwrap() {
                0 => Pixel::White,
                1 => Pixel::Black,
                2 => Pixel::Transparent,
                p => panic!("Invalid pixel {}", p),
            })
            .collect();

        raw_image
            .chunks(width * height)
            .map(|c| c.to_vec())
            .collect()
    }

    fn part_1(image_data: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        let mut fewest_zeros = std::usize::MAX;
        let mut multiplied = 0;
        for layer in image_data.iter() {
            let (zeros, ones, twos) = layer.iter().fold((0, 0, 0), |(zero, one, two), p| match p {
                Pixel::White => (zero + 1, one, two),
                Pixel::Black => (zero, one + 1, two),
                Pixel::Transparent => (zero, one, two + 1),
            });
            if zeros < fewest_zeros {
                fewest_zeros = zeros;
                multiplied = ones * twos;
            }
        }

        Some(format!("{}", multiplied))
    }

    fn part_2(image_data: &Self::Input, state: &ProblemState<Self::Extra>) -> Option<String> {
        let (width, height) = state.extra;

        let mut image_data = image_data.clone();
        let mut image = image_data.pop().unwrap();
        image_data.reverse();

        log::trace!("\n{}\n", render_image(&image, width, height));

        for layer in image_data.iter() {
            log::trace!("layer\n{}", render_image(layer, width, height));
            for (i, pixel) in layer.iter().enumerate() {
                match pixel {
                    Pixel::White | Pixel::Black => {
                        image[i] = *pixel;
                    }
                    Pixel::Transparent => {
                        // Transparent, so do nothing
                    }
                }
            }
            log::trace!("{}\n", render_image(&image, width, height));
        }

        Some(render_image(&image, width, height))
    }

    fn problem_number() -> usize {
        8
    }
}

fn render_image(image: &[Pixel], width: usize, height: usize) -> String {
    // WIDTH + 1 for \newlines
    let mut rendered_image = String::with_capacity((width + 1) * height);
    for y in 0..height {
        for x in 0..width {
            let pixel = match image[(width * y) + x] {
                Pixel::White => ' ',
                Pixel::Black => '*',
                Pixel::Transparent => '_',
            };
            rendered_image.push(pixel)
        }
        rendered_image.push('\n');
    }

    rendered_image
}

fn main() {
    env_logger::init_from_env(Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn"));

    example!(Eight; RunFor::Part2, (2, 2), "0222112222120000");
    run::<Eight>((25, 6), include_str!("8_input.txt"));
}

#[cfg(test)]
mod eight {
    use super::*;
    use advent_of_code_2019::assert_solution;

    #[test]
    fn test() {
        assert_solution::<Eight>(
            include_str!("8_input.txt"),
            (25, 6),
            "2064",
            r#"*  *  **  *  * ****  **  
* *  *  * *  *    * *  * 
**   *  * *  *   *  *  * 
* *  **** *  *  *   **** 
* *  *  * *  * *    *  * 
*  * *  *  **  **** *  * 
"#,
        );
    }
}
