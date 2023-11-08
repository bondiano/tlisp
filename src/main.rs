mod environment;
mod eval;
mod lexer;
mod object;
mod operators;
mod parser;
mod runtime;

use std::fs::File;
use std::io::Read;
use std::{cell::RefCell, rc::Rc};

use linefeed::{Interface, ReadResult, Signal};

const PROMPT: &str = "tlisp> ";

fn load_file(path: &str, env: &mut Rc<RefCell<environment::Environment>>) {
  let file = File::open(path);

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

  reader.set_report_signal(Signal::Break, true);
  reader.set_report_signal(Signal::Continue, true);
  reader.set_report_signal(Signal::Interrupt, true);
  reader.set_report_signal(Signal::Suspend, true);
  reader.set_report_signal(Signal::Quit, true);

  let runtime = runtime::Runtime::new();
  let mut env = Rc::new(RefCell::new(environment::Environment::new(runtime)));

  reader.set_prompt(format!("{}", PROMPT).as_ref())?;

  while let ReadResult::Input(input) = reader.read_line()? {
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
        _ if command.starts_with("load") => {
          let path = input.trim_start_matches(".load ");
          load_file(path, &mut env);
        }
        _ => {
          println!("Unknown command: {}", command);
        }
      }
      continue;
    }

    match eval::eval(input.as_ref(), &mut env) {
      Ok(result) => println!("{}", result),
      Err(e) => println!("{}", e),
    }

    reader.add_history_unique(input);
  }

  Ok(())
}
