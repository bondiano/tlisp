mod environment;
mod eval;
mod lexer;
mod object;
mod operators;
mod parser;
mod runtime;

use std::cell::RefCell;
use std::rc::Rc;

pub fn tlisp_eval(input: &str) -> Result<String, String> {
  let runtime = runtime::Runtime::new();
  let mut env = Rc::new(RefCell::new(environment::Environment::new(runtime)));

  match eval::eval(input.as_ref(), &mut env) {
    Ok(object) => Ok(format!("{}", object)),
    Err(e) => Err(format!("{}", e)),
  }
}
