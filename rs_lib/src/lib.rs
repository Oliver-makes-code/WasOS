use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn test() -> String {
  owo().to_string()
}

#[wasm_bindgen(module = "/../std/std.ts")]
extern "C" {
  fn owo() -> String;
}