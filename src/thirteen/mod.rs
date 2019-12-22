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
    use crate::coordinates::CanvasPixel;
    use crate::cpu::{parse_program, ExecutionState};
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::Clamped;
    use wasm_bindgen::__rt::std::collections::VecDeque;
    use wasm_bindgen::{JsCast, JsValue};
    use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

    #[wasm_bindgen]
    pub struct ThirteenGame {
        execution: Execution,
        screen: Grid<Tile>,
        my_winning_game: VecDeque<IntCode>,
        score: i64,
        image_data: Vec<u8>,
        canvas_ctx: CanvasRenderingContext2d,
    }

    #[wasm_bindgen]
    impl ThirteenGame {
        #[wasm_bindgen(constructor)]
        pub fn new(
            canvas: &HtmlCanvasElement,
            custom_program: Option<String>,
            load_winning_game: bool,
        ) -> Result<ThirteenGame, JsValue> {
            let program = custom_program
                .as_ref()
                .map(|x| &**x)
                .unwrap_or(include_str!("../thirteen/13_input.txt"));
            let mut paid_program = parse_program(program);
            // pay 2 "quarters" for our game
            paid_program[0] = 2;

            let mut execution = Execution::new(paid_program);
            execution.run().map_err(|e| format!("CPU Error: {:?}", e))?;

            let mut screen = Grid::new_from_inclusive_range(0..=44, 0..=44);

            // pre-load our screen output before calculating our canvas size
            read_output(&mut execution, &mut screen);

            let winning_game_data = if load_winning_game {
                VecDeque::from(parse_program(include_str!(
                    "../thirteen/13_perfect_game.txt"
                )))
            } else {
                VecDeque::new()
            };

            let image_data = screen.build_img_data();

            canvas.set_width(screen.canvas_width() as u32);
            canvas.set_height(screen.canvas_height() as u32);

            let canvas_ctx = canvas
                .get_context("2d")?
                .expect("We need a canvas context")
                .dyn_into::<CanvasRenderingContext2d>()?;

            let mut game = ThirteenGame {
                execution,
                screen,
                my_winning_game: winning_game_data,
                score: 0,
                image_data,
                canvas_ctx,
            };

            // render once, so there is something to see
            game.render()?;

            Ok(game)
        }

        pub fn step(&mut self, user_input: isize) -> Result<ExecutionState, JsValue> {
            let input = if let Some(auto_input) = self.my_winning_game.pop_front() {
                auto_input
            } else {
                user_input as i64
            };

            self.execution.input.push_back(input as i64);
            let state = self
                .execution
                .run()
                .map_err(|e| format!("CPU Error: {:?}", e))?;

            let (score, _, _) = read_output(&mut self.execution, &mut self.screen);

            if score > 0 {
                self.score = score;
            }

            self.render()?;

            Ok(state)
        }

        fn render(&mut self) -> Result<(), JsValue> {
            self.screen.render(&mut self.image_data);

            let image_data = ImageData::new_with_u8_clamped_array_and_sh(
                Clamped(&mut self.image_data),
                self.screen.canvas_width() as u32,
                self.screen.canvas_height() as u32,
            )?;

            self.canvas_ctx.put_image_data(&image_data, 0.0, 0.0)
        }

        pub fn score(&self) -> isize {
            self.score as isize
        }
    }

    const PIXEL_WIDTH: usize = 10;
    const PIXEL_HEIGHT: usize = 10;

    #[allow(clippy::unreadable_literal)]
    static EMPTY_TILE: [u32; PIXEL_WIDTH * PIXEL_HEIGHT] = [0xf9f4efff; PIXEL_WIDTH * PIXEL_HEIGHT];
    #[allow(clippy::unreadable_literal)]
    static WALL_TILE: [u32; PIXEL_WIDTH * PIXEL_HEIGHT] = [0x716040ff; PIXEL_WIDTH * PIXEL_HEIGHT];
    #[rustfmt::skip]
    #[allow(clippy::unreadable_literal)]
    static BLOCK_TILE: [u32; PIXEL_WIDTH * PIXEL_HEIGHT] = [
        0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff,
        0xf9f4efff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0xf9f4efff,
        0xf9f4efff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0xf9f4efff,
        0xf9f4efff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0xf9f4efff,
        0xf9f4efff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0xf9f4efff,
        0xf9f4efff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0xf9f4efff,
        0xf9f4efff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0xf9f4efff,
        0xf9f4efff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0xf9f4efff,
        0xf9f4efff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0x8c7851ff, 0xf9f4efff,
        0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff,
    ];
    #[rustfmt::skip]
    #[allow(clippy::unreadable_literal)]
    static PADDLE_TILE: [u32; PIXEL_WIDTH * PIXEL_HEIGHT] = [
        0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff,
        0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff,
        0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff,
        0x020826ff, 0x020826ff, 0x020826ff, 0x020826ff, 0x020826ff, 0x020826ff, 0x020826ff, 0x020826ff, 0x020826ff, 0x020826ff,
        0x020826ff, 0x020826ff, 0x020826ff, 0x020826ff, 0x020826ff, 0x020826ff, 0x020826ff, 0x020826ff, 0x020826ff, 0x020826ff,
        0x020826ff, 0x020826ff, 0x020826ff, 0x020826ff, 0x020826ff, 0x020826ff, 0x020826ff, 0x020826ff, 0x020826ff, 0x020826ff,
        0x020826ff, 0x020826ff, 0x020826ff, 0x020826ff, 0x020826ff, 0x020826ff, 0x020826ff, 0x020826ff, 0x020826ff, 0x020826ff,
        0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff,
        0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff,
        0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff,
    ];
    #[rustfmt::skip]
    #[allow(clippy::unreadable_literal)]
    static BALL_TILE: [u32; PIXEL_WIDTH * PIXEL_HEIGHT] = [
        0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff,
        0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf25042ff, 0xf25042ff, 0xf25042ff, 0xf25042ff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff,
        0xf9f4efff, 0xf9f4efff, 0xf25042ff, 0xf25042ff, 0xf25042ff, 0xf25042ff, 0xf25042ff, 0xf25042ff, 0xf9f4efff, 0xf9f4efff,
        0xf9f4efff, 0xf25042ff, 0xf25042ff, 0xf25042ff, 0xf25042ff, 0xf25042ff, 0xf25042ff, 0xf25042ff, 0xf25042ff, 0xf9f4efff,
        0xf9f4efff, 0xf25042ff, 0xf25042ff, 0xf25042ff, 0xf25042ff, 0xf25042ff, 0xf25042ff, 0xf25042ff, 0xf25042ff, 0xf9f4efff,
        0xf9f4efff, 0xf25042ff, 0xf25042ff, 0xf25042ff, 0xf25042ff, 0xf25042ff, 0xf25042ff, 0xf25042ff, 0xf25042ff, 0xf9f4efff,
        0xf9f4efff, 0xf25042ff, 0xf25042ff, 0xf25042ff, 0xf25042ff, 0xf25042ff, 0xf25042ff, 0xf25042ff, 0xf25042ff, 0xf9f4efff,
        0xf9f4efff, 0xf9f4efff, 0xf25042ff, 0xf25042ff, 0xf25042ff, 0xf25042ff, 0xf25042ff, 0xf25042ff, 0xf9f4efff, 0xf9f4efff,
        0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf25042ff, 0xf25042ff, 0xf25042ff, 0xf25042ff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff,
        0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff, 0xf9f4efff,
    ];

    impl CanvasPixel for Tile {
        fn render(&self) -> &[u32] {
            match *self {
                Tile::Empty => &EMPTY_TILE,
                Tile::Wall => &WALL_TILE,
                Tile::Block => &BLOCK_TILE,
                Tile::Paddle => &PADDLE_TILE,
                Tile::Ball => &BALL_TILE,
            }
        }

        fn width() -> usize {
            PIXEL_WIDTH
        }

        fn height() -> usize {
            PIXEL_HEIGHT
        }
    }
}
