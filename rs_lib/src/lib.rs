use wasm_bindgen::prelude::*;
pub mod stdlib;
use stdlib::stdout;
use stdlib::stdin;
use stdlib::stdfs;
use stdlib::stdexec;

static mut CURR_PATH: &str = "/";

#[wasm_bindgen]
pub async fn main_fn() {
  wasos_main().await;
}

#[wasm_bindgen]
pub async fn wasos_main() {
  stdout::writeln("===============".to_string());
  stdout::writeln("= WasOS 0.0.0 =".to_string());
  stdout::writeln("===============".to_string());
  stdout::writeln("".to_string());
  loop {
    unsafe {
      stdout::write(CURR_PATH.to_string());
      stdout::write(" $>".to_string());
      let input = stdin::promptln();
      let split = split_to_args(input);
      let builtin_succeeded = test_builtins(split).await;
      if !builtin_succeeded {


        stdout::writeln("Unknown command".to_string());
      }
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
      if !curr_str.is_empty() {
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

async unsafe fn test_builtins(args: Vec<String>) -> bool {
  if args[0] == "ls".to_string() {
    let files = stdfs::list_dir(CURR_PATH.to_string());
    for file in files.iter() {
      stdout::write(file.name());
      if file.isDir() {
        stdout::write("/".to_string());
      }
      stdout::writeln("".to_string());
    }
    return true
  } else if args[0] == "exit" {
    let mut code: usize = 0;
    if args.len() > 1 {
      let code_res = usize::from_str_radix(&args[1], 10);
      if code_res.is_ok() {
        code = code_res.unwrap();
      }
    }
    stdout::writeln(format!("Exiting with code {}", code.to_string()));
    stdexec::exit(code);
    return true
  } else if args[0] == "test_run_file" {
    let code = stdexec::execWasm("lib/wasos_bg.wasm".to_string()).await.as_f64().unwrap();
    stdout::writeln(format!("Program exited with code {}", code.to_string()));
    return true
  }
  false
}