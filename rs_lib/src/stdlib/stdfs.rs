use super::stdout;

use wasm_bindgen::{prelude::*, JsObject, convert::FromWasmAbi};

#[wasm_bindgen(module = "/../std/stdio.ts")]
extern "C" {
    #[wasm_bindgen(js_name = "stdfs$listDir")]
    pub fn list_dir(path: String) -> Box<[File]>;

    pub type File;

    #[wasm_bindgen(method, getter)]
    pub fn name(this: &File) -> String;
    #[wasm_bindgen(method, getter)]
    pub fn isDir(this: &File) -> bool;
}