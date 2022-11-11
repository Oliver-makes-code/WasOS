use std::array;
use super::stdout;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/../std/stdio.ts")]
extern "C" {
    #[wasm_bindgen(js_name = "stdfs$listDir")]
    fn list_dir_internal(path: String) -> Box<[JsValue]>;
}

pub fn list_dir(path: String) -> Vec<String> {
    stdout::writeln("test 3".to_string());
    let dir = list_dir_internal(path);
    stdout::writeln("test 4".to_string());
    let mut out: Vec<String> = Vec::new();
    for i in &*dir {
        out.push(i.as_string().expect("The function doesn't return any nulls."));
    }
    out
}