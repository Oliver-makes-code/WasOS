use wasm_bindgen::prelude::*;

pub mod stdout;
pub mod stdin;
pub mod stdfs;
pub mod stdexec;

#[wasm_bindgen(module = "/../std/std.ts")]
extern "C" {
    pub fn getRealPath(path: String) -> String;
    pub fn pathExists(path: String) -> bool;
    pub fn fileExists(path: String) -> bool;
    pub fn getCurrPath() -> String;
    pub fn setCurrPath(val: String);
}

#[wasm_bindgen(module = "/../std/stdenv.ts")]
extern "C" {
    pub fn getEnv(key: String) -> String;
    pub fn setEnv(key: String, value: String);
}