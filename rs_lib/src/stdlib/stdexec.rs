use super::stdout;

use wasm_bindgen::{prelude::*, JsObject, convert::FromWasmAbi};

#[wasm_bindgen(module = "/../std/stdexec.ts")]
extern "C" {
    pub fn exit(code: isize);
}