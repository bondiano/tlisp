mod environment;
mod eval;
mod lexer;
mod object;
mod parser;

use std::fs::File;
use std::io::Read;
use std::{cell::RefCell, rc::Rc};

use linefeed::{Interface, ReadResult};

const PROMPT: &str = "tlisp> ";

fn load_prelude(env: &mut Rc<RefCell<environment::Environment>>) {
  let file = File::open("prelude.tl");

  match file {
    Ok(mut f) => {
      let mut prelude = String::new();
      f.read_to_string(&mut prelude).expect("Failed to read file");

      prelude = format!("({})", prelude);

      eval::eval(&prelude, env).unwrap();
    }
    Err(e) => {
      println!("Error while loading prelude: {}", e);
    }
  }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let reader = Interface::new(PROMPT).unwrap();
  let mut env = Rc::new(RefCell::new(environment::Environment::new()));

  load_prelude(&mut env);

  reader.set_prompt(format!("{}", PROMPT).as_ref()).unwrap();

  while let ReadResult::Input(input) = reader.read_line().unwrap() {
    if input.eq("exit") {
      break;
    }

    if input.is_empty() {
      continue;
    }

    if input.starts_with(".") {
      let command = input.trim_start_matches(".");
      match command {
        "env" => {
          println!("{}", &env.borrow());
        }
        _ => {
          println!("Unknown command: {}", command);
        }
      }
      continue;
    }

    match eval::eval(input.as_ref(), &mut env) {
      Ok(value) => {
        println!("{}", value);
      },
      Err(e) => {
        println!("{}", e);
      }
    }
  }

  Ok(())
}
