use crate::coordinates::Grid;
use crate::cpu::{Execution, IntCode};
use std::fmt;
use std::fmt::{Display, Formatter, Write};

#[derive(Debug, Clone, PartialEq)]
pub enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl Tile {
    pub fn new(tile_id: i64) -> Tile {
        match tile_id {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => panic!("Invalid tile_id"),
        }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Empty
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Tile::Empty => f.write_char(' ')?,
            Tile::Wall => f.write_char('#')?,
            Tile::Block => f.write_char('░')?,
            Tile::Paddle => f.write_char('─')?,
            Tile::Ball => f.write_char('●')?,
        }

        Ok(())
    }
}

pub fn read_output(
    execution: &mut Execution,
    screen: &mut Grid<Tile>,
) -> (IntCode, IntCode, IntCode) {
    let mut score = 0;
    let mut paddle = 0;
    let mut ball = 0;
    while !execution.output.is_empty() {
        let x = execution.expect_pop();
        let y = execution.expect_pop();
        let tile_id = execution.expect_pop();

        if x == -1 && y == 0 {
            score = tile_id;
        } else {
            let tile = Tile::new(tile_id);

            match tile {
                Tile::Paddle => paddle = x,
                Tile::Ball => ball = x,
                _ => (),
            }

            screen.set(x as isize, y as isize, tile);
        }
    }

    (score, paddle, ball)
}

mod wasm {
    use super::*;
    use crate::coordinates::canvas::render_grid;
    use crate::cpu::{parse_program, ExecutionState};
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::Clamped;
    use wasm_bindgen::{JsCast, JsValue};
    use web_sys::console;
    use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

    static PIXEL_SIZE: usize = 10;

    #[wasm_bindgen]
    pub struct ThirteenGame {
        execution: Execution,
        screen: Grid<Tile>,
        score: i64,
        image_dimensions: (u32, u32),
        image_data: Vec<u8>,
        canvas_ctx: CanvasRenderingContext2d,
    }

    #[wasm_bindgen]
    impl ThirteenGame {
        #[wasm_bindgen(constructor)]
        pub fn new(canvas: &HtmlCanvasElement) -> Result<ThirteenGame, JsValue> {
            let mut paid_program = parse_program(include_str!("../thirteen/13_input.txt")).clone();
            // pay 2 "quarters" for our game
            paid_program[0] = 2;

            let mut execution = Execution::new(paid_program);
            execution.run().map_err(|e| format!("CPU Error: {:?}", e))?;

            let screen = Grid::new_with_dimensions(0..=44, 0..=44);
            let image_dimensions = (
                (screen.width() * PIXEL_SIZE) as u32,
                (screen.height() * PIXEL_SIZE) as u32,
            );

            console::log_1(
                &format!(
                    "{},{}     {}",
                    image_dimensions.0,
                    image_dimensions.1,
                    (image_dimensions.0 * image_dimensions.1 * 4)
                )
                .into(),
            );

            let image_data = vec![0; (image_dimensions.0 * image_dimensions.1 * 4) as usize];

            canvas.set_width(image_dimensions.0);
            canvas.set_height(image_dimensions.1);

            let canvas_ctx = canvas
                .get_context("2d")?
                .expect("We need a canvas context")
                .dyn_into()?;

            Ok(ThirteenGame {
                execution,
                screen,
                score: 0,
                image_dimensions,
                image_data,
                canvas_ctx,
            })
        }

        pub fn step(&mut self, input: isize) -> Result<ExecutionState, JsValue> {
            self.execution.input.push_back(input as i64);
            let state = self
                .execution
                .run()
                .map_err(|e| format!("CPU Error: {:?}", e))?;

            let (score, _, _) = read_output(&mut self.execution, &mut self.screen);

            if score > 0 {
                self.score = score;
            }

            Ok(state)
        }

        pub fn render_game(&mut self) -> Result<(), JsValue> {
            render_grid(PIXEL_SIZE, &mut self.image_data, &self.screen, |tile| {
                Some(match tile {
                    Tile::Empty => [0xf9, 0xf4, 0xef, 0xff],
                    Tile::Wall => [0x71, 0x60, 0x40, 0xff],
                    Tile::Block => [0x8c, 0x78, 0x51, 0xff],
                    Tile::Paddle => [0x02, 0x08, 0x26, 0xff],
                    Tile::Ball => [0xf2, 0x50, 0x42, 0xff],
                })
            });

            let image_data = ImageData::new_with_u8_clamped_array_and_sh(
                Clamped(&mut self.image_data),
                self.image_dimensions.0,
                self.image_dimensions.1,
            )?;

            self.canvas_ctx.put_image_data(&image_data, 0.0, 0.0)
        }

        pub fn score(&self) -> isize {
            self.score as isize
        }
    }
}
