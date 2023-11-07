use std::{
  cell::RefCell,
  fmt::{self, Debug},
  rc::Rc,
};

use crate::environment::Environment;

#[derive(Clone, PartialEq)]
pub enum Object {
  Void,
  Cond,
  Quote(Rc<Object>),
  Keyword(String),
  Operator(String),
  Float(f64),
  Integer(i64),
  Bool(bool),
  String(String),
  Symbol(String),
  Lambda(Vec<String>, Box<Object>, Rc<RefCell<Environment>>),
  List(Vec<Object>),
}

impl Debug for Object {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Object::Void => write!(f, "#nil"),
      Object::Integer(n) => write!(f, "Integer({})", n),
      Object::Float(n) => write!(f, "Float({})", n),
      Object::Bool(b) => {
        if *b {
          write!(f, "#t")
        } else {
          write!(f, "#f")
        }
      }
      Object::String(s) => write!(f, "String({})", s),
      Object::Symbol(s) => write!(f, "Symbol({})", s),
      Object::Keyword(s) => write!(f, "Keyword({})", s),
      Object::Lambda(params, body, _env) => {
        let params_str = params.join(" ");

        write!(f, "Lambda(params: ({}), body: {:?})", params_str, body)
      }
      Object::List(list) => {
        let list_str = list
          .iter()
          .map(|x| format!("{:?}", x))
          .collect::<Vec<String>>()
          .join(", ");

        write!(f, "List({})", list_str)
      }
      Object::Cond => write!(f, "Cond"),
      Object::Quote(o) => write!(f, "Quote({:?})", o),
      Object::Operator(s) => write!(f, "Operator({})", s),
    }
  }
}

impl fmt::Display for Object {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Object::Void => write!(f, "#nil"),
      Object::Integer(n) => write!(f, "{}", n),
      Object::Bool(b) => {
        if *b {
          write!(f, "#t")
        } else {
          write!(f, "#f")
        }
      }
      Object::Symbol(s) => write!(f, "{}", s),
      Object::Lambda(params, body, _env) => {
        let params_str = params.join(" ");

        write!(f, "(lambda ({}) {})", params_str, body)
      }
      Object::List(list) => {
        let list_str = list
          .iter()
          .map(|x| format!("{}", x))
          .collect::<Vec<String>>()
          .join(" ");

        write!(f, "({})", list_str)
      }
      Object::Keyword(s) => write!(f, "{}", s),
      Object::Operator(s) => write!(f, "{}", s),
      Object::Float(n) => write!(f, "{}", n),
      Object::String(s) => write!(f, "{}", s),
      Object::Quote(o) => write!(f, "'{}", o),
      Object::Cond => write!(f, "cond"),
    }
  }
}
