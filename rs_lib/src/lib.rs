use wasm_bindgen::prelude::*;
pub mod stdlib;
use stdlib::stdout;

#[wasm_bindgen]
pub fn main() {
  stdout::writeln("Hello, world!")
}

