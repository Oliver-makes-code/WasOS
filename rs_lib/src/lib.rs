use wasm_bindgen::prelude::*;
pub mod stdlib;
use stdlib::stdout;
use stdlib::stdin;
use stdlib::stdfs;

static mut CURR_PATH: &str = "/";

#[wasm_bindgen]
pub fn wasos_main() {
  loop {
    unsafe {
      stdout::write(CURR_PATH.to_string());
      stdout::write(" $>".to_string());
      let input = stdin::promptln();
      stdout::writeln("test 1".to_string());
      let files = stdfs::list_dir(CURR_PATH.to_string());
      stdout::writeln("test 2".to_string());
      
    }
  }
}
