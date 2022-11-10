use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/../std/stdio.ts")]
extern "C" {
    #[wasm_bindgen(js_name = "stdout$write")]
    pub fn write(str: &str);
    #[wasm_bindgen(js_name = "stdout$writeln")]
    pub fn writeln(str: &str);
}
