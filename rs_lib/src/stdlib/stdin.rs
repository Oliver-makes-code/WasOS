use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/../std/stdio.ts")]
extern "C" {
    #[wasm_bindgen(js_name = "stdin$promptln")]
    pub fn promptln() -> String;
}
