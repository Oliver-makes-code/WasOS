use super::stdout;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/../std/stdio.ts")]
extern "C" {
    #[wasm_bindgen(js_name = "stdfs$listDir")]
    fn list_dir_internal(path: String) -> Box<[JsValue]>;
}

pub fn list_dir(path: String) -> Vec<String> {
    let dir = list_dir_internal(path);
    let mut out: Vec<String> = Vec::new();
    for i in &*dir {
        out.push(i.as_string().expect("The function doesn't return any nulls."));
    }
    out
}