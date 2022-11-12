use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/../std/stdexec.ts")]
extern "C" {
    pub fn exit(code: usize);
    pub async fn execWex(path: String) -> JsValue;
}