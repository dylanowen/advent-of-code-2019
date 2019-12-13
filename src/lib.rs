use wasm_bindgen::prelude::*;

pub mod problem;

pub mod coordinates;
pub mod cpu;

pub mod thirteen;

#[wasm_bindgen]
pub fn init() {
    console_error_panic_hook::set_once();
}
