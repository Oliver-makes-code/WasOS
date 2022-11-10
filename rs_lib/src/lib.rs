use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn test(a: &str) -> String {
  a.to_string()
}