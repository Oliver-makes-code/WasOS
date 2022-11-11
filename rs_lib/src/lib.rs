use wasm_bindgen::prelude::*;
pub mod stdlib;
use stdlib::stdout;
use stdlib::stdin;
use stdlib::stdfs;
use stdlib::stdexec;

static mut CURR_PATH: &str = "/";

#[wasm_bindgen]
pub fn wasos_main() {
  loop {
    unsafe {
      stdout::write(CURR_PATH.to_string());
      stdout::write(" $>".to_string());
      let input = stdin::promptln();
      let split = split_to_args(input);
      for inp in split {
        stdout::writeln(inp);
      }
      // if input == "ls".to_string() {
      //   let files = stdfs::list_dir(CURR_PATH.to_string());
      //   for file in files.iter() {
      //     stdout::write(file.name());
      //     if file.isDir() {
      //       stdout::write("/".to_string());
      //     }
      //     stdout::writeln("".to_string());
      //   }
      // }
    }
  }
}

pub fn split_to_args(string: String) -> Vec<String> {
  let mut out: Vec<String> = Vec::new();

  let mut curr_str: String = "".to_string();
  let mut i: usize = 0;
  while i < string.len() {
    let c = string.chars().nth(i).unwrap();
    if c == ' ' {
      if (!curr_str.is_empty()) {
        out.push(curr_str.clone());
      }
      curr_str = "".to_string();
    } else if c == '"' {
      let (_, cut) = string.split_at(i+1);
      for (i2, c2) in cut.chars().enumerate() {
        if c2== '"' {
          i += i2+1;
          break;
        } else {
          curr_str.push(c2);
        }
      }
      out.push(curr_str.clone());
      curr_str = "".to_string();
    } else {
      curr_str.push(c);
    }
    i += 1;
  }
  out.push(curr_str.clone());

  out
}