use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/../std/stdio.ts")]
extern "C" {
    #[wasm_bindgen(js_name = "std$write")]
    pub fn write(str: &str);
    #[wasm_bindgen(js_name = "std$writeln")]
    pub fn writeln(str: &str);
}