use wasm_bindgen::prelude::*;
pub mod stdlib;
use stdlib::stdout;
use stdlib::stdin;

#[wasm_bindgen]
pub fn main() {
  stdout::write("/ $");
  let input = stdin::promptln();
  stdout::writeln(&input)
}
