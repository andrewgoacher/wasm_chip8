extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

extern crate lib_chip;
use lib_chip::state::{State, delay_timer, sound_timer};
use lib_chip::memory::Memory;
use lib_chip::rom::Rom;

#[wasm_bindgen]
pub struct Chip8 {

}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {

        }
    }
}