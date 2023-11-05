use std::{cell::RefCell, fmt, rc::Rc};

use crate::environment::Environment;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
  Void,
  If,
  Keyword(String),
  BinaryOp(String),
  Float(f64),
  Integer(i64),
  Bool(bool),
  String(String),
  Symbol(String),
  Lambda(Vec<String>, Vec<Object>, Rc<RefCell<Environment>>),
  List(Vec<Object>),
}

impl fmt::Display for Object {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Object::Void => write!(f, "Void"),
      Object::Integer(n) => write!(f, "{}", n),
      Object::Bool(b) => write!(f, "{}", b),
      Object::Symbol(s) => write!(f, "{}", s),
      Object::Lambda(params, body, _env) => {
        let body_str = body
          .iter()
          .map(|x| format!("{}", x))
          .collect::<Vec<String>>()
          .join(" ");
        let params_str = params.join(" ");
        write!(f, "(lambda ({}) ({}))", params_str, body_str)?;

        Ok(())
      }
      Object::List(list) => {
        let list_str = list
          .iter()
          .map(|x| format!("{}", x))
          .collect::<Vec<String>>()
          .join(" ");

        write!(f, "({})", list_str)?;

        Ok(())
      }
      Object::Keyword(s) => write!(f, "{}", s),
      Object::BinaryOp(s) => write!(f, "{}", s),
      Object::Float(n) => write!(f, "{}", n),
      Object::String(s) => write!(f, "{}", s),
      Object::If => write!(f, "if"),
    }
  }
}
